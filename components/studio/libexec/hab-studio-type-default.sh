#!/bin/sh

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# shellcheck disable=2034
studio_type="default"
studio_path="$HAB_ROOT_PATH/bin"
studio_enter_environment="STUDIO_ENTER=true"
studio_enter_command="$HAB_ROOT_PATH/bin/hab pkg exec core/hab-backline bash --login +h"
studio_build_environment=
studio_build_command="_record_build $HAB_ROOT_PATH/bin/build"
studio_run_environment=
studio_run_command="$HAB_ROOT_PATH/bin/hab pkg exec core/hab-backline bash --login"

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
      if key_text=$($hab origin key export --type secret "$key"); then
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
      if key_text=$($hab origin key export --type public "$key" 2> /dev/null); then
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
  _hab install "$HAB_STUDIO_BACKLINE_PKG"

  bash_path=$(_pkgpath_for core/bash)
  coreutils_path=$(_pkgpath_for core/coreutils)

  # shellcheck disable=2086,2154
  $bb mkdir -p $v "$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin

  # Put `hab` on the default `$PATH`
  _hab pkg binlink --dest "$HAB_ROOT_PATH"/bin core/hab hab

  # Create `/bin/{sh,bash}` for software that hardcodes these shells
  _hab pkg binlink core/bash bash
  _hab pkg binlink core/bash sh

  # Create a wrapper to `build` so that any calls to it have a super-stripped
  # `$PATH` and not whatever augmented version is currently in use. This should
  # mean that running `build` from inside a `studio enter` and running `studio
  # build` leads to the exact same experience, at least as far as initial
  # `$PATH` is concerned.
  $bb cat <<EOF > "$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin/build
#!$bash_path/bin/sh
exec $HAB_ROOT_PATH/bin/hab pkg exec core/hab-plan-build hab-plan-build "\$@"
EOF
  # shellcheck disable=2086
  $bb chmod $v 755 "$HAB_STUDIO_ROOT""$HAB_ROOT_PATH"/bin/build

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$bash_path/bin/bash,g" -i "$HAB_STUDIO_ROOT"/etc/passwd

  $bb cat >> "$HAB_STUDIO_ROOT"/etc/profile <<PROFILE
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
TERMINFO=$(_pkgpath_for core/ncurses)/share/terminfo

emacs() {
  if type -P emacs > /dev/null; then
    command emacs "\$@"
  else
    mg "\$@"
  fi
}

if [[ -n "\${HAB_STUDIO_SUP}" ]]; then
  # This environment variable does not handle spaces well, so we'll re-add
  # them...
  HAB_STUDIO_SUP="\$(echo "\$HAB_STUDIO_SUP" | sed 's/__sp__/ /g')"
fi

sup-run() {
  mkdir -p /hab/sup/default
  echo "--> Launching the Habitat Supervisor in the background..."
  echo "    Running: hab sup run \$@"
  setsid hab sup run "\$@" > /hab/sup/default/sup.log 2>&1 &
  disown \$!
  echo "    * Use 'hab svc start' & 'hab svc stop' to start and stop services"
  echo "    * Use 'sup-log' to tail the Supervisor's output (Ctrl+c to stop)"
  echo "    * Use 'sup-term' to terminate the Supervisor"
  if [[ -z "\${HAB_STUDIO_SUP:-}" ]]; then
    echo "    * To pass custom arguments to run the Supervisor, export"
    echo "      'HAB_STUDIO_SUP' with the arguments before running"
    echo "      'hab studio enter'."
  fi
  echo ""
}

sup-term() {
  if hab sup term ; then
    echo "--> Killed background running Habitat Supervisor."
  else
    echo "--> Error killing Supervisor; it may not be running"
  fi
}

sup-log() {
  mkdir -p /hab/sup/default
  touch /hab/sup/default/sup.log
  echo "--> Tailing the Habitat Supervisor's output (use 'Ctrl+c' to stop)"
  tail -f /hab/sup/default/sup.log
}

alias sr='sup-run'
alias st='sup-term'
alias sl='sup-log'

if [[ -n "\${STUDIO_ENTER:-}" ]]; then
  unset STUDIO_ENTER
  source /etc/profile.enter
fi

# Add command line completion
source <(hab cli completers --shell bash)
PROFILE

  $bb cat > "$HAB_STUDIO_ROOT"/etc/profile.enter <<PROFILE_ENTER
# Source /src/.studiorc so we can apply user-specific configuration
if [[ -f /src/.studiorc && -z "\${HAB_STUDIO_NOSTUDIORC:-}" ]]; then
  echo "--> Detected and loading /src/.studiorc"
  echo ""
  source /src/.studiorc
fi

# Automatically run the Habitat Supervisor
case "\${HAB_STUDIO_SUP:-}" in
  false|FALSE|no|NO|0)
    # If false, we don't run the Supervisor
    ;;
  *)
    # shellcheck disable=2086
    sup-run \${HAB_STUDIO_SUP:-}
    echo "--> To prevent a Supervisor from running automatically in your"
    echo "    Studio, export 'HAB_STUDIO_SUP=false' before running"
    echo "    'hab studio enter'."
    echo ""
    ;;
esac
PROFILE_ENTER

  echo "${run_user}:x:42:42:root:/:/bin/sh" >> "$HAB_STUDIO_ROOT"/etc/passwd
  echo "${run_group}:x:42:${run_user}" >> "$HAB_STUDIO_ROOT"/etc/group

  studio_env_command="$coreutils_path/bin/env"
}

# Intentionally using a subshell here so `unset` doesn't affect the
# caller's environment.
_hab() (
    # We remove a couple of env vars we do not want for this instance of the studio
    unset HAB_CACHE_KEY_PATH
    unset HAB_BLDR_CHANNEL
    $bb env FS_ROOT="$HAB_STUDIO_ROOT" "$hab" "$@"
)

_pkgpath_for() {
  _hab pkg path "$1" | $bb sed -e "s,^$HAB_STUDIO_ROOT,,g"
}
