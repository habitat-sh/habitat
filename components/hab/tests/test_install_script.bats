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
  else
    echo "Not running in CI, skipping cleanup"
  fi
}

darwin() {
  [ "$(uname)" == "Darwin" ]
}

linux() {
  [ "$(uname)" == "Linux" ]
}

installed_version() {
  hab --version | cut -d'/' -f1
}

installed_target() {
  local origin="${1:-core}"

  version_release="$(hab --version | cut -d' ' -f2)"
  version="$(cut -d'/' -f1 <<< "$version_release")"
  release="$(cut -d'/' -f2 <<< "$version_release")"
  cat /hab/pkgs/"${origin}"/hab/"$version"/"$release"/TARGET
}

@test "Install latest for x86_86-linux" {
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
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install legacy package for x86_84-linux" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh -v 0.79.1

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.79.1" ]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install package for x86_84-linux from acceptance" {
  linux || skip "Did not detect a Linux system"
  run components/hab/install.sh -c acceptance

  [ "$status" -eq 0 ]
  [[ "$(installed_version)" =~ ^hab\ 2\.[0-9]+\.[0-9]+$ ]]
  [ "$(installed_target chef)" == "x86_64-linux" ]
}

@test "Install latest for x86_86-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/hab/install.sh

  [ "$status" -eq 0 ]
}

@test "Install specific version for x86_64-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/hab/install.sh -v 0.90.6

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.90.6" ]
}

@test "Install legacy package for x86_84-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/hab/install.sh -v 0.79.1

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.79.1" ]
}

@test "Install ignores release when installing from packages.chef.io" {
  run components/hab/install.sh -v "0.90.6/20191112141314"
  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "hab 0.90.6" ]
}
