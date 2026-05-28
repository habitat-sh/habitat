#!/bin/sh

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# # shellcheck disable=2034
studio_type="default"
studio_env_command="/usr/bin/env"
studio_enter_environment="STUDIO_ENTER=true"
# shellcheck disable=SC2154
studio_enter_command="$libexec_path/hab pkg exec ${HAB_STUDIO_BACKLINE_PKG} bash --rcfile $HAB_STUDIO_ROOT/etc/profile"
studio_build_environment=
studio_build_command="${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}/bin/build"
studio_run_environment=
studio_run_command="$libexec_path/hab pkg exec chef/hab-backline bash --rcfile $HAB_STUDIO_ROOT/etc/profile"

run_user="hab"
run_group="$run_user"

# shellcheck disable=SC2154
_pkgpath_for() {
  "$system_hab_cmd" pkg path "$1" | $sed_cmd -e "s,^$HAB_STUDIO_ROOT,,g"
}

# shellcheck disable=SC2154
finish_setup() {
    # Import origin keys from the host key cache (HAB_CACHE_KEY_PATH, typically
    # ~/.hab/cache/keys) into the studio's key cache (/opt/hab/cache/keys).
    # Unlike Linux (which uses a chroot), the darwin studio runs directly on the
    # host, so we must explicitly target HAB_ROOT_PATH/cache/keys on import.
    if [ -n "${HAB_ORIGIN_KEYS:-}" ]; then
        $mkdir_cmd -p "$HAB_ROOT_PATH/cache/keys"
        for key in $(echo "$HAB_ORIGIN_KEYS" | $sed_cmd 's/,/ /g'); do
            info "Importing '$key' secret origin key"
            # shellcheck disable=SC2154
            if key_text=$(HAB_LICENSE="${HAB_LICENSE:-}" "$system_hab_cmd" origin key export --type secret "$key"); then
                printf -- "%s" "${key_text}" | HAB_CACHE_KEY_PATH="$HAB_ROOT_PATH/cache/keys" "$system_hab_cmd" origin key import
            else
                echo "Error exporting $key key"
                echo "${key_text}"
                echo "Habitat was unable to export your secret signing key. Please"
                echo "verify that you have a signing key for $key present in"
                echo "~/.hab/cache/keys. You can test this by running:"
                echo ""
                echo "    hab origin key export --type secret $key"
                echo ""
                echo "This will print your signing key or error if it cannot be found."
                echo "To create a signing key, run:"
                echo ""
                echo "    hab origin key generate $key"
                echo ""
                exit 1
            fi
            # Import the public key too; required for verifying installed packages.
            if key_text=$("$system_hab_cmd" origin key export --type public "$key" 2>/dev/null); then
                info "Importing '$key' public origin key"
                printf -- "%s" "${key_text}" | HAB_CACHE_KEY_PATH="$HAB_ROOT_PATH/cache/keys" "$system_hab_cmd" origin key import
            else
                info "Tried to import '$key' public origin key, but key was not found"
            fi
        done
    else
        info "No secret keys imported! Did you mean to set HAB_ORIGIN?"
        echo "To specify a HAB_ORIGIN, either set the HAB_ORIGIN environment"
        echo "variable to your origin name or run 'hab cli setup' and specify a"
        echo "default origin."
    fi

    src_dir="$($pwd_cmd)"
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/etc
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/bin
    $mkdir_cmd -p "$HAB_STUDIO_ROOT"/tmp
    $mkdir_cmd -p "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin

    $cat_cmd <<EOF > "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin/build
#!/bin/sh
HAB_STUDIO_ROOT=${HAB_STUDIO_ROOT} \
HAB_STUDIO_HAB_BIN=$libexec_path/bin/hab \
$libexec_path/hab pkg exec chef/hab-backline hab-plan-build "\$@"
EOF
    $chmod_cmd +x "${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}"/bin/build

    $cat_cmd >"${HAB_STUDIO_ROOT}"/etc/profile <<PROFILE
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

    coreutils_path=$(_pkgpath_for core/coreutils)

    # Install the hab backline
    "$system_hab_cmd" pkg install "$HAB_STUDIO_BACKLINE_PKG"

    # Install any local artifacts. This is required for the default to work in the incremental
    # mode.
    if [ -n "${HAB_STUDIO_INSTALL_PKGS:-}" ]; then
      echo "Installing additional packages in default studio"
      deps=$(echo "$HAB_STUDIO_INSTALL_PKGS" | "$coreutils_path"/bin/tr ":" "\n")
      for dep in $deps; do
        "$system_hab_cmd" pkg install "$dep"
      done
    fi

    info "Setup Finished."
    return 0
}
