#!/bin/sh
set -u
: "$CERT_PATH"
: "$ARTIFACT_PATH"

build_sandbox_env() {
    sandbox_env="LC_ALL=POSIX TERM=${TERM:-} HAB_LICENSE=accept-no-persist"
    if [ -n "${HAB_BLDR_URL:-}" ]; then
        sandbox_env="$sandbox_env HAB_BLDR_URL=$HAB_BLDR_URL"
    fi
    # If a Habitat Auth Token is set, then propagate it into the Studio's
    # sandbox_environment.
    if [ -n "${HAB_AUTH_TOKEN:-}" ]; then
        sandbox_env="$sandbox_env HAB_AUTH_TOKEN=$HAB_AUTH_TOKEN"
    fi
    # If a Habitat Builder URL is set, then propagate it into the Studio's
    # sandbox_environment.
    if [ -n "${HAB_BLDR_URL:-}" ]; then
        sandbox_env="$sandbox_env HAB_BLDR_URL=$HAB_BLDR_URL"
    fi
    # If a Habitat Depot Channel is set, then propagate it into the Studio's
    # sandbox_environment.
    if [ -n "${HAB_BLDR_CHANNEL:-}" ]; then
        sandbox_env="$sandbox_env HAB_BLDR_CHANNEL=$HAB_BLDR_CHANNEL"
    fi
    # If a Habitat origin name is set, then propagate it into the Studio's
    # sandbox_environment.
    if [ -n "${HAB_ORIGIN:-}" ]; then
        sandbox_env="$sandbox_env HAB_ORIGIN=$HAB_ORIGIN"
    fi
    # If DO_CHECK is set, then propagate it into the Studio's sandbox_environment.
    if [ -n "${DO_CHECK:-}" ]; then
        sandbox_env="$sandbox_env DO_CHECK=$DO_CHECK"
    fi
}

build_sandbox_env

HAB_PACKAGE="core/build-tools-hab"
HAB_PLAN_BUILD_PACKAGE="core/build-tools-hab-plan-build"

sandbox_exec_cmd="$(command -v sandbox-exec)"
readlink_cmd="$(command -v readlink)"
pwd_cmd="$(command -v pwd)"
env_cmd="$(command -v env)"
system_hab_cmd="$(command -v hab)"
packaged_hab_cmd="$($system_hab_cmd pkg path $HAB_PACKAGE)/bin/hab"

script_path="${0%/*}"
script_path="$(
    cd "$script_path" || exit
    $pwd_cmd
)/${0##*/}"
script_path=$($readlink_cmd -f "$script_path")
script_dir="${script_path%/*}"

plan_context="$1"
plan_context="$(
    cd "$plan_context" || exit
    $pwd_cmd
)"
plan_context=$($readlink_cmd -f "$plan_context")

work_dir=$(pwd)
work_dir=$($readlink_cmd -f "$work_dir")

libexec_dir="$script_dir/../libexec"
sandbox_profile_path="$libexec_dir/darwin-sandbox.sb"

# $env_cmd -i DEBUG=1 RUST_LOG=trace TERM="$TERM" HAB_LICENSE=accept-no-persist "$sandbox_exec_cmd" -f "$sandbox_profile_path" -DPLAN_CONTEXT_DIR="$plan_context" -DWORK_DIR="$work_dir" -DALLOW_NETWORKING=1  /hab/pkgs/core/build-tools-hab/1.6.740/20230615123857/bin/hab pkg install -u https://bldr.habitat.sh --channel stable core/coreutils
$env_cmd -i \
    $sandbox_env \
    "$sandbox_exec_cmd" -f "$sandbox_profile_path" \
    -DHAB_CACHE_CERT_PATH="$CERT_PATH" \
    -DHAB_CACHE_ARTIFACT_PATH="$ARTIFACT_PATH" \
    -DPLAN_CONTEXT_DIR="$plan_context" \
    -DWORK_DIR="$work_dir" \
    -DALLOW_NETWORKING=1 \
    "$packaged_hab_cmd" pkg exec $HAB_PLAN_BUILD_PACKAGE hab-plan-build -- \
    "$plan_context"
