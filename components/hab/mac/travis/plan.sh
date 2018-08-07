# shellcheck disable=2154
PLAN_CONTEXT=$(dirname "$PLAN_CONTEXT") source ../../plan.sh

pkg_name=hab
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
# There is no true equivalent here (yet), so dependency arrays will be empty.
pkg_deps=()
pkg_build_deps=()

pkg_version=$(cat "$PLAN_CONTEXT/../../../../VERSION")

do_begin() {
  # Set the real hab component directory as the "root" of this plan.
  PLAN_CONTEXT="$(abspath ../..)"
}

# shellcheck disable=2155
do_prepare() {
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  export rustc_target="x86_64-apple-darwin"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

  formulas="$PLAN_CONTEXT/mac/travis/homebrew"

  la_ldflags="-L$(brew --prefix zlib)/lib -lz"
  la_ldflags="$la_ldflags -L$(brew --prefix xz)/lib -llzma"
  la_ldflags="$la_ldflags -L$(brew --prefix bzip2)/lib -lbz2"
  la_ldflags="$la_ldflags -L$(brew --prefix expat)/lib -lexpat"
  la_ldflags="$la_ldflags -L$(brew --prefix "$formulas"/hab-libiconv.rb)/lib -liconv"

  export LIBARCHIVE_LIB_DIR=$(brew --prefix "$formulas"/hab-libarchive.rb)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(brew --prefix "$formulas"/hab-libarchive.rb)/include
  export LIBARCHIVE_LDFLAGS="$la_ldflags"
  export LIBARCHIVE_STATIC=true
  export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
  export OPENSSL_STATIC=true
  export SODIUM_LIB_DIR=$(brew --prefix libsodium)/lib
  export SODIUM_STATIC=true
}
