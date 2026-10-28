# Pages (Points d'Entrée)

Points d'entrée JavaScript (bootstraps) chargés individuellement par chaque page HTML via `<script type="module">`.

## Fichiers
- `home.js` : Bootstrap de la page d'accueil (initialise le badge de compte).
- `login.js` : Bootstrap de la page de connexion (initialise le formulaire de login et le badge de compte).
- `signup.js` : Bootstrap de la page d'inscription (initialise le formulaire d'inscription et le badge de compte).

## Règle SOLID
Chaque fichier de page est minimal (< 10 lignes) et se contente d'orchestrer et d'instancier les contrôleurs UI nécessaires à la vue. Aucune logique métier ni DOM brut n'est écrit ici.
