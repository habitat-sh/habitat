# This test is designed to catch the regression described in
# https://github.com/habitat-sh/habitat/issues/6771
#
# When a user runs `hab studio enter` for the first time after installing a
# new Habitat release, the `chef/hab-studio` package won't be present on the
# system and the cli will automatically download and install the appropirate
# package. Since we always install the studio as part of our build process to
# ensure we're using the correct version, this behavior needs to be exercised
# as its own test.

# Ensure there are no studios installed
if(Test-Path /hab/pkgs/chef/hab-studio) {
    hab pkg uninstall chef/hab-studio
}

Describe "Studio install" {
    # 'studio enter' requires a signing key to be present for the current origin
    hab origin key generate "$env:HAB_ORIGIN"

    It "can create a new studio when no studio package is installed" {
        hab studio new
        $LASTEXITCODE | Should -Be 0
    }
}
