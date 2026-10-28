# Déploiement

Miraxy se déploie sur le VPS avec un assistant interactif versionné dans le dépôt : `deploy/setup.sh`.
Il se lance **depuis le portable** et pilote le VPS par ssh. Il est **rejouable** : chaque étape
vérifie l'état et saute ce qui est déjà en place, donc on peut le relancer après un changement, ou sur un
autre VPS en répondant aux mêmes questions.

## Utilisation

    deploy/setup.sh --dry-run     # simulation : montre tout, ne modifie rien
    deploy/setup.sh               # pose les questions, puis déploie l'infra
    deploy/setup.sh --yes         # sans question : réponses enregistrées, sinon défauts
    deploy/setup.sh --list        # liste les étapes
    deploy/setup.sh --only 50     # ne relance que l'étape 50 (nginx)
    deploy/setup.sh --reset       # oublie les réponses enregistrées

Les réponses sont enregistrées dans `deploy/answers.env` (non versionné, mode 0600).

Une fois l'infra posée, on livre le code par un push :

    git push vps main

Le hook `post-receive` du VPS construit l'image (les tests Rust tournent pendant la construction),
redémarre le service et attend `/api/health`. Si le build, les tests ou la santé échouent,
l'ancienne version reste (ou revient) en service.

## Ce que fait chaque étape

| Étape | Rôle |
|---|---|
| 10 | Vérifie ssh, podman, git, sudo sans mot de passe, et le port |
| 20 | Dossiers, dépôt git nu et hook `post-receive` sur le compte des conteneurs |
| 30 | Propriété des volumes (données, PostgreSQL), secrets et mot de passe de la base (générés au premier passage) |
| 40 | Quadlets : réseau privé, PostgreSQL, serveur ; démarre la base |
| 50 | Vhost nginx, avec `nginx -t` et retour arrière si la configuration est invalide |
| 60 | Remote git local et crochet `pre-commit` |
| 70 | Vérification de l'état du service et de l'URL publique |

## Organisation

    deploy/
    ├── setup.sh            point d'entrée
    ├── questions.conf      les questions posées (clé, défaut, validation, texte)
    ├── env.spec            les variables d'environnement du serveur
    ├── Containerfile       l'image (serveur + ui)
    ├── templates/          quadlet, vhost nginx, hook : des @CLÉS@ remplies par les réponses
    ├── lib/                affichage, réponses, rendu, ssh
    └── steps/              une étape par fichier, numérotée

## Garder l'outil à jour

Le déploiement doit évoluer avec le code, à chaque commit qui le touche. Ce n'est pas laissé à la
mémoire : `tools/verify/check-deploy.sh` est lancé par le crochet `pre-commit` (activé par
`tools/git/install-hooks.sh`, appelé par l'étape 60) et **refuse le commit** si :

- le code lit une variable `MIRAXY_*` absente de `deploy/env.spec` (ou l'inverse) ; les variables
  `MIRAXY_TEST_*`, réservées aux tests, sont ignorées ;
- un modèle utilise une `@CLÉ@` inconnue de `questions.conf` ;
- une question n'est utilisée nulle part, ou `questions.conf` est mal formé ;
- deux étapes ont le même numéro, ou un script shell a une erreur de syntaxe ;
- le Rust n'est pas formaté.

Règles pratiques :

- **Nouvelle variable d'environnement dans le serveur** : la déclarer dans `deploy/env.spec`
  (`fixed` pour une valeur écrite dans le quadlet, `secret` pour une valeur aléatoire générée).
- **Nouveau réglage à demander** : une ligne dans `deploy/questions.conf`, puis `@CLÉ@` dans un modèle.
- **Nouvelle étape** : un fichier `deploy/steps/NN-nom.sh` avec `step_title` et `step_run`.
- Quand `deploy/` change, relancer `deploy/setup.sh` : le hook du VPS prévient si la version de
  `deploy/` poussée diffère de celle appliquée au dernier setup.

## Choix de conception

- **Autonome** : dépôt nu et hook propres à Miraxy, sans autre outil de déploiement à installer.
- **Image** : `rust:1.99-bookworm` pour construire, `debian:bookworm-slim` pour exécuter. Pas de `curl`
  dans l'image : le contrôle de santé du conteneur est la commande `miraxy-server healthcheck`.
- **Durcissement** : utilisateur sans droits, système de fichiers en lecture seule, aucune capability,
  mémoire et processus plafonnés, port publié sur `127.0.0.1` seulement.
- **TLS** : le certificat et sa clé doivent déjà être sur le VPS (leurs chemins sont demandés) ; nginx termine TLS, sur le port 443 ou derrière un routeur SNI selon la réponse donnée.

- **PostgreSQL** : conteneur `selixy-miraxy-db` (image `postgres:17-bookworm`) sur le réseau privé
  `selixy-miraxy`, sans port publié. Le serveur le joint par `MIRAXY_DATABASE_URL`, composée à l'étape 30
  (type `derived` dans `env.spec`). Le mot de passe est dans `db.env` et `secrets.env` (0600). Le push ne
  redémarre que le serveur : la base n'est jamais touchée par un déploiement.

## À améliorer

- Épingler les images de base par empreinte (comme brain) plutôt que par tag.
- Sauvegarde automatique (timer) de la base : elle contient maintenant des comptes.
