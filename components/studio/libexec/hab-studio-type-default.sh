studio_type="default"
studio_path="$HAB_ROOT_PATH/bin"
studio_enter_environment=
studio_enter_command="$HAB_ROOT_PATH/bin/hab pkg exec core/hab-backline bash --login +h"
studio_build_environment=
studio_build_command="record \${1:-} $HAB_ROOT_PATH/bin/build"
studio_run_environment=
studio_run_command="$HAB_ROOT_PATH/bin/hab pkg exec core/hab-backline bash -l"

pkgs="core/hab-backline"

run_user="hab"
run_group="$run_user"

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

  if [ -h "$HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/hab" ]; then
    return 0
  fi

  for pkg in $pkgs; do
    _hab install $pkg
  done

  local bash_path=$(_pkgpath_for core/bash)
  local coreutils_path=$(_pkgpath_for core/coreutils)

  $bb mkdir -p $v $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin

  # Put `hab` on the default `$PATH`
  _hab pkg binlink --dest $HAB_ROOT_PATH/bin core/hab hab

  # Create `/bin/{sh,bash}` for software that hardcodes these shells
  _hab pkg binlink core/bash bash
  _hab pkg binlink core/bash sh

  # Create a wrapper to `build` so that any calls to it have a super-stripped
  # `$PATH` and not whatever augmented version is currently in use. This should
  # mean that running `build` from inside a `studio enter` and running `studio
  # build` leads to the exact same experience, at least as far as initial
  # `$PATH` is concerned.
  $bb cat <<EOF > $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/build
#!$bash_path/bin/sh
exec $HAB_ROOT_PATH/bin/hab pkg exec core/hab-plan-build hab-plan-build \$*
EOF
  $bb chmod $v 755 $HAB_STUDIO_ROOT$HAB_ROOT_PATH/bin/build

  # Set the login shell for any relevant user to be `/bin/bash`
  $bb sed -e "s,/bin/sh,$bash_path/bin/bash,g" -i $HAB_STUDIO_ROOT/etc/passwd

  $bb cat >> $HAB_STUDIO_ROOT/etc/profile <<PROFILE
# Add hab to the default PATH at the front so any wrapping scripts will
# be found and called first
export PATH=$HAB_ROOT_PATH/bin:\$PATH

# Colorize grep/egrep/fgrep by default
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

PROFILE

  echo "${run_user}:x:42:42:root:/:/bin/sh" >> $HAB_STUDIO_ROOT/etc/passwd
  echo "${run_group}:x:42:${run_user}" >> $HAB_STUDIO_ROOT/etc/group

  studio_env_command="$coreutils_path/bin/env"
}

_hab() {
  $bb env FS_ROOT=$HAB_STUDIO_ROOT $hab $*
}

_pkgpath_for() {
  _hab pkg path $1 | $bb sed -e "s,^$HAB_STUDIO_ROOT,,g"
}
