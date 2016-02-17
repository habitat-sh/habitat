pkg_name=rust
pkg_derivation=chef
pkg_version=1.6.0
pkg_license=('Apache-2.0' 'MIT')
pkg_source=https://static.rust-lang.org/dist/${pkg_name}-${pkg_version}-x86_64-unknown-linux-gnu.tar.gz
pkg_dirname=${pkg_name}-${pkg_version}-x86_64-unknown-linux-gnu
pkg_shasum=8630cc02432b4423d64eeae4ef071ec58e5dd1f3d555a3a3cc34b759202813f6
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_lib_dirs=(lib)
pkg_deps=(chef/glibc chef/gcc-libs chef/zlib chef/gcc chef/cacerts)
pkg_build_deps=(chef/patchelf chef/findutils chef/coreutils)

do_build() {
  return 0
}

do_install() {
  ./install.sh --prefix=$pkg_prefix --disable-ldconfig
  patchelf --interpreter "$(pkg_path_for chef/glibc)/lib/ld-linux-x86-64.so.2" \
           --set-rpath "$LD_RUN_PATH" \
           "$pkg_prefix/bin/rustc"
  patchelf --interpreter "$(pkg_path_for chef/glibc)/lib/ld-linux-x86-64.so.2" \
           --set-rpath "$LD_RUN_PATH" \
           "$pkg_prefix/bin/cargo"
  patchelf --interpreter "$(pkg_path_for chef/glibc)/lib/ld-linux-x86-64.so.2" \
           --set-rpath "$LD_RUN_PATH" \
           "$pkg_prefix/bin/rustdoc"


  # Going to want to write a cargo wrapper
  #    SSL_CERT_FILE=$(pkg_path_for chef/cacerts)/ssl/cert.pem \

  find $pkg_path/lib -name *.so | xargs -I '%' patchelf --set-rpath "$LD_RUN_PATH" %
}

do_strip() {
  return 0
}
