#!/bin/bash

source /etc/habitat-studio/logging.sh


# Removes any potential malicious secrets
sanitize_secrets() {
  for x in HAB_BINLINK_DIR HAB_LICENSE HAB_ORIGIN HOME LC_ALL PATH PWD STUDIO_TYPE TERM TERMINFO; do
    unset "HAB_STUDIO_SECRET_$x"
  done
}

# Builds up a secret environment based on the prefix `HAB_STUDIO_SECRET_`
# to pass into the studio
load_secrets() {
  sanitize_secrets
  hab pkg exec core/hab-backline env | hab pkg exec core/hab-backline awk -F '=' '/^HAB_STUDIO_SECRET_/ {gsub(/HAB_STUDIO_SECRET_/, ""); print}'
}

if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
info "Exported: HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
fi
if [ -n "${HAB_AUTH_TOKEN:-}" ]; then
info "Exported: HAB_AUTH_TOKEN=[redacted]"
fi
if [ -n "${HAB_LICENSE:-}" ]; then
info "Exported: HAB_LICENSE=$HAB_LICENSE"
fi
if [ -n "${HAB_ORIGIN:-}" ]; then
info "Exported: HAB_ORIGIN=$HAB_ORIGIN"
fi
if [ -n "${HAB_BLDR_URL:-}" ]; then
info "Exported: HAB_BLDR_URL=$HAB_BLDR_URL"
fi
if [ -n "${HAB_BLDR_CHANNEL:-}" ]; then
info "Exported: HAB_BLDR_CHANNEL=$HAB_BLDR_CHANNEL"
fi
if [ -n "${HAB_NOCOLORING:-}" ]; then
info "Exported: HAB_NOCOLORING=$HAB_NOCOLORING"
fi
if [ -n "${HAB_NONINTERACTIVE:-}" ]; then
info "Exported: HAB_NONINTERACTIVE=$HAB_NONINTERACTIVE"
fi
if [ -n "${HAB_STUDIO_NOSTUDIORC:-}" ]; then
info "Exported: HAB_STUDIO_NOSTUDIORC=$HAB_STUDIO_NOSTUDIORC"
fi
if [ -n "${HAB_STUDIO_SUP:-}" ]; then
info "Exported: HAB_STUDIO_SUP=$HAB_STUDIO_SUP"
fi
if [ -n "${HAB_UPDATE_STRATEGY_FREQUENCY_MS:-}" ]; then
info "Exported: HAB_UPDATE_STRATEGY_FREQUENCY_MS=$HAB_UPDATE_STRATEGY_FREQUENCY_MS"
fi
if [ -n "${http_proxy:-}" ]; then
info "Exported: http_proxy=$http_proxy"
fi
if [ -n "${https_proxy:-}" ]; then
info "Exported: https_proxy=$https_proxy"
fi
if [ -n "${no_proxy:-}" ]; then
info "Exported: no_proxy=$no_proxy"
fi

for secret_name in $(load_secrets | hab pkg exec core/coreutils cut -d = -f 1); do
info "Exported: $secret_name=[redacted]"
done
