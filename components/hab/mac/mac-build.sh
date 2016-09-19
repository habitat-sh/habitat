#!/bin/bash

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;33m$(basename $0): \033[1;37m$1\033[0m\n"
      ;;
    *)
      printf -- "   $(basename $0): $1\n"
      ;;
  esac
  return 0
}

install_if_missing() {
  local pkg="$1"
  if [[ -n "${2:-}" ]]; then
    local formula="$2"
  else
    local formula="$pkg"
  fi

  if [[ $(brew list --versions $pkg | wc -l) -eq 0 ]]; then
    info "Installing missing Homebrew package $formula"
    sudo -u $SUDO_USER brew install $formula
  fi
}

if (( $EUID != 0 )); then
  info "Please run as root (with \`sudo $0 $*\`)"
  exit 1
fi

if [[ ! -f /bin/hab ]]; then
  info "Habitat CLI missing, attempting to install latest release"
  sh $(dirname $0)/install.sh
fi

while true; do
  if [[ $(ls -1 /hab/cache/keys/core-*.sig.key 2> /dev/null | wc -l) -gt 0 ]]; then
    break
  else
    info "System is missing core origin signing key"
    info "You will be prompted to paste in the secret key now."
    info "After pasting, type Ctrl-D to send EOF"
    info ""
    info "Paste your key now:"
    info ""
    hab origin key import
  fi
done

if ! pkgutil --pkgs=com.apple.pkg.CLTools_Executables >/dev/null; then
  info "Xcode CLI tools missing, attempting to install"
  # Implementation graciously borrowed and modified from the build-essential
  # Chef cookbook which has been graciously borrowed and modified from Tim
  # Sutton's osx-vm-templates project.
  #
  # Source: https://github.com/chef-cookbooks/build-essential/blob/a4f9621020e930a0e4fa0ccb5b7957dbef8ab347/libraries/xcode_command_line_tools.rb#L182-L188
  # Source: https://github.com/timsutton/osx-vm-templates/blob/b001475df54a9808d3d56d06e71b8fa3001fff42/scripts/xcode-cli-tools.sh
  touch /tmp/.com.apple.dt.CommandLineTools.installondemand.in-progress
  PROD=$(softwareupdate -l | grep "\*.*Command Line" | head -n 1 | awk -F"*" '{print $2}' | sed -e 's/^ *//' | tr -d '\n')
  softwareupdate -i "$PROD" -v
  rm -f /tmp/.com.apple.dt.CommandLineTools.installondemand.in-progress
fi

if ! command -v brew >/dev/null; then
  info "Homebrew missing, attempting to install"
  sudo -u $SUDO_USER /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" < /dev/null
fi

# Homebrew pacakges required to run `hab-plan-build.sh
install_if_missing coreutils
install_if_missing gnu-tar
install_if_missing wget

# Homebrew packages required to build `hab`
install_if_missing zlib homebrew/dupes/zlib
install_if_missing xz
install_if_missing bzip2 homebrew/dupes/bzip2
install_if_missing expat
install_if_missing openssl
install_if_missing libsodium
install_if_missing hab-libiconv $(dirname $0)/homebrew/hab-libiconv.rb
install_if_missing hab-libarchive $(dirname $0)/homebrew/hab-libarchive.rb

if ! command -v rustc >/dev/null; then
  info "Rust missing, attempting to install"
  curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y
fi

cargo_bin=/opt/cargo/bin/cargo
if [[ ! -f "$cargo_bin" ]] || [[ $($cargo_bin --version | cut -d ' ' -f 2) < "0.13.0-nightly" ]]; then
  info "Cargo nightly missing or old, attempting to install"
  cargo_workdir=/tmp/cargo-dl
  mkdir -p "$cargo_workdir"
  pushd /tmp/cargo-dl > /dev/null
  cargo_url="https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-apple-darwin.tar.gz"
  curl -LO "$cargo_url"
  tar xf "$(basename $cargo_url)"
  cd "$(basename ${cargo_url%.tar.gz})"
  ./install.sh --prefix=/opt/cargo
  popd > /dev/null
  rm -rf "$cargo_workdir"
fi

info "Updating PATH to include GNU toolchain from HomeBrew and custom Cargo"
gnu_path="$(brew --prefix coreutils)/libexec/gnubin"
gnu_path="$gnu_path:$(brew --prefix gnu-tar)/libexec/gnubin"
export PATH="$gnu_path:$(dirname $cargo_bin):$PATH"
info "Setting PATH=$PATH"

program="$(dirname $0)/../../plan-build/bin/hab-plan-build.sh"
info "Executing: $program $*"
echo
exec $program $*
