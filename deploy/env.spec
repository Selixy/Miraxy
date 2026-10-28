# Variables d'environnement du serveur, telles que le conteneur les reçoit.
# Format : NOM|type|valeur|description
#   type fixed  : valeur écrite dans le quadlet (peut contenir des @CLÉ@)
#   type secret : générée aléatoirement une fois, stockée dans le fichier de secrets (0600)
#   type derived : calculée par une étape (voir 30-data-secrets.sh), stockée dans le fichier de secrets
# tools/verify/check-deploy.sh vérifie que chaque MIRAXY_* lu par le code est déclaré ici, et inversement.
MIRAXY_BIND|fixed|0.0.0.0:8080|Adresse d'écoute dans le conteneur
RUST_LOG|fixed|miraxy_server=info,tower_http=info|Niveau de logs
MIRAXY_UI_DIR|fixed|/app/ui|Dossier des fichiers de l'ui dans l'image
MIRAXY_DATABASE_URL|derived||URL PostgreSQL, composée à l'étape 30 depuis le mot de passe de la base (secrets.env)
