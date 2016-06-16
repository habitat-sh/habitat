pkg_name=elasticsearch
pkg_origin=core
pkg_version=2.3.3
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Revised BSD')
pkg_source=https://download.elastic.co/elasticsearch/release/org/elasticsearch/distribution/tar/elasticsearch/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=5fe0a6887432bb8a8d3de2e79c9b81c83cfa241e6440f0f0379a686657789165
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
