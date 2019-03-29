#!/bin/bash

# Fails on unset variables & whenever a command returns a non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

BT_ROOT="https://api.bintray.com/content/habitat"
BT_SEARCH="https://api.bintray.com/packages/habitat"

main() {
  # Use stable Bintray channel by default
  channel="stable"
  # Set an empty version variable, signaling we want the latest release
  version=""

  # Parse command line flags and options.
  while getopts "c:hv:t:" opt; do
    case "${opt}" in
      c)
        channel="${OPTARG}"
        ;;
      h)
        print_help
        exit 0
        ;;
      v)
        version="${OPTARG}"
        ;;
      t)
        target="${OPTARG}"
        ;;
      \?)
        echo "" >&2
        print_help >&2
        exit_with "Invalid option" 1
        ;;
    esac
  done

  info "Installing Habitat 'hab' program"
  create_workdir
  get_platform
  validate_target
  get_version
  download_archive
  verify_archive
  extract_archive
  install_hab
  print_hab_version
  info "Installation of Habitat 'hab' program complete."
}

print_help() {
  need_cmd cat
  need_cmd basename

  local _cmd
  _cmd="$(basename "${0}")"
  cat <<USAGE
${_cmd}

Authors: The Habitat Maintainers <humans@habitat.sh>

Installs the Habitat 'hab' program.

USAGE:
    ${_cmd} [FLAGS]

FLAGS:
    -c    Specifies a channel [values: stable, unstable] [default: stable]
    -h    Prints help information
    -v    Specifies a version (ex: 0.15.0, 0.15.0/20161222215311)
    -t    Specifies the ActiveTarget of the 'hab' program to download.
            [values: x86_64-linux, x86_64-linux-kernel2] [default: x86_64-linux]
            This option is only valid on Linux platforms

ENVIRONMENT VARIABLES:
     SSL_CERT_FILE   allows you to verify against a custom cert such as one
                     generated from a corporate firewall

USAGE
}

create_workdir() {
  need_cmd mktemp
  need_cmd rm
  need_cmd mkdir

  if [ -n "${TMPDIR:-}" ]; then
    local _tmp="${TMPDIR}"
  elif [ -d /var/tmp ]; then
    local _tmp=/var/tmp
  else
    local _tmp=/tmp
  fi
  workdir="$(mktemp -d -p "$_tmp" 2> /dev/null || mktemp -d "${_tmp}/hab.XXXX")"
  # Add a trap to clean up any interrupted file downloads
  # shellcheck disable=SC2154
  trap 'code=$?; rm -rf $workdir; exit $code' INT TERM EXIT
  cd "${workdir}"
}

get_platform() {
  need_cmd uname
  need_cmd tr

  local _ostype
  _ostype="$(uname -s)"

  case "${_ostype}" in
    Darwin|Linux)
      sys="$(uname -s | tr '[:upper:]' '[:lower:]')"
      arch="$(uname -m | tr '[:upper:]' '[:lower:]')"
      ;;
    *)
      exit_with "Unrecognized OS type when determining platform: ${_ostype}" 2
      ;;
  esac

  case "${sys}" in
    darwin)
      need_cmd shasum

      ext=zip
      shasum_cmd="shasum -a 256"
      ;;
    linux)
      need_cmd sha256sum

      ext=tar.gz
      shasum_cmd="sha256sum"
      ;;
    *)
      exit_with "Unrecognized sys type when determining platform: ${sys}" 3
      ;;
  esac

  if [ -z "${target:-}" ]; then 
    target="${arch}-${sys}"
  fi
}

get_version() {
  need_cmd grep
  need_cmd head
  need_cmd sed
  need_cmd tr

  local _btv
  local _j="${workdir}/version.json"

  _btv="$(echo "${version:-%24latest}" | tr '/' '-')"

  if [ -z "${_btv##*%24latest*}" ]; then
    btv=$_btv
  else
    info "Determining fully qualified version of package for \`$version'"
    dl_file "${BT_SEARCH}/${channel}/hab-${target}" "${_j}"
    # This is nasty and we know it. Clap your hands. If the install.sh stops
    # work its likely related to this here sed command. We have to pull
    # versions out of minified json. So if this ever stops working its likely
    # BT api output is no longer minified.
    _rev="$(sed -e 's/^.*"versions":\[\([^]]*\)\].*$/\1/' -e 's/"//g' "${_j}" \
      | tr ',' '\n' \
      | grep "^${_btv}" \
      | head -1)"
    if [ -z "${_rev}" ]; then
      _e="Version \`${version}' could not used or version doesn't exist."
      _e="$_e Please provide a simple version like: \"0.15.0\""
      _e="$_e or a fully qualified version like: \"0.15.0/20161222203215\"."
      exit_with "$_e" 6
    else
      btv=$_rev
      info "Using fully qualified Bintray version string of: $btv"
    fi
  fi
}

# Validate the CLI Target requested.  In most cases ${arch}-${sys}
# for the current system is the only valid Target.  In the case of 
# x86_64-linux systems we also need to support the x86_64-linux-kernel2
# Target. Creates an array of valid Targets for the current system,
# adding any valid alternate Targets, and checks if the requested 
# Target is present in the array.
validate_target() {
  local valid_targets=("${arch}-${sys}")
  case "${sys}" in
   linux)
    valid_targets+=("x86_64-linux-kernel2")
    ;;
  esac

  if ! (_array_contains "${target}" "${valid_targets[@]}") ; then
    local _vts
    printf -v _vts "%s, " "${valid_targets[@]}"
    _e="${target} is not a valid target for this system. Please specify one of: [${_vts%, }]"
    exit_with "$_e" 7
  fi
}

