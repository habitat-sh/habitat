pkg_name="testpkgbindconsumer"
pkg_origin="habitat-testing"
pkg_version="0.1.0"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")
pkg_deps=()
pkg_build_deps=()
pkg_binds=(
  [alias]="setting"
)

do_build() {
  return 0
}

# shellcheck disable=SC2154
do_install() {
  mkdir -p "$pkg_prefix/hooks"
  cat <<EOT >> "$pkg_prefix/hooks/run"
#!/bin/sh
set -e

while true; do
  sleep 1
done
EOT
  chmod 755 "$pkg_prefix/hooks/run"
}
