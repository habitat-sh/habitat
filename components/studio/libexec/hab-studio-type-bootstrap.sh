# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# shellcheck disable=2034
studio_type="bootstrap"
studio_path="$HAB_ROOT_PATH/bin"
studio_enter_environment="STUDIO_ENTER=true"
studio_enter_command="$HAB_ROOT_PATH/bin/hab pkg exec core/build-tools-hab-backline bash --login +h"
studio_build_environment=
studio_build_command="$HAB_ROOT_PATH/bin/build"
studio_run_environment=
studio_run_command="$HAB_ROOT_PATH/bin/hab pkg exec core/build-tools-hab-backline bash --login"

run_user="hab"
run_group="$run_user"

finish_setup() {
  if [ -n "$HAB_ORIGIN_KEYS" ]; then
    # There's a method to this madness: `$hab` is the raw path to `hab`
    # will use the outside cache key path, whereas the `_hab` function has
    # the `$FS_ROOT` set for the inside of the Studio. We're copying from
    # the outside in, using `hab` twice. I love my job.
    # shellcheck disable=SC2154
    for key in $(echo "$HAB_ORIGIN_KEYS" | $bb tr ',' ' '); do
      # Import the secret origin key, required for signing packages
      info "Importing '$key' secret origin key"
      # shellcheck disable=2154
      if key_text=$(HAB_LICENSE="$HAB_LICENSE" $hab origin key export --type secret "$key"); then
        printf -- "%s" "${key_text}" | _hab origin key import
      else
        echo "Error exporting $key key"
        # key_text will contain an error message
        echo "${key_text}"
        echo "Habitat was unable to export your secret signing key. Please"
        echo "verify that you have a signing key for $key present in either"
        # shellcheck disable=2088
        echo "~/.hab/cache/keys (if running via sudo) or /hab/cache/keys"
        echo "(if running as root). You can test this by running:"
        echo ""
        echo "    hab origin key export --type secret $key"
        echo ""
        echo "This test will print your signing key to the console or error"
        echo "if it cannot find the key. To create a signing key, you can run: "
        echo ""
        echo "    hab origin key generate $key"
        echo ""
        echo "You'll also be prompted to create an origin signing key when "
        echo "you run 'hab setup'."
        echo ""
        exit 1
      fi
      # Attempt to import the public origin key, which can be used for local
      # package installations where the key may not yet be uploaded.
      if key_text=$(HAB_LICENSE="$HAB_LICENSE" $hab origin key export --type public "$key" 2>/dev/null); then
        info "Importing '$key' public origin key"
        printf -- "%s" "${key_text}" | _hab origin key import
      else
        info "Tried to import '$key' public origin key, but key was not found"
      fi
    done
  else
    info "No secret keys imported! Did you mean to set HAB_ORIGIN?"
    echo "To specify a HAB_ORIGIN, either set the HAB_ORIGIN environment"
    echo "variable to your origin name or run 'hab setup' and specify a"
    echo "default origin."
  fi

  if [ -h "$HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab" ]; then
    return 0
  fi

  # In the overwhelming majority of instances, you will want to use
  # stable components in your Studio.
  #
  # However, the Habitat team needs to build studios using unstable
  # components in order to fully test things. This `CI_OVERRIDE_CHANNEL`
  # is supplied as a back-door to enable that to happen. It should be
  # set to the name of a channel to preferentially pull packages from.
  #
  # If you're not on the Habitat core team, you will likely never need
  # this, or even have to know it exists.
  #
  # (This is also why we're not using HAB_BLDR_CHANNEL for this and
  # replicating the fallback logic from hab-plan-build; it'd be too
  # easy to create an unstable studio.)
  _hab pkg install "$HAB_STUDIO_BACKLINE_PKG"

  bash_path=$(_pkgpath_for core/build-tools-bash-static)
  coreutils_path=$(_pkgpath_for core/build-tools-coreutils)

  # shellcheck disable=2086,2154
  $bb mkdir -p $v "$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin

  # Put `hab` on the default `$PATH`
  _hab pkg binlink --dest "$HAB_ROOT_PATH"/bin core/build-tools-hab hab

  # Create `/bin/{sh,bash}` for software that hardcodes these shells
  _hab pkg binlink core/build-tools-bash-static bash
  _hab pkg binlink core/build-tools-bash-static sh

  # Create a wrapper to `build` so that any calls to it have a super-stripped
  # `$PATH` and not whatever augmented version is currently in use. This should
  # mean that running `build` from inside a `studio enter` and running `studio
  # build` leads to the exact same experience, at least as far as initial
  # `$PATH` is concerned.
  $bb cat <<EOF >"$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin/build
#!$bash_path/bin/sh
exec $HAB_ROOT_PATH/bin/hab pkg exec core/build-tools-hab-plan-build hab-plan-build "\$@"
EOF
  # shellcheck disable=2086
  $bb chmod $v 755 "$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin/build

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$bash_path/bin/bash,g" -i "$HAB_STUDIO_ROOT"/etc/passwd

  $bb cat >>"$HAB_STUDIO_ROOT"/etc/profile <<PROFILE
# Add hab to the default PATH at the front so any wrapping scripts will
# be found and called first
export PATH=$HAB_ROOT_PATH/bin:\$PATH
export HAB_BINLINK_DIR=/hab/bin

# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

# Set TERMINFO so hab can give us a delightful experience.
export TERMINFO
TERMINFO=$(_pkgpath_for core/build-tools-ncurses)/share/terminfo

if [[ -n "\${STUDIO_ENTER:-}" ]]; then
  unset STUDIO_ENTER
  source /etc/profile.enter
fi

# Add command line completion
source <(hab cli completers --shell bash)
PROFILE

  $bb cat >"$HAB_STUDIO_ROOT"/etc/profile.enter <<PROFILE_ENTER
# Source /src/.studiorc so we can apply user-specific configuration
if [[ -f /src/.studiorc && -z "\${HAB_STUDIO_NOSTUDIORC:-}" ]]; then
  echo "--> Detected and loading /src/.studiorc"
  echo ""
  source /src/.studiorc
fi
PROFILE_ENTER

  echo "${run_user}:x:42:42:root:/:/bin/sh" >>"$HAB_STUDIO_ROOT"/etc/passwd
  echo "${run_group}:x:42:${run_user}" >>"$HAB_STUDIO_ROOT"/etc/group

  studio_env_command="$coreutils_path/bin/env"

  # This installs any additional packages before starting the studio.
  # It is useful in scenarios where you have a newer version of a package
  # and want habitat to pick the newer locally installed version during 
  # a studio build. We do exactly this during the package refresh process.
  if [ -n "${HAB_STUDIO_INSTALL_PKGS:-}" ]; then
    echo "Installing additional packages in bootstrap studio"
    deps=$(echo "$HAB_STUDIO_INSTALL_PKGS" | "$coreutils_path"/bin/tr ":" "\n")
    for dep in $deps; do
      _hab pkg install "$dep"
    done
  fi

}

# Intentionally using a subshell here so `unset` doesn't affect the
# caller's environment.
_hab() (
  # We remove a couple of env vars we do not want for this instance of the studio
  unset HAB_CACHE_KEY_PATH
  unset HAB_BLDR_CHANNEL
  # Set the HAB_LICENSE because the license accepted files don't yet exist on the chroot filesystem
  $bb env FS_ROOT="$HAB_STUDIO_ROOT" HAB_LICENSE="$HAB_LICENSE" "$hab" "$@"
)

_pkgpath_for() {
  _hab pkg path "$1" | $bb sed -e "s,^$HAB_STUDIO_ROOT,,g"
}
