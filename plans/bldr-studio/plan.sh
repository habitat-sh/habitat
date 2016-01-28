pkg_name=bldr-studio
pkg_derivation=chef
pkg_version=0.1.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/tar chef/xz chef/wget chef/busybox-static chef/bpm)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  cp -v $PLAN_CONTEXT/bin/studio studio
  cp -v $PLAN_CONTEXT/libexec/bldr-studio-type-*.sh .

  # Embed the release version and author information of the program.
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_rel,g" \
    -i studio
}

do_install() {
  install -v -D studio $pkg_path/bin/studio
  for f in `ls bldr-studio-type-*.sh`; do
    install -v -D $f $pkg_path/libexec/$f
  done

  lbb="$pkg_path/libexec/busybox"

  # Install a copy of a statically built busybox under `libexec/`
  install -v -D $(pkg_path_for busybox-static)/bin/busybox $lbb

  bpm_dir=$(cat $(pkg_path_for bpm)/IDENT | tr '/' '-')
  install -v -D $(pkg_path_for bpm)/bin/bpm \
    $pkg_path/libexec/$bpm_dir/bin/bpm
  for f in `ls $(pkg_path_for bpm)/libexec/*`; do
    install -v -D $f $pkg_path/libexec/$bpm_dir/libexec/$(basename $f)
  done
  ln -sv $bpm_dir/bin/bpm $pkg_path/libexec/bpm
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
