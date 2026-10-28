# Rendu des modèles : remplace les @CLÉ@ par leur valeur. À sourcer.
# Liste FERMÉE de clés : un @X@ inconnu reste en place et fait échouer le rendu.

# render FICHIER_MODÈLE : écrit le résultat sur la sortie standard.
render() {
  local template=$1 key content
  content="$(cat "$template")"
  for key in $(question_keys) $DERIVED_KEYS; do
    content="${content//@$key@/${!key}}"
  done
  if grep -qE '@[A-Z_]+@' <<<"$content"; then
    die "Modèle $template : clé inconnue $(grep -oE '@[A-Z_]+@' <<<"$content" | sort -u | tr '\n' ' ')"
  fi
  printf '%s\n' "$content"
}
