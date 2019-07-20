# These functions constitute the public API of plan.sh
#
# These functions are supported by the Habitat project, and can be used in your
# own plan.sh files.

# Check that the command exists, 0 if it does, 1 if it does not.
#
# ```sh
# exists gsha256sum
# ```
#
# Would return 0 if gsha256sum exists, 1 if it does not.
exists() {
  if command -v "$1" >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# Print a line of build output. Takes the rest of the line as its only
# argument.
#
# ```sh
# build_line "Checksum verified - ${pkg_shasum}"
# ```
build_line() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    # shellcheck disable=2154
    echo "   ${pkg_name}: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "   \033[1;36m${pkg_name}: \033[1;37m$1\033[0m"
        ;;
      *)
        echo "   ${pkg_name}: $1"
        ;;
    esac
  fi
  return 0
}

# Print a warning line on stderr. Takes the rest of the line as its only
# argument.
#
# ```sh
# warn "Checksum failed"
# ```
warn() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    >&2 echo "   ${pkg_name}: WARN: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        >&2 echo -e "   \033[1;36m${pkg_name}: \033[1;33mWARN: \033[1;37m$1\033[0m"
        ;;
      *)
        >&2 echo "   ${pkg_name}: WARN: $1"
        ;;
    esac
  fi
  return 0
}

# Prints a line only if the `$DEBUG` environment value is set.
#
# ```sh
# DEBUG=1
# debug "Only if things are set"
# # "DEBUG: Only if things are set"
# DEBUG=0
# debug "Not so much anymore"
# ```
#
debug() {
  if [[ -z "$DEBUG" ]]; then
    return 0
  fi

  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "   ${pkg_name}: DEBUG: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "   \033[1;36m${pkg_name}: \033[0mDEBUG: $1"
        ;;
      *)
        echo "   ${pkg_name}: DEBUG: $1"
        ;;
    esac
  fi
  return 0
}

# Exit the program with an error message and a status code.
#
# ```
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "   ${pkg_name}: ERROR: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "   \033[1;36m${pkg_name}: \033[1;31mERROR: \033[1;37m$1\033[0m"
        ;;
      *)
        echo "   ${pkg_name}: ERROR: $1"
        ;;
    esac
  fi
  exit "${2:-1}"
}

# Trim leading and trailing whitespace.  [Thanks to these
# guys](http://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-bash-variable)
# for the tip.
#
# ```sh
# local data=$(cat /tmp/somefile)
# local trimmed=$(trim $data)
# ```
trim() {
  local var="$*"
  var="${var#"${var%%[![:space:]]*}"}"   # remove leading whitespace characters
  var="${var%"${var##*[![:space:]]}"}"   # remove trailing whitespace characters
  echo "$var"
}

# Returns the path for the desired build or runtime direct package dependency
# on stdout from the resolved dependency set.
#
# ```
# pkg_all_deps_resolved=(
#   /hab/pkgs/acme/zlib/1.2.8/20151216221001
#   /hab/pkgs/acme/nginx/1.8.0/20150911120000
#   /hab/pkgs/acme/glibc/2.22/20151216221001
# )
#
# pkg_path_for acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# pkg_path_for zlib
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# pkg_path_for glibc/2.22
# # /hab/pkgs/acme/glibc/2.22/20151216221001
# ```
#
# Will return 0 if a package is found locally on disk, and 1 if a package
# cannot be found. A message will be printed to stderr to provide context.
pkg_path_for() {
  local dep="$1"
  local e
  local cutn
  cutn="$(($(echo "$HAB_PKG_PATH" | grep -o '/' | wc -l)+2))"
  # shellcheck disable=2154
  for e in "${pkg_all_deps_resolved[@]}"; do
    if echo "$e" | cut -d "/" -f ${cutn}- | grep -E -q "(^|/)${dep}(/|$)"; then
      echo "$e"
      return 0
    fi
  done
  warn "pkg_path_for() '$dep' did not find a suitable installed package"
  warn "Resolved package set: (${pkg_all_deps_resolved[*]})"
  return 1
}

# Attach to an interactive debugging session which lets the user check the
# state of variables, call arbitrary functions, turn on higher levels of
# logging (with `set -x`), or whatever else is useful.
#
# Usage: simply add `attach` in a `plan.sh` file and a debugging session will
# spawn, similar to:
#
# ```
# ### Attaching to debugging session
#
# From: /plans/glibc/plan.sh @ line 66 :
#
#     56:
#     57:   # Modify the ldd rewrite script to remove lib64 and libx32
#     58:   sed -i'' '/RTLDLIST/d' sysdeps/unix/sysv/linux/*/ldd-rewrite.sed
#     59:
#     60:   rm -rf ../${pkg_name}-build
#     61:   mkdir ../${pkg_name}-build
#     62:   pushd ../${pkg_name}-build > /dev/null
#     63:     # Configure Glibc to install its libraries into `$pkg_prefix/lib`
#     64:     echo "libc_cv_slibdir=$pkg_prefix/lib" >> config.cache
#     65:
#  => 66:     attach
#     67:
#     68:     ../$pkg_dirname/configure \
#     69:       --prefix=$pkg_prefix \
#     70:       --libdir=$pkg_prefix/lib \
#     71:       --libexecdir=$pkg_prefix/lib/glibc \
#     72:       --enable-obsolete-rpc \
#     73:       --disable-profile \
#     74:       --enable-kernel=2.6.32 \
#     75:       --cache-file=config.cache
#     76:     make
#
# [1] glibc(build)>
# ```
attach() {
  if [[ "${STUDIO_TYPE}" == "" ]]; then
    warn "Non interactive studio, skipping 'attach'"
    return 0
  fi
  printf "\n### Attaching to debugging session\n"
  local cmd=""
  local fname="${FUNCNAME[1]}"
  local replno=1
  # Print out our current code context (source file, line number, etc.)
  _attach_whereami
  # Clear on exit traps and allow for non-zero returning commands as we're
  # entering a debugging session, remember?
  trap - 1 2 3 15 ERR
  set +e
  # Loop through input, REPL-style until either `"exit"` or `"quit"` is found
  while [[ "$cmd" != "exit" && "$cmd" != "quit" ]]; do
    read -e -r -p "[$replno] ${pkg_name}($fname)> " cmd
    history -s "$cmd"
    case "$cmd" in
      vars) (set -o posix; set);;
      whereami*|\@*)
        _attach_whereami "$(echo "$cmd" \
         | awk '{if (NF == 2) print $2; else print "10"}')"
        ;;
      exit|quit) ;;
      exit-program|quit-program) exit $?;;
      help)
        echo "
Help
  help          Show a list of command or information about a specific command.

Context
  whereami      Show the code surrounding the current context
                (add a number to increase the lines of context).

Environment
  vars          Prints all the environment variables that are currently in scope.

Navigating
  exit          Pop to the previous context.
  exit-program  End the $0 program.

Aliases
  @             Alias for \`whereami\`.
  quit          Alias for \`exit\`.
  quit-program  Alias for \`exit-program\`.
"
        ;;
      *) eval "$cmd";;
    esac
    # Increment our REPL command line count, cause that's helpful
    replno=$((replno + 1))
  done
  # Re-enable on exit trap and restore exit-on-non-zero behavior
  trap _on_exit 1 2 3 15 ERR
  set -e
  printf "\n### Leaving debugging session\n\n"
  return 0
}

# Return the absolute path for a path, which might be absolute or relative.
#
# ```sh
# abspath .
# # /a/b/c
# abspath /tmp/
# # /tmp
# ```
#
# Thanks to [Stack
# Overflow](http://stackoverflow.com/questions/7126580/expand-a-possible-relative-path-in-bash#answer-13087801)
abspath() {
  if [[ -d "$1" ]]; then
    pushd "$1" > /dev/null
    pwd
    popd >/dev/null
  elif [[ -e $1 ]]; then
    pushd "$(dirname "$1")" > /dev/null
    echo "$(pwd)/$(basename "$1")"
    popd >/dev/null
  else
    echo "$1" does not exist! >&2
    return 127
  fi
}

# Returns all items joined by defined IFS
#
# ```sh
# join_by , a "b c" d
# # a,b c,d
# join_by / var local tmp
# # var/local/tmp
# join_by , "${FOO[@]}"
# # a,b,c
# ```
#
# Thanks to [Stack Overflow](http://stackoverflow.com/a/17841619/515789)
join_by() {
  local IFS="$1"
  shift
  echo "$*"
}

add_env() {
    exit_with "DEPRECATED: use 'set_runtime_env' instead!"
}

add_path_env() {
    exit_with "DEPRECATED: use 'push_runtime_env' instead!"
}

add_build_env() {
    exit_with "DEPRECATED: use 'set_buildtime_env' instead!"
}

add_build_path_env() {
    exit_with "DEPRECATED: use 'push_buildtime_env' instead!"
}

# Downloads a file from a source URL to a local file and uses an optional
# shasum to determine if an existing file can be used.
#
# If an existing file is present and the third argument is set with a shasum
# digest, the file will be checked to see if it's valid. If so, the function
# ends early and returns 0. Otherwise, the shasums do not match so the
# file-on-disk is removed and a normal download proceeds as though no previous
# file existed. This is designed to restart an interrupted download.
#
# Any valid `wget` URL will work.
#
# ```sh
# download_file http://example.com/file.tar.gz file.tar.gz
# # Downloads every time, even if the file exists locally
# download_file http://example.com/file.tar.gz file.tar.gz abc123...
# # Downloads if no local file is found
# download_file http://example.com/file.tar.gz file.tar.gz abc123...
# # File matches checksum: download is skipped, local file is used
# download_file http://example.com/file.tar.gz file.tar.gz oh noes...
# # File doesn't match checksum: local file removed, download attempted
# ```
#
# Will return 0 if a file was downloaded or if a valid cached file was found.
download_file() {
  local url="$1"
  local dst="$2"
  local sha="$3"

  pushd "$HAB_CACHE_SRC_PATH" > /dev/null
  if [[ -f $dst && -n "$sha" ]]; then
    build_line "Found previous file '$dst', attempting to re-use"
    if verify_file "$dst" "$sha"; then
      build_line "Using cached and verified '$dst'"
      return 0
    else
      build_line "Clearing previous '$dst' file and re-attempting download"
      rm -fv "$dst"
    fi
  fi

  build_line "Downloading '$url' to '$dst'"
  # shellcheck disable=2154
  $_wget_cmd "$url" -O "$dst"
  build_line "Downloaded '$dst'";
  popd > /dev/null
}

# Verifies that a file on disk matches the given shasum. If the given shasum
# doesn't match the file's shasum then a warning is printed with the expected
# and computed shasum values.
#
# ```sh
# verify_file file.tar.gz abc123...
# ```
#
# Will return 0 if the shasums match, and 1 if they do not match. A message
# will be printed to stderr with the expected and computed shasum values.
verify_file() {
  local filename=$1
  local expected_checksum=$2
  local computed_checksum

  build_line "Verifying $filename"
  expected_checksum=$(normalize_checksum "$expected_checksum")

  # shellcheck disable=2154
  read -r computed_checksum _ < <($_shasum_cmd "$HAB_CACHE_SRC_PATH"/"$filename")
  computed_checksum=$(normalize_checksum "$computed_checksum")

  if [[ "$expected_checksum" = "$computed_checksum" ]]; then
    build_line "Checksum verified for $1"
  else
    warn "Checksum invalid for $filename:"
    warn "   Expected: $expected_checksum"
    warn "   Computed: $computed_checksum"
    return 1
  fi
  return 0
}

# normalizes the given checksum for comparison with other checksums.
#
# ```sh
# normalize_checksum 81C8C4D253FFE5DA5A3C1AE96956403E07B5B1A5087276E48B1B2C3A30ACEE62
# ```
#
# Prints the normalized checksum to stdout, returns non-zero on error.
normalize_checksum() {
    local checksum=$1
    echo "$checksum" | tr '[:upper:]' '[:lower:]'
}

# Unpacks an archive file in a variety of formats.
#
# ```sh
# unpack_file file.tar.gz
# ```
#
# Will return 0 if the file archive is extracted, and 1 if the file archive
# type could not be found or was not supported (given the file extension). A
# message will be printed to stderr to provide context.
unpack_file() {
  build_line "Unpacking $1"
  local unpack_file="$HAB_CACHE_SRC_PATH/$1"
  # Thanks to:
  # http://stackoverflow.com/questions/17420994/bash-regex-match-string
  if [[ -f $unpack_file ]]; then
    pushd "$HAB_CACHE_SRC_PATH" > /dev/null
    case $unpack_file in
      *.tar.bz2|*.tbz2|*.tar.gz|*.tgz|*.tar|*.xz)
        # shellcheck disable=2154
        $_tar_cmd xf "$unpack_file" --no-same-owner
        ;;
      *.bz2)  bunzip2 "$unpack_file"    ;;
      *.rar)  rar x "$unpack_file"      ;;
      *.gz)   gunzip "$unpack_file"     ;;
      *.zip)  unzip -o "$unpack_file"   ;;
      *.Z)    uncompress "$unpack_file" ;;
      *.7z)   7z x "$unpack_file"       ;;
      *)
        warn "Error: unknown archive format '.${unpack_file##*.}'"
        return 1
        ;;
    esac
  else
    warn "'$1' is not a valid file!"
    return 1
  fi
  popd > /dev/null
  return 0
}

