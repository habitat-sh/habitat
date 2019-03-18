# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# shellcheck disable=2034
studio_type="stage1"
studio_path="/bin:/tools/bin"
studio_env_command="/tools/bin/env"
studio_enter_environment=
studio_enter_command="/tools/bin/bash --login +h"
studio_build_environment=
studio_build_command=
studio_run_environment=
studio_run_command="/tools/bin/bash --login"

: "${STAGE1_TOOLS_URL:=http://s3-us-west-2.amazonaws.com/habitat-studio-stage1/habitat-studio-stage1-20180312233639.tar.xz}"
: "${TAR_DIR:=/tmp}"

finish_setup() {
  if [ -n "$HAB_ORIGIN_KEYS" ]; then
    # shellcheck disable=2154
    for key in $(echo "$HAB_ORIGIN_KEYS" | $bb tr ',' ' '); do
      local key_text
      # Import the secret origin key, required for signing packages
      info "Importing '$key' secret origin key"
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
  fi

  if [ -x "$HAB_STUDIO_ROOT/tools/bin/bash" ]; then
    return 0
  fi

  tar_file="$TAR_DIR/$($bb basename $STAGE1_TOOLS_URL)"

  if [ ! -f "$tar_file" ]; then
    trap '$bb rm -f $tar_file; exit $?' INT TERM EXIT
    info "Downloading $STAGE1_TOOLS_URL"
    $bb wget $STAGE1_TOOLS_URL -O "$tar_file"
    trap - INT TERM EXIT
  fi

  info "Extracting $($bb basename "$tar_file")"
  $bb xzcat "$tar_file" | $bb tar xf - -C "$HAB_STUDIO_ROOT"

  # Create symlinks from the minimal toolchain installed under `/tools` into
  # the root of the chroot environment. This is done to satisfy tools such as
  # `make(1)` which expect `/bin/sh` to exist.

  # shellcheck disable=2086
  {
  # shellcheck disable=2154
  $bb ln -sf $v /tools/bin/bash "$HAB_STUDIO_ROOT"/bin
  $bb ln -sf $v /tools/bin/cat "$HAB_STUDIO_ROOT"/bin
  $bb ln -sf $v /tools/bin/echo "$HAB_STUDIO_ROOT"/bin
  $bb ln -sf $v /tools/bin/pwd "$HAB_STUDIO_ROOT"/bin
  $bb ln -sf $v /tools/bin/stty "$HAB_STUDIO_ROOT"/bin

  $bb ln -sf $v /tools/bin/perl "$HAB_STUDIO_ROOT"/usr/bin
  $bb ln -sf $v /tools/lib/libgcc_s.so "$HAB_STUDIO_ROOT"/usr/lib
  $bb ln -sf $v /tools/lib/libgcc_s.so.1 "$HAB_STUDIO_ROOT"/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so "$HAB_STUDIO_ROOT"/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so.6 "$HAB_STUDIO_ROOT"/usr/lib    
  } # end shellcheck disable
  # TODO fn: Used for older versions of the stage1 tarball, so this check can
  # eventually be removed as newer tarballs will most likely always skip this
  # step.
  if [ -f "$HAB_STUDIO_ROOT/tools/lib/libstdc++.la" ]; then
    $bb sed 's/tools/usr/' "$HAB_STUDIO_ROOT"/tools/lib/libstdc++.la \
      > "$HAB_STUDIO_ROOT"/usr/lib/libstdc++.la
  fi
  # shellcheck disable=2086
  $bb ln -sf $v bash "$HAB_STUDIO_ROOT"/bin/sh

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e 's,/bin/sh,/bin/bash,g' -i "$HAB_STUDIO_ROOT"/etc/passwd

  $bb cat >> "$HAB_STUDIO_ROOT"/etc/profile <<'PROFILE'
# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE
}

# Intentionally using a subshell here so `unset` doesn't affect the
# caller's environment.
_hab() (
    unset HAB_CACHE_KEY_PATH
    $bb env FS_ROOT="$HAB_STUDIO_ROOT" "$hab" "$@"
)
