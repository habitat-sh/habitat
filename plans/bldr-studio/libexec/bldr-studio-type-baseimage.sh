studio_type="baseimage"
studio_path="/opt/bldr/bin"
studio_enter_environment=
studio_build_environment=
studio_build_command="/opt/bldr/bin/build"
studio_run_environment=

bldr_pkgs="chef/hab-bpm chef/bldr chef/busybox-static"
PKGS="$PKGS"

finish_setup() {
  if [ -x "$STUDIO_ROOT/opt/bldr/bin/hab-bpm" ]; then
    return 0
  fi

  for embed in $PKGS; do
    if [ -d "/opt/bldr/pkgs/$embed" ]; then
      echo "> Using local package for $embed"
      embed_path=$(_outside_pkgpath_for $embed)
      $bb mkdir -p $STUDIO_ROOT/$embed_path
      $bb cp -ra $embed_path/* $STUDIO_ROOT/$embed_path
      for tdep in $($bb cat $embed_path/TDEPS); do
        echo "> Using local package for $tdep via $embed"
        $bb mkdir -p $STUDIO_ROOT/opt/bldr/pkgs/$tdep
        $bb cp -ra /opt/bldr/pkgs/$tdep/* $STUDIO_ROOT/opt/bldr/pkgs/$tdep
      done
    else
      _bpm install $embed
    fi
  done

  for pkg in $bldr_pkgs; do
    _bpm install $pkg
  done

  local bpm_path=$(_pkgpath_for chef/hab-bpm)
  local bldr_path=$(_pkgpath_for chef/bldr)
  local busybox_path=$(_pkgpath_for chef/busybox-static)

  local full_path=""
  for path_pkg in $PKGS chef/bldr chef/busybox-static; do
    local path_file="$STUDIO_ROOT/$(_pkgpath_for $path_pkg)/PATH"
    if [ -f "$path_file" ]; then
      if [ -z "$full_path" ]; then
        full_path="$($bb cat $path_file)"
      else
        full_path="$full_path:$($bb cat $path_file)"
      fi
    fi

    local tdeps_file="$STUDIO_ROOT/$(_pkgpath_for $path_pkg)/TDEPS"
    if [ -f "$tdeps_file" ]; then
      for tdep in $($bb cat $tdeps_file); do
        local tdep_path_file="$STUDIO_ROOT/$(_pkgpath_for $tdep)/PATH"
        if [ -f "$tdep_path_file" ]; then
          full_path="$full_path:$($bb cat $tdep_path_file)"
        fi
      done
    fi
  done
  full_path="$full_path:/opt/bldr/bin"

  studio_path="$full_path"
  studio_enter_command="${busybox_path}/bin/sh --login"

  $bb mkdir -p $v $STUDIO_ROOT/opt/bldr/bin

  # Put `hab-bpm` on the default `$PATH` and ensure that it gets a sane shell
  # and initial `busybox` (sane being its own vendored version)
  $bb cat <<EOF > $STUDIO_ROOT/opt/bldr/bin/hab-bpm
#!$busybox_path/bin/sh
exec $bpm_path/bin/hab-bpm \$*
EOF
  $bb chmod $v 755 $STUDIO_ROOT/opt/bldr/bin/hab-bpm
  $bb ln -s $v $busybox_path/bin/sh $STUDIO_ROOT/bin/bash
  $bb ln -s $v $busybox_path/bin/sh $STUDIO_ROOT/bin/sh
  $bb ln -s $v $bldr_path/bin/bldr $STUDIO_ROOT/opt/bldr/bin/bldr

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$busybox_path/bin/bash,g" -i $STUDIO_ROOT/etc/passwd

  $bb cat <<PROFILE > $STUDIO_ROOT/etc/profile
# Add hab-bpm to the default \$PATH at the front so any wrapping scripts will
# be found and called first
export PATH=$full_path:\$PATH

# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE

  $bb cat <<EOT > $STUDIO_ROOT/etc/resolv.conf
nameserver 8.8.8.8
nameserver 8.8.4.4
EOT

  $bb cat <<EOT > $STUDIO_ROOT/etc/nsswitch.conf
passwd:     files
group:      files
shadow:     files

hosts:      files dns
networks:   files

rpc:        files
services:   files
EOT
  echo bldr:x:42:42:root:/:/bin/sh >> $STUDIO_ROOT/etc/passwd
  echo bldr:x:42:bldr >> $STUDIO_ROOT/etc/group
  for X in null ptmx random stdin stdout stderr tty urandom zero
  do
      $bb cp -a /dev/$X $STUDIO_ROOT/dev
  done

  $bb cat <<EOT > $STUDIO_ROOT/init.sh
#!$busybox_path/bin/sh
export PATH=$full_path
exec $bldr_path/bin/bldr "\$@"
EOT
  $bb chmod a+x $STUDIO_ROOT/init.sh

  $bb rm $STUDIO_ROOT/opt/bldr/cache/pkgs/*

  studio_env_command="$busybox_path/bin/env"
}

_bpm() {
  $bb env BUSYBOX=$bb FS_ROOT=$STUDIO_ROOT $bb sh $bpm $*
}

_pkgpath_for() {
  _bpm pkgpath $1 | $bb sed -e "s,^$STUDIO_ROOT,,g"
}

_outside_pkgpath_for() {
  $bb env BUSYBOX=$bb $bb sh $bpm pkgpath $1
}

