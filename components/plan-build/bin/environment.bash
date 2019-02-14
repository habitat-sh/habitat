#!/bin/bash
# Functions for resolving the runtime environment of a package.

declare -A __runtime_environment
declare -A __buildtime_environment

# shellcheck disable=2034
declare -A __runtime_environment_provenance
# shellcheck disable=2034
declare -A __buildtime_environment_provenance

# Layer all direct dependencies' environment together.
#
# Priority is last-one-wins, based on the order in which you define
# the dependencies in your plan file.
#
# (I'm sure there are many other common variables we could add here;
# PRs welcome!)
declare -A -g __well_known_aggregate_env_vars=(
    # Shell
    [PATH]=":"

    # Go
    [GOPATH]=":"

    # Java
    [CLASSPATH]=";"

    # NodeJS
    [NODE_PATH]=":"

    # Python
    [PYTHONPATH]=":"

    # Ruby
    [BUNDLE_PATH]=":"
    [BUNDLE_WITHOUT]=":"
    [GEM_PATH]=":"
    [RUBYLIB]=":"
    [RUBYPATH]=":"
)

# Purely internal implementation function to ensure we are operating
# on the correct data structures. See call sites for further context.
__fail_on_unrecognized_env() {
    local env_name=${1}
    if [ "${env_name}" != "__runtime_environment" ] &&
           [ "${env_name}" != "__buildtime_environment" ]; then
        exit_with "INTERNAL CODE ERROR: ${FUNCNAME[1]} was called at ${BASH_SOURCE[2]}:${BASH_LINENO[1]} with unrecognized environment variable name: ${env_name}"
    fi
}

__fail_on_protected_env_var_manipulation() {
    declare -A protected=(
        [PATH]="pkg_deps"
        [LD_RUN_PATH]="pkg_lib_dirs"
        [LDFLAGS]="pkg_lib_dirs"
        [CFLAGS]="pkg_include_dirs"
        [CPPFLAGS]="pkg_include_dirs"
        [CXXFLAGS]="pkg_include_dirs"
        [PKG_CONFIG_PATH]="pkg_pconfig_dirs"
    )
    local var=${1}
    for p in "${!protected[@]}"; do
        if [ "${var}" == "${p}" ]; then
            exit_with "Cannot directly manipulate environment variable ${var}! Add appropriate entries to the '${protected[${var}]}' variable in plan.sh instead!"
        fi
    done
}

# Each environment we deal with is populated using a different list of
# dependencies. Given an environment, return the name of the proper
# dependency list to use.
#
# Note: inputs and outputs of this function are Bash data structure
# *names*.
__dep_array_for_environment() {
    local env_name=${1}
    __fail_on_unrecognized_env "${env_name}"

    local dep_array_name
    case "${env_name}" in
        "__runtime_environment")
            dep_array_name="pkg_deps"
            ;;
        "__buildtime_environment")
            dep_array_name="pkg_build_deps"
            ;;
    esac
    echo "${dep_array_name}"
}

__provenance_for_environment() {
    declare -A map=(
        [__runtime_environment]="__runtime_environment_provenance"
        [__buildtime_environment]="__buildtime_environment_provenance"
    )
    local env_name=${1}
    __fail_on_unrecognized_env "${env_name}"
    echo "${map[${env_name}]}"
}

# Determine whether a given environment variable is a primitive or
# aggregate (i.e., PATH-style) variable.
__env_var_type() {
    local var_name="${1}"
    declare -n hint_var="HAB_ENV_${var_name}_TYPE"

    if [ -n "${hint_var}" ]; then
        # Look for user-specified hints first
        echo "${hint_var}"
    elif [ -n "${__well_known_aggregate_env_vars[${var_name}]}" ]; then
        # Look in our built-in map to see if we know anything about it
        echo 'aggregate'
    else
        # We know nothing about it; treat it as a primitive
        warn "Treating \$${var_name} as a primitive type. If you would like to change this, add \`HAB_ENV_${var_name}_TYPE=aggregate\` to your plan."
        echo 'primitive'
    fi
}

# Given that a variable is an aggregate (i.e., PATH-style) variable,
# return the separator character used to delimit items in the value.
__env_aggregate_separator() {
    local var_name="${1}"
    declare -n hint_var="HAB_ENV_${var_name}_SEPARATOR"

    if [ -n "${hint_var}" ]; then
        # Look for user-specified hints first
        echo "${hint_var}"
    elif [ -n "${__well_known_aggregate_env_vars[${var_name}]}" ]; then
        # Look in our built-in map to see if we know anything about it
        echo "${__well_known_aggregate_env_vars[${var_name}]}"
    else
        # Just assume it's the default
        warn "Using \`:\` as a separator for \$${var_name}. If you would like to change this, add \`HAB_ENV_${var_name}_SEPARATOR=<YOUR_SEPARATOR>\` to your plan."
        echo ':'
    fi
}

