pkg_name=libstdc++
pkg_derivation=chef
pkg_version=5.2.1
pkg_license=('GPLv2' 'LGPLv2.1')
pkg_source=http://ftp.gnu.org/gnu/libc/${pkg_name}-${pkg_version}.tar.bz2
pkg_deps=(chef/glibc chef/libgcc)
pkg_dev_deps=(chef/patchelf)
pkg_shasum=d17a843419530035e73f65e8ecf75e3bb7ea9548d3469bd67f3f769a03ee39c1
pkg_lib_dirs=(usr/lib/x86_64-linux-gnu)
pkg_gpg_key=3853DA6B

download() {
  return 0
}

verify() {
  return 0
}

unpack() {
  return 0
}

prepare() {
  return 0
}

build() {
  dpkg -L libstdc++6 > $BLDR_SRC_CACHE/$pkg_dirname/files
}

install() {
  mkdir -p $pkg_path
  rsync -vaP --no-dirs --files-from=$BLDR_SRC_CACHE/$pkg_dirname/files / $pkg_path
  glibc=$(latest_package "chef/glibc")
  libgcc=$(latest_package "chef/libgcc")
  for lib in "$pkg_path/usr/lib/x86_64-linux-gnu/*"; do
    $(latest_package "chef/patchelf") --set-rpath "$glibc/lib/x86_64-linux-gnu:$libgcc/lib/x86_64-linux-gnu" $lib
  done
}

strip_binaries() {
  return 0
}
