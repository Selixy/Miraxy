# CSS de Miraxy

Miraxy utilise exclusivement la bibliothèque **selixy-design** pour ses classes générales : jetons, reset, typographie, en-tête, burger, panneaux, boutons, champs, carte de formulaire, soulignement, composants.

## Structure

- [main.css](main.css) : point d'entrée, charge la bibliothèque dans l'ordre de son README (tokens, base, typography, header, burger, panel, underline, button, components) puis le CSS propre à Miraxy. `project-card.css` n'est pas chargé : Miraxy n'a pas de carte de projet.
- `selixy-design/` : copie de la bibliothèque, version dans `selixy-design/SELIXY-DESIGN` (actuellement `v2.1.1`).
- [miraxy/](miraxy/README.md) : seulement ce qui est propre à Miraxy (le logo).

## Règle : la copie n'est jamais modifiée

Pour changer une classe générale, on modifie la bibliothèque selixy-design (mon dépôt à part), on tague, puis on recopie la version choisie dans `ui/css/selixy-design`.

Ensuite, incrémenter `?v=` du lien `main.css` dans chaque page de `ui/` : les `@import` ne sont pas versionnés, c'est le lien de la page qui invalide le cache du navigateur.

## Arbitrages

- Les anciens dossiers `tokens/`, `base/`, `primitives/` et `components/` sont supprimés : la bibliothèque les couvre. Correspondance : `--text-dim` etc. gardent leur nom, `--border-fort` devient `--border-strong`, `--accent-fort` devient `--accent-strong`, `--alerte` devient `--warning`.
- La classe `burger-list` n'était définie nulle part : retirée du HTML.
- Le champ de saisie est habillé par `.field input` de la bibliothèque (enveloppe `.field`), sans classe supplémentaire.
