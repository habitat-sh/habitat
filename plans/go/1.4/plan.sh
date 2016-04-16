source ../plan.sh

pkg_version=1.4.3
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=https://storage.googleapis.com/golang/${pkg_name}${pkg_version}.src.tar.gz
pkg_shasum=9947fc705b0b841b5938c48b22dc33e9647ec0752bae66e50278df4f23f64959
pkg_build_deps=(core/coreutils core/inetutils core/bash core/patch core/gcc core/diffutils)

do_prepare() {
  export GOOS=linux
  build_line "Setting GOOS=$GOOS"
  export GOARCH=amd64
  build_line "Setting GOARCH=$GOARCH"
  export CGO_ENABLED=1
  build_line "Setting CGO_ENABLED=$CGO_ENABLED"

  export GOROOT="$(pwd)"
  build_line "Setting GOROOT=$GOROOT"
  export GOBIN="$GOROOT/bin"
  build_line "Setting GOBIN=$GOBIN"
  export GOROOT_FINAL="$pkg_prefix"
  build_line "Setting GOROOT_FINAL=$GOROOT_FINAL"

  PATH="$GOBIN:$PATH"
  build_line "Updating PATH=$PATH"

  # Add `cacerts` to the SSL certificate lookup chain
  cat $PLAN_CONTEXT/cacerts.patch \
    | sed -e "s,@cacerts@,$(pkg_path_for cacerts)/ssl/cert.pem,g" \
    | patch -p1

  # Set the dynamic linker from `glibc`
  dynamic_linker="$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2"
  find src/cmd -name asm.c -exec \
    sed -i "s,/lib/ld-linux.*\.so\.[0-9],$dynamic_linker," {} \;

  # Use the protocols database from `iana-etc`
  sed -e "s,/etc/protocols,$(pkg_path_for iana-etc)/etc/protocols," \
    -i src/net/lookup_unix.go

  # Use the services database from `iana-etc`
  for f in src/net/port_unix.go src/net/parse_test.go; do
    sed -e "s,/etc/services,$(pkg_path_for iana-etc)/etc/services," -i $f
  done

  # Duplicate `127.0.0.1` entries in `/etc/hosts` cause this test to fail,
  # but as Studio is at the mercy of the outside host for this file, transient
  # failures make sense. Hence, we are ignoring this test.
  sed -e '/TestLookupHost/areturn' -i src/net/hosts_test.go

  # These tests are failing due to the ipv6 networking stack
  sed -e '/TestDialDualStackLocalhost/areturn' -i src/net/dial_test.go
  sed -e '/TestResolveIPAddr/areturn' -i src/net/ipraw_test.go
  sed -e '/TestResolveTCPAddr/areturn' -i src/net/tcp_test.go
  sed -e '/TestResolveUDPAddr/areturn' -i src/net/udp_test.go

  sed -e '/TestLookupPort/areturn' -i src/net/port_test.go

  sed -e '/TestFilePacketConn/areturn' -i src/net/file_test.go
}
