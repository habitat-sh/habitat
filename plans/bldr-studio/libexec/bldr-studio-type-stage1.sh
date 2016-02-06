studio_type="stage1"
studio_path="/tools/bin"
studio_env_command="/tools/bin/env"
studio_enter_environment=
studio_enter_command="/tools/bin/bash --login +h"
studio_build_environment=
studio_build_command=
studio_run_environment=

: ${STAGE1_TOOLS_URL:=http://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/lfs-tools-20160205204818.tar.xz}
: ${TAR_DIR:=/tmp}

finish_setup() {
  if [ -x "$STUDIO_ROOT/tools/bin/bash" ]; then
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
  $bb xzcat $tar_file | $bb tar xf - -C $STUDIO_ROOT

  # Create symlinks from the minimal toolchain installed under `/tools` into
  # the root of the chroot environment. This is done to satisfy tools such as
  # `make(1)` which expect `/bin/sh` to exist.

  $bb ln -sf $v /tools/bin/bash $STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/cat $STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/echo $STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/pwd $STUDIO_ROOT/bin
  $bb ln -sf $v /tools/bin/stty $STUDIO_ROOT/bin

  $bb ln -sf $v /tools/bin/perl $STUDIO_ROOT/usr/bin
  $bb ln -sf $v /tools/lib/libgcc_s.so $STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libgcc_s.so.1 $STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so $STUDIO_ROOT/usr/lib
  $bb ln -sf $v /tools/lib/libstdc++.so.6 $STUDIO_ROOT/usr/lib
  $bb sed 's/tools/usr/' $STUDIO_ROOT/tools/lib/libstdc++.la > $STUDIO_ROOT/usr/lib/libstdc++.la
  $bb ln -sf $v bash $STUDIO_ROOT/bin/sh

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e 's,/bin/sh,/bin/bash,g' -i $STUDIO_ROOT/etc/passwd

  $bb cat >> $STUDIO_ROOT/etc/profile <<'PROFILE'
# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE
}
