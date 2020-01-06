pkg_name="nats-event-stream-test"
pkg_origin="habitat-testing"
pkg_version="2.1.2"
pkg_license=("Apache-2.0")
pkg_upstream_url="https://nats.io"
pkg_description="NATS.io is a simple, secure and high performance open source messaging system for cloud native applications, IoT messaging, and microservices architectures."
pkg_source="https://github.com/nats-io/nats-server/releases/download/v$pkg_version/nats-server-v$pkg_version-linux-amd64.zip"
pkg_shasum="ca2629091c1c06bf3f92195e54d0bc8f69bb5e0adad662e057f972050923c6b4"
pkg_filename="nats-server-v$pkg_version-linux-amd64.zip"
pkg_bin_dirs=("bin")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"

do_unpack() {
  cd "${HAB_CACHE_SRC_PATH}" || exit
  unzip "${pkg_filename}" -d "${pkg_name}-${pkg_version}"
}

do_build() {
  return 0
}

# shellcheck disable=SC2154
do_install() {
  install -D "nats-server-v$pkg_version-linux-amd64/nats-server" "${pkg_prefix}/bin/nats-server"
}

do_check() {
  version=$(./nats-server --version | cut -d'v' -f3)
  if [ "$version" != "$pkg_version" ]; then
    build_line "Check failed to confirm nats version as $pkg_version got $version"
    return 1
  fi
}
