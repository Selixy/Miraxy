# Outils de Développement Local (tools/dev)

Ce module contient les outils d'assistance pour le travail sur l'interface et le développement en local.

## Fichiers
- `server.py` : Serveur HTTP local autonome sans dépendance externe.
  - Écoute par défaut sur `http://localhost:7001/` (configurable via la variable `PORT`).
  - Détecte en temps réel les changements dans `ui/` via un thread de surveillance.
  - Injecte à la volée un script de synchronisation SSE (*Server-Sent Events*).
  - Réapplique les feuilles de style CSS à chaud sans rechargement de page.
  - Déclenche un rechargement complet de la page lors de modifications HTML ou JS.

## Utilisation

```bash
# Lancement direct
python3 tools/dev/server.py

# Choix d'un port spécifique
PORT=8080 python3 tools/dev/server.py
```
