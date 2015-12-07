pkg_name=bldr
pkg_derivation=chef
pkg_version=0.2.0
pkg_license=('Apache2')
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0e21be5d7c5e6ab6adcbed257619897db59be9e1ded7ef6fd1582d0cdb5e5bb7
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/glibc chef/libgcc chef/busybox chef/openssl chef/runit chef/gpgme)

bldr_begin() {
	mkdir -p /opt/bldr/cache/keys
	mkdir -p /opt/bldr/cache/gpg
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc
	gpg --import chef-public.gpg || true
	gpg --import chef-private.gpg || true
	gpg --homedir /opt/bldr/cache/gpg --import chef-public.gpg || true
	gpg --homedir /opt/bldr/cache/gpg --import chef-private.gpg || true
	pushd ../../
	tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 \
		--exclude 'plans' --exclude 'bldr-plan' --exclude 'demo' --exclude 'images' \
		--exclude '.git' --exclude '.gitignore' --exclude 'target' --exclude '.delivery' \
		--transform "s,^\.,bldr-$pkg_version," .
	popd
	pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/bldr-${pkg_version}.tar.bz2 | cut -d " " -f 1))
	cargo clean
	cargo build --release
	BLDR_BIN=$(abspath "$BLDR_CONTEXT/../../target/release/bldr")
}

download() {
	return 0
}

build() {
  cargo clean
  env OPENSSL_LIB_DIR=$(latest_package chef/openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(latest_package chef/openssl)/include \
      cargo build --release
}

install() {
	mkdir -p $pkg_path/bin
	cp target/release/bldr $pkg_path/bin
	# if [[ -f "$BLDR_BIN" ]]; then
	# 	$BLDR_BIN key chef-public -u $BLDR_REPO
	# 	$BLDR_BIN install chef/cacerts -u $BLDR_REPO
	# 	$BLDR_BIN install chef/gnupg -u $BLDR_REPO
	# 	$BLDR_BIN install chef/zlib -u $BLDR_REPO
	# fi
}

verify() {
	return 0
}

dockerfile() {
  ./mkimage.sh
  docker build -t "bldr/base:${pkg_version}-${pkg_rel}" .
  docker tag -f bldr/base:${pkg_version}-${pkg_rel} bldr/base:latest
}
