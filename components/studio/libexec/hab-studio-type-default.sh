studio_type="default"
studio_path="$HAB_ROOT_PATH/bin"
studio_enter_environment=
studio_enter_command="$HAB_ROOT_PATH/bin/hab-bpm exec core/hab-backline bash --login +h"
studio_build_environment=
studio_build_command="record \${1:-} $HAB_ROOT_PATH/bin/build"
studio_run_environment=
studio_run_command="$HAB_ROOT_PATH/bin/hab-bpm exec core/hab-backline bash -l"

pkgs="core/hab-bpm core/hab-backline core/hab-studio"

finish_setup() {
  if [ -x "$HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-bpm" ]; then
    return 0
  fi

  for pkg in $pkgs; do
    _bpm install $pkg
  done

  local bpm_path=$(_pkgpath_for core/hab-bpm)
  local bash_path=$(_pkgpath_for core/bash)
  local coreutils_path=$(_pkgpath_for core/coreutils)

  $bb mkdir -p $v $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin

  # Put `hab-bpm` on the default `$PATH` and ensure that it gets a sane shell
  # and initial `busybox` (sane being its own vendored version)
  $bb cat <<EOF > $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-bpm
#!$bpm_path/libexec/busybox sh
export BUSYBOX=$bpm_path/libexec/busybox
exec \$BUSYBOX sh $bpm_path/bin/hab-bpm \$*
EOF
  $bb chmod $v 755 $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-bpm

  # Create a wrapper to `build` so that any calls to it have a super-stripped
  # `$PATH` and not whatever augmented version is currently in use. This should
  # mean that running `build` from inside a `studio enter` and running `studio
  # build` leads to the exact same experience, at least as far as initial
  # `$PATH` is concerned.
  $bb cat <<EOF > $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/build
#!$bpm_path/libexec/busybox sh
exec $HAB_ROOT_PATH/bin/hab-bpm exec core/hab-plan-build hab-plan-build \$*
EOF
  $bb chmod $v 755 $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/build

  # Create a wrapper to studio
  $bb cat <<EOF > $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/studio
#!$bpm_path/libexec/busybox sh
exec $HAB_ROOT_PATH/bin/hab-bpm exec core/hab-studio hab-studio \$*
EOF
  $bb chmod $v 755 $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/studio

  $bb ln -s $v $bash_path/bin/bash $HAB_STUDIO_ROOT/bin/bash
  $bb ln -s $v bash $HAB_STUDIO_ROOT/bin/sh

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$bash_path/bin/bash,g" -i $HAB_STUDIO_ROOT/etc/passwd

  $bb cat >> $HAB_STUDIO_ROOT/etc/profile <<PROFILE
# Add hab-bpm to the default PATH at the front so any wrapping scripts will
# be found and called first
export PATH=$HAB_ROOT_PATH/bin:\$PATH

# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE

  # TODO FIN: Remove when public origin keys are downloaded on package installation
  $bb mkdir -p $HAB_STUDIO_ROOT$HAB_ROOT_PATH/cache/keys
  (cd $HAB_STUDIO_ROOT$HAB_ROOT_PATH/cache/keys; $bb wget http://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/core-20160423193745.pub)

  studio_env_command="$coreutils_path/bin/env"
}

_bpm() {
  $bb env BUSYBOX=$bb FS_ROOT=$HAB_STUDIO_ROOT $bb sh $bpm $*
}

_pkgpath_for() {
  _bpm pkgpath $1 | $bb sed -e "s,^$HAB_STUDIO_ROOT,,g"
}
