pkg_name=phantomjs
pkg_version=2.1.1
pkg_origin=chef
pkg_license=('bsd')
pkg_source=https://bitbucket.org/ariya/phantomjs/downloads/${pkg_name}-${pkg_version}-linux-x86_64.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}-linux-x86_64.tar.bz2
pkg_dirname=${pkg_name}-${pkg_version}-linux-x86_64
pkg_shasum=86dd9a4bf4aee45f1a84c9f61cf1947c1d6dce9b9e8d2a907105da7852460d2f
pkg_gpg_key=3853DA6B

# Ensure we depend on all the libraries that the prebuilt phantomjs
# links against here:
pkg_deps=(chef/glibc chef/freetype chef/fontconfig chef/patchelf
          chef/zlib chef/libpng chef/expat chef/gcc-libs)

# We need curl instead of wget because wget doesn't work for
# downloading from bitbucket URLs. Sometimes.
pkg_build_deps=(chef/curl chef/cacerts)

pkg_bin_dirs=(bin)

do_download() {
  # downloading from bitbucket with wget results in a 403.
  # So then we implement our own `do_download` with `curl`.
  pushd $BLDR_SRC_CACHE > /dev/null
  if [[ -f $pkg_filename ]]; then
    build_line "Found previous file '${pkg_filename}', attempting to re-use"
    if verify_file $pkg_filename $pkg_shasum; then
      build_line "Using cached and verified '${pkg_filename}'"
      return 0
    else
      build_line "Clearing previous '${pkg_filename}' and re-attempting download"
      rm -fv $pkg_filename
    fi
  fi

  build_line "Downloading '${pkg_source}' to '${pkg_filename}' with curl"
  curl -L -O $pkg_source --cacert $(pkg_path_for chef/cacerts)/ssl/cert.pem
  build_line "Downloaded '${pkg_filename}'";
  popd > /dev/null
}

do_build () {
  # We don't need to build because phantomjs is a prebuilt binary!
  return 0
}

do_strip() {
  # Because we're a) using a prebuilt binary, and b) running
  # patchelf against it, strip will remove "commonly the strings
  # that represent the names associated with symbol table entries"
  # https://refspecs.linuxfoundation.org/LSB_3.0.0/LSB-PDA/LSB-PDA/specialsections.html
  return 0
}

do_install() {
  cp -vR * ${pkg_prefix}

  build_line "Setting interpreter for '${pkg_prefix}/bin/phantomjs' '$(pkg_path_for chef/glibc)/lib/ld-linux-x86-64.so.2'"
  build_line "Setting rpath for '${pkg_prefix}/bin/phantomjs' to '$LD_RUN_PATH'"

  patchelf --interpreter "$(pkg_path_for chef/glibc)/lib/ld-linux-x86-64.so.2" \
           --set-rpath ${LD_RUN_PATH} \
           ${pkg_prefix}/bin/phantomjs
}
