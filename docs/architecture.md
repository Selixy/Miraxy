# Architecture

```
navigateur (ui)  --HTTPS-->  serveur  <--HTTPS/WS--  daemon (poste)
                                                                |
                                                   addons des DCC (Blender, Unity)
```

- **server** : source de vérité (comptes, droits, tâches) et stockage des fichiers USD.
- **daemon** : dialogue avec les DCC et télécharge en fond les assets concernant l'utilisateur.
- **addons** (à venir) : publient depuis les DCC, via le daemon.
- **ui** : publier les tâches, visualiser les USD (WebAssembly). Appelle le serveur, et le daemon en local.
- **common** : le contrat entre serveur et daemon.

## Décisions prises

Voir `vision.md` pour le produit.

- Refonte complète le 2026-10-13. Rust pour le backend.
- Le dépôt est un monorepo : `server/`, `ui/`, `daemon/`, `common/`.
- Les fichiers sont servis en HTTP par le serveur (plus de P2P).
- L'ui est servie par le serveur. Son embarquement dans le daemon est repoussé (rendre une interface web en natif reste à étudier).

- Base de données : PostgreSQL (décidé le 2026-10-13), pour tenir l'échelle de 2 à 2 000 personnes. Modèle, droits et authentification : voir `data-model.md`.

## À décider

- Routage des pages de l'ui.
