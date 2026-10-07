# Specific/legacy version installs below are only ever published to
# packages.chef.io for x86_64-windows (there's no aarch64-windows history
# there, and no release pipeline for it yet - see components/hab/install.ps1).
# Only the "latest" test is meaningful on aarch64-windows, since it goes
# through the Builder-direct fallback instead.
$isArm64 = ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") -Or ($env:PROCESSOR_ARCHITEW6432 -eq "ARM64")

Describe "Install habitat using install.ps1" {
    It "can install the latest version of Habitat" {
        components/hab/install.ps1
        $LASTEXITCODE | Should -Be 0
        (Get-Command hab).Path | Should -Be "C:\ProgramData\Habitat\hab.exe"
    }

    It "can install a specific version of Habitat" -Skip:$isArm64 {
        components/hab/install.ps1 -v 0.90.6
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.90.6/*"
    }

    It "can install a specific version of legacy Habitat package" -Skip:$isArm64 {
        components/hab/install.ps1 -v 0.79.1
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.79.1/*"
    }

    It "ignores release when installing from packages.chef.io" -Skip:$isArm64 {
        components/hab/install.ps1 -v "0.90.6/20191112141314"
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.90.6/*"
    }
}
