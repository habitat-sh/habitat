Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "$HOME/.hab/accepted-licenses"
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "/hab/accepted-licenses"

Describe "license" {
    It "version check without license works" {
        ./target/debug/hab --version
        $LastExitCode | Should -Be 0

        ./target/debug/hab -V
        $LastExitCode | Should -Be 0

        ./target/debug/hab svc load --version
        $LastExitCode | Should -Be 0

        ./target/debug/hab sup -V
        $LastExitCode | Should -Be 0
    }

    It "help without license works" {
        ./target/debug/hab --help
        $LastExitCode | Should -Be 0

        ./target/debug/hab -h
        $LastExitCode | Should -Be 0

        ./target/debug/hab svc load --help
        $LastExitCode | Should -Be 0

        ./target/debug/hab sup -h
        $LastExitCode | Should -Be 0
    }

    It "non-version and non-help commands timeout on license check" {
        $process = Start-Process "./target/debug/hab" -ArgumentList "svc status" -PassThru
        {
            $process | Wait-Process -Timeout 1 -ErrorAction Stop
        } | Should -Throw "time-out"

        $process | Stop-Process -Force
    }

    It "non-version and non-help commands do no work when denying license" {
        $Env:HAB_LICENSE = "deny"

        ./target/debug/hab svc load
        $LastExitCode | Should -Be 1

        ./target/debug/hab sup run
        $LastExitCode | Should -Be 1

        ./target/debug/hab pkg list --all
        $LastExitCode | Should -Be 1
    }

    It "HAB_LICENSE=accept-no-persist works" {
        $Env:HAB_LICENSE = "accept-no-persist"

        ./target/debug/hab pkg list --all
        $LastExitCode | Should -Be 0
    }

    It "all commands work with license" {
        ./target/debug/hab license accept

        ./target/debug/hab --version
        $LastExitCode | Should -Be 0

        ./target/debug/hab svc load --help
        $LastExitCode | Should -Be 0

        ./target/debug/hab pkg list --all
        $LastExitCode | Should -Be 0
    }

    It "HAB_LICENSE=deny causes commands to fail even if license was previously accpeted" {
        $Env:HAB_LICENSE = "deny"
        ./target/debug/hab license accept

        ./target/debug/hab pkg list --all
        $LastExitCode | Should -Be 1
    }
}