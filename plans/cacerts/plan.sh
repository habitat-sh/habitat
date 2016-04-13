pkg_name=cacerts
pkg_origin=chef
pkg_version=_set_from_downloaded_cacerts_file_
pkg_license=('mplv1.1' 'gplV2' 'lgplv2.1')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://curl.haxx.se/ca/cacert.pem
pkg_shasum=nopenopebucketofnope
pkg_deps=()
pkg_build_deps=()

do_download() {
  do_default_download
  update_pkg_version
}

# Verify? This file? From the internet? Not just yet... ;)
do_verify() {
  build_line "Not going to verify this until we have a stable solution"
  return 0
}

do_unpack() {
  mkdir -pv $HAB_CACHE_SRC_PATH/$pkg_dirname
  cp -v $HAB_CACHE_SRC_PATH/$pkg_filename $HAB_CACHE_SRC_PATH/$pkg_dirname
}

do_build() {
  return 0
}

do_install() {
  mkdir -pv $pkg_prefix/ssl/certs
  cp -v $pkg_filename $pkg_prefix/ssl/certs
  ln -sv certs/cacert.pem $pkg_prefix/ssl/cert.pem
}

update_pkg_version() {
  # Extract the build date of the certificates file
  local build_date=$(cat $HAB_CACHE_SRC_PATH/$pkg_filename \
    | grep 'Certificate data from Mozilla' \
    | sed 's/^## Certificate data from Mozilla as of: //')

  # Update the `$pkg_version` value with the build date
  pkg_version=$(date --date="$build_date" "+%Y.%m.%d")
  build_line "Version updated to $pkg_version from CA Certs file"

  # Several metadata values get their defaults from the value of `$pkg_version`
  # so we must update these as well
  pkg_dirname=${pkg_name}-${pkg_version}
  pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_rel}
}
