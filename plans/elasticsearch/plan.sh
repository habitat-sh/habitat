pkg_name=elasticsearch
pkg_origin=core
pkg_version=2.3.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Revised BSD')
pkg_source=https://download.elastic.co/elasticsearch/release/org/elasticsearch/distribution/tar/elasticsearch/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=04c4d3913d496d217e038da88df939108369ae2e78eea29cb1adf1c4ab3a000a
pkg_deps=(core/glibc core/server-jre)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_svc_run="es/bin/elasticsearch --default.path.conf=$pkg_svc_config_path"
pkg_expose=(9200 9300)

do_build() {
  return 0
}

do_install() {
  build_line "Copying files from $PWD"
  # Elasticsearch is greedy when grabbing config files from /bin/..
  # so we need to put the untemplated config dir out of reach
  mkdir -p $pkg_prefix/es
  cp -r * $pkg_prefix/es
}
