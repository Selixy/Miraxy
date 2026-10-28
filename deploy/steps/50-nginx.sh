step_title() { echo "Vhost nginx (sudo)"; }

step_run() {
  local tmp="/tmp/miraxy-nginx.$$.conf" target="/etc/nginx/conf.d/miraxy.conf"

  remote_read "$SSH_ADMIN" "sudo -n test -f '$TLS_CERT' && sudo -n test -f '$TLS_KEY'" \
    || die "Certificat ou clé TLS introuvable sur le VPS ($TLS_CERT, $TLS_KEY)"

  render "$DEPLOY_DIR/templates/nginx.conf.in" | push_file "$SSH_ADMIN" "$tmp" 644
  remote "$SSH_ADMIN" <<SCRIPT
set -euo pipefail
if sudo -n cmp -s '$tmp' '$target' 2>/dev/null; then
  echo '   vhost déjà à jour'
  rm -f '$tmp'
  exit 0
fi
backup=''
if sudo -n test -f '$target'; then backup='$target.avant-setup'; sudo -n cp '$target' "\$backup"; fi
sudo -n cp '$tmp' '$target'
rm -f '$tmp'
if sudo -n nginx -t 2>/dev/null; then
  sudo -n systemctl reload nginx
  echo '   vhost installé, nginx rechargé'
else
  echo '   nginx -t a échoué : retour arrière' >&2
  if [ -n "\$backup" ]; then sudo -n mv "\$backup" '$target'; else sudo -n rm -f '$target'; fi
  exit 1
fi
SCRIPT
}
