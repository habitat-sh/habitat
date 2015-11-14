pkg_name=bldr
pkg_derivation=chef
pkg_version=0.0.1
pkg_license=('Apache2')
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0e21be5d7c5e6ab6adcbed257619897db59be9e1ded7ef6fd1582d0cdb5e5bb7
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/glibc chef/libgcc chef/busybox chef/openssl chef/runit)

bldr_begin() {
	pushd ../../
	tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 --exclude 'plans' --exclude '.git' --exclude '.gitignore' --exclude 'target' --exclude '.delivery' --transform "s,^\.,bldr-0.0.1," .
	popd
	pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/bldr-0.0.1.tar.bz2 | cut -d " " -f 1))
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
}

dockerfile() {
  ./mkimage.sh
  docker build -t "bldr/base:${pkg_version}-${pkg_rel}" .
  docker tag -f bldr/base:${pkg_version}-${pkg_rel} bldr/base:latest
}
