#!/bin/bash

set -euo pipefail

# This is problematic if you want to be able to run this script from anywhere other than the root of the project,
# but changing it to an idiom like we have in rustfmt.sh breaks BK, so I dunno?
source ./support/ci/shared.sh

toolchain="${1:-stable}"
install_rustup
install_rust_toolchain "$toolchain"

# TODO: these should be in a shared script?
install_hab_pkg core/bzip2 core/libarchive core/libsodium core/openssl core/xz core/zeromq core/protobuf
export SODIUM_STATIC=true # so the libarchive crate links to sodium statically
export LIBARCHIVE_STATIC=true # so the libarchive crate *builds* statically
export OPENSSL_DIR # so the openssl crate knows what to build against
OPENSSL_DIR="$(hab pkg path core/openssl)"
export OPENSSL_STATIC=true # so the openssl crate builds statically
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include openssl and zeromq so thney exists in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/libsodium)/lib:$(hab pkg path core/zeromq)/lib"
# include these so that the cargo tests can bind to libarchive (which dynamically binds to xz, bzip, etc), openssl, and sodium at *runtime*
export LIBRARY_PATH
LIBRARY_PATH="$(hab pkg path core/bzip2)/lib:$(hab pkg path core/libsodium)/lib:$(hab pkg path core/openssl)/lib:$(hab pkg path core/xz)/lib"
# setup pkgconfig so the libarchive crate can use pkg-config to fine bzip2 and xz at *build* time
export PKG_CONFIG_PATH
PKG_CONFIG_PATH="$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"

# Install clippy
echo "--- :rust: Installing clippy"
rustup component add clippy

# Lints we need to work through and decide as a team whether to allow or fix
unexamined_lints=()

# Lints we disagree with and choose to keep in our code with no warning
allowed_lints=(clippy::module_inception \
               clippy::new_ret_no_self \
               clippy::new_without_default \
               clippy::new_without_default_derive)

# Known failing lints we want to receive warnings for, but not fail the build
lints_to_fix=(clippy::cyclomatic_complexity \
               clippy::large_enum_variant \
               clippy::needless_return \
               clippy::too_many_arguments)

# Lints we don't expect to have in our code at all and want to avoid adding
# even at the cost of failing the build
denied_lints=(clippy::assign_op_pattern \
               clippy::blacklisted_name \
               clippy::block_in_if_condition_stmt \
               clippy::bool_comparison \
               clippy::cast_lossless \
               clippy::clone_on_copy \
               clippy::cmp_owned \
               clippy::collapsible_if \
               clippy::const_static_lifetime \
               clippy::correctness \
               clippy::deref_addrof \
               clippy::expect_fun_call \
               clippy::for_kv_map \
               clippy::get_unwrap \
               clippy::identity_conversion \
               clippy::if_let_some_result \
               clippy::just_underscores_and_digits \
               clippy::len_without_is_empty \
               clippy::len_zero \
               clippy::let_and_return \
               clippy::let_unit_value \
               clippy::map_clone \
               clippy::match_bool \
               clippy::match_ref_pats \
               clippy::needless_bool \
               clippy::needless_collect \
               clippy::needless_pass_by_value \
               clippy::needless_range_loop \
               clippy::needless_update \
               clippy::ok_expect \
               clippy::op_ref \
               clippy::option_map_unit_fn \
               clippy::or_fun_call \
               clippy::println_empty_string \
               clippy::ptr_arg \
               clippy::question_mark \
               clippy::redundant_closure \
               clippy::redundant_field_names \
               clippy::redundant_pattern_matching \
               clippy::single_char_pattern \
               clippy::single_match \
               clippy::string_lit_as_bytes \
               clippy::toplevel_ref_arg \
               clippy::trivially_copy_pass_by_ref \
               clippy::unit_arg \
               clippy::unnecessary_operation \
               clippy::unreadable_literal \
               clippy::unused_label \
               clippy::unused_unit \
               clippy::useless_asref \
               clippy::useless_format \
               clippy::useless_let_if_seq \
               clippy::useless_vec \
               clippy::write_with_newline \
               clippy::wrong_self_convention \
               renamed_and_removed_lints)

clippy_args=()

add_lints_to_clippy_args() {
  flag=$1
  shift
  for lint
  do
    clippy_args+=("$flag" "${lint}")
  done
}

set +u # See https://stackoverflow.com/questions/7577052/bash-empty-array-expansion-with-set-u/39687362#39687362
add_lints_to_clippy_args -A "${unexamined_lints[@]}"
add_lints_to_clippy_args -A "${allowed_lints[@]}"
add_lints_to_clippy_args -W "${lints_to_fix[@]}"
add_lints_to_clippy_args -D "${denied_lints[@]}"
set -u

echo "--- Running clippy!"
echo "Clippy rules: cargo clippy --all-targets --tests -- ${clippy_args[*]}"
cargo +"$toolchain" clippy --all-targets --tests -- "${clippy_args[@]}"
