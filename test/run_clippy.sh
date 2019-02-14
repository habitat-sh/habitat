#!/bin/bash

set -eoux pipefail

sudo hab pkg install core/bzip2
sudo hab pkg install core/libarchive
sudo hab pkg install core/libsodium
sudo hab pkg install core/openssl
sudo hab pkg install core/xz
sudo hab pkg install core/zeromq
sudo hab pkg install core/protobuf --binlink
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

# TODO: fix this upstream so it's already on the path and set up
export RUSTUP_HOME=/opt/rust
export CARGO_HOME=/home/buildkite-agent/.cargo
export PATH=/opt/rust/bin:$PATH
# TODO: fix this upstream, it looks like it's not saving correctly.
sudo chown -R buildkite-agent /home/buildkite-agent

 # Lints we need to work through and decide as a team whether to allow or fix
unexamined_lints=("clippy::cyclomatic_complexity" \
                  "clippy::large_enum_variant" \
                  "clippy::len_without_is_empty" \
                  "clippy::module_inception" \
                  "clippy::needless_pass_by_value" \
                  "clippy::needless_return" \
                  "clippy::new_ret_no_self" \
                  "clippy::new_without_default" \
                  "clippy::new_without_default_derive" \
                  "clippy::question_mark" \
                  "clippy::redundant_field_names" \
                  "clippy::too_many_arguments" \
                  "clippy::trivially_copy_pass_by_ref" \
                  "clippy::wrong_self_convention" \
                  "renamed_and_removed_lints")

 # Lints we disagree with and choose to keep in our code with no warning
allowed_lints=("")

 # Known failing lints we want to receive warnings for, but not fail the build
lints_to_fix=("")

 # Lints we don't expect to have in our code at all and want to avoid adding
# even at the cost of failing the build
denied_lints=("clippy::assign_op_pattern" \
              "clippy::blacklisted_name" \
              "clippy::block_in_if_condition_stmt" \
              "clippy::bool_comparison" \
              "clippy::cast_lossless" \
              "clippy::clone_on_copy" \
              "clippy::cmp_owned" \
              "clippy::collapsible_if" \
              "clippy::const_static_lifetime" \
              "clippy::correctness" \
              "clippy::deref_addrof" \
              "clippy::expect_fun_call" \
              "clippy::for_kv_map" \
              "clippy::get_unwrap" \
              "clippy::identity_conversion" \
              "clippy::if_let_some_result" \
              "clippy::len_zero" \
              "clippy::let_and_return" \
              "clippy::let_unit_value" \
              "clippy::map_clone" \
              "clippy::match_bool" \
              "clippy::match_ref_pats" \
              "clippy::needless_bool" \
              "clippy::needless_collect" \
              "clippy::needless_range_loop" \
              "clippy::ok_expect" \
              "clippy::op_ref" \
              "clippy::option_map_unit_fn" \
              "clippy::or_fun_call" \
              "clippy::println_empty_string" \
              "clippy::ptr_arg" \
              "clippy::redundant_closure" \
              "clippy::redundant_pattern_matching" \
              "clippy::single_char_pattern" \
              "clippy::single_match" \
              "clippy::string_lit_as_bytes" \
              "clippy::toplevel_ref_arg" \
              "clippy::unit_arg" \
              "clippy::unnecessary_operation" \
              "clippy::unreadable_literal" \
              "clippy::unused_label" \
              "clippy::unused_unit" \
              "clippy::useless_asref" \
              "clippy::useless_format" \
              "clippy::useless_let_if_seq" \
              "clippy::useless_vec" \
              "clippy::write_with_newline")

 # stick together unexamined lints
for unexamined_lint in "${unexamined_lints[@]}"
do
  [[ -n $unexamined_lint ]] && rules_string+="-A ${unexamined_lint} "
done

 # stick together allowed lints
for allowed_lint in "${allowed_lints[@]}"
do
  [[ -n $allowed_lint ]] && rules_string+="-A ${allowed_lint} "
done

 # stick together lints to fix
for lint_to_fix in "${lints_to_fix[@]}"
do
  [[ -n $lint_to_fix ]] && rules_string+="-W ${lint_to_fix} "
done

 # stick together denied lints
for denied_lint in "${denied_lints[@]}"
do
  [[ -n $denied_lint ]] && rules_string+="-D ${denied_lint} "
done

 component=${1?component argument required}
echo "--- Running clippy!"
echo "Clippy rules: cargo clippy --all-targets --tests -- ${rules_string}"
cd "components/$component"
# If we include double quotes in this string, the invocation of cargo breaks
# shellcheck disable=SC2086
cargo clippy --all-targets --tests -- ${rules_string} 
