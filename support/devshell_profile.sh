info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36mHabitat devshell: \033[1;37m$1\033[0m\n"
      ;;
    *)
      printf -- "   devshell: $1\n"
      ;;
  esac
  return 0
}

echo
info 'Plan for success!'

if [[ -n "$HAB_ORIGIN" ]]; then
  info "Exported: HAB_ORIGIN=$HAB_ORIGIN"
fi
if [[ -n "$HAB_DEPOT_URL" ]]; then
  info "Exported: HAB_DEPOT_URL=$HAB_DEPOT_URL"
fi
if [[ -n "$http_proxy" ]]; then
  info "Exported: http_proxy=$http_proxy"
fi
if [[ -n "$https_proxy" ]]; then
  info "Exported: https_proxy=$https_proxy"
fi
echo
