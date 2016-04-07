. $libexec_path/hab-studio-type-bldr-slim.sh

studio_type="bldr"
studio_enter_command="$BLDR_ROOT/bin/hab-bpm exec chef/hab-backline bash --login +h"
studio_run_environment="$BLDR_ROOT/bin/hab-bpm exec chef/hab-backline bash -l"

bldr_pkgs="chef/hab-bpm chef/hab-backline chef/hab-studio"
