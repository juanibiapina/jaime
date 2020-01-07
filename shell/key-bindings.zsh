if [[ $- == *i* ]]; then

__jaime_launcher() {
  setopt localoptions pipefail no_aliases 2> /dev/null
  jaime
  local ret=$?
  echo
  return $ret
}

jaime_widget() {
  LBUFFER="${LBUFFER}$(__jaime_launcher)"
  local ret=$?
  zle reset-prompt
  return $ret
}
zle     -N   jaime_widget
bindkey '^@' jaime_widget

fi
