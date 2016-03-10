pkg_name=go
pkg_origin=chef
pkg_version=1.6
pkg_license=('bsd')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://storage.googleapis.com/golang/${pkg_name}${pkg_version}.src.tar.gz
pkg_shasum=a96cce8ce43a9bf9b2a4c7d470bc7ee0cb00410da815980681c8353218dcf146
pkg_dirname=$pkg_name
pkg_deps=(chef/glibc chef/iana-etc chef/cacerts)
pkg_build_deps=(chef/coreutils chef/inetutils chef/bash chef/patch chef/gcc chef/go/1.4.3 chef/perl)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

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

  # Building Go after 1.5 requires a previous version of Go to bootstrap with.
  # This environment variable tells the build system to use our 1.4.x release
  # as the bootstrapping Go.
  export GOROOT_BOOTSTRAP="$(pkg_path_for go)"
  build_line "Setting GOROOT_BOOTSTRAP=$GOROOT_BOOTSTRAP"

  # Add `chef/cacerts` to the SSL certificate lookup chain
  cat $PLAN_CONTEXT/cacerts.patch \
    | sed -e "s,@cacerts@,$(pkg_path_for cacerts)/ssl/cert.pem,g" \
    | patch -p1

  # Set the dynamic linker from `chef/glibc`
  dynamic_linker="$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2"
  sed -e "s,/lib64/ld-linux-x86-64.so.2,$dynamic_linker," \
    -i src/cmd/link/internal/amd64/obj.go

  # Use the services database from `chef/iana-etc`
  for f in src/net/port_unix.go src/net/parse_test.go; do
    sed -e "s,/etc/services,$(pkg_path_for iana-etc)/etc/services," -i $f
  done
}

do_build() {
  pushd src > /dev/null
    bash make.bash --no-clean
  popd > /dev/null
}

do_check() {
  # The test suite requires several hardcoded commands to be present, so we'll
  # add symlinks if they are not already present
  local _clean_cmds=()
  if [[ ! -r /bin/pwd ]]; then
    ln -sv $(pkg_path_for coreutils)/bin/pwd /bin/pwd
    _clean_cmds+=(/bin/pwd)
  fi
  if [[ ! -r /usr/bin/env ]]; then
    ln -sv $(pkg_path_for coreutils)/bin/env /usr/bin/env
    _clean_cmds+=(/usr/bin/env)
  fi
  if [[ ! -r /bin/hostname ]]; then
    ln -sv $(pkg_path_for inetutils)/bin/hostname /bin/hostname
    _clean_cmds+=(/bin/hostname)
  fi

  pushd src > /dev/null
    env LD_LIBRARY_PATH="$(pkg_path_for gcc)/lib" bash run.bash --no-rebuild
  popd > /dev/null

  # Clean up any symlinks that were added to support the build's test suite.
  for cmd in "${_clean_cmds[@]}"; do
    rm -fv $cmd
  done
}

do_install() {
  cp -av bin src lib doc misc $pkg_path/

  mkdir -pv $pkg_path/bin $pkg_path/pkg
  cp -av pkg/{linux_$GOARCH,tool} $pkg_path/pkg/
  if [[ -d "pkg/linux_${GOARCH}_race" ]]; then
    cp -av pkg/linux_${GOARCH}_race $pkg_path/pkg/
  fi

  # For godoc
  install -v -Dm644 favicon.ico $pkg_path/favicon.ico

  # Install the license
  install -v -Dm644 LICENSE $pkg_path/share/licenses/LICENSE

  # Remove unneeded Windows files
  rm -fv $pkg_path/src/*.bat
}

do_strip() {
  # Strip manually since `strip` will not process Go's static libraries.
  for f in $pkg_path/bin/* $pkg_path/pkg/tool/linux_$GOARCH/*; do
    strip -s "$f"
  done
}