# Edit the `#!` shebang of the target file in-place. Useful for
# changing hardcoded `/usr/bin/env` to our coreutils, for example. Be
# sure to depend on the required package that provides the expected
# path for the shebang in `pkg_deps`. This does a greedy match against
# the specified interpreter in the target file(s).
#
# To use this function in your plan.sh, specify the following
# arguments:
#
# 1. The target file or files
# 2. The name of the package that contains the interpreter
# 3. The relative directory and binary path to the interpreter
#
# For example, to replace all the files in `node_modules/.bin` that
# have `#!/usr/bin/env` with the `coreutils` path
# to `bin/env` (which resolves to
# /hab/pkgs/acme/coreutils/8.24/20160219013458/bin/env), be sure
# to quote the wildcard target:
#
#     fix_interpreter "node_modules/.bin/*" acme/coreutils bin/env
#
# For a single target:
#
#     fix_interpreter node_modules/.bin/concurrent acme/coreutils bin/env
#
# To get the interpreters exposed by a package, look in that package's
# INTERPRETERS metadata file, e.g.,
# `/hab/pkgs/acme/coreutils/8.24/20160219013458/INTERPRETERS`

fix_interpreter() {
    local targets=$1
    local pkg=$2
    local int=$3
    local interpreter_old=".*${int}"
    local interpreter_new
    # shellcheck disable=2154
    if ! interpreter_new="$(pkg_interpreter_for "${pkg}" "${int}")" || [[ -z $interpreter_new ]]; then
      warn "fix_interpreter() '$pkg' is not a runtime dependency"
      warn "Only runtime packages may be used as your interpreter must travel"
      warn "with the '$pkg_name' in order to run."
      warn "Resolved runtime package set: ${pkg_deps_resolved[*]}"
      return 1
    fi

    for t in ${targets}; do
      if [[ ! -f $t ]]; then
        debug "Ignoring non-file target: ${t}"
        continue
      fi

      # Resolve symbolic links to fix the actual file instead of replacing it
      if [[ -L $t ]]; then
        t="$(readlink --canonicalize --no-newline "$t")"
      fi

      build_line "Replacing '${interpreter_old}' with '${interpreter_new}' in '${t}'"
      sed -e "s#\#\!${interpreter_old}#\#\!${interpreter_new}#" -i "$t"
    done
}

