# Vision

Miraxy est un outil de suivi de production. On y publie des tâches, puis le résultat des tâches effectuées.

## Les quatre briques

| Brique | Rôle |
|---|---|
| `ui/` (interface web) | Publier des tâches, publier les tâches effectuées, visualiser les USD |
| `daemon/` | Dialoguer avec les DCC de l'utilisateur et télécharger en arrière-plan les assets qui le concernent directement |
| `addons/` (à venir) | Un addon par DCC, pour publier depuis le DCC lui-même |
| `server/` | Source de vérité : comptes, droits, tâches, et fichiers USD |

DCC visés en premier : **Blender** et **Unity**. L'intégration de la **2D** reste à réfléchir.

## Le serveur ne stocke que de l'USD

- Il conserve des fichiers USD et leurs textures, rien d'autre. Les fichiers sources (`.blend`…) restent chez l'artiste.
- Ces USD suivent une configuration commune, le **master**, qui repose sur deux principes :
  - un **layering par département** ;
  - une **interprétation et normalisation à l'export** : quel que soit le DCC source, le résultat publié est le même.
- Le master doit pouvoir être **visualisé directement dans l'interface web**, avec un visualiseur USD en **WebAssembly** (décision prise ; l'outil précis reste à choisir).
- Les fichiers étant sur le serveur, le daemon les télécharge en HTTP. Le P2P/BitTorrent est abandonné.

## Droits des comptes

- Les droits sont une **liste de booléens cumulables**, par exemple « peut publier des tâches ».
- Ils sont **portés par département** : être responsable FX permet de publier des tâches FX, mais pas des tâches de modélisation.
- Un compte peut avoir **tous les droits dans tous les départements**.

## Échelle

Aucune hypothèse sur la taille de l'équipe : Miraxy doit fonctionner aussi bien à 2 qu'à 2 000 personnes. Cela impose dès le départ une base de données robuste, la pagination partout, un stockage de fichiers qui pourra changer de support, et des droits modélisés proprement plutôt que codés en dur.

## Zones d'ombre

- **Shaders** : tant que le matériau n'est pas du PBR classique, on ne sait pas comment le traiter. Pistes à étudier : USD Material, MaterialX, UsdPreviewSurface.
- **Spécification du master** : conventions précises (structure des layers par département, nommage, unités, axes) et règles de normalisation par DCC.
- **Cycle de vie d'une tâche** : statuts, assignation, et existence d'une validation avant qu'une publication devienne la version officielle.
- **2D** : quels formats, quel DCC.
- **Visualiseur WebAssembly** : quel outil, quelles limites de rendu.
- **Intervenants externes** : à trancher plus tard.
