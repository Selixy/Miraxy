#!/usr/bin/env bash
# Assistant de déploiement de Miraxy. À lancer depuis le portable ; il pilote le VPS par ssh.
# Rejouable : chaque étape vérifie l'état et saute ce qui est déjà en place.
# Documentation : docs/deploy.md
set -euo pipefail

DEPLOY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$DEPLOY_DIR")"
export DEPLOY_DIR REPO_DIR

source "$DEPLOY_DIR/lib/ui.sh"
source "$DEPLOY_DIR/lib/answers.sh"
source "$DEPLOY_DIR/lib/render.sh"
source "$DEPLOY_DIR/lib/remote.sh"

DRY_RUN=0 ASSUME_YES=0 ONLY="" LIST=0

usage() {
  cat <<'FIN'
Usage : deploy/setup.sh [options]

  --dry-run     montre ce qui serait fait, ne modifie rien (les lectures ssh ont lieu)
  --yes         ne pose aucune question : réponses enregistrées, sinon valeurs par défaut
  --only NN     ne lance que l'étape NN (voir --list)
  --list        liste les étapes
  --reset       oublie les réponses enregistrées (deploy/answers.env)
  -h, --help    cette aide
FIN
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run) DRY_RUN=1 ;;
    --yes)     ASSUME_YES=1 ;;
    --only)    ONLY="${2:?--only demande un numero}"; shift ;;
    --list)    LIST=1 ;;
    --reset)   rm -f "$ANSWERS_FILE"; say "Réponses oubliées." ;;
    -h|--help) usage; exit 0 ;;
    *)         usage >&2; die "Option inconnue : $1" ;;
  esac
  shift
done
export DRY_RUN ASSUME_YES

step_files() { ls "$DEPLOY_DIR"/steps/[0-9][0-9]-*.sh; }

if [[ "$LIST" == 1 ]]; then
  for file in $(step_files); do
    source "$file"
    printf '%s  %s\n' "$(basename "$file" | cut -c1-2)" "$(step_title)"
  done
  exit 0
fi

[[ "$DRY_RUN" == 1 ]] && say "MODE SIMULATION : rien n'est modifié."

load_answers
say "Configuration (Entrée pour garder la valeur entre crochets)"
ask_all
[[ "$DRY_RUN" == 1 ]] || save_answers
derive_answers

for file in $(step_files); do
  number="$(basename "$file" | cut -c1-2)"
  [[ -z "$ONLY" || "$ONLY" == "$number" ]] || continue
  source "$file"
  say "Étape $number : $(step_title)"
  step_run
done

say "Terminé."
