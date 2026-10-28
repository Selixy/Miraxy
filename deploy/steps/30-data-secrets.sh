step_title() { echo "Données, base PostgreSQL (propriété des volumes) et secrets"; }

step_run() {
  local secrets
  secrets="$(secret_names | tr '\n' ' ')"
  remote "$SSH_DEPLOY" <<SCRIPT
set -euo pipefail
podman unshare chown -R $CONTAINER_UID:$CONTAINER_UID '$DATA_DIR'
# L'image postgres tourne sous l'utilisateur postgres (uid 999).
podman unshare chown -R 999:999 '$DB_DIR'
file='$SECRETS_DIR/secrets.env'
[ -f "\$file" ] || { umask 077; : > "\$file"; echo '   fichier de secrets créé'; }
chmod 600 "\$file"
for name in $secrets; do
  if ! grep -q "^\$name=" "\$file"; then
    echo "\$name=\$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')" >> "\$file"
    echo "   secret \$name généré"
  fi
done

# Mot de passe de la base : db.env pour PostgreSQL, URL de connexion pour le serveur (même mot de passe).
db_env='$SECRETS_DIR/db.env'
[ -f "\$db_env" ] || { umask 077; : > "\$db_env"; }
chmod 600 "\$db_env"
if ! grep -q '^POSTGRES_PASSWORD=' "\$db_env"; then
  echo "POSTGRES_PASSWORD=\$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')" >> "\$db_env"
  echo '   mot de passe PostgreSQL généré'
fi
if ! grep -q '^MIRAXY_DATABASE_URL=' "\$file"; then
  password="\$(grep '^POSTGRES_PASSWORD=' "\$db_env" | cut -d= -f2)"
  echo "MIRAXY_DATABASE_URL=postgres://miraxy:\$password@selixy-miraxy-db:5432/miraxy" >> "\$file"
  echo '   MIRAXY_DATABASE_URL composée'
fi
SCRIPT
}