# Read in the RUNTIME_ENVIRONMENT files from all direct dependencies
# (in `pkg_deps` / `pkg_build_deps` order!) and layer them as appropriate.
__populate_environment_from_deps() {
    local path_to_dep

    local env_name=${1}
    __fail_on_unrecognized_env "${env_name}"
    declare -n env="${env_name}"
    declare -n provenance
    provenance="$(__provenance_for_environment "${env_name}")"

    local dep_array_name
    dep_array_name="$(__dep_array_for_environment "${env_name}")"
    declare -n dep_array="${dep_array_name}"


    for dep in "${dep_array[@]}"; do

        case "${env_name}" in
            "__runtime_environment")
                path_to_dep=$(_pkg_path_for_deps "${dep}")
                ;;
            "__buildtime_environment")
                path_to_dep=$(_pkg_path_for_build_deps "${dep}")
                ;;
        esac

        local dep_ident
        dep_ident=$(cat "${path_to_dep}/IDENT")

        if [ -f "${path_to_dep}/RUNTIME_ENVIRONMENT" ]; then
            while read -r line; do
                IFS="=" read -r var val <<< "${line}"
                # Any values of `PATH` are skipped as we will be computing the
                # runtime path independently of the RUNTIME_ENVIRONMENT
                # metadata files. Additionally, this acts as backwards
                # compatibility for all `RUNTIME_ENVIRONMENT` files that
                # contain a `PATH` key.
                if [[ "${var}" == "PATH" ]]; then
                  continue;
                fi

                if [ -n "${env["${var}"]}" ]; then
                    # There was a previous value; need to figure out
                    # how to proceed

                    # Where did the value come from originally?
                    local source="${provenance[${var}]}"
                    local current_value="${env[${var}]}"

                    if [ "${val}" == "${current_value}" ]; then
                        # If the value is the same as what we've got,
                        # there's nothing to do
                        continue
                    fi

                    case $(__env_var_type "${var}") in
                        primitive)
                            if [ -n "${current_value}" ]; then
                                warn "Overwriting \$${var} originally set from ${source}"
                            fi
                            __set_env "${env_name}" "${var}" "${val}" "${dep_ident}"
                        ;;
                        aggregate)
                            if [ -n "${current_value}" ]; then
                                warn "Prepending to \$${var} originally set from ${source}"
                            fi

                            # if aggregate, push to front with separator
                            local separator
                            separator=$(__env_aggregate_separator "${var}")
                            __push_env "${env_name}" "${var}" "${val}" "${separator}" "${dep_ident}"
                            ;;
                    esac
                else
                    # There was no previous value; just add this one
                    env["${var}"]="${val}"
                    provenance["${var}"]="${dep_ident}"
                fi
            done < <(_read_metadata_file_for "${path_to_dep}" RUNTIME_ENVIRONMENT)
        fi
    done
}

set_buildtime_env() {
    set_env "$@" "__buildtime_environment"
}

set_runtime_env() {
    set_env "$@" "__runtime_environment"
}

# shellcheck disable=2154
set_env(){
    local force=false
    local option

    OPTIND=1
    while getopts ":f" option; do
        case "${option}" in
            f) force=true
               ;;
            *) echo "Warning - unknown option ${option} ignored!"
               ;;
        esac
    done
    shift "$((OPTIND - 1))"

    local key="${1}"
    __fail_on_protected_env_var_manipulation "${key}"

    local value="${2}"

    local env_name=${3}
    __fail_on_unrecognized_env "${env_name}"
    # shellcheck disable=2178
    # This appears to be a bug: https://github.com/koalaman/shellcheck/issues/1225
    declare -n env="${env_name}"
    declare -n provenance
    provenance="$(__provenance_for_environment "${env_name}")"

    if [ -n "${env[${key}]}" ]; then
        if [ "${force}" == "false" ]; then
            exit_with "Already have a value for \$${key}, set by ${provenance[${key}]}: ${env[${key}]}. If you really wish to overwrite this value, pass the '-f' (\"force\") option when setting it."
        else
            warn "Already have a value for \$${key}, set by ${provenance[${key}]}: ${env[${key}]}. Overwriting value because the '-f' flag was passed"
        fi
    fi

    __set_env "$env_name" "${key}" "${value}" "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
}

# Internal function implementing core "set" logic for environment variables.
__set_env(){
    local env_name=${1}
    local var_name=${2}
    local value=${3}
    local ident=${4}

    __fail_on_unrecognized_env "${env_name}"
    # shellcheck disable=2178
    # This appears to be a bug: https://github.com/koalaman/shellcheck/issues/1225
    declare -n env="${env_name}"

    declare -n provenance
    provenance="$(__provenance_for_environment "${env_name}")"

    env["${var_name}"]="${value}"
    provenance["${var_name}"]="${ident}"
}

