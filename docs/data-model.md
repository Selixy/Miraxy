# Modèle de données

PostgreSQL. Les migrations sont dans `server/migrations/`, embarquées dans le binaire et appliquées
au démarrage du serveur (`server/src/db.rs`). Elles ne se modifient jamais après un déploiement : on
en ajoute une nouvelle.

## Tables

| Table | Contenu |
|---|---|
| `accounts` | Un compte : e-mail (unique, en minuscules), nom, empreinte Argon2id du mot de passe, `is_admin` |
| `departments` | Un département : `slug` (unique, ex. `fx`, `modeling`) et nom |
| `account_rights` | Un droit accordé à un compte **dans un département** : `(account_id, department_id, right_name)` |
| `sessions` | Une connexion : empreinte SHA-256 du jeton, compte, date d'expiration (30 jours) |

## Droits

- Un droit est un **booléen cumulable** : sa présence dans `account_rights` l'accorde, son absence le refuse.
- Il est **porté par département** : `publish_tasks` dans `fx` ne donne rien dans `modeling`.
- `is_admin` = tous les droits dans tous les départements. C'est un drapeau, pas une ligne par droit,
  donc un département créé plus tard est couvert d'office.
- Le catalogue des droits est dans le code (`server/src/rights.rs`), pas en base : ajouter un droit ne
  demande pas de migration. Un droit inconnu du catalogue est ignoré à la lecture.

| Droit | Sens |
|---|---|
| `publish_tasks` | Publier des tâches dans le département |
| `publish_results` | Publier le résultat des tâches effectuées |
| `manage_members` | Accorder ou retirer des droits dans le département |

## Authentification

- **Mots de passe** : Argon2id (paramètres par défaut de la crate), 10 caractères minimum, 1 024 octets maximum.
- **Session** : un jeton aléatoire de 256 bits dans un cookie `miraxy_session` (`HttpOnly`, `Secure`,
  `SameSite=Lax`). La base ne garde que son empreinte SHA-256 : une fuite de la base ne donne aucune session.
- **CSRF** : `SameSite=Lax`, et les routes d'écriture n'acceptent que du JSON (un formulaire d'un autre site
  ne peut pas en envoyer sans passer par une requête préalable refusée).
- **Énumération des comptes** : une adresse inconnue et un mauvais mot de passe donnent la même réponse, avec
  le même coût de calcul.
- **Premier compte** : le premier compte créé devient administrateur (amorçage). Un verrou empêche que deux
  inscriptions simultanées le soient toutes les deux.

## API

| Route | Rôle |
|---|---|
| `POST /api/signup` | Crée un compte et ouvre une session |
| `POST /api/login` | Ouvre une session |
| `POST /api/logout` | Ferme la session |
| `GET /api/me` | Compte courant et ses droits par département |

## Tests

Les tests qui ont besoin d'une base ne tournent que si `MIRAXY_TEST_DATABASE_URL` est définie ; sinon ils
sont ignorés (sans base disponible, ils passent sans rien vérifier). Chacun crée sa propre base, migrée.

    podman run -d --rm --name miraxy-test-pg -e POSTGRES_PASSWORD=test -p 127.0.0.1:54329:5432 docker.io/library/postgres:17-bookworm
    MIRAXY_TEST_DATABASE_URL=postgres://postgres:test@127.0.0.1:54329/postgres cargo test

## Pas encore fait

- Création et gestion des départements et des droits (aujourd'hui seulement par SQL).
- Limitation du nombre d'essais de connexion (aujourd'hui, rien ne la limite).
- Vérification de l'adresse e-mail, réinitialisation du mot de passe.
- Inscription ouverte à tout le monde : à restreindre si besoin (voir « intervenants externes » dans `vision.md`).
