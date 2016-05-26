source ../plan.sh

pkg_name=hab
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
# There is no true equivalent here (yet), so dependency arrarys will be empty.
pkg_deps=()
pkg_build_deps=()

nproc() {
  sysctl -n hw.ncpu
}

do_begin() {
  # Set the parent directory as the "root" of this plan.
  PLAN_CONTEXT="$(abspath ..)"
}

do_prepare() {
  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  export rustc_target="x86_64-apple-darwin"
  build_line "Setting rustc_target=$rustc_target"

  formulas="$PLAN_CONTEXT/mac/homebrew"

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
