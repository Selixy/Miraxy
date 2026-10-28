# CSS propre à Miraxy

Seulement ce que la bibliothèque `selixy-design` ne couvre pas. Aucune classe de la bibliothèque n'est redéfinie ici et aucune couleur n'est écrite en dur.

- `logo.css` : branche le logo de Miraxy sur le jeton `--header-logo` prévu par la bibliothèque.

Les pages (accueil, login, inscription) n'ont pas de mise en page propre pour l'instant : `main`, `.card` et `.field` de la bibliothèque suffisent. Une mise en page propre à une page se met ici, dans un fichier à part, avec les jetons `var(--…)`.
