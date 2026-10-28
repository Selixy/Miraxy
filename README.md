# Miraxy

Miraxy est un projet d'étude : un outil de suivi de production pour une petite équipe d'animation. L'idée est qu'on y publie des tâches, puis le résultat des tâches faites, en fichiers OpenUSD.

C'est le terrain de mon mémoire de M2, qui porte sur l'importance des logiciels de suivi de production. Je le construis, même petit, pour en parler en connaissance de cause.

**Où j'en suis : au tout début.** J'ai repris le projet de zéro en octobre 2026. Ce n'est pas un outil utilisable aujourd'hui, c'est un squelette.

Page du projet : https://selixy.dev/miraxy/

## Ce qui existe aujourd'hui

- Un serveur en Rust (Axum, SQLx) avec PostgreSQL.
- Des comptes réels : inscription, connexion, déconnexion. Mots de passe hachés en Argon2id, session par cookie `HttpOnly` dont la base ne garde que l'empreinte. Le premier compte créé devient administrateur.
- Des départements et des droits par département dans la base (booléens cumulables, par exemple « publier des tâches » dans `fx` mais pas dans `modeling`). Il n'y a pas encore d'écran pour les gérer, seulement du SQL.
- Une interface web minimale, sans étape de build : un en-tête, une page de connexion, une page d'inscription. Le CSS vient de ma bibliothèque selixy-design.
- 23 tests Rust. Une partie a besoin d'une base PostgreSQL et ne tourne que si `MIRAXY_TEST_DATABASE_URL` est définie (voir [docs/data-model.md](docs/data-model.md)).

## Ce qui n'existe pas encore

- Les tâches : aucune création, assignation ni suivi.
- La publication de couches USD.
- Le visualiseur 3D dans le navigateur (WebAssembly, outil pas encore choisi).
- Le daemon : `daemon/` ne contient qu'un point d'entrée vide, `common/` aussi.
- Les addons Blender et Unity.

Tout ça est décrit dans [docs/vision.md](docs/vision.md) comme des intentions et des questions ouvertes (cycle de vie d'une tâche, shaders hors PBR, 2D), pas comme des fonctions finies.

## L'idée

- Le suivi passe en premier : des tâches, puis leur résultat.
- Tout est en USD. Le serveur ne garde que de l'USD et ses textures ; les fichiers sources (`.blend` et autres) restent chez l'artiste. Un seul format évite de traiter chaque type d'asset et chaque logiciel au cas par cas.
- Les droits sont portés par département.
- Il doit tenir à 2 comme à 2 000 personnes, donc une vraie base de données et de la pagination.

## Lancer en local

Il faut Rust (édition 2024) et Python 3.

Interface seule, avec rechargement automatique :

```bash
python3 tools/dev/server.py
```

Puis http://localhost:7001/. Sans serveur Rust, les pages s'affichent mais la connexion ne marche pas.

Serveur Rust :

```bash
cargo test
cargo run -p miraxy-server
```

Sans base de données, le serveur démarre quand même et sert l'interface. Pour avoir les comptes, définir `MIRAXY_DATABASE_URL` vers une base PostgreSQL ; les migrations s'appliquent au démarrage. Les variables lues sont `MIRAXY_BIND` (défaut `127.0.0.1:7001`), `MIRAXY_UI_DIR` et `MIRAXY_DATABASE_URL`.

## Déployer sur son VPS

`deploy/setup.sh` est un assistant interactif qui installe Miraxy sur un VPS : le serveur et sa base PostgreSQL dans deux conteneurs Podman, derrière nginx. Il pose ses questions (alias SSH, nom de domaine, ports, chemins du certificat TLS), propose une valeur par défaut à chaque fois, puis pose tout par SSH. On peut le relancer sans risque : chaque étape saute ce qui est déjà en place.

```bash
deploy/setup.sh --dry-run   # simulation, ne modifie rien
deploy/setup.sh             # pose les questions puis déploie
```

Il faut sur le VPS : Podman, git, nginx, un compte avec sudo, et un certificat TLS déjà présent. Le détail de chaque étape est dans [docs/deploy.md](docs/deploy.md).

## Structure

- `server/` : API et base de données
- `ui/` : fichiers statiques de l'interface
- `daemon/` : agent local (vide pour l'instant)
- `common/` : types partagés (vide pour l'instant)
- `deploy/` : l'assistant de déploiement sur un VPS
- `tools/` : serveur de développement, vérification du déploiement
- `docs/` : notes de conception

## Licence

MIT, voir [LICENSE](LICENSE).
