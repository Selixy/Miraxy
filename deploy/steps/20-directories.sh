step_title() { echo "Dossiers, dépôt git nu et hook post-receive"; }

step_run() {
  # ~/volumes appartient à root : le dossier du service y est créé avec sudo, puis donné au compte des conteneurs.
  remote "$SSH_ADMIN" <<SCRIPT
set -euo pipefail
sudo -n install -d -o '$REMOTE_USER' -g '$REMOTE_USER' -m 755 '$ROOT_DIR'
SCRIPT

  remote "$SSH_DEPLOY" <<SCRIPT
set -euo pipefail
mkdir -p '$ROOT_DIR' '$DATA_DIR' '$DB_DIR' '$SECRETS_DIR' "\$HOME/.config/containers/systemd"
chmod 700 '$SECRETS_DIR'
if [ ! -d '$ROOT_DIR/miraxy.git' ]; then
  git init --quiet --bare -b main '$ROOT_DIR/miraxy.git'
  echo '   dépôt nu créé'
else
  echo '   dépôt nu déjà présent'
fi
SCRIPT
  render "$DEPLOY_DIR/templates/post-receive.in" | push_file "$SSH_DEPLOY" "$ROOT_DIR/miraxy.git/hooks/post-receive" 755
  [[ "$DRY_RUN" == 1 ]] || note "hook post-receive installé"
}