download_archive() {
  need_cmd cut
  need_cmd mv

  url="${BT_ROOT}/${channel}/${sys}/${arch}/hab-${btv}-${target}.${ext}"
  query="?bt_package=hab-${target}"

  local _hab_url="${url}${query}"
  local _sha_url="${url}.sha256sum${query}"

  dl_file "${_hab_url}" "${workdir}/hab-latest.${ext}"
  dl_file "${_sha_url}" "${workdir}/hab-latest.${ext}.sha256sum"

  archive="${workdir}/$(cut -d ' ' -f 3 hab-latest.${ext}.sha256sum)"
  sha_file="${archive}.sha256sum"

  info "Renaming downloaded archive files"
  mv -v "${workdir}/hab-latest.${ext}" "${archive}"
  mv -v "${workdir}/hab-latest.${ext}.sha256sum" "${archive}.sha256sum"
}

verify_archive() {
  if command -v gpg >/dev/null; then
    info "GnuPG tooling found, verifying the shasum digest is properly signed"
    local _sha_sig_url="${url}.sha256sum.asc${query}"
    local _sha_sig_file="${archive}.sha256sum.asc"
    local _key_url="https://bintray.com/user/downloadSubjectPublicKey?username=habitat"
    local _key_file="${workdir}/habitat.asc"

    dl_file "${_sha_sig_url}" "${_sha_sig_file}"
    dl_file "${_key_url}" "${_key_file}"

    gpg --no-permission-warning --dearmor "${_key_file}"
    gpg --no-permission-warning \
      --keyring "${_key_file}.gpg" --verify "${_sha_sig_file}"
  fi

  info "Verifying the shasum digest matches the downloaded archive"
  ${shasum_cmd} -c "${sha_file}"
}

extract_archive() {
  need_cmd sed

  info "Extracting ${archive}"
  case "${ext}" in
    tar.gz)
      need_cmd zcat
      need_cmd tar

      zcat "${archive}" | tar x -C "${workdir}"
      archive_dir="${archive%.tar.gz}"
      ;;
    zip)
      need_cmd unzip

      unzip "${archive}" -d "${workdir}"
      archive_dir="${archive%.zip}"
      ;;
    *)
      exit_with "Unrecognized file extension when extracting: ${ext}" 4
      ;;
  esac
}

install_hab() {
  case "${sys}" in
    darwin)
      need_cmd mkdir
      need_cmd install

      info "Installing hab into /usr/local/bin"
      mkdir -pv /usr/local/bin
      install -v "${archive_dir}"/hab /usr/local/bin/hab
      ;;
    linux)
      local _ident="core/hab"
      if [ -n "${version-}" ]; then
        _ident+="/$version";
      fi
      info "Installing Habitat package using temporarily downloaded hab"
      # Install hab release using the extracted version and add/update symlink
      "${archive_dir}/hab" install --channel "$channel" "$_ident"
      # TODO fn: The updated binlink behavior is to skip targets that already
      # exist so we want to use the `--force` flag. Unfortunetly, old versions
      # of `hab` don't have this flag. For now, we'll run with the new flag and
      # fall back to running the older behavior. This can be removed at a
      # future date when we no lnger are worrying about Habitat versions 0.33.2
      # and older. (2017-09-29)
      "${archive_dir}/hab" pkg binlink "$_ident" hab --force \
        || "${archive_dir}/hab" pkg binlink "$_ident" hab
      ;;
    *)
      exit_with "Unrecognized sys when installing: ${sys}" 5
      ;;
  esac
}

print_hab_version() {
  need_cmd hab

  info "Checking installed hab version"
  hab --version
}

need_cmd() {
  if ! command -v "$1" > /dev/null 2>&1; then
    exit_with "Required command '$1' not found on PATH" 127
  fi
}

info() {
  echo "--> hab-install: $1"
}

warn() {
  echo "xxx hab-install: $1" >&2
}

exit_with() {
  warn "$1"
  exit "${2:-10}"
}

_array_contains() {
  local e
  for e in "${@:2}"; do
    if [[ "$e" == "$1" ]]; then
      return 0
    fi
  done
  return 1
}

dl_file() {
  local _url="${1}"
  local _dst="${2}"
  local _code
  local _wget_extra_args=""
  local _curl_extra_args=""

  # Attempt to download with wget, if found. If successful, quick return
  if command -v wget > /dev/null; then
    info "Downloading via wget: ${_url}"
    if [ -n "${SSL_CERT_FILE:-}" ]; then
      wget ${_wget_extra_args:+"--ca-certificate=${SSL_CERT_FILE}"} -q -O "${_dst}" "${_url}"
    else
      wget -q -O "${_dst}" "${_url}"
    fi
    _code="$?"
    if [ $_code -eq 0 ]; then
      return 0
    else
      local _e="wget failed to download file, perhaps wget doesn't have"
      _e="$_e SSL support and/or no CA certificates are present?"
      warn "$_e"
    fi
  fi

  # Attempt to download with curl, if found. If successful, quick return
  if command -v curl > /dev/null; then
    info "Downloading via curl: ${_url}"
    if [ -n "${SSL_CERT_FILE:-}" ]; then
      curl ${_curl_extra_args:+"--cacert ${SSL_CERT_FILE}"} -sSfL "${_url}" -o "${_dst}"
    else
      curl -sSfL "${_url}" -o "${_dst}"
    fi
    _code="$?"
    if [ $_code -eq 0 ]; then
      return 0
    else
      local _e="curl failed to download file, perhaps curl doesn't have"
      _e="$_e SSL support and/or no CA certificates are present?"
      warn "$_e"
    fi
  fi

  # If we reach this point, wget and curl have failed and we're out of options
  exit_with "Required: SSL-enabled 'curl' or 'wget' on PATH with" 6
}

main "$@" || exit 99
