#!/bin/bash
# These functions are used when building a composite package.

# Ensure that a composite package is internally consistent.
_validate_composite() {
    _assert_more_than_one_service
    # shellcheck disable=2154
    _resolve_service_dependencies "${pkg_services[@]}"
    _validate_services

    # Validate all the bind mappings
    _resolve_all_exports
    _validate_bind_mappings
}

# TODO (CM): normalize names (assert/validate/ensure?)

# Create global variable for mapping a service name as given in plan.sh to
# the path-on-disk of the fully-qualified service that is resolved at plan
# build time
_setup_resolved_services(){
  _setup_global_associative_array resolved_services
}

# Create the pkg_export_map associative array.
_setup_pkg_export_map(){
  _setup_global_associative_array pkg_export_map
}

# Helper function to create, um... global associative arrays.
_setup_global_associative_array(){
  local var_name=${1}
  debug "Creating '${var_name}' global associative array"
  declare -A -g "${var_name}"
}

_setup_composite_build_global_variables(){
    _setup_resolved_services
    _setup_pkg_export_map
}

# If you didn't specify any packages, why are you making a composite?
# If you only specified one, why aren't you just using that directly?
_assert_more_than_one_service() {
    if [ "${#pkg_services[@]}" -lt "2" ]; then
        exit_with "A composite package should have at least two services specified in \$pkg_services; otherwise just build a non-composite Habitat package" 1
    fi
}

# Pass in an array of service names from plan.sh and install them locally.
# Record in the global `resolved_services` associative array the mapping from
# the service as-given to the path-on-disk of the fully-resolved package.
#
# TODO (CM): borrowed from resolve_run_dependencies & company;
# consider further refactoring and consolidation
#
# TODO (CM): Consider just reading from pkg_services globally
_resolve_service_dependencies() {
    build_line "Resolving service dependencies"

    local services=("${@}")
    local resolved
    local service

    for service in "${services[@]}"; do
      build_line "Installing ${service} locally"
      _install_dependency "${service}"
      if resolved="$(_resolve_dependency "$service")"; then
        build_line "Resolved service '$service' to $resolved"
        resolved_services[$service]=$resolved
      else
        exit_with "Resolving '$service' failed, should this be built first?" 1
      fi
    done
}

# Ensure that all the services are actually services.
_validate_services() {
    local resolved

    for rs in "${!resolved_services[@]}"; do
        resolved=${resolved_services[$rs]}
        _assert_package_is_a_service "${resolved}"
    done
}

# Given the path to an expanded package on disk, determine if it's a service
# (i.e., a package that has a run script)
_assert_package_is_a_service() {
    local pkg_path="${1}"
    build_line "Verifying that ${pkg_path} is a service"
    if [ ! -e "${pkg_path}/run" ] && [ ! -e "${pkg_path}/hooks/run" ]; then
        exit_with "'${pkg_path}' is not a service. Only services are allowed in composite packages"
    fi
}

# Assemble a list of all the exports from a given package and return the list on
# standard output.
_exports_for_pkg() {
  local path_to_pkg_on_disk=${1}
  local exports=()
  local line

  while read -r line; do
    exports+=("${line%%=*}")
  done < <(_read_metadata_file_for "${path_to_pkg_on_disk}" EXPORTS)

  echo "${exports[@]}"
}

# Grab all the exports for the universe of packages
# e.g. core/builder-api-proxy => "foo bar baz"
_resolve_all_exports() {
  local resolved
  local exports

  for rs in "${!resolved_services[@]}"; do
    resolved=${resolved_services[$rs]}
    exports=("$(_exports_for_pkg "${resolved}")")
    pkg_export_map[$resolved]="${exports[*]}"
  done
}

