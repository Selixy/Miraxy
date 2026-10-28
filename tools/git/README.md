# Outils Git (tools/git)

Ce module gère la configuration et l'outillage liés au dépôt Git local.

## Fichiers
- `install-hooks.sh` : Configure le dépôt local pour utiliser le dossier versionné `.githooks/` au lieu des hooks non versionnés par défaut de Git.

## Utilisation

```bash
# À lancer une seule fois après avoir cloné le projet
bash tools/git/install-hooks.sh
```
