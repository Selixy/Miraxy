# Core HTTP

Couche technique d'abstraction réseau. Elle ne contient **aucune logique métier** ni manipulation du DOM.

## Fichiers
- `client.js` :
  - Fonction exportée `httpClient(path, options)`.
  - Encapsule l'API `fetch` avec sérialisation JSON automatique.
  - Renvoie systématiquement un objet standard `{ ok: boolean, status: number, data: any }`.
  - Ne lève jamais d'exception non gérée vers les appelants.
- `endpoints.js` :
  - Dictionnaire centralisé des URLs (`ENDPOINTS`).
  - Évite les chaînes de caractères d'URL en dur dans les services.

## Règle SOLID
Single Responsibility Principle (SRP) : uniquement responsable de la communication HTTP et de la conformité du contrat de retour.
