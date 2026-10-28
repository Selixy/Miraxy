step_title() { echo "Quadlets (réseau, PostgreSQL, serveur)"; }

step_run() {
  local dir="$REMOTE_HOME/.config/containers/systemd"
  render "$DEPLOY_DIR/templates/miraxy.network.in" | push_file "$SSH_DEPLOY" "$dir/selixy-miraxy.network" 644
  render "$DEPLOY_DIR/templates/miraxy-db.container.in" | push_file "$SSH_DEPLOY" "$dir/selixy-miraxy-db.container" 644
  render "$DEPLOY_DIR/templates/miraxy.container.in" | push_file "$SSH_DEPLOY" "$dir/selixy-miraxy.container" 644
  remote "$SSH_DEPLOY" <<'SCRIPT'
export XDG_RUNTIME_DIR="/run/user/$(id -u)"
export DBUS_SESSION_BUS_ADDRESS="unix:path=${XDG_RUNTIME_DIR}/bus"
systemctl --user daemon-reload
# La base démarre dès maintenant (le serveur, lui, attend le premier push).
systemctl --user start selixy-miraxy-db.service
SCRIPT

  # Mémorise la version de deploy/ appliquée : le hook prévient si elle diverge.
  local tree
  tree="$(git -C "$REPO_DIR" rev-parse HEAD:deploy 2>/dev/null || true)"
  if [[ -n "$(git -C "$REPO_DIR" status --porcelain -- deploy 2>/dev/null)" ]]; then
    warn "deploy/ contient des changements non commités : le hook signalera une divergence."
  fi
  [[ -n "$tree" ]] && printf '%s\n' "$tree" | push_file "$SSH_DEPLOY" "$ROOT_DIR/setup.tree" 644
  return 0
}
