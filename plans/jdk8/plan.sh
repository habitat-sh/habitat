pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_name=jdk8
pkg_version=8u92
pkg_source=http://download.oracle.com/otn-pub/java/jdk/${pkg_version}-b14/jdk-${pkg_version}-linux-x64.tar.gz
pkg_shasum=79a3f25e9b466cb9e969d1772ea38550de320c88e9119bf8aa11ce8547c39987
pkg_filename=jdk-${pkg_version}-linux-x64.tar.gz
pkg_license=('Oracle Binary Code License Agreement for the Java SE Platform Products and JavaFX')
pkg_description=('Oracle Java Development Kit. This package is made available to you to allow you to run your applications as provided in and subject to the terms of the Oracle Binary Code License Agreement for the Java SE Platform Products and JavaFX, found at http://www.oracle.com/technetwork/java/javase/terms/license/index.html')
pkg_deps=(core/glibc)
pkg_build_deps=(core/patchelf core/file)
pkg_bin_dirs=(bin jre/bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

source_dir=$HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}

## Refer to habitat/components/plan-build/bin/hab-plan-build.sh for help

# Customomized download_file() to work around the Oracle EULA Cookie-wall
#  See: http://stackoverflow.com/questions/10268583/downloading-java-jdk-on-linux-via-wget-is-shown-license-page-instead
download_file() {
  local url="$1"
  local dst="$2"
  local sha="$3"

  pushd $HAB_CACHE_SRC_PATH > /dev/null
  if [[ -f $dst && -n "$sha" ]]; then
    build_line "Found previous file '$dst', attempting to re-use"
    if verify_file $dst $sha; then
      build_line "Using cached and verified '$dst'"
      return 0
    else
      build_line "Clearing previous '$dst' file and re-attempting download"
      rm -fv $dst
    fi
  fi

  build_line "Downloading '$url' to '$dst'"
  $_wget_cmd --no-check-certificate --no-cookies --header "Cookie: oraclelicense=accept-securebackup-cookie"  $url -O $dst
  build_line "Downloaded '$dst'";
  popd > /dev/null
}

do_unpack() {
  local unpack_file="$HAB_CACHE_SRC_PATH/$pkg_filename"
  mkdir $source_dir
  pushd $source_dir >/dev/null
  tar xz --strip-components=1 -f $unpack_file

  popd > /dev/null
  return 0
}

do_build() {
  return 0
}

do_install() {
  cd $source_dir
  cp -r * $pkg_prefix

  build_line "Setting interpreter for '${pkg_prefix}/bin/java' '$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2'"
  build_line "Setting rpath for '${pkg_prefix}/bin/java' to '$LD_RUN_PATH'"

  export LD_RUN_PATH=$LD_RUN_PATH:$pkg_prefix/lib/amd64/jli

  find $pkg_prefix/bin -type f -executable \
    -exec sh -c "file -i '{}' | grep -q 'x-executable; charset=binary'" \; \
    -exec patchelf --interpreter "$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2" --set-rpath ${LD_RUN_PATH} {} \;
}
