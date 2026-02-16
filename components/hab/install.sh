#!/bin/bash

set -eou pipefail

umask 0022

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

readonly pcio_root="https://packages.chef.io/files"
export HAB_LICENSE="accept-no-persist"

# This is the main function that sets up the Habitat environment on macOS.
# It creates, mounts, and configures a designated volume (Habitat Store) with the necessary settings,
# including file system options and encryption (if needed).
#
# High-level steps performed:
#
# 1. **Volume Creation**:
#    - Creates a new APFS volume for Habitat on the identified disk.
#    - Encrypts the volume (if needed) with a randomly generated password.
#
# 2. **Volume Configuration**:
#    - Verifies that the Habitat root (`/hab`) is properly configured.
#    - Updates `/etc/synthetic.conf` to ensure the mount point is set correctly.
#    - Configures `/etc/fstab` with the appropriate volume mount options.
#
# 3. **Volume Mounting and Daemon Configuration**:
#    - Generates and installs a LaunchDaemon plist to mount the volume at system boot.
#    - Ensures the volume is mounted automatically on startup.
setup_hab_root() {
    SCRATCH=$(mktemp -d)
    readonly SCRATCH

    finish_cleanup() {
        rm -rf "$SCRATCH"
    }

    readonly HAB_ROOT="/hab"
    readonly HAB_VOLUME_LABEL="Habitat Store"
    readonly HAB_SERVICE_TARGET="sh.habitat.bldr.darwin-store"
    readonly HAB_VOLUME_MOUNTD_DEST="/Library/LaunchDaemons/$HAB_SERVICE_TARGET.plist"

    root_disk() {
        /usr/sbin/diskutil info -plist / | xmllint --xpath "/plist/dict/key[text()='ParentWholeDisk']/following-sibling::string[1]/text()" -
    }
    HAB_VOLUME_USE_DISK="$(root_disk)"
    readonly HAB_VOLUME_USE_DISK

    if /usr/bin/fdesetup isactive >/dev/null; then
        test_filevault_in_use() { return 0; }
        HAB_VOLUME_DO_ENCRYPT=1
    else
        test_filevault_in_use() { return 1; }
        HAB_VOLUME_DO_ENCRYPT=0
    fi

    should_encrypt_volume() {
        test_filevault_in_use && (( HAB_VOLUME_DO_ENCRYPT == 1 ))
    }

    volume_encrypted() {
        local volume="$1" # (i.e., disk1s3)
        /usr/sbin/diskutil apfs listCryptoUsers -plist "$volume" | /usr/bin/grep -q APFSCryptoUserUUID
    }

    test_fstab() {
        /usr/bin/grep -q "$HAB_ROOT apfs rw" /etc/fstab 2>/dev/null
    }

    test_synthetic_conf_mountable() {
        /usr/bin/grep -q "^${HAB_ROOT:1}$" /etc/synthetic.conf 2>/dev/null
    }

    create_synthetic_objects() {
        {
            /System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util -t || true # Big Sur and above
        } >/dev/null 2>&1
    }

    test_hab() {
        test -d "$HAB_ROOT"
    }

    test_volume_daemon() {
        test -f "$HAB_VOLUME_MOUNTD_DEST"
    }

    generate_mount_command() {
        local cmd_type="$1" # encrypted|unencrypted
        local volume_uuid mountpoint cmd=()
        printf -v volume_uuid "%q" "$2"
        printf -v mountpoint "%q" "$HAB_ROOT"

        case "$cmd_type" in
            encrypted)
                cmd=(/bin/sh -c "/usr/bin/security find-generic-password -s '$volume_uuid' -w | /usr/sbin/diskutil apfs unlockVolume '$volume_uuid' -mountpoint '$mountpoint' -stdinpassphrase")
                ;;
            unencrypted)
                cmd=(/usr/sbin/diskutil mount -mountPoint "$mountpoint" "$volume_uuid")
                ;;
            *)
                exit_with "Invalid first arg $cmd_type to generate_mount_command"
                ;;
        esac

        printf "    <string>%s</string>\n" "${cmd[@]}"
    }

    generate_mount_daemon() {
        local cmd_type="$1" # encrypted|unencrypted
        local volume_uuid="$2"

        cat <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>RunAtLoad</key>
  <true/>
  <key>Label</key>
   <string>${HAB_SERVICE_TARGET}</string>
  <key>ProgramArguments</key>
  <array>
$(generate_mount_command "$cmd_type" "$volume_uuid")
  </array>
</dict>
</plist>
EOF
    }

    add_hab_vol_fstab_line() {
        local uuid="$1"
        local mountpoint="${HAB_ROOT}"
        shift

        cat > "$SCRATCH/ex_cleanroom_wrapper" <<EOF
#!/bin/sh
/usr/bin/ex -u NONE -n "\$@"
EOF
        chmod 755 "$SCRATCH/ex_cleanroom_wrapper"

        EDITOR="$SCRATCH/ex_cleanroom_wrapper" "$@" <<EOF
:a
UUID=$uuid $mountpoint apfs rw,noauto,nobrowse,suid,owners
.
:x
EOF
    }

    setup_synthetic_conf() {
        if ! test_synthetic_conf_mountable; then
            echo "Configuring /etc/synthetic.conf to make a mount-point at $HAB_ROOT" >&2
            /usr/bin/ex -u NONE -n /etc/synthetic.conf <<EOF
:a
${HAB_ROOT:1}
.
:x
EOF
            if ! test_synthetic_conf_mountable; then
                exit_with "error: failed to configure synthetic.conf"
            fi
            create_synthetic_objects
            if ! test_hab; then
                exit_with "error: failed to bootstrap $HAB_ROOT"
            fi
        fi
    }

    setup_fstab() {
        local volume_uuid="$1"
        if ! test_fstab; then
            echo "Configuring /etc/fstab to specify volume mount options" >&2
            add_hab_vol_fstab_line "$volume_uuid" /usr/sbin/vifs
        fi
    }

    encrypt_volume() {
        local volume_uuid="$1"
        local volume_label="$2"
        local password

        echo "Encrypt the Habitat volume" >&2

        /usr/sbin/diskutil mount "$volume_label"

        password="$(/usr/bin/xxd -l 32 -p -c 256 /dev/random)"
        /usr/bin/security -i <<EOF
add-generic-password -a "$volume_label" -s "$volume_uuid" -l "$volume_label encryption password" -D "Encrypted volume password" -j "Added automatically by the Habitat installer for use by $HAB_VOLUME_MOUNTD_DEST" -w "$password" -T /System/Library/CoreServices/APFSUserAgent -T /System/Library/CoreServices/CSUserAgent -T /usr/bin/security "/Library/Keychains/System.keychain"
EOF
        builtin printf "%s" "$password" | /usr/sbin/diskutil apfs encryptVolume "$volume_label" -user disk -stdinpassphrase

        /usr/sbin/diskutil unmount force "$volume_label"
    }

    create_volume() {
        /usr/sbin/diskutil apfs addVolume "$HAB_VOLUME_USE_DISK" "APFS" "$HAB_VOLUME_LABEL" -nomount | /usr/bin/awk '/Created new APFS Volume/ {print $5}'
    }

    volume_uuid_from_special() {
        local volume_special="$1" # (i.e., disk1s3)
        /System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util -k "$volume_special" || true
    }

    await_volume() {
        local timeout=30 # sufficiently long enough to mount the volume
        local remaining_time=$timeout
        while (( remaining_time > 0 )); do
            /usr/sbin/diskutil info "$HAB_ROOT" &>/dev/null && return 0  # If the volume is found, return successfully
            ((remaining_time--))
            sleep 1
        done
        exit_with "Error: Volume did not appear within $timeout seconds."
    }

    setup_volume() {
        local use_special use_uuid profile_packages
        echo "Creating a Habitat volume" >&2

        use_special="$(create_volume)"

        /usr/sbin/diskutil unmount force "$use_special" || true # might not be mounted

        use_uuid="$(volume_uuid_from_special "$use_special")"
        readonly use_uuid

        setup_fstab "$use_uuid"

        if should_encrypt_volume; then
            encrypt_volume "$use_uuid" "$HAB_VOLUME_LABEL"
            setup_volume_daemon "encrypted" "$use_uuid"
        elif volume_encrypted "$use_special"; then
            setup_volume_daemon "encrypted" "$use_uuid"
        else
            setup_volume_daemon "unencrypted" "$use_uuid"
        fi

        await_volume

        if [ "$(/usr/sbin/diskutil info -plist "$HAB_ROOT" | xmllint --xpath "(/plist/dict/key[text()='GlobalPermissionsEnabled'])/following-sibling::*[1]" -)" = "<false/>" ]; then
            /usr/sbin/diskutil enableOwnership "$HAB_ROOT"
        fi
    }

    setup_volume_daemon() {
        local cmd_type="$1" # encrypted|unencrypted
        local volume_uuid="$2"
        if ! test_volume_daemon; then
            echo "Configuring LaunchDaemon to mount '$HAB_VOLUME_LABEL'" >&2
            /usr/bin/ex -u NONE -n "$HAB_VOLUME_MOUNTD_DEST" <<EOF
:a
$(generate_mount_daemon "$cmd_type" "$volume_uuid")
.
:x
EOF
            launchctl bootstrap system "$HAB_VOLUME_MOUNTD_DEST" || true
            launchctl kickstart -k system/"$HAB_SERVICE_TARGET"
        fi
    }

    setup_hab_volume() {
        if ! test_hab; then
            setup_synthetic_conf
            setup_volume
        fi
    }

    setup_hab_volume
}

