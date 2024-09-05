pkg_name="dummy-scaffolding"
pkg_origin=$HAB_ORIGIN
pkg_version="0.1.0"

do_build() { :; }
do_install() {
  install -D -m 0644 "$PLAN_CONTEXT/lib/scaffolding.sh" "$pkg_prefix/lib/scaffolding.sh"
}
