pkg_name=mymysql
pkg_origin=core
pkg_version=0.1.0
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=()
pkg_source=0
pkg_shasum=0
pkg_deps=(core/mysql core/which core/ncurses core/shadow)
pkg_build_deps=()

do_build() {
  attach
  groupadd mysql
  useradd -r -g mysql -s /bin/false mysql

  echo $(pkg_path_for mysql)

}

do_install() {
  return 0
}

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
