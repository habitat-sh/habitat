#!/bin/sh

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# # shellcheck disable=2034
studio_type="bootstrap"
studio_env_command="/usr/bin/env"
studio_enter_environment="STUDIO_ENTER=true"
studio_enter_command="$libexec_path/hab pkg exec core/build-tools-hab-backline bash --login +h"
studio_build_environment=
studio_build_command="$libexec_path/hab pkg exec core/build-tools-hab-plan-build hab-plan-build --"
studio_run_environment=
studio_run_command="$libexec_path/hab pkg exec core/build-tools-hab-backline bash --login"

run_user="hab"
run_group="$run_user"