pkg_name=cacerts
pkg_derivation=chef
pkg_version=_set_from_downloaded_cacerts_file_
pkg_license=('mplv1.1' 'gplV2' 'lgplv2.1')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://curl.haxx.se/ca/cacert.pem
pkg_shasum=nopenopebucketofnope
pkg_deps=()
pkg_build_deps=()
pkg_gpg_key=3853DA6B

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
  mkdir -pv $BLDR_SRC_CACHE/$pkg_dirname
  cp -v $BLDR_SRC_CACHE/$pkg_filename $BLDR_SRC_CACHE/$pkg_dirname
}

do_build() {
  return 0
}

do_install() {
  mkdir -pv $pkg_path/ssl/certs
  cp -v $pkg_filename $pkg_path/ssl/certs
  ln -sv certs/cacert.pem $pkg_path/ssl/cert.pem
}

update_pkg_version() {
  # Extract the build date of the certificates file
  local build_date=$(cat $BLDR_SRC_CACHE/$pkg_filename \
    | grep 'Certificate data from Mozilla' \
    | sed 's/^## Certificate data from Mozilla as of: //')

  # Update the `$pkg_version` value with the build date
  pkg_version=$(date --date="$build_date" "+%Y.%m.%d")
  build_line "Version updated to $pkg_version from CA Certs file"

  # Several metadata values get their defaults from the value of `$pkg_version`
  # so we must update these as well
  pkg_dirname=${pkg_name}-${pkg_version}
  pkg_prefix=$BLDR_PKG_ROOT/${pkg_derivation}/${pkg_name}/${pkg_version}/${pkg_rel}
  pkg_path=$pkg_prefix
}
