studio_type="stage1"
studio_path="/bin:/tools/bin"
studio_env_command="/tools/bin/env"
studio_enter_environment=
studio_enter_command="/tools/bin/bash --login +h"
studio_build_environment=
studio_build_command=
studio_run_environment=
studio_run_command=

: ${STAGE1_TOOLS_URL:=https://s3-us-west-2.amazonaws.com/habitat-studio-stage1/habitat-studio-stage1-20160612022150.tar.xz}
: ${TAR_DIR:=/tmp}

finish_setup() {
  if [ -n "$HAB_ORIGIN_KEYS" ]; then
    for key in $(echo $HAB_ORIGIN_KEYS | $bb tr ',' ' '); do
      info "Importing $key secret origin key"
      # There's a method to this madness: `$hab` is the raw path to `hab`
      # will use the outside cache key path, whereas the `_hab` function has
      # the `$FS_ROOT` set for the inside of the Studio. We're copying from
      # the outside in, using `hab` twice. I love my job.

      # if we don't set +e here, then the subshell exits upon
      # error without any output
      set +e
      key_text=$($hab origin key export --type secret $key)
      # capture the result now before calling other commands
      # that will overwrite the result
      local result=$?
      # reenable exit upon error
      set -e

      # NOTE: quotes MUST appear around ${key_text} to preserve
      # newlines in the hab export output
      if [ $result -eq 0 ]; then
        echo "${key_text}" | _hab origin key import
      else
        echo "Error exporting $key key"
        # key_text will contain an error message
        echo "${key_text}"
        exit 1
      fi

    done
  fi

  if [ -x "$HAB_STUDIO_ROOT/tools/bin/bash" ]; then
    return 0
  fi

  tar_file="$TAR_DIR/$($bb basename $STAGE1_TOOLS_URL)"

  if [ ! -f $tar_file ]; then
    trap '$bb rm -f $tar_file; exit $?' INT TERM EXIT
    info "Downloading $STAGE1_TOOLS_URL"
    $bb wget $STAGE1_TOOLS_URL -O $tar_file
    trap - INT TERM EXIT
  fi

  info "Extracting $($bb basename $tar_file)"
  $bb xzcat $tar_file | $bb tar xf - -C $HAB_STUDIO_ROOT

  # Create symlinks from the minimal toolchain installed under `/tools` into
  # the root of the chroot environment. This is done to satisfy tools such as
  # `make(1)` which expect `/bin/sh` to exist.

  $bb ln -sf $v /tools/bin/bash $HAB_STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/cat $HAB_STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/echo $HAB_STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/pwd $HAB_STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/stty $HAB_STUDIO_ROOT/bin

  $bb ln -sf $v /tools/bin/perl $HAB_STUDIO_ROOT/usr/bin
  $bb ln -sf $v /tools/lib/libgcc_s.so $HAB_STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libgcc_s.so.1 $HAB_STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so $HAB_STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so.6 $HAB_STUDIO_ROOT/usr/lib
  $bb sed 's/tools/usr/' $HAB_STUDIO_ROOT/tools/lib/libstdc++.la > $HAB_STUDIO_ROOT/usr/lib/libstdc++.la
  $bb ln -sf $v bash $HAB_STUDIO_ROOT/bin/sh

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e 's,/bin/sh,/bin/bash,g' -i $HAB_STUDIO_ROOT/etc/passwd

  $bb cat >> $HAB_STUDIO_ROOT/etc/profile <<'PROFILE'
# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE
}

_hab() {
  $bb env FS_ROOT=$HAB_STUDIO_ROOT $hab $*
}
