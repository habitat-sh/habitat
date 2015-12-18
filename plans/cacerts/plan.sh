pkg_name=cacerts
pkg_derivation=chef
pkg_version=$(date --date="$build_date" "+%Y.%m.%d")
pkg_license=('MPL1.1 GPLv2.0 LGPLv2.1')
pkg_source=http://curl.haxx.se/ca/cacert.pem
pkg_shasum=nopenopebucketofnope
pkg_gpg_key=3853DA6B

do_unpack() {
	# We are hijacking unpack to get the version from inside the CA Certs
	local build_date=$(cat $BLDR_SRC_CACHE/$pkg_filename | grep 'Certificate data from Mozilla' | sed 's/^## Certificate data from Mozilla as of: //')
	pkg_dirname=${pkg_name}-${pkg_version}
	mkdir -p $BLDR_SRC_CACHE/$pkg_dirname
	cp $BLDR_SRC_CACHE/$pkg_filename $BLDR_SRC_CACHE/$pkg_dirname
	return 0
}

# Verify? This file? From the internet? Not just yet... ;)
do_verify() {
  build_line "Not going to verify this until we have a stable solution"
  return 0
}

do_clean() {
	return 0
}

do_build() {
	return 0
}

do_install() {
	mkdir -p $pkg_path/ssl/certs
	cp $pkg_filename $pkg_path/ssl/certs
	ln -s $pkg_path/ssl/certs/cacert.pem $pkg_path/ssl/cert.pem
}
