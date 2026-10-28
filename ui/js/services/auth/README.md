# Services Auth

Couche métier d'authentification et de gestion de session utilisateur.
Ces modules sont **purs et indépendants du DOM** : ils ne contiennent aucun `document.querySelector` ni élément HTML.

## Fichiers
- `session.js` :
  - `getCurrentUser()` : Récupère les données de l'utilisateur connecté via l'endpoint `/api/me`. Renvoie l'objet utilisateur ou `null`.
- `logout.js` :
  - `logoutUser()` : Déclenche la déconnexion via `/api/logout` et redirige vers l'accueil.

## Règle SOLID
Séparation des responsabilités : les services gèrent les flux de données et les règles d'authentification. L'affichage graphique est délégué aux contrôleurs dans `ui/js/components/`.
