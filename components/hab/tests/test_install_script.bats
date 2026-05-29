setup() {
  if [ -n "$CI" ]; then
    # This is where our curlbash installer puts the link
    rm -f /bin/hab
    # This is where our CI systems link Chef Workstation's `hab`
    # binary. It comes earlier in the path than `/bin`, so we need to
    # remove it. We don't use Workstation in our tests, so this is
    # fine.
    rm -f /usr/bin/hab
    rm -rf /hab/pkgs/core/hab
    # On macOS, hab pkg install puts packages under /opt/hab.
    # Remove it so aarch64 routing tests start from a clean state.
    rm -rf /opt/hab 2>/dev/null || true
    # Remove any stale symlink or binary left at the macOS install location.
    rm -f /usr/local/bin/hab 2>/dev/null || true
    # Remove any stale share directory
    rm -rf /usr/local/share/habitat 2>/dev/null || true
  else
    echo "Not running in CI, skipping cleanup"
  fi
}

darwin() {
  [ "$(uname)" == "Darwin" ]
}

darwin_aarch64() {
  [ "$(uname)" == "Darwin" ] && [ "$(uname -m)" == "arm64" ]
}

darwin_x86_64() {
  [ "$(uname)" == "Darwin" ] && [ "$(uname -m)" == "x86_64" ]
}

linux() {
  [ "$(uname)" == "Linux" ]
}

installed_version() {
  hab --version | cut -d'/' -f1
}

installed_target() {
  local origin="${1:-chef}"

  version_release="$(hab --version | cut -d' ' -f2)"
  version="$(cut -d'/' -f1 <<< "$version_release")"
  release="$(cut -d'/' -f2 <<< "$version_release")"
  cat /hab/pkgs/"${origin}"/hab/"$version"/"$release"/TARGET
}

@test "Install latest for x86_64-linux" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh

  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install specific version for x86_64-linux" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh -v 0.90.6

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.90.6" ]
  [ "$(installed_target core)" == "x86_64-linux" ]
}

@test "Install legacy package for x86_64-linux" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh -v 0.79.1

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.79.1" ]
  [ "$(installed_target core)" == "x86_64-linux" ]
}

@test "Install package for x86_64-linux from acceptance" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh -c acceptance

  [ "$status" -eq 0 ]
  [[ "$(installed_version)" =~ ^hab\ 2\.[0-9]+\.[0-9]+$ ]]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install latest for darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/hab/install.sh

  [ "$status" -eq 0 ]
}

@test "Install package for x86_64-darwin from acceptance" {
  darwin_x86_64 || skip "Did not detect a Darwin system"
  run components/hab/install.sh -c acceptance

  [ "$status" -eq 0 ]
  [ -f "/usr/local/share/habitat/NOTICES.txt" ]
}

@test "Install specific version for x86_64-darwin" {
  darwin_x86_64 || skip "Did not detect an x86_64 Darwin system"
  run components/hab/install.sh -v 0.90.6

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.90.6" ]
}

@test "Install legacy package for x86_84-darwin" {
  darwin_x86_64 || skip "Did not detect an x86_64 Darwin system"
  run components/hab/install.sh -v 0.79.1

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.79.1" ]
}

@test "Install ignores release when installing from packages.chef.io" {
  ! darwin_aarch64 || skip "Old version 0.90.6 has no aarch64-darwin binary"
  run components/hab/install.sh -v "0.90.6/20191112141314"
  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.90.6" ]
}

@test "Install version <= 2.0.504 on aarch64-darwin uses stable install path (no /opt/hab)" {
  darwin_aarch64 || skip "Did not detect an aarch64 Darwin system"
  run components/hab/install.sh -v 2.0.504

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 2.0.504" ]
  [ ! -d "/opt/hab" ]
}

@test "Install version > 2.0.504 on aarch64-darwin uses aarch64 install path (/opt/hab exists)" {
  darwin_aarch64 || skip "Did not detect an aarch64 Darwin system"
  run components/hab/install.sh -c acceptance

  [ "$status" -eq 0 ]
  [ -d "/opt/hab" ]
}
