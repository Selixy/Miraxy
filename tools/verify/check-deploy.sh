#!/usr/bin/env bash
# Vérifie que l'outil de déploiement est cohérent avec le code. Lancé par le crochet pre-commit.
# Un code qui lit une variable MIRAXY_* absente de deploy/env.spec (ou l'inverse) fait échouer.
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEPLOY_DIR="$REPO_DIR/deploy"
source "$DEPLOY_DIR/lib/answers.sh"

errors=0
fail() { printf '  ✗ %s\n' "$*" >&2; errors=$((errors + 1)); }

# 1. Syntaxe de tous les scripts shell du déploiement et des outils.
for file in "$DEPLOY_DIR"/setup.sh "$DEPLOY_DIR"/lib/*.sh "$DEPLOY_DIR"/steps/*.sh \
            "$REPO_DIR"/tools/*/*.sh "$REPO_DIR"/.githooks/*; do
  bash -n "$file" 2>/dev/null || fail "erreur de syntaxe : ${file#"$REPO_DIR"/}"
done

# 2. Variables MIRAXY_* : code <-> env.spec.
in_code="$(grep -rhoE 'MIRAXY_[A-Z0-9_]+' "$REPO_DIR"/server/src "$REPO_DIR"/common/src "$REPO_DIR"/daemon/src 2>/dev/null | grep -v '^MIRAXY_TEST_' | sort -u)"
in_spec="$(grep -v '^#' "$DEPLOY_DIR/env.spec" | cut -d'|' -f1 | grep '^MIRAXY_' | sort -u || true)"
for name in $(comm -23 <(echo "$in_code") <(echo "$in_spec")); do
  fail "$name est lue par le code mais absente de deploy/env.spec"
done
for name in $(comm -13 <(echo "$in_code") <(echo "$in_spec")); do
  fail "$name est déclarée dans deploy/env.spec mais le code ne la lit plus"
done

# 3. Clés @CLÉ@ des modèles : toutes connues de questions.conf ou dérivées.
known=" $(question_keys | tr '\n' ' ') $DERIVED_KEYS "
for template in "$DEPLOY_DIR"/templates/*.in; do
  for key in $(grep -oE '@[A-Z_]+@' "$template" | tr -d '@' | sort -u); do
    [[ "$known" == *" $key "* ]] || fail "$(basename "$template") utilise @$key@, inconnu de questions.conf"
  done
done
# ... et chaque question sert dans au moins un modèle.
for key in $(question_keys); do
  grep -qs "@$key@" "$DEPLOY_DIR"/templates/*.in "$DEPLOY_DIR"/steps/*.sh "$DEPLOY_DIR"/lib/*.sh \
    || grep -qs "\$$key\|\${$key" "$DEPLOY_DIR"/steps/*.sh "$DEPLOY_DIR"/lib/*.sh \
    || fail "la question $key n'est utilisée nulle part"
done

# 3b. Format de questions.conf : 4 champs séparés par « | » (une regex ne peut donc pas en contenir).
while IFS= read -r line; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  [[ "$(tr -cd '|' <<<"$line")" == "|||" ]] || fail "questions.conf : ligne mal formée (3 « | » attendus) : $line"
done < "$DEPLOY_DIR/questions.conf"

# 4. Étapes numérotées sans doublon.
dups="$(ls "$DEPLOY_DIR"/steps | cut -c1-2 | sort | uniq -d)"
[[ -z "$dups" ]] || fail "numéros d'étape en double : $dups"

# 5. shellcheck, s'il est installé.
if command -v shellcheck >/dev/null; then
  shellcheck -x -S warning "$DEPLOY_DIR"/setup.sh "$DEPLOY_DIR"/lib/*.sh "$DEPLOY_DIR"/steps/*.sh "$REPO_DIR"/tools/*/*.sh \
    || fail "shellcheck a des remarques"
else
  echo "  (shellcheck absent : contrôle ignoré)"
fi

if [[ "$errors" -gt 0 ]]; then
  echo "Déploiement incohérent : $errors problème(s). Mettre à jour deploy/ (voir docs/deploy.md)." >&2
  exit 1
fi
echo "Déploiement cohérent."
