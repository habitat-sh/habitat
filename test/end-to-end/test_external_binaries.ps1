Describe "`hab` correctly executes external binaries" {
    It "container exporter help" {
        $le = "`n"
        if ($IsWindows) {
            $le = "`r`n"
        }
        $out = (hab pkg export container --help | Out-String)
        $LastExitCode | Should -Be 0
        $out | Should -BeLike "*Creates a container image from a set of Habitat packages (and optionally pushes to a remote${le}repository)*"

        $out = (hab pkg export docker --help | Out-String)
        $LastExitCode | Should -Be 0
        $out | Should -BeLike "*Creates a container image from a set of Habitat packages (and optionally pushes to a remote${le}repository)*"
    }

    It "tar exporter help" {
        $out = hab pkg export tar --help
        $LastExitCode | Should -Be 0
        "Creates a tar package from a Habitat package" | Should -BeIn $out
    }

    It "`hab pkg export` with bad exporter" {
        hab pkg export a_bad_exporter --help
        $LastExitCode | Should -Be 1
    }

    It "`hab sup --version` correctly reports version" {
        # Install an use an old supervisor to ensure version match
        Invoke-NativeCommand hab pkg install "core/hab-sup/1.6.56"
        $env:HAB_SUP_BINARY = "$(hab pkg path core/hab-sup/1.6.56)/bin/hab-sup"
        $out = hab sup --version | Join-String
        $out | Should -BeLike "*1.6.56*"
        $env:HAB_SUP_BINARY = ""
    }
}

