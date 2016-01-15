pkg_name=pcre
pkg_derivation=chef
pkg_version=8.38
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('bsd')
pkg_source=http://ftp.csx.cam.ac.uk/pub/software/programming/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b9e02d36e23024d6c02a2e5b25204b3a4fa6ade43e0a5f869f254f49535079df
pkg_deps=(chef/glibc chef/gcc-libs)
pkg_build_deps=(chef/gcc chef/coreutils chef/patchelf)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  find . -iname "ltmain.sh" | while read file; do
    build_line "Fixing libtool script $file"
    sed -i -e 's^eval sys_lib_.*search_path=.*^^' "$file"
  done
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --enable-unicode-properties \
    --enable-pcre16 \
    --enable-pcre32 \
    --enable-jit
  make -j$(nproc)

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}

do_install() {
  do_default_install

  # Install license file
  install -Dm644 LICENCE $pkg_path/share/licenses/LICENSE
}
