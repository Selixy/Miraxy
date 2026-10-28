# Architecture JavaScript de Miraxy

L'architecture JavaScript de Miraxy repose sur des **micro-modules ES6 natifs**, sans framework lourd ni étape de compilation obligatoire.

## Séparation des Responsabilités (SOLID)

1. [core/](core/http/README.md) : Socle technique d'abstraction réseau (`httpClient`, `ENDPOINTS`).
2. [services/](services/auth/README.md) : Logique métier pure (authentification, session) indépendante du DOM.
3. [components/](components/header/README.md) : Contrôleurs d'interface utilisateur manipulant le DOM :
   - [header/](components/header/README.md) : Rendu de l'identité utilisateur connectée.
   - [forms/](components/forms/README.md) : Gestion générique de la soumission de formulaires.
4. [pages/](pages/README.md) : Points d'entrée légers chargés par chaque page HTML via `<script type="module">`.
