# shellcheck disable=2034
pkg_name=hab-studio
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=()
pkg_build_deps=(core/coreutils
                core/tar
                core/xz
                core/wget
                core/busybox-static
                core/hab)
pkg_bin_dirs=(bin)

do_build() {
  cp -v "$SRC_PATH"/bin/hab-studio.sh hab-studio
  cp -v "$SRC_PATH"/libexec/hab-studio-profile.sh .
  cp -v "$SRC_PATH"/libexec/hab-studio-type-*.sh .

  # Embed the release version and author information of the program.
  # shellcheck disable=2154
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i hab-studio
}

do_install() {
  # shellcheck disable=2154
  install -v -D hab-studio "$pkg_prefix"/bin/hab-studio
  install -v -D hab-studio-profile.sh "$pkg_prefix"/libexec/hab-studio-profile.sh
  for f in hab-studio-type-*.sh; do
    [[ -e $f ]] || break # see http://mywiki.wooledge.org/BashPitfalls#pf1
    install -v -D "$f" "$pkg_prefix"/libexec/"$f"
  done

  # Busybox allows us to link each command busybox includes to the statically
  # built busybox binary and it will behave as if the command was called. By
  # installing copies of all busybox commands under `libexec` we will have
  # access to them while we build up the studio in the studio scripts instead
  # of prefixing commands with $bb.
  lbb="$pkg_prefix/libexec/busybox"

  install -v -D "$(pkg_path_for busybox-static)"/bin/busybox "$lbb"
  for i in $("$pkg_prefix"/libexec/busybox --list); do
    ln -sv busybox "$pkg_prefix"/libexec/"$i"
  done

  # shellcheck disable=2154
  hab_dir=$(tr '/' '-' < "$(pkg_path_for hab)"/IDENT)
  install -v -D "$(pkg_path_for hab)"/bin/hab \
    "$pkg_prefix"/libexec/hab
}
