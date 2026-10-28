# Affichage et questions interactives. À sourcer.

say()  { printf '\033[1m>>\033[0m %s\n' "$*"; }
note() { printf '   %s\n' "$*"; }
warn() { printf '\033[33m!! %s\033[0m\n' "$*" >&2; }
die()  { printf '\033[31mERREUR : %s\033[0m\n' "$*" >&2; exit 1; }

# ask KEY DEFAULT REGEX QUESTION : pose la question, valide, stocke dans la variable KEY.
# Avec ASSUME_YES=1, prend la valeur déjà connue ou le défaut, sans interaction.
ask() {
  local key=$1 default=$2 regex=$3 prompt=$4 current reply
  current="${!key:-$default}"
  while true; do
    if [[ "${ASSUME_YES:-0}" == 1 ]]; then
      reply=""
    else
      read -r -p "$prompt [$current] : " reply
    fi
    reply="${reply:-$current}"
    if [[ "$reply" =~ $regex ]]; then
      printf -v "$key" '%s' "$reply"
      return 0
    fi
    [[ "${ASSUME_YES:-0}" == 1 ]] && die "$key : « $reply » ne respecte pas $regex"
    warn "Réponse invalide (attendu : $regex)"
  done
}

# confirm QUESTION : oui/non, « oui » par défaut. Toujours oui avec ASSUME_YES=1.
confirm() {
  [[ "${ASSUME_YES:-0}" == 1 ]] && return 0
  local reply
  read -r -p "$1 [O/n] : " reply
  [[ -z "$reply" || "$reply" =~ ^[oOyY] ]]
}
