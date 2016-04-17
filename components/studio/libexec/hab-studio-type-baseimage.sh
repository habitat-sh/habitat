studio_type="baseimage"
studio_path="$HAB_ROOT_PATH/bin"
studio_enter_environment=
studio_build_environment=
studio_build_command="$HAB_ROOT_PATH/bin/build"
studio_run_environment=
studio_run_command=

base_pkgs="core/hab-bpm core/hab-sup core/busybox-static"
: ${PKGS:=}

run_user="hab"
run_group="$run_group"

finish_setup() {
  if [ -x "$HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-sup" ]; then
    return 0
  fi

  for embed in $PKGS; do
    if [ -d "$HAB_PKG_PATH/$embed" ]; then
      echo "> Using local package for $embed"
      embed_path=$(_outside_pkgpath_for $embed)
      $bb mkdir -p $HAB_STUDIO_ROOT/$embed_path
      $bb cp -ra $embed_path/* $HAB_STUDIO_ROOT/$embed_path
      for tdep in $($bb cat $embed_path/TDEPS); do
        echo "> Using local package for $tdep via $embed"
        $bb mkdir -p $HAB_STUDIO_ROOT$HAB_PKG_PATH/$tdep
        $bb cp -ra $HAB_PKG_PATH/$tdep/* $HAB_STUDIO_ROOT$HAB_PKG_PATH/$tdep
      done
    else
      _bpm install $embed
    fi
  done

  for pkg in $base_pkgs; do
    _bpm install $pkg
  done

  local bpm_path=$(_pkgpath_for core/hab-bpm)
  local sup_path=$(_pkgpath_for core/hab-sup)
  local busybox_path=$(_pkgpath_for core/busybox-static)

  local full_path=""
  for path_pkg in $PKGS core/hab-sup core/busybox-static; do
    local path_file="$HAB_STUDIO_ROOT/$(_pkgpath_for $path_pkg)/PATH"
    if [ -f "$path_file" ]; then
      if [ -z "$full_path" ]; then
        full_path="$($bb cat $path_file)"
      else
        full_path="$full_path:$($bb cat $path_file)"
      fi
    fi

    local tdeps_file="$HAB_STUDIO_ROOT/$(_pkgpath_for $path_pkg)/TDEPS"
    if [ -f "$tdeps_file" ]; then
      for tdep in $($bb cat $tdeps_file); do
        local tdep_path_file="$HAB_STUDIO_ROOT/$(_pkgpath_for $tdep)/PATH"
        if [ -f "$tdep_path_file" ]; then
          full_path="$full_path:$($bb cat $tdep_path_file)"
        fi
      done
    fi
  done
  full_path="$full_path:$HAB_ROOT_PATH/bin"

  studio_path="$full_path"
  studio_enter_command="${busybox_path}/bin/sh --login"

  $bb mkdir -p $v $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin

  # Put `hab-bpm` on the default `$PATH` and ensure that it gets a sane shell
  # and initial `busybox` (sane being its own vendored version)
  $bb cat <<EOF > $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-bpm
#!$busybox_path/bin/sh
exec $bpm_path/bin/hab-bpm \$*
EOF
  $bb chmod $v 755 $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-bpm
  $bb ln -s $v $busybox_path/bin/sh $HAB_STUDIO_ROOT/bin/bash
  $bb ln -s $v $busybox_path/bin/sh $HAB_STUDIO_ROOT/bin/sh
  $bb ln -s $v $sup_path/bin/hab-sup $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab-sup

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$busybox_path/bin/bash,g" -i $HAB_STUDIO_ROOT/etc/passwd

  $bb cat <<PROFILE > $HAB_STUDIO_ROOT/etc/profile
# Add hab-bpm to the default \$PATH at the front so any wrapping scripts will
# be found and called first
export PATH=$full_path:\$PATH

# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE

  $bb cat <<EOT > $HAB_STUDIO_ROOT/etc/resolv.conf
nameserver 8.8.8.8
nameserver 8.8.4.4
EOT

  $bb cat <<EOT > $HAB_STUDIO_ROOT/etc/nsswitch.conf
passwd:     files
group:      files
shadow:     files

hosts:      files dns
networks:   files

rpc:        files
services:   files
EOT
  echo "${run_user}:x:42:42:root:/:/bin/sh" >> $HAB_STUDIO_ROOT/etc/passwd
  echo "${run_group}:x:42:${run_user}" >> $HAB_STUDIO_ROOT/etc/group

  local sup=$sup_path/bin/hab-sup
  $bb touch $HAB_STUDIO_ROOT/.hab_pkg
  $bb cat <<EOT > $HAB_STUDIO_ROOT/init.sh
#!$busybox_path/bin/sh
export PATH=$full_path
case \$1 in
  -h|--help|help|-V|--version) exec $sup "\$@";;
  -*) exec $sup start \$(cat /.hab_pkg) "\$@";;
  *) exec $sup "\$@";;
esac
EOT
  $bb chmod a+x $HAB_STUDIO_ROOT/init.sh

  $bb rm $HAB_STUDIO_ROOT$HAB_CACHE_ARTIFACT_PATH/*

  studio_env_command="$busybox_path/bin/env"
}

_bpm() {
  $bb env BUSYBOX=$bb FS_ROOT=$HAB_STUDIO_ROOT $bb sh $bpm $*
}

_pkgpath_for() {
  _bpm pkgpath $1 | $bb sed -e "s,^$HAB_STUDIO_ROOT,,g"
}

_outside_pkgpath_for() {
  $bb env BUSYBOX=$bb $bb sh $bpm pkgpath $1
}