push_buildtime_env() {
    build_line "PUSH TO BUILD"
    do_push_env "__buildtime_environment" "$@"
}

push_runtime_env() {
    build_line "PUSH TO RUN"
    do_push_env "__runtime_environment" "$@"
}

do_push_env() {
    local env_name=${1}
    __fail_on_unrecognized_env "${env_name}"

    local key=${2}
    __fail_on_protected_env_var_manipulation "${key}"

    local value=${3}

    local sep
    sep="$(__env_aggregate_separator "${key}")"

    __push_env "${env_name}" "${key}" "${value}" "${sep}" "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
}

# Internal function implementing core "push" logic for environment variables.
__push_env() {

    local env_name=${1}
    local var_name=${2}
    local value=${3}
    local separator=${4}
    local ident=${5}

    __fail_on_unrecognized_env "${env_name}"
    # shellcheck disable=2178
    # This appears to be a bug: https://github.com/koalaman/shellcheck/issues/1225
    declare -n env="${env_name}"

    declare -n provenance
    provenance="$(__provenance_for_environment "${env_name}")"

    # If there is no current value (that is, $current_value == ""), we
    # can still push onto that with no loss of generality. Because
    # push_to_path also dedupes the result, this allows us to take
    # $value inputs that are themselves paths, which may have
    # duplicate or blank entries (as is the case with some existing
    # Habitat metadata files) and this will effectively "clean" them
    # for us!
    local current_value="${env[${var_name}]}"
    local new_value
    new_value=$(push_to_path "${value}" "${current_value}" "${separator}")
    env["${var_name}"]="${new_value}"

    local existing_provenance="${provenance[${var_name}]}"
    provenance["${var_name}"]="$(push_to_path "${ident}" "${existing_provenance}" " ")"
}

dedupe_path(){
    local separator=${2:-:}
    local original_path=${1}${separator}

    local new_path
    local path_item

    if [ -n "${original_path}" ]; then
      while [ -n "${original_path}" ]; do
        path_item="${original_path%%${separator}*}"       # the first remaining entry
        case "${new_path}" in
            *${separator}${path_item})
              ;&
            ${path_item}${separator}*)
              ;&
            *${separator}${path_item}${separator}*)
              ;;         # already there
            *)
              new_path="${new_path}${separator}${path_item}"
              ;;    # not there yet
        esac
        original_path="${original_path#*${separator}}"
      done
      new_path="${new_path#${separator}}"
    fi

    echo "${new_path}"
}

# Pushes $ITEM onto $PATH (using optional $SEPARATOR) and then
# deduplicates entries.
#
# push_to_path "foo" "bar:foo:baz"
#   -> "foo:bar:baz"
#
# push_to_path "foo" ""
#   -> "foo"
#
# push_to_path "foo" "bar;baz" ";"
#   -> "foo;bar;baz"
#
push_to_path() {
    local item=${1}
    local path=${2}
    local separator=${3:-:}

    local temp

    if [ "" == "${path}" ]; then
        temp="${item}"
    else
        temp="${item}${separator}${path}"
    fi

    dedupe_path "${temp}" "${separator}"
}

do_setup_environment_wrapper() {
    build_line "Setting up environment"
    build_line "Populating runtime environment from dependencies"
    __populate_environment_from_deps "__runtime_environment"
    build_line "Populating buildtime environment from dependencies"
    __populate_environment_from_deps "__buildtime_environment"

    do_setup_environment

    build_line "Layering runtime environment on top of system environment"
    # Export everything from our collected runtime environment into
    # the real environment, except for PATH; for that, push the
    # runtime path onto the front of the system path
    for k in "${!__runtime_environment[@]}"; do
        local v="${__runtime_environment[${k}]}"
        export "${k}"="${v}"
    done

    build_line "Layering buildtime environment on top of system environment"
    # Layer buildtime environment values into the system environment,
    # which has already had the runtime values merged in. This is a
    # stripped-down version of the logic used to layer environments
    # from dependencies in the first place.
    for k in "${!__buildtime_environment[@]}"; do
        local v="${__buildtime_environment[${k}]}"
        if [ -n "${!k}" ]; then
            # There was a previous value; need to figure out
            # how to proceed
            if [ "${!k}" == "${v}" ]; then
                # If the value is the same as what we've got,
                # there's nothing to do
                continue
            fi

            case $(__env_var_type "${k}") in
                primitive)
                    export "${k}"="${v}"
                    ;;
                aggregate)
                    local separator
                    separator=$(__env_aggregate_separator "${k}")
                    export "${k}"="$(push_to_path "${v}" "${!k}" "${separator}")"
                    ;;
            esac
        else
            # There was no previous value; just set this one
            export "${k}"="${v}"
        fi
    done
}

do_setup_environment() {
    return 0
}
