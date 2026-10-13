install_gh_cli() {
    local runtime_dir="${1:?runtime directory is required}"
    local version="2.72.0"
    local checksum archive archive_root
    local command

    case "$(uname -s)/$(uname -m)" in
        Linux/x86_64 | Linux/amd64) ;;
        *)
            echo "Cannot install GitHub CLI on $(uname -s)/$(uname -m)" >&2
            return 1
            ;;
    esac

    checksum="ffd3a9791075cf6119531302b06554e658f38ed12675a7fbfbf4d6b114c77f38"
    for command in curl sha256sum tar; do
        if ! command -v "${command}" >/dev/null 2>&1; then
            echo "Required command '${command}' is not installed on this Buildkite agent" >&2
            return 127
        fi
    done

    archive="gh_${version}_linux_amd64.tar.gz"
    archive_root="gh_${version}_linux_amd64"
    curl --fail --location --silent --show-error --retry 3 \
        "https://github.com/cli/cli/releases/download/v${version}/${archive}" \
        --output "${runtime_dir}/${archive}"
    if ! printf '%s  %s\n' "${checksum}" "${runtime_dir}/${archive}" \
        | sha256sum --check --status; then
        echo "SHA-256 verification failed for GitHub CLI ${version}" >&2
        return 1
    fi
    tar -xzf "${runtime_dir}/${archive}" -C "${runtime_dir}" "${archive_root}/bin/gh"
    export PATH="${runtime_dir}/${archive_root}/bin:${PATH}"
}
