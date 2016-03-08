pkg_name=bpm
pkg_origin=chef
pkg_version=0.1.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/tar chef/xz chef/wget chef/busybox-static
                chef/coreutils-static chef/gnupg-static chef/jq-static
                chef/wget-static)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  # Prepare the main program by embedding the full path to specific command
  # binaries so that it can operate with any arbitrary `$PATH` set (or even
  # none).
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_rel,g" \
    $PLAN_CONTEXT/bin/bpm.sh > bpm
}

do_install() {
  install -v -D bpm $pkg_path/bin/bpm

  # Install a copy of a statically built busybox under `libexec/` and add
  # symlinks
  install -v -D $(pkg_path_for busybox-static)/bin/busybox \
    $pkg_path/libexec/busybox
  for l in "${bb_cmds[@]}"; do
    ln -sv busybox $pkg_path/libexec/$l
  done

  install -v -D $(pkg_path_for coreutils-static)/bin/coreutils \
    $pkg_path/libexec/coreutils

  install -v -D $(pkg_path_for gnupg-static)/bin/gpg \
    $pkg_path/libexec/gpg

  install -v -D $(pkg_path_for jq-static)/bin/jq \
    $pkg_path/libexec/jq

  install -v -D $(pkg_path_for wget-static)/bin/wget \
    $pkg_path/libexec/wget
}

do_end() {
  build_line "Creating slim tarball"
  pushd $BLDR_SRC_CACHE > /dev/null
    dir="$(cat $pkg_path/IDENT | tr '/' '-')"
    rm -rfv $dir
    mkdir -pv $dir
    cp -rpv $pkg_path/* $dir/
    tar cpf $BLDR_PKG_CACHE/${dir}.tar $dir
    xz -z -9 -T 0 --verbose $BLDR_PKG_CACHE/${dir}.tar
  popd > /dev/null
  build_line "Slim tarball: $BLDR_PKG_CACHE/${dir}.tar.xz"
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
