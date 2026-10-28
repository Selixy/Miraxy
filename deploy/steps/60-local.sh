step_title() { echo "Remote git local et crochet pre-commit"; }

step_run() {
  local url="$SSH_DEPLOY:volumes/$NAME/miraxy.git" current
  current="$(git -C "$REPO_DIR" remote get-url "$GIT_REMOTE" 2>/dev/null || true)"
  if [[ "$current" == "$url" ]]; then
    note "remote $GIT_REMOTE déjà configuré"
  elif [[ -z "$current" ]]; then
    [[ "$DRY_RUN" == 1 ]] && note "[simulation] git remote add $GIT_REMOTE $url" || git -C "$REPO_DIR" remote add "$GIT_REMOTE" "$url"
  elif confirm "Le remote $GIT_REMOTE pointe sur $current. Le remplacer par $url ?"; then
    [[ "$DRY_RUN" == 1 ]] && note "[simulation] git remote set-url $GIT_REMOTE $url" || git -C "$REPO_DIR" remote set-url "$GIT_REMOTE" "$url"
  else
    warn "remote $GIT_REMOTE inchangé"
  fi

  if [[ "$DRY_RUN" == 1 ]]; then
    note "[simulation] tools/git/install-hooks.sh"
  else
    bash "$REPO_DIR/tools/git/install-hooks.sh"
  fi
}
