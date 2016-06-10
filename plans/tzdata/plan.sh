pkg_name=tzdata
pkg_origin=core
pkg_version=2015f
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gpl')
pkg_source=http://www.iana.org/time-zones/repository/releases/${pkg_name}${pkg_version}.tar.gz
pkg_shasum=959f81b541e042ecb13c50097d264ae92ff03a57979c478dbcf24d5da242531d

# This is an incomplete plan, but is used by glibc to install timezone data.

# Re-override the defaults as this plan is sourced externally
pkg_filename="$(basename $pkg_source)"
pkg_dirname="${pkg_name}-${pkg_version}"

do_unpack() {
  mkdir -p "$HAB_CACHE_SRC_PATH/$pkg_dirname"
  pushd $HAB_CACHE_SRC_PATH/$pkg_dirname > /dev/null
    tar xzf "$HAB_CACHE_SRC_PATH/$pkg_filename"
  popd > /dev/null
}

do_build() {
  return 0
}

do_install() {
  return 0
}