# Ensure that all the bind mappings supplied are valid. This means:
#
# * The binds are actually defined for the given package
# * The package that satisfies the bind actually exports what the bind requires
_validate_bind_mappings() {
  # shellcheck disable=2154
  for pkg in "${!pkg_bind_map[@]}"; do
    debug "Resolving binds for ${pkg}"

    # TODO (CM): Here we are implicitly assuming that the values in
    # the pkg_bind_map are exactly the same as given in
    # pkg_services. Is this the right thing, or should we normalize to
    # `origin/package` instead, regardless of what # was given in
    # pkg_services?

    # Need to grab all the binds of `pkg` from its metadata on disk
    unset all_binds_for_pkg
    declare -A all_binds_for_pkg

    resolved="${resolved_services[$pkg]}"

    if [[ -f "${resolved}/BINDS" ]]; then
      while read -r line; do
        IFS="=" read -r bind_name exports <<< "${line}"
        all_binds_for_pkg[$bind_name]="${exports[*]}"
      done < <(_read_metadata_file_for "${resolved}" BINDS)
    fi
    if [[ -f "${resolved}/BINDS_OPTIONAL" ]]; then
      while read -r line; do
        IFS="=" read -r bind_name exports <<< "${line}"
        if [[ -n "${all_binds_for_pkg[$bind_name]}" ]]; then
          exit_with "The bind ${bind_name} has already been declared in pkg_binds for package ${pkg}, it cannot also be declared in pkg_binds_optional"
        fi
        all_binds_for_pkg[$bind_name]="${exports[*]}"
      done < <(_read_metadata_file_for "${resolved}" BINDS_OPTIONAL)
    fi

    unset bind_mappings
    bind_mappings=("${pkg_bind_map[$pkg]}")

    # This is space-delimited, so no quotes
    # shellcheck disable=2068,2180
    for mapping in ${bind_mappings[@]}; do
      # Each mapping is of the form `bind_name:package`, like so:
      #     router:core/builder-router
      IFS=: read -r bind_name satisfying_package <<< "${mapping}"

      # Assert that the named bind exists
      debug "Verifying that ${resolved} has a bind named '${bind_name}'"
      if [ -z "${all_binds_for_pkg[$bind_name]}" ]; then
        exit_with "The bind '${bind_name}' specified in \$pkg_bind_map for the package '${pkg}' does not exist in ${resolved_services[$pkg]}."
      fi

      resolved_satisfying_package="${resolved_services[$satisfying_package]}"
      satisfying_package_exports=("${pkg_export_map[$resolved_satisfying_package][@]}")

      debug "Checking that the bind '${bind_name}' for ${resolved} can be satisfied by ${resolved_satisfying_package}"

      # Assert that the mapped service satisfies all the exports
      # of this bind
      for required_exported_value in ${all_binds_for_pkg[$bind_name][@]}; do
        if ! _array_contains "$required_exported_value" "${satisfying_package_exports[@]}"; then
          exit_with "${satisfying_package} does not export '${required_exported_value}', which is required by the '${bind_name}' bind of ${resolved}"
        fi
      done
    done
  done
}

################################################################################
# Composite Package Metadata Rendering functions

_render_composite_metadata() {
    build_line "Building package metadata"

    _render_metadata_BIND_MAP
    _render_metadata_RESOLVED_SERVICES
    _render_metadata_SERVICES

    # TODO (CM): Consider renaming to reflect "common" metadata, or
    # just have the functions be robust enough so that we can just
    # render EVERYTHING and have it just work.

    # NOTE: These come from the shared.bash library
    _render_metadata_IDENT
    _render_metadata_TARGET
    _render_metadata_TYPE
}

# Render the services AS GIVEN IN THE PLAN; DO NOT perform any
# truncation based on HAB_PKG_PATH as with other dependency-type
# metadata files.
_render_metadata_SERVICES() {
  deps="$(printf '%s\n' "${pkg_services[@]}" | sort)"
  if [[ -n "$deps" ]]; then
    debug "Rendering SERVICES metadata file"
    # shellcheck disable=2154
    echo "$deps" > "$pkg_prefix"/SERVICES
  fi
}

# Render the services AS RESOLVED at build time; when you install the
# composite, these are the releases that will be downloaded.
_render_metadata_RESOLVED_SERVICES() {
    _render_dependency_metadata_file "$pkg_prefix" RESOLVED_SERVICES resolved_services
}

_render_metadata_BIND_MAP() {
  _render_associative_array_file "${pkg_prefix}" BIND_MAP pkg_bind_map
}
