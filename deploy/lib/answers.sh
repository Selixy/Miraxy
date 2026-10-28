# Chargement, saisie et enregistrement des réponses. À sourcer.
# Dépend de : ui.sh. Variables attendues : DEPLOY_DIR.

ANSWERS_FILE="$DEPLOY_DIR/answers.env"

# Clés calculées à partir des réponses (utilisables comme @CLÉ@ dans les modèles).
DERIVED_KEYS="NAME PUBLIC_URL REMOTE_USER REMOTE_HOME ROOT_DIR DATA_DIR DB_DIR SECRETS_DIR ENVIRONMENT"

# Clés de questions.conf, une par ligne.
question_keys() { grep -v '^#' "$DEPLOY_DIR/questions.conf" | grep -v '^$' | cut -d'|' -f1; }

load_answers() {
  [[ -f "$ANSWERS_FILE" ]] && source "$ANSWERS_FILE"
  return 0
}

ask_all() {
  local key default regex prompt
  while IFS='|' read -r key default regex prompt; do
    [[ -z "$key" || "$key" == \#* ]] && continue
    ask "$key" "$default" "$regex" "$prompt"
  done < "$DEPLOY_DIR/questions.conf"
}

save_answers() {
  local key
  : > "$ANSWERS_FILE"
  chmod 600 "$ANSWERS_FILE"
  echo "# Réponses de deploy/setup.sh (non versionné). Supprimer ce fichier pour tout reposer." >> "$ANSWERS_FILE"
  for key in $(question_keys); do
    printf '%s=%q\n' "$key" "${!key}" >> "$ANSWERS_FILE"
  done
}

# Calcule les clés dérivées. REMOTE_HOME vient du VPS (lecture seule, y compris en simulation).
derive_answers() {
  NAME=miraxy
  PUBLIC_URL="https://$DOMAIN"
  REMOTE_HOME="${REMOTE_HOME:-$(ssh -o BatchMode=yes "$SSH_DEPLOY" 'printf %s "$HOME"' 2>/dev/null || true)}"
  [[ -n "$REMOTE_HOME" ]] || die "Impossible de lire le dossier personnel via ssh $SSH_DEPLOY"
  REMOTE_USER="${REMOTE_USER:-$(ssh -o BatchMode=yes "$SSH_DEPLOY" 'id -un' 2>/dev/null || true)}"
  [[ -n "$REMOTE_USER" ]] || die "Impossible de lire l'utilisateur via ssh $SSH_DEPLOY"
  ROOT_DIR="$REMOTE_HOME/volumes/$NAME"
  DATA_DIR="$REMOTE_HOME/.local/share/selixy/services/$NAME/data"
  DB_DIR="$REMOTE_HOME/.local/share/selixy/services/$NAME/postgres"
  SECRETS_DIR="$REMOTE_HOME/.config/selixy/secrets/$NAME"
  ENVIRONMENT="$(render_environment)"
}

# Lignes Environment= du quadlet, depuis env.spec (type fixed).
render_environment() {
  local name kind value _desc
  while IFS='|' read -r name kind value _desc; do
    [[ -z "$name" || "$name" == \#* || "$kind" != fixed ]] && continue
    printf 'Environment=%s=%s\n' "$name" "$value"
  done < "$DEPLOY_DIR/env.spec"
}

# Lignes NOM de type secret dans env.spec.
secret_names() {
  grep -v '^#' "$DEPLOY_DIR/env.spec" | awk -F'|' '$2=="secret"{print $1}'
}
