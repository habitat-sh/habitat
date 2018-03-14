# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# shellcheck disable=2034
studio_type="busybox"
studio_path="/opt/busybox"
studio_env_command="/opt/busybox/busybox env"
studio_enter_environment=
studio_enter_command="/opt/busybox/busybox sh -l"
studio_build_environment=
studio_build_command=
studio_run_environment=
studio_run_command="/opt/busybox/busybox sh -l"

finish_setup() {
  # Copy in the busybox binary under `/opt/busybox`
  # shellcheck disable=2154,2086
  mkdir -p $v "$HAB_STUDIO_ROOT"/opt/busybox
  # shellcheck disable=2154,2086
  cp $v "$libexec_path"/busybox "$HAB_STUDIO_ROOT"/opt/busybox/

  if [ ! -f "$HAB_STUDIO_ROOT/opt/busybox/sh" ]; then
    # Symlink all tools to busybox under `/opt/busybox`
    for c in $("$HAB_STUDIO_ROOT"/opt/busybox/busybox --list); do
      # shellcheck disable=2086
      ln -sf $v busybox "$HAB_STUDIO_ROOT"/opt/busybox/$c
    done
  fi

  # Set the login shell for any relevant user to be busybox's `sh`
  sed -e 's,/bin/sh,/opt/busybox/sh,g' -i "$HAB_STUDIO_ROOT"/etc/passwd
}
