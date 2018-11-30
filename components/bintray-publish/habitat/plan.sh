# shellcheck disable=2154
pkg_name=hab-bintray-publish
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_deps=(core/coreutils
          core/util-linux
          core/bash
          core/zip
          core/tar
          core/xz
          core/gzip
          core/jfrog-cli
          core/grep
          core/findutils
          core/gawk
          core/hab)
pkg_bin_dirs=(bin)

do_build() {
    cp -v "${SRC_PATH}/bin/publish-hab.sh" "${CACHE_PATH}/publish-hab"

    # Use the bash from our dependency list as the shebang. Also, embed the
    # release version of publish-hab.
    sed \
      -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
      -e "s,@author@,$pkg_maintainer,g" \
      -e "s,@version@,$pkg_version/$pkg_release,g" \
      -i "${CACHE_PATH}/publish-hab"
}

do_install() {
    install -v -D "${CACHE_PATH}/publish-hab" "${pkg_prefix}/bin/publish-hab"
}
