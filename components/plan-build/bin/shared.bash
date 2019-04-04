# These functions are intended for internal usage for building plans.
#
# End-users should not attempt to use these functions directly, as no guarantees
# about their API are made.

################################################################################
# Common Metadata Rendering functions

_render_metadata_BINDS() {
    # shellcheck disable=2154
    _render_associative_array_file "${pkg_prefix}" BINDS pkg_binds
}

_render_metadata_BINDS_OPTIONAL() {
    _render_associative_array_file "${pkg_prefix}" BINDS_OPTIONAL pkg_binds_optional
}

_render_metadata_BUILD_DEPS() {
  _render_dependency_metadata_file "${pkg_prefix}" BUILD_DEPS pkg_build_deps_resolved
}

_render_metadata_BUILD_TDEPS() {
  _render_dependency_metadata_file "${pkg_prefix}" BUILD_TDEPS pkg_build_tdeps_resolved
}

_render_metadata_CFLAGS() {
    _render_c_includes_metadata_file "${pkg_prefix}" CFLAGS pkg_include_dirs
}

_render_metadata_CPPFLAGS() {
    _render_c_includes_metadata_file "${pkg_prefix}" CPPFLAGS pkg_include_dirs
}

_render_metadata_CXXFLAGS() {
    _render_c_includes_metadata_file "${pkg_prefix}" CXXFLAGS pkg_include_dirs
}

_render_metadata_BUILDTIME_ENVIRONMENT(){
    debug "Rendering BUILDTIME_ENVIRONMENT metadata file"
    _render_associative_array_file "${pkg_prefix}" BUILDTIME_ENVIRONMENT __buildtime_environment
}

_render_metadata_BUILDTIME_ENVIRONMENT_PROVENANCE(){
    debug "Rendering BUILDTIME_ENVIRONMENT_PROVENANCE metadata file"
    _render_associative_array_file "${pkg_prefix}" BUILDTIME_ENVIRONMENT_PROVENANCE __buildtime_environment_provenance
}

_render_metadata_DEPS() {
  _render_dependency_metadata_file "${pkg_prefix}" DEPS pkg_deps_resolved
}

_render_metadata_EXPORTS() {
    _render_associative_array_file "${pkg_prefix}" EXPORTS pkg_exports
}

_render_metadata_EXPOSES() {
  # TODO (CM): rename port_part and make it an array
  local port_part=""
  # shellcheck disable=2154
  for export in "${pkg_exposes[@]}"; do
    if [[ ! ${pkg_exports[$export]+abc} ]]; then
      exit_with "Bad value in pkg_exposes; No pkg_export found matching key: ${export}"
    fi
    key=${pkg_exports[$export]}
    port=$($_rq_cmd -t < "$PLAN_CONTEXT"/default.toml "at \"${key}\"" | tr -d '"')
    if ! _port_is_valid "$port"; then
      exit_with "Bad pkg_export in pkg_exposes; Value of key \"${key}\" does not contain a valid TCP or UDP port number: ${port}"
    fi

    if [[ -z "$port_part" ]]; then
      port_part="$port";
    else
      port_part="$port_part $port";
    fi
  done

  if [[ -n "${port_part}" ]]; then
    debug "Rendering EXPOSES metadata file"
    echo "$port_part" > "$pkg_prefix"/EXPOSES
  fi
}

# Generate the blake2b hashes of all the files in the package. This
# is not in the resulting MANIFEST because MANIFEST is included!
_render_metadata_FILES() {

  pushd "$CACHE_PATH" || exit
  build_line "Generating blake2b hashes of all files in the package"

  # shellcheck disable=2154
  find "$pkg_prefix" -type f \
    | sort \
    | $HAB_BIN pkg hash > "${pkg_name}"_blake2bsums

  build_line "Generating signed metadata FILES"
  # shellcheck disable=2154
  $HAB_BIN pkg sign --origin "$pkg_origin" "${pkg_name}"_blake2bsums "$pkg_prefix"/FILES
  popd || exit
}

_render_metadata_IDENT() {
  debug "Rendering IDENT metadata file"
  # shellcheck disable=2154
  echo "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}" >> "$pkg_prefix"/IDENT
}

