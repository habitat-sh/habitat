pkg_version=undefined
builder_build_type="--release"

do_begin() {
  do_builder_begin
}

do_prepare() {
  do_builder_prepare
}

do_build() {
  do_builder_build
}

do_install() {
  do_builder_install
}

do_strip() {
  do_builder_strip
}

do_builder_begin() {
  update_pkg_version
}

do_builder_build() {
  pushd "$PLAN_CONTEXT"/.. > /dev/null
  cargo build "${builder_build_type#--debug}" --target="$rustc_target" --verbose
  popd > /dev/null
}

do_builder_install() {
  install -v -D "$CARGO_TARGET_DIR/$rustc_target/${builder_build_type#--}/$bin" \
    "$pkg_prefix/bin/$bin"
}

do_builder_prepare() {
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_line "Building artifacts with \`${builder_build_type#--}' mode"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_builder_strip() {
  if [[ "$builder_build_type" != "--debug" ]]; then
    do_default_strip
  fi
}

update_pkg_version() {
  # Update the `$pkg_version` using Git to determine the value
  pkg_version="$(git rev-list master --count)"
  build_line "Version updated to $pkg_version"

  # Several metadata values get their defaults from the value of `$pkg_version`
  # so we must update these as well
  pkg_dirname=${pkg_name}-${pkg_version}
  pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
  if [[ "$CACHE_PATH" == "$SRC_PATH" ]]; then
    local update_src_path=true
  fi
  CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  if [[ "${update_src_path:-}" == true ]]; then
    SRC_PATH="$CACHE_PATH"
  fi
}

# Turn the remaining default phases into no-ops
do_download() {
  return 0
}

do_unpack() {
  return 0
}

do_verify() {
  return 0
}
