Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "$HOME/.hab/accepted-licenses"
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "/hab/accepted-licenses"
$env:HAB_LICENSE = $null

Describe "license" {
    It "version check without license works" {
        hab --version
        $LastExitCode | Should -Be 0

        hab -V
        $LastExitCode | Should -Be 0

        hab sup --version
        $LastExitCode | Should -Be 0

        hab sup -V
        $LastExitCode | Should -Be 0
    }

    It "help without license works" {
        hab --help
        $LastExitCode | Should -Be 0

        hab -h
        $LastExitCode | Should -Be 0

        hab svc load --help
        $LastExitCode | Should -Be 0

        hab sup -h
        $LastExitCode | Should -Be 0
    }

    It "non-version and non-help commands timeout on license check" {
        $process = Start-Process "hab" -ArgumentList "svc status" -PassThru
        {
            Wait-ProcessExit $process -Timeout 1 -ErrorAction Stop
        } | Should -Throw "Timed out"

        $process | Stop-Process -Force
    }

    It "non-version and non-help commands do no work when denying license" {
        $Env:HAB_LICENSE = "deny"

        hab svc load
        $LastExitCode | Should -Be 1

        hab sup run
        $LastExitCode | Should -Be 1

        hab pkg list --all
        $LastExitCode | Should -Be 1
    }

    It "HAB_LICENSE=accept-no-persist works" {
        $Env:HAB_LICENSE = "accept-no-persist"

        hab pkg list --all
        $LastExitCode | Should -Be 0
    }

    It "all commands work with license" {
        hab license accept

        hab --version
        $LastExitCode | Should -Be 0

        hab svc load --help
        $LastExitCode | Should -Be 0

        hab pkg list --all
        $LastExitCode | Should -Be 0
    }

    It "HAB_LICENSE=deny causes commands to fail even if license was previously accpeted" {
        $Env:HAB_LICENSE = "deny"
        hab license accept

        hab pkg list --all
        $LastExitCode | Should -Be 1
    }
}