main() {
  # Use stable Bintray channel by default
  channel="stable"
  # Set an empty version variable, signaling we want the latest release
  version=""

  # Parse command line flags and options.
  while getopts "c:hv:t:u:b:" opt; do
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
    u)
      bldrUrl="${OPTARG}"
      ;;
    b)
      # We use this CLI switch to download the *bootstrap* packages
      # from this channel on the builder. This applies only for *darwin*
      # TODO: Once we start publishing packages on chef.io, this should go
      # away
      bldrChannel="${OPTARG}" # for temporary use
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
  download_archive "$version" "$channel" "$target"
  verify_archive
  extract_archive
  origin="$(get_origin_from_manifest)"
  info "Discovered origin from manifest: $origin"
  install_hab "$origin"
  print_hab_version
  info "Installation of Habitat 'hab' program complete."
}

print_help() {
  need_cmd cat
  need_cmd basename

  local _cmd
  _cmd="$(basename "${0}")"
  cat <<-HEREDOC
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
		            [values: x86_64-linux, aarch64-linux] [default: x86_64-linux]
		            This option is only valid on Linux platforms

		ENVIRONMENT VARIABLES:
		     SSL_CERT_FILE   allows you to verify against a custom cert such as one
		                     generated from a corporate firewall

	HEREDOC
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

  workdir="$(mktemp -d -p "$_tmp" 2>/dev/null || mktemp -d "${_tmp}/hab.XXXX")"
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
  Darwin | Linux)
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
# for the current system is the only valid Target. Creates an
# array of valid Targets for the current system,
# adding any valid alternate Targets, and checks if the requested
# Target is present in the array.
validate_target() {
  local valid_targets=("${arch}-${sys}")
  case "${sys}" in
  linux)
    valid_targets+=("x86_64-linux-kernel2")
    ;;
  esac

  if ! (_array_contains "${target}" "${valid_targets[@]}"); then
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
    url="${pcio_root}/${_channel}/habitat/latest"
  else
    local -r _release="$(echo "${_version}" | cut -d'/' -f2)"
    if [ "${_release:+release}" == "release" ]; then
      _version="$(echo "${_version}" | cut -d'/' -f1)"
      info "packages.chef.io does not support 'version/release' format. Using $_version for the version"
    fi
    url="${pcio_root}/habitat/${_version}"
  fi

  dl_file "${url}/hab-${_target}.${ext}" "${workdir}/hab-${_version}.${ext}"
  dl_file "${url}/hab-${_target}.${ext}.sha256sum" "${workdir}/hab-${_version}.${ext}.sha256sum"

  # Download manifest.json to extract origin information
  manifest_file="manifest.json"
  dl_file "${url}/manifest.json" "${workdir}/manifest.json" || {
    warn "Failed to download manifest.json, will fallback to default origin"
    touch "${workdir}/manifest.json"  # Create empty file for fallback
  }

  archive="hab-${_target}.${ext}"
  sha_file="hab-${_target}.${ext}.sha256sum"

  mv -v "${workdir}/hab-${_version}.${ext}" "${archive}"
  mv -v "${workdir}/hab-${_version}.${ext}.sha256sum" "${sha_file}"

  if command -v gpg >/dev/null; then
    info "GnuPG tooling found, downloading signatures"
    sha_sig_file="${archive}.sha256sum.asc"
    key_file="${workdir}/chef.asc"
    local _key_url="https://packages.chef.io/chef.asc"

    dl_file "${url}/hab-${_target}.${ext}.sha256sum.asc" "${sha_sig_file}"
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
  local _origin="${1:-chef}"

  case "${sys}" in
  darwin)
    case "${arch}" in
    x86_64)
      # No core packages are available yet for x86_64; proceed with the old approach.
      need_cmd mkdir
      need_cmd install

      info "Installing hab into /usr/local/bin"
      mkdir -pv /usr/local/bin
      install -v "${archive_dir}"/hab /usr/local/bin/hab

      # Copy NOTICES.txt if it exists in the archive
      if [ -f "${archive_dir}/NOTICES.txt" ]; then
        info "Installing NOTICES.txt into /usr/local/share/habitat"
        mkdir -pv /usr/local/share/habitat
        install -v "${archive_dir}/NOTICES.txt" /usr/local/share/habitat/NOTICES.txt
      fi
      ;;
    aarch64)
      info "Determined aarch64-darwin."
      setup_hab_root

      local _ident="${_origin}/hab"

      if [ -n "${version-}" ] && [ "${version}" != "latest" ]; then
          _ident+="/$version"
      fi

      # The Habitat packages for macOS (aarch64) are not currently available in the SaaS Builder.
      # This is a temporary fix until they become available.
      _channel="${bldrChannel:-$channel}"
      "${archive_dir}/hab" --version
      info "Installing from channel '${_channel}'." 
      "${archive_dir}/hab" pkg install --binlink --force --channel "$_channel" "$_ident" ${bldrUrl:+-u "$bldrUrl"}
      ;;
    *)
      exit_with "Unrecognized sys when installing: ${sys}" 5
      ;;
    esac
    ;;
  linux)
    local _ident="${_origin}/hab"

    if [ -n "${version-}" ] && [ "${version}" != "latest" ]; then
      _ident+="/$version"
    fi

    info "Installing Habitat package using temporarily downloaded hab"
    # NOTE: For people (rightly) wondering why we download hab only to use it
    # to install hab from Builder, the main reason is because it allows /bin/hab
    # to be a binlink, meaning that future upgrades can be easily done via
    # hab pkg install chef/hab -bf and everything will Just Work. If we put
    # the hab we downloaded into /bin, then future hab upgrades done via hab
    # itself won't work - you'd need to run this script every time you wanted
    # to upgrade hab, which is not intuitive. Putting it into a place other than
    # /bin means now you have multiple copies of hab on your system and pathing
    # shenanigans might ensue. Rather than deal with that mess, we do it this
    # way.
    "${archive_dir}/hab" pkg install --binlink --force --channel "$channel" "$_ident" ${bldrUrl:+-u "$bldrUrl"}
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
  if ! command -v "$1" >/dev/null 2>&1; then
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
  if command -v wget >/dev/null; then
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
  if command -v curl >/dev/null; then
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

# Extract origin from manifest.json file
get_origin_from_manifest() {
  local origin="chef"  # Default fallback

  # Use basic text processing to extract origin from package identifiers
  # Look for package identifiers and extract the origin (first part before /)
  # Package identifiers are in format: origin/name/version/release
  origin=$(grep -o '"[^"]*\/[^"]*\/[^"]*\/[^"]*"' "$manifest_file" 2>/dev/null | \
            head -1 | \
            sed 's/^"\([^/]*\)\/.*$/\1/' 2>/dev/null)

  # Validate that we got a non-empty origin
  if [ -z "$origin" ] || [ "$origin" = "null" ]; then
    origin="chef"
  fi

  echo "$origin"
}

main "$@" || exit 99
