# Header UI Component

Contrôleur d'interface utilisateur pour la barre d'en-tête.

## Fichiers
- `account-badge.js` :
  - Fonction exportée `initAccountBadge()`.
  - Vérifie la session utilisateur via le service `getCurrentUser()`.
  - Si l'utilisateur est connecté, remplace le lien "Login" par le nom de l'utilisateur (`span.tab`) et un bouton "Logout" (`button.tab`) connecté à `logoutUser()`.

## Règle SOLID
Ce contrôleur ne fait aucune requête fetch directe : il dépend de l'abstraction fournie par les services `session.js` et `logout.js`.
