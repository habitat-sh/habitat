#!/bin/bash
#
set -eou pipefail

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

readonly pcio_root="https://packages.chef.io/files"
export HAB_LICENSE="accept-no-persist"

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
  download_archive "$version" "stable" "$target"
  verify_archive
  extract_archive
  install_hab "$channel"
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
      arch=${arch/arm64/aarch64}
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
  need_cmd mv

  local _version="${1:-latest}"
  local -r _channel="${2:?}"
  local -r _target="${3:?}"
  local url

  if [ "$_version" == "latest" ]; then
    url="${pcio_root}/${_channel}/habitat/latest/hab-${_target}.${ext}"
  else
    local -r _release="$(echo "${_version}" |cut -d'/' -f2)"
    if [ "${_release:+release}" == "release" ]; then
      _version="$(echo "${_version}" |cut -d'/' -f1)"
      info "packages.chef.io does not support 'version/release' format. Using $_version for the version"
    fi
    url="${pcio_root}/habitat/${_version}/hab-${_target}.${ext}"
  fi

  dl_file "${url}" "${workdir}/hab-${_version}.${ext}"
  dl_file "${url}.sha256sum" "${workdir}/hab-${_version}.${ext}.sha256sum"

  archive="hab-${_target}.${ext}"
  sha_file="hab-${_target}.${ext}.sha256sum"

  mv -v "${workdir}/hab-${_version}.${ext}" "${archive}"
  mv -v "${workdir}/hab-${_version}.${ext}.sha256sum" "${sha_file}"

  if command -v gpg >/dev/null; then
    info "GnuPG tooling found, downloading signatures"
    sha_sig_file="${archive}.sha256sum.asc"
    key_file="${workdir}/chef.asc"
    local _key_url="https://packages.chef.io/chef.asc"

    dl_file "${url}.sha256sum.asc" "${sha_sig_file}"
    dl_file "${_key_url}" "${key_file}"
  fi
}

verify_archive() {
  if command -v gpg >/dev/null; then
    info "GnuPG tooling found, verifying the shasum digest is properly signed"

    gpg --no-permission-warning --dearmor "${key_file}"
    gpg --no-permission-warning \
      --keyring "${key_file}.gpg" --verify "${sha_sig_file}"
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

      archive_dir="${archive%.tar.gz}"
      mkdir "${archive_dir}"
      zcat "${archive}" | tar --extract --directory "${archive_dir}" --strip-components=1

      ;;
    zip)
      need_cmd unzip

      archive_dir="${archive%.zip}"
      # -j "junk paths" Strips leading paths from files,
      unzip -j "${archive}" -d "${archive_dir}"
      ;;
    *)
      exit_with "Unrecognized file extension when extracting: ${ext}" 4
      ;;
  esac
}

install_hab() {
  local _channel=${1:-${channel}}
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

      if [ -n "${version-}" ] && [ "${version}" != "latest" ]; then
        _ident+="/$version";
      fi

      info "Installing Habitat package using temporarily downloaded hab"
      # NOTE: For people (rightly) wondering why we download hab only to use it
      # to install hab from Builder, the main reason is because it allows /bin/hab
      # to be a binlink, meaning that future upgrades can be easily done via
      # hab pkg install core/hab -bf and everything will Just Work. If we put
      # the hab we downloaded into /bin, then future hab upgrades done via hab
      # itself won't work - you'd need to run this script every time you wanted
      # to upgrade hab, which is not intuitive. Putting it into a place other than
      # /bin means now you have multiple copies of hab on your system and pathing
      # shenanigans might ensue. Rather than deal with that mess, we do it this
      # way.
      "${archive_dir}/hab" pkg install --binlink --force --channel "$_channel" "$_ident"
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
