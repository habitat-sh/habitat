#!/bin/sh

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# # shellcheck disable=2034
studio_type="bootstrap"
studio_env_command="/usr/bin/env"
studio_enter_environment="STUDIO_ENTER=true"
# shellcheck disable=SC2154
studio_enter_command="$libexec_path/hab pkg exec ${HAB_STUDIO_BACKLINE_PKG} bash --rcfile $HAB_STUDIO_ROOT/etc/profile"
studio_build_environment=
studio_build_command="${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}/bin/build"
studio_run_environment=
studio_run_command="$libexec_path/hab pkg exec ${HAB_STUDIO_BACKLINE_PKG} bash --rcfile $HAB_STUDIO_ROOT/etc/profile"

run_user="hab"
run_group="$run_user"

# shellcheck disable=SC2154
finish_setup() {
    src_dir="$($pwd_cmd)"
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/etc
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/bin
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/tmp
    $mkdir_cmd -p "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin

    $cat_cmd <<EOF > "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin/build
#!/bin/sh
HAB_STUDIO_ROOT=${HAB_STUDIO_ROOT} \
HAB_STUDIO_HAB_BIN=$libexec_path/bin/hab \
$libexec_path/hab pkg exec ${HAB_STUDIO_BACKLINE_PKG} hab-plan-build "\$@"
EOF
    $chmod_cmd +x "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin/build

    $cat_cmd >"$HAB_STUDIO_ROOT"/etc/profile <<PROFILE
if [[ -n "\${STUDIO_ENTER:-}" ]]; then
  unset STUDIO_ENTER
  source $HAB_STUDIO_ROOT/etc/profile.enter
fi

# Add command line completion
source <(hab cli completers --shell bash)
PROFILE

    $cat_cmd >"$HAB_STUDIO_ROOT"/etc/profile.enter <<PROFILE_ENTER
# Source .studiorc so we can apply user-specific configuration
if [[ -f $src_dir/.studiorc && -z "\${HAB_STUDIO_NOSTUDIORC:-}" ]]; then
  echo "--> Detected and loading /src/.studiorc"
  echo ""
  source $src_dir/.studiorc
fi

PROFILE_ENTER

    coreutils_path=$(_pkgpath_for core/build-tools-coreutils)

    # Install the hab backline
    "$system_hab_cmd" pkg install "$HAB_STUDIO_BACKLINE_PKG"

    # Install any local artifacts. This is required for the bootstrap to work in the incremental
    # mode.
    if [ -n "${HAB_STUDIO_INSTALL_PKGS:-}" ]; then
      echo "Installing additional packages in bootstrap studio"
      deps=$(echo "$HAB_STUDIO_INSTALL_PKGS" | "$coreutils_path"/bin/tr ":" "\n")
      for dep in $deps; do
        "$system_hab_cmd" pkg install "$dep"
      done
    fi

    return 0
}

_pkgpath_for() {
  "$system_hab_cmd" pkg path "$1" | $sed_cmd -e "s,^$HAB_STUDIO_ROOT,,g"
}
