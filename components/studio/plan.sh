pkg_name=hab-studio
pkg_origin=core
pkg_version=0.6.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
pkg_deps=()
pkg_build_deps=(core/coreutils core/tar core/xz core/wget core/busybox-static core/hab)
pkg_bin_dirs=(bin)

do_build() {
  cp -v $PLAN_CONTEXT/bin/hab-studio.sh hab-studio
  cp -v $PLAN_CONTEXT/libexec/hab-studio-type-*.sh .

  # Embed the release version and author information of the program.
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i hab-studio
}

do_install() {
  install -v -D hab-studio $pkg_prefix/bin/hab-studio
  for f in `ls hab-studio-type-*.sh`; do
    install -v -D $f $pkg_prefix/libexec/$f
  done

  lbb="$pkg_prefix/libexec/busybox"

  # Install a copy of a statically built busybox under `libexec/`
  install -v -D $(pkg_path_for busybox-static)/bin/busybox $lbb

  hab_dir=$(cat $(pkg_path_for hab)/IDENT | tr '/' '-')
  install -v -D $(pkg_path_for hab)/bin/hab \
    $pkg_prefix/libexec/$hab_dir/bin/hab
  ln -sv $hab_dir/bin/hab $pkg_prefix/libexec/hab
}

# Turn the remaining default phases into no-ops

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}
