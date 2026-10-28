# Exécution sur le VPS, avec mode simulation. À sourcer.
# DRY_RUN=1 : affiche ce qui serait fait, ne modifie rien.

# remote ALIAS SCRIPT... : exécute un script bash sur le VPS (lu sur l'entrée standard).
remote() {
  local alias=$1
  if [[ "${DRY_RUN:-0}" == 1 ]]; then
    note "[simulation] ssh $alias bash -s <<'SCRIPT'"
    sed 's/^/        | /'
    return 0
  fi
  ssh -o BatchMode=yes "$alias" 'bash -s'
}

# remote_read ALIAS COMMANDE : lecture seule, toujours exécutée.
remote_read() {
  local alias=$1; shift
  ssh -o BatchMode=yes "$alias" "$@"
}

# push_file CONTENU_STDIN ALIAS DESTINATION MODE : dépose un fichier (atomique, avec mode).
push_file() {
  local alias=$1 dest=$2 mode=$3
  if [[ "${DRY_RUN:-0}" == 1 ]]; then
    note "[simulation] dépôt de $dest (mode $mode) sur $alias :"
    sed 's/^/        | /' | head -n 8
    return 0
  fi
  ssh -o BatchMode=yes "$alias" "cat > '$dest.tmp' && chmod $mode '$dest.tmp' && mv '$dest.tmp' '$dest'"
}