_render_metadata_INTERPRETERS() {
    local metadata_file_name="INTERPRETERS"

    # shellcheck disable=2154
    if [[ ${#pkg_interpreters[@]} -gt 0 ]]; then
        debug "Rendering ${metadata_file_name} metadata file"
        local interpreters
        # shellcheck disable=2034
        interpreters="$(printf "${pkg_prefix}/%s\n" "${pkg_interpreters[@]}")"
        printf "%s\n" "${pkg_interpreters[@]}" \
            | sed "s|^|${pkg_prefix}/|" > "$pkg_prefix"/${metadata_file_name}
    else
        debug "Would have rendered ${metadata_file_name}, but there was no data for it"
    fi
}

_render_metadata_LDFLAGS(){
    local metadata_file_name="LDFLAGS"

    local ld_lib_part=()
    # shellcheck disable=2154
    for lib in "${pkg_lib_dirs[@]}"; do
        ld_lib_part+=("-L${pkg_prefix}/$lib")
    done
    if [[ -n ${ld_lib_part[*]} ]]; then
        debug "Rendering LDFLAGS metadata file"
        join_by ' ' "${ld_lib_part[@]}" > "$pkg_prefix/${metadata_file_name}"
    else
        debug "Would have rendered ${metadata_file_name}, but there was no data for it"
    fi
}

_render_metadata_LD_RUN_PATH() {
    local metadata_file_name="LD_RUN_PATH"

    local ld_run_path_part=()
    for lib in "${pkg_lib_dirs[@]}"; do
        ld_run_path_part+=("${pkg_prefix}/$lib")
    done
    if [[ -n ${ld_run_path_part[*]} ]]; then
        debug "Rendering ${metadata_file_name} metadata file"
        join_by ':' "${ld_run_path_part[@]}" > "$pkg_prefix/${metadata_file_name}"
    else
        debug "Would have rendered ${metadata_file_name}, but there was no data for it"
    fi
}

# The PATH metadata file contains ONLY the bins contained in your package
# for `pkg_bin_dirs`
#
_render_metadata_PATH() {
  # shellcheck disable=2154
  if [[ ${#pkg_bin_dirs[@]} -gt 0 ]]; then
    local paths=()
    local dir

    debug "Rendering PATH metadata file"
    # Contents of `pkg_bin_dirs` are relative to the plan root;
    # prepend the full path to this release so everything resolves
    # properly once the package is installed.
    for dir in "${pkg_bin_dirs[@]}"; do
        paths+=("${pkg_prefix}/${dir}")
    done

    join_by ':' "${paths[@]}" > "${pkg_prefix}/PATH"
  else
    debug "Would have rendered PATH, but there was no data for it"
  fi
}

_render_metadata_PKG_CONFIG_PATH() {
    local pconfig_path_part=()
    local metadata_file_name="PKG_CONFIG_PATH"

  # shellcheck disable=2154
  if [[ ${#pkg_pconfig_dirs[@]} -eq 0 ]]; then
    # Plan doesn't define pkg-config paths so let's try to find them in the conventional locations
    local locations=(lib/pkgconfig share/pkgconfig)
    for dir in "${locations[@]}"; do
      if [[ -d "${pkg_prefix}/${dir}" ]]; then
        pconfig_path_part+=("${pkg_prefix}/${dir}")
      fi
    done
  else
    # Plan explicitly defines pkg-config paths so we don't provide defaults
    for inc in "${pkg_pconfig_dirs[@]}"; do
      pconfig_path_part+=("${pkg_prefix}/${inc}")
    done
  fi

  if [[ -n ${pconfig_path_part[*]} ]]; then
    debug "Rendering ${metadata_file_name} metadata file"
    join_by ':' "${pconfig_path_part[@]}" > "$pkg_prefix/${metadata_file_name}"
  else
      debug "Would have rendered ${metadata_file_name}, but there was no data for it"
  fi
}

_render_metadata_RUNTIME_ENVIRONMENT(){
    debug "Rendering RUNTIME_ENVIRONMENT metadata file"
    _render_associative_array_file "${pkg_prefix}" RUNTIME_ENVIRONMENT __runtime_environment
}

_render_metadata_RUNTIME_ENVIRONMENT_PROVENANCE(){
    debug "Rendering RUNTIME_ENVIRONMENT_PROVENANCE metadata file"
    _render_associative_array_file "${pkg_prefix}" RUNTIME_ENVIRONMENT_PROVENANCE __runtime_environment_provenance
}

_render_metadata_RUNTIME_PATH(){
  local runtime_path

  runtime_path="$(_assemble_runtime_path)"
  if [[ -n "$runtime_path" ]]; then
    debug "Rendering RUNTIME_PATH metadata file"
    echo "$runtime_path" > "${pkg_prefix}/RUNTIME_PATH"

    # **Internal**  Backwards Compatibility: Set the `PATH` key for the runtime
    # environment if a computed runtime path is necessary which will be used by
    # Habitat releases between 0.50.0 (released 2017-11-30) and up to including
    # 0.55.0 (released 2018-03-20). All future releases will ignore the `PATH`
    # entry in favor of using the `RUNTIME_PATH` metadata file.
    # shellcheck disable=2034
    __runtime_environment["PATH"]="$runtime_path"
  else
    debug "Would have rendered RUNTIME_PATH, but there was no data for it"
  fi
}

_render_metadata_SVC_GROUP() {
  debug "Rendering SVC_GROUP metadata file"
  # shellcheck disable=2154
  echo "$pkg_svc_group" > "$pkg_prefix"/SVC_GROUP
}

_render_metadata_SVC_USER() {
  debug "Rendering SVC_USER metadata file"
  # shellcheck disable=2154
  echo "$pkg_svc_user" > "$pkg_prefix"/SVC_USER
}

_render_metadata_TARGET() {
  debug "Rendering TARGET metadata file"
  # shellcheck disable=2154
  echo "$pkg_target" > "$pkg_prefix"/TARGET
}

_render_metadata_TDEPS() {
  _render_dependency_metadata_file "${pkg_prefix}" TDEPS pkg_tdeps_resolved
}

# Metadata-rendering Helper Functions
########################################################################

# Give the pkg_prefix, the name of a metadata file to write, and the
# *name* of an associative array data structure (see Bash namerefs),
# and we will write the data in that array out to that metadata file.
#
# Data will be written as "key=value", one pair per line.
#
# If the associative array is empty, nothing is written out; it's a no-op.
_render_associative_array_file() {
  local prefix=${1}
  local metadata_file_name=${2}
  declare -n assoc_arr=${3}

  if [[ ${#assoc_arr[@]} -gt 0 ]]; then
    debug "Rendering ${metadata_file_name} metadata file"
    for key in "${!assoc_arr[@]}"; do
      echo "${key}=${assoc_arr[${key}]}" >> "${prefix}"/"${metadata_file_name}"
    done
  else
    debug "Would have rendered ${metadata_file_name}, but there was no data for it"
  fi
}

# Give the pkg_prefix, the name of a metadata file to write, and the
# *name* of an array data structure of includes directories, write a
# space-delimited list of include flags (e.g., '-I/path/to/dir') to
# the indicated metadata file
#
# Use this to render CFLAGS, CPPFLAGS, and CXXFLAGS files.
_render_c_includes_metadata_file() {
    local prefix=${1}
    local metadata_file_name=${2}
    declare -n arr=${3}
    local flags=()

    for inc in "${arr[@]}"; do
        flags+=("-I${pkg_prefix}/${inc}")
    done
    if [[ ${#flags[@]} -gt 0 ]]; then
        debug "Rendering ${metadata_file_name} metadata file"
        join_by ' ' "${flags[@]}" > "$pkg_prefix/${metadata_file_name}"
    else
        debug "Would have rendered ${metadata_file_name}, but there was no data for it"
    fi
}

# Metadata files that deal with dependencies (DEPS, TDEPS, etc) deal
# with paths-to-releases-on-disk. To get the identifiers of those
# packages, we currently derive it from the path.
_render_dependency_metadata_file() {
  local prefix=${1}
  local metadata_file_name=${2}
  declare -n arr=${3}

  local cutn="$(($(echo "$HAB_PKG_PATH" | grep -o '/' | wc -l)+2))"
  local deps

  deps="$(printf '%s\n' "${arr[@]}" \
    | cut -d "/" -f ${cutn}- | sort)"
  if [[ -n "$deps" ]]; then
    debug "Rendering ${metadata_file_name} metadata file"
    echo "$deps" > "${prefix}"/"${metadata_file_name}"
  else
      debug "Would have rendered ${metadata_file_name}, but there was no data for it"
  fi
}

########################################################################

# Given a path to a package's directory on disk and the name of a package
# metadata file, returns the contents of that file on standard output.
_read_metadata_file_for() {
  local pkg_path="${1}"
  local filename="${2}"
  local full_path="${pkg_path}/${filename}"
  if [[ -f "${full_path}" ]]; then
    cat "${full_path}"
  else
    echo
  fi
}
