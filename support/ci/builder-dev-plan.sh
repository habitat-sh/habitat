pkg_build_deps+=(core/sccache)
pkg_origin=habitat-dev

do_dev_prepare() {
  # Order matters here
  export CARGO_HOME="/tmp/cargo_cache"
  export builder_build_type="--debug"
  export RUSTC_WRAPPER="$(pkg_path_for core/sccache)/bin/sccache"
  export SCCACHE_DIR="/tmp/cargo_cache"
  export SCCACHE_START_SERVER=0
  do_builder_prepare
  export CARGO_TARGET_DIR="/tmp/target"
  PLAN_CONTEXT="../habitat"
}

do_prepare() {
  do_dev_prepare
}

do_clean() {
  build_line "Leaving $CACHE_PATH entact"
  return 0
}

do_builder_install() {
  rm "$(hab pkg path core/$pkg_name)/bin/$bin"
  ln -sf "$CARGO_TARGET_DIR/$rustc_target/${builder_build_type#--}/$bin" \
    "$(hab pkg path core/$pkg_name)/bin/$bin"
}

do_install_wrapper() {
  do_install
}

# TED: Shame! Nobody should ever do this in real life

do_build_config() {
  return 0
}

do_build_service() {
  return 0
}

_generate_artifact() {
  return 0
}

_render_metadata_FILES() {
  return 0
}

_build_manifest() {
  return 0
}

_prepare_build_outputs() {
  return 0
}

_build_metadata() {
  return 0
}

do_end() {
  rm -rf "${pkg_prefix}/../../${pkg_version}"
}