# Returns the path for the given package and interpreter by reading it
# from the INTERPRETERS metadata in the package. The directory of the
# interpreter needs to be specified, as an interpreter binary might
# live in `bin`, `sbin`, or `libexec`, depending on the software.
#
# ```
# pkg_interpreter_for acme/coreutils bin/env
# ```
#
# Will return 0 if the specified package and interpreter were found,
# and 1 if the package could not be found or the interpreter is not
# specified for that package.
pkg_interpreter_for() {
    local pkg=$1
    local int=$2
    local path
    if ! path=$(_pkg_path_for_deps "$pkg") || [[ -z $path ]]; then
      warn "Could not resolve the path for ${pkg}, is it specified in 'pkg_deps'?"
      return 1
    fi

   local int_path
   int_path=$(grep -x ".*${int}" "${path}"/INTERPRETERS)
    if [[ -n "$int_path" ]]; then
      echo "$int_path"
      return 0
    fi
    warn "Could not find interpreter ${int} in package ${pkg}"
    return 1
}

# Updates the value of `$pkg_version` and recomputes any relevant variables.
# This function must be called before the `do_prepare()` build phase otherwise
# it will fail the build process.
#
# This function depends on the Plan author implementing a `pkg_version()`
# function which prints a computed version string on standard output. Then,
# this function must be explicitly called in an appropriate build phase--most
# likely `do_before()`. For example:
#
# ```sh
# pkg_origin=acme
# pkg_name=myapp
#
# pkg_version() {
#   cat "$SRC_PATH/version.txt"
# }
#
# do_before() {
#   do_default_before
#   update_pkg_version
# }
# ```
update_pkg_version() {
  local update_src_path

  if [[ "${_verify_vars:-}" == true ]]; then
    local e
    e="Plan called 'update_pkg_version()' in phase 'do_prepare()' or later"
    e="$e which is not supported. Package version must be determined before"
    e="$e 'do_prepare()' phase."
    exit_with "$e" 21
  fi

  if [[ "$(type -t pkg_version)" == "function" ]]; then
    pkg_version="$(pkg_version)"
    build_line "Version updated to '$pkg_version'"
  else
    debug "pkg_version() function not found, retaining pkg_version=$pkg_version"
  fi

  # `$pkg_dirname` needs to be recomputed, unless it was explicitly set by the
  # Plan author.
  if [[ "${_pkg_dirname_initially_unset:-}" == true ]]; then
    pkg_dirname="${pkg_name}-${pkg_version}"
  fi
  # shellcheck disable=2034,2154
  pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
  # shellcheck disable=2034,2154
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
  # If the `$CACHE_PATH` and `$SRC_PATH` are the same, then we are building
  # third party software using `$pkg_source` and
  # downloading/verifying/unpacking it.
  if [[ "$CACHE_PATH" == "$SRC_PATH" ]]; then
    update_src_path=true
  fi
  CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  # Only update `$SRC_PATH` if we are building third party software using
  # `$pkg_source`.
  if [[ "${update_src_path:-}" == true ]]; then
    SRC_PATH="$CACHE_PATH"
  fi
  # Replace the unset placeholders with the computed value
  PATH=$(__resolve_version_placeholder "$PATH" "${pkg_version}")
  build_line "Updating PATH=$PATH"

  # TODO (CM): Do not like this separation of concerns (or lack of
  # separation, as the case may be).
  #
  # NOTE: we specifically handle PATH above (and make that live in the
  # environment). We are implicitly assuming that any other instances
  # of the version placeholder are not going to need to be propagated
  # back into the active environment.
  __resolve_all_version_placeholders "__runtime_environment" "${pkg_version}"
  __resolve_all_version_placeholders "__buildtime_environment" "${pkg_version}"
  __resolve_all_version_placeholders "__runtime_environment_provenance" "${pkg_version}"
  __resolve_all_version_placeholders "__buildtime_environment_provenance" "${pkg_version}"
}

# Replace all instances of the "__pkg__version__unset__" placeholder
# in the given string with the real version number.
__resolve_version_placeholder(){
    local original=${1}
    local real_version=${2}
    # shellcheck disable=2001
    echo "${original}" | sed "s,__pkg__version__unset__,${real_version},g"
}

# Replace all instances of the "__pkg__version__unset__" placeholder
# in the values of the given associative array with the real version number.
#
# NOTE: the associative array is specified *by name*.
__resolve_all_version_placeholders() {
    local datastructure_name=${1}
    local real_version=${2}

    declare -n map="${datastructure_name}"

    for k in "${!map[@]}"; do
        v=$(__resolve_version_placeholder "${map[${k}]}" "${real_version}")
        map["${k}"]="${v}"
    done
}
