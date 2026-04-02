# Given a fresh install of Habitat we should be able to install packages from builder
# Ensure that we don't have any ssl certificates cached that might influence our ability
# to connect.
# Note: since we're testing pre-release this won't be a "pure" fresh install, as we
# have to curlbash install stable first, in order to get the pre-release version.

Describe "Clean hab installation" {
    It "has no root ssl cache" {
        Test-Path /hab/cache/ssl | Should -Be $false
    }
    It "has no user ssl cache" {
        if ($IsMacOS) {
            # On macOS the hab user has shell /usr/bin/false, so use dscl to
            # find its home directory and check directly
            $habHome = (dscl . -read /Users/hab NFSHomeDirectory 2>$null) -replace 'NFSHomeDirectory:\s*',''
            if (!$habHome) { $habHome = "/var/empty" }
            Test-Path (Join-Path $habHome ".hab/cache/ssl") | Should -Be $false
        } else {
            su hab -c "test ! -d ~/.hab/cache/ssl"
            $LASTEXITCODE | Should -Be 0
        }
    }
    It "can talk to builder" {
        if ($IsMacOS) {
            $pkgChannel = "aarch64-darwin"
            # core/redis may not exist for aarch64-darwin; use core/nginx instead
            hab pkg install core/nginx --channel $pkgChannel
        } else {
            hab pkg install core/redis --channel stable
        }
        $LASTEXITCODE | Should -Be 0
    }
}
