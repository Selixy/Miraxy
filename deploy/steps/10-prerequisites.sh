step_title() { echo "Prérequis (ssh, podman, port libre)"; }

step_run() {
  remote_read "$SSH_DEPLOY" true 2>/dev/null || die "ssh $SSH_DEPLOY injoignable (clé, alias dans ~/.ssh/config ?)"
  remote_read "$SSH_ADMIN" true 2>/dev/null || die "ssh $SSH_ADMIN injoignable"
  note "ssh $SSH_DEPLOY et $SSH_ADMIN : ok"

  remote_read "$SSH_DEPLOY" 'command -v podman >/dev/null && command -v git >/dev/null' \
    || die "podman ou git manquant sur $SSH_DEPLOY"
  note "podman et git présents"

  remote_read "$SSH_ADMIN" 'sudo -n true' 2>/dev/null || die "sudo sans mot de passe indisponible pour $SSH_ADMIN"
  note "sudo sans mot de passe : ok"

  local owner
  owner="$(remote_read "$SSH_ADMIN" "sudo -n ss -ltnH 'sport = :$HOST_PORT'" 2>/dev/null || true)"
  if [[ -n "$owner" ]]; then
    note "Le port $HOST_PORT est déjà écouté (normal si Miraxy tourne déjà) :"
    note "$owner"
  else
    note "Port $HOST_PORT libre"
  fi
}
