# Forms UI Component

Contrôleur d'interface utilisateur pour la gestion des formulaires interactifs.

## Fichiers
- `auth-form.js` :
  - Fonction exportée `initAuthForm(formSelector)`.
  - Écoute l'événement `submit` du formulaire ciblé.
  - Récupère l'endpoint via `data-endpoint`.
  - Désactive le bouton d'envoi et masque l'erreur précédente pendant la requête.
  - En cas de succès : redirige vers l'accueil (`/`).
  - En cas d'échec : affiche le message d'erreur retourné par l'API et réactive le bouton.

## Règle SOLID
Ce composant est réutilisable pour n'importe quel formulaire respectant la convention `data-endpoint` (connexion, inscription, etc.), sans duplication de code.
