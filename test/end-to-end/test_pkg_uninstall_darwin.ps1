# macOS-specific pkg uninstall tests.

$pkg = "core/sqlite"
$nginxPkg = "core/nginx"
$channel = "aarch64-darwin-opt"
$env:HAB_NOCOLORING = "true"

Describe "pkg uninstall (macOS)" {
    BeforeAll {
        hab pkg install "$pkg" --channel $channel
        hab pkg install "$nginxPkg" --channel $channel
    }

    It "lists installed package" {
        $list = hab pkg list "$pkg"
        $list | Should -Not -BeNullOrEmpty
    }

    It "uninstalls the package" {
        hab pkg uninstall "$pkg"
        $LASTEXITCODE | Should -Be 0
    }

    It "package is removed after uninstall" {
        hab pkg list "$pkg" | Should -BeExactly @()
    }

    It "dry run does not remove the package" {
        hab pkg install "$nginxPkg" --channel $channel
        hab pkg uninstall -d "$nginxPkg"
        hab pkg list "$nginxPkg" | Should -Not -BeNullOrEmpty
    }

    It "uninstalls with keep-latest=0" {
        hab pkg uninstall --keep-latest=0 "$nginxPkg"
        $LASTEXITCODE | Should -Be 0
        hab pkg list "$nginxPkg" | Should -BeExactly @()
    }

    AfterAll {
        hab pkg uninstall --keep-latest=0 "$pkg" 2>$null
        hab pkg uninstall --keep-latest=0 "$nginxPkg" 2>$null
    }
}
