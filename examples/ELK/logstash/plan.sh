pkg_origin=core
pkg_name=logstash
pkg_description="Demo Logstash instance"
pkg_version=0.1.0
pkg_maintainer="Habitat Maintainers"
pkg_license=("Apache-2.0")
pkg_binds_optional=(
  [elasticsearch]="http-port"
)
pkg_deps=(core/jruby1 core/jre8 core/curl)
pkg_build_deps=(core/logstash/5.1.1)

do_build() {
  return 0
}

do_install() {
  export USE_RUBY
  USE_RUBY=1
  export JAVA_HOME
  JAVA_HOME=$(hab pkg path core/jre8)

  # `logstash-plugin install' targets the Logstash installation it is
  # being executed from. If we attempt to use the `logstash-plugin'
  # script that ships in `core/logstash' our Logstash plugins will be
  # installed into the `core/logstash' directory in the build studio
  # only. The actual plugins will not be available at runtime as a new
  # copy of the `core/logstash' package will be fetched at runtime. The
  # easiest way around these issues is to treat `core/logstash' as a
  # build dep and vendor the entire Logstash tree into our pkg.
  find "$(hab pkg path core/logstash)/" \
    -mindepth 1 \
    -maxdepth 1 \
    -regextype egrep \
    -regex ".*(bin|data|lib|logstash-.*|vendor|Gemfile.*)$" \
    -exec cp -p -R {} "$pkg_prefix/" \;
}
