# Outils de Vérification et Qualité (tools/verify)

Ce module regroupe les scripts d'analyse statique, de linting et de vérification d'intégrité du dépôt.

## Fichiers
- `check-deploy.sh` :
  - Vérifie la syntaxe de tous les scripts shell du dépôt (`bash -n`).
  - Assure la cohérence stricte entre les variables d'environnement lues dans le code Rust (`MIRAXY_*`) et celles documentées dans `deploy/env.spec`.
  - Valide la conformité des templates et des clés déclarées dans `deploy/questions.conf`.
  - Contrôle l'unicité des numéros d'étapes de déploiement.
  - Exécute `shellcheck` si disponible.

## Utilisation

```bash
# Vérification manuelle
bash tools/verify/check-deploy.sh
```

Ce script est également invoqué automatiquement par le hook git `.githooks/pre-commit` avant chaque commit.
