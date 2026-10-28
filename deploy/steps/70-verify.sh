step_title() { echo "Vérification (avant le premier push, l'échec est normal)"; }

step_run() {
  local status
  status="$(remote_read "$SSH_DEPLOY" 'systemctl --user is-active selixy-miraxy.service' 2>/dev/null || true)"
  note "service selixy-miraxy : ${status:-inconnu}"
  status="$(remote_read "$SSH_DEPLOY" 'systemctl --user is-active selixy-miraxy-db.service' 2>/dev/null || true)"
  note "service selixy-miraxy-db (PostgreSQL) : ${status:-inconnu}"

  if curl -fsS -m 10 -o /dev/null "$PUBLIC_URL/api/health" 2>/dev/null; then
    say "$PUBLIC_URL/api/health répond."
  else
    warn "$PUBLIC_URL/api/health ne répond pas encore."
    note "Étape suivante : git push $GIT_REMOTE main"
  fi
}
