pkg_name=radiant
pkg_version=2.0.0-alpha-jt

pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('mit')

pkg_source=https://github.com/jtimberman/radiant/archive/${pkg_version}.tar.gz
pkg_shasum=24e6527eec98df16f3857f3d0cdf630729b0b833117589c32b98b0078f1bea05

pkg_deps=(
  core/bundler
  core/cacerts
  core/glibc
  core/libffi
  core/libxml2
  core/libxslt
  core/libyaml
  core/openssl
  core/postgresql
  core/ruby
  core/sqlite
  core/zlib
)

pkg_build_deps=(
  core/coreutils
  core/gcc
  core/make
)

pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_expose=(80 443 3000)

# do_download() {
#   export GIT_SSL_CAINFO="$(pkg_path_for cacerts)/ssl/certs/cacert.pem"
#   git clone https://github.com/jtimberman/radiant
#   pushd radiant
#   git checkout $radiant_git_shasum
#   popd
#   tar -cjvf $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}.tar.bz2 \
#       --transform "s,^\./radiant,radiant-${pkg_version}," ./radiant \
#       --exclude radiant/.git --exclude radiant/spec
#   pkg_shasum=$(trim $(sha256sum $HAB_CACHE_SRC_PATH/${pkg_filename} | cut -d " " -f 1))
# }

# The configure scripts for some RubyGems that build native extensions
# use `/usr/bin` paths for commands. This is not going to work in a
# studio where we don't have any of those commands. But we're kind of
# stuck because the native extension is going to be built during
# `bundle install`.
#
# We clean this link up in `do_install`.
do_prepare() {
  # build_line "Setting link for /usr/bin/file to 'file'"
  # [[ ! -f /usr/bin/file ]] && ln -s $(pkg_path_for file)/bin/file /usr/bin/file

  build_line "Setting link for /usr/bin/env to 'coreutils'"
  [[ ! -f /usr/bin/env ]] && ln -s $(pkg_path_for coreutils)/bin/env /usr/bin/env
  return 0
}

do_build() {
  export CPPFLAGS="${CPPFLAGS} ${CFLAGS}"

  local _bundler_dir=$(pkg_path_for bundler)
  local _libxml2_dir=$(pkg_path_for libxml2)
  local _libxslt_dir=$(pkg_path_for libxslt)
  local _postgresql_dir=$(pkg_path_for postgresql)
  local _pgconfig=$_postgresql_dir/bin/pg_config
  local _sqlite_dir=$(pkg_path_for sqlite)
  local _zlib_dir=$(pkg_path_for zlib)

  export GEM_HOME=${pkg_path}/vendor/bundle
  export GEM_PATH=${_bundler_dir}:${GEM_HOME}

  # don't let bundler split up the nokogiri config string (it breaks
  # the build), so specify it as an env var instead
  export NOKOGIRI_CONFIG="--use-system-libraries --with-zlib-dir=${_zlib_dir} --with-xslt-dir=${_libxslt_dir} --with-xml2-include=${_libxml2_dir}/include/libxml2 --with-xml2-lib=${_libxml2_dir}/lib"
  bundle config build.nokogiri '${NOKOGIRI_CONFIG}'
  bundle config build.pg --with-pg-config=${_pgconfig}
  bundle config build.sqlite3 --with-sqlite3-include=${_sqlite_dir}/include --with-sqlite3-lib=${_sqlite_dir}/lib

  # We don't need mysql, so let's not even have it in the gemfile
  sed -e 's/gem "mysql"/#removed mysql gem/' -i rails40.gemfile

  bundle install --jobs 2 --retry 5 --path vendor/bundle \
         --binstubs --gemfile=rails40.gemfile
}

do_install() {
  cp -R . ${pkg_path}/dist

  for binstub in ${pkg_path}/dist/bin/*; do
    build_line "Setting shebang for ${binstub} to 'ruby'"
    [[ -f $binstub ]] && sed -e "s#/usr/bin/env ruby#$(pkg_path_for ruby)/bin/ruby#" -i $binstub
  done

  # if [[ `readlink /usr/bin/file` = "$(pkg_path_for file)/bin/file" ]]; then
  #   build_line "Removing the symlink we created for '/usr/bin/file'"
  #   rm /usr/bin/file
  # fi

  if [[ `readlink /usr/bin/env` = "$(pkg_path_for coreutils)/bin/env" ]]; then
    build_line "Removing the symlink we created for '/usr/bin/env'"
    rm /usr/bin/env
  fi

}
