function Get-RustfmtToolchain {
  # It turns out that every nightly version of rustfmt has slight tweaks from the previous version.
  # This means that if we're always using the latest version, then we're going to have enormous
  # churn. Even PRs that don't touch rust code will likely fail CI, since master will have been
  # formatted with a different version than is running in CI. Because of this, we're going to pin
  # the version of nightly that's used to run rustfmt and bump it when we do a new release.
  #
  # Note that not every nightly version of rust includes rustfmt. Sometimes changes are made that
  # break the way rustfmt uses rustc. Therefore, before updating the pin below, double check
  # that the nightly version you're going to update it to includes rustfmt. You can do that
  # using https://mexus.github.io/rustup-components-history/x86_64-unknown-linux-gnu.html
  Get-Content "$PSScriptRoot\..\..\RUSTFMT_VERSION"
}

# function Install-Rustfmt($Toolchain) {
#   local toolchain="${1?toolchain argument required}"
#   Install-RustToolchain $Toolchain
#   rustup component add --toolchain $Toolchain rustfmt
# }





# On buildkite, the rust binaries will be directly in C:
if($env:BUILDKITE) {
    # this will avoid a path length limit from the long buildkite working dir path
    $env:CARGO_TARGET_DIR = "c:\target"
}
