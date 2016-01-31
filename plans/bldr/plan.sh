pkg_name=bldr
pkg_derivation=chef
pkg_version=0.3.0
pkg_license=('Apache2')
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0e21be5d7c5e6ab6adcbed257619897db59be9e1ded7ef6fd1582d0cdb5e5bb7
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/glibc chef/libgcc chef/busybox chef/openssl chef/runit chef/gpgme chef/libassuan chef/libarchive chef/libgpg-error)
pkg_build_deps=(chef/patchelf)

do_begin() {
	mkdir -p /opt/bldr/cache/keys
	mkdir -p /opt/bldr/cache/gpg
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc
	gpg --import chef-public.gpg || true
	gpg --import chef-private.gpg || true
	gpg --homedir /opt/bldr/cache/gpg --import chef-public.gpg || true
	gpg --homedir /opt/bldr/cache/gpg --import chef-private.gpg || true
	pushd ../../
	tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 \
		--exclude 'plans' --exclude 'bldr-plan' --exclude 'demo' --exclude 'images' --exclude 'web' \
		--exclude '.git' --exclude '.gitignore' --exclude 'target' --exclude '.delivery' \
		--transform "s,^\.,bldr-$pkg_version," .
	popd
	pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/bldr-${pkg_version}.tar.bz2 | cut -d " " -f 1))
	# We build ourselves twice, once to fetch stuff, another time to actually release ourselves
	#build
	#BLDR_BIN=$(abspath "$BLDR_CONTEXT/../../target/release/bldr")
}

do_download() {
	return 0
}

do_build() {
  # cargo clean
  env OPENSSL_LIB_DIR=$(pkg_path_for chef/openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(pkg_path_for chef/openssl)/include \
      GPGME_CONFIG=$(pkg_path_for chef/gpgme)/bin/gpgme-config \
      GPG_ERROR_CONFIG=$(pkg_path_for chef/libgpg-error)/bin/gpg-error-config \
      LIBARCHIVE_LIB_DIR=$(pkg_path_for chef/libarchive)/lib \
      LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for chef/libarchive)/include \
      cargo build
  # Make double-sure our binary is completely pure (no accidental linking leaks
  # outside `/opt/bldr/pkgs`)
  patchelf --set-rpath "$LD_RUN_PATH" target/debug/bldr
}

do_install() {
	mkdir -p $pkg_path/bin
	cp target/debug/bldr $pkg_path/bin
	# if [[ -f "$BLDR_BIN" ]]; then
	# 	$BLDR_BIN key chef-public -u $BLDR_REPO
	# 	$BLDR_BIN install chef/cacerts -u $BLDR_REPO
	# 	$BLDR_BIN install chef/gnupg -u $BLDR_REPO
	# 	$BLDR_BIN install chef/zlib -u $BLDR_REPO
	# fi
}

do_verify() {
	return 0
}

do_docker_image() {
  ./mkimage.sh
  docker build -t "bldr/base:${pkg_version}-${pkg_rel}" .
  docker tag -f bldr/base:${pkg_version}-${pkg_rel} bldr/base:latest
}
