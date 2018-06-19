info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36mHabitat devshell: \033[1;37m%s\033[0m\n" "$1"
      ;;
    *)
      printf -- "   devshell: %s\n" "$1"
      ;;
  esac
  return 0
}

echo
info 'Plan for success!'

if [[ -n "$HAB_ORIGIN" ]]; then
  info "Exported: HAB_ORIGIN=$HAB_ORIGIN"
fi
if [[ -n "$HAB_BLDR_URL" ]]; then
  info "Exported: HAB_BLDR_URL=$HAB_BLDR_URL"
fi
# shellcheck disable=2154
if [[ -n "$http_proxy" ]]; then
  info "Exported: http_proxy=$http_proxy"
fi
# shellcheck disable=2154
if [[ -n "$https_proxy" ]]; then
  info "Exported: https_proxy=$https_proxy"
fi
echo
