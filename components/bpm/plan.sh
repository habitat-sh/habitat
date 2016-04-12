pkg_name=hab-bpm
pkg_origin=chef
pkg_version=0.1.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/tar chef/xz chef/wget chef/busybox-static
                chef/coreutils-static chef/gnupg-static chef/jq-static
                chef/wget-static)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  # Prepare the main program by embedding the full path to specific command
  # binaries so that it can operate with any arbitrary `$PATH` set (or even
  # none).
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_rel,g" \
    $PLAN_CONTEXT/bin/hab-bpm.sh > hab-bpm
}

do_install() {
  install -v -D hab-bpm $pkg_prefix/bin/hab-bpm

  # Install a copy of a statically built busybox under `libexec/` and add
  # symlinks
  install -v -D $(pkg_path_for busybox-static)/bin/busybox \
    $pkg_prefix/libexec/busybox
  for l in "${bb_cmds[@]}"; do
    ln -sv busybox $pkg_prefix/libexec/$l
  done

  install -v -D $(pkg_path_for coreutils-static)/bin/coreutils \
    $pkg_prefix/libexec/coreutils

  install -v -D $(pkg_path_for jq-static)/bin/jq \
    $pkg_prefix/libexec/jq

  install -v -D $(pkg_path_for wget-static)/bin/wget \
    $pkg_prefix/libexec/wget
}

do_end() {
  build_line "Creating slim tarball"
  pushd $HAB_CACHE_SRC_PATH > /dev/null
    dir="$(cat $pkg_prefix/IDENT | tr '/' '-')"
    rm -rfv $dir
    mkdir -pv $dir
    cp -rpv $pkg_prefix/* $dir/
    tar cpf $HAB_CACHE_ARTIFACT_PATH/${dir}.tar $dir
    xz -z -9 -T 0 --verbose $HAB_CACHE_ARTIFACT_PATH/${dir}.tar
  popd > /dev/null
  build_line "Slim tarball: $HAB_CACHE_ARTIFACT_PATH/${dir}.tar.xz"
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
