pkg_name=jfrog-cli
pkg_origin=core
pkg_version=1.3.1
pkg_license=('apachev2')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=https://jfrog.bintray.com/jfrog-cli-go/1.3.1/jfrog-cli-linux-amd64/jfrog
pkg_shasum=81265ba5133ea7e735c6ec04f25f1ddcdba504dd41fce07a7ac44d012d993f21
pkg_deps=(core/glibc core/busybox-static core/cacerts)
pkg_build_deps=(core/coreutils core/patchelf)
pkg_bin_dirs=(bin)

do_unpack() {
  return 0
}

do_build() {
  return 0
}

do_install() {
  install -D $HAB_CACHE_SRC_PATH/$pkg_filename $pkg_prefix/bin/jfrog
  cgo_wrap_binary jfrog
}

cgo_wrap_binary() {
  local bin="$pkg_prefix/bin/$1"
  build_line "Adding wrapper $bin to ${bin}.real"
  mv -v "$bin" "${bin}.real"
  local certs="$(pkg_path_for cacerts)/ssl/cert.pem"
  cat <<EOF > "$bin"
#!$(pkg_path_for busybox-static)/bin/sh
set -e
if [ ! -f "/etc/ssl/certs/ca-certificates.crt" ]; then
  echo "Adding symlink of $certs under /etc"
  mkdir -p /etc/ssl/certs
  ln -snf $certs /etc/ssl/certs/ca-certificates.crt
fi
export LD_LIBRARY_PATH="$LD_RUN_PATH"
exec $(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2 ${bin}.real \$@
EOF
  chmod -v 755 "$bin"
}
