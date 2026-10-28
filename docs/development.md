# Guide de Développement & Architecture Technique

Ce document regroupe les directives techniques, l'arborescence du monorepo, les commandes de développement et les conventions de code pour Miraxy.

---

## 1. Arborescence du Monorepo

| Dossier | Rôle | Environnement d'exécution |
|---|---|---|
| `server/` | API REST, WebSocket, authentification, PostgreSQL | Serveur (Rust / Axum) |
| `ui/` | Interface web moderne, modulaire, sans étape de build | Navigateur (servie par le serveur) |
| `daemon/` | Agent local en arrière-plan : dialogue avec les DCC et synchronisation d'assets | Poste de travail artiste (Rust) |
| `common/` | Protocoles, types et structures partagés entre serveur et daemon | Compilé dans `server/` et `daemon/` |
| `docs/` | Spécifications, architecture, modèle de données, décisions (ADR) | Documentation |
| `tools/` | Serveur de développement de l'interface | Environnement local |

---

## 2. Règles d'Architecture et Dépendances

- **Sens unique de dépendance** : `server` et `daemon` dépendent de `common`. Jamais l'inverse, jamais l'un de l'autre.
- **Frontend découplé** : L'`ui` est purement statique et communique avec le serveur via l'API REST en JSON.
- **Organisation par domaine** : Découpage fonctionnel (`auth/`, `tasks/`, `projects/`...) plutôt que technique (`routes/`, `models/`).
- **Principe de responsabilité unique (SRP)** : Un fichier = une responsabilité ciblée (< 50 lignes en moyenne).
- **Architecture Frontend modulaire (SOLID)** :
  - CSS : bibliothèque `selixy-design` copiée dans `ui/css/selixy-design/` (jamais modifiée sur place), orchestrée par `main.css`, plus le CSS propre à Miraxy dans `ui/css/miraxy/`.
  - JS découpé en couches étanches : `core/http/`, `services/auth/`, `components/`, `pages/`.
  - Chaque module UI dispose de son propre `README.md` explicatif.

---

## 3. Démarrage et Commandes Locales

### Compilation et Tests Backend (Rust)

```bash
# Compilation de l'ensemble du workspace
cargo build

# Lancer la suite de tests unitaires et d'intégration
cargo test

# Lancer le serveur d'API en local (port 7001 par défaut)
cargo run -p miraxy-server
```

### Serveur de Développement Frontend (Live Reload)

Un serveur de développement léger avec rechargement automatique en temps réel (SSE) est fourni dans `tools/dev/` :

```bash
# Démarre le serveur local sur http://localhost:7001/ avec Hot Reload CSS
python3 tools/dev/server.py
```

- Modifiez un fichier `.css` : les styles sont injectés instantanément sans rechargement de page.
- Modifiez un fichier `.html` ou `.js` : la page se recharge automatiquement.

---

## 4. Déploiement

Le dépôt public ne contient pas de scripts de déploiement : ils dépendent de mon propre serveur. Le serveur se lance en local (voir plus haut) ou dans un conteneur de votre choix.

---

## 5. Conventions de Nommage et Langue

- **Code en anglais** : Identifiants, fichiers, fonctions, types, variables, endpoints d'API.
- **Documentation et commentaires en français** : Fichiers Markdown, documentation de conception, commits et explications métier.
