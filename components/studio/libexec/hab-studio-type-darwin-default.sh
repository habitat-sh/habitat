#!/bin/sh

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# # shellcheck disable=2034
studio_type="default"
studio_env_command="/usr/bin/env"
studio_enter_environment="STUDIO_ENTER=true"
studio_enter_command="$libexec_path/hab pkg exec core/hab-backline bash --rcfile $HAB_STUDIO_ROOT/.bashrc"
studio_build_environment=
studio_build_command=""
studio_run_environment=
studio_run_command="$libexec_path/hab pkg exec core/hab-backline bash --rcfile $HAB_STUDIO_ROOT/.bashrc"

run_user="hab"
run_group="$run_user"

finish_setup() {
    $cat_cmd <<EOF > "$HAB_STUDIO_ROOT"/.bashrc
function build() {
    $libexec_path/hab pkg exec core/hab-plan-build hab-plan-build -- "$@"
}
export -f build
EOF
    return 0
}