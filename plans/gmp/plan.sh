pkg_name=gmp
pkg_derivation=chef
pkg_version=6.1.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=68dadacce515b0f8a54f510edf07c1b636492bcdb8e8d54c56eb216225d16989
pkg_build_deps=(chef/binutils chef/m4)
pkg_deps=(chef/glibc)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  find . -iname "ltmain.sh" | while read file; do
    build_line "Fixing libtool script $file"
    sed -i -e 's^eval sys_lib_.*search_path=.*^^' "$file"
  done

  # Set RUNPATH for c++ compiled code
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_build() {
  ./configure --prefix=$pkg_prefix --enable-cxx
  make -j$(nproc)

  if [ -n "${DO_CHECK}" ]; then
    make check
  fi
}
