Describe "version check without license" {
    Remove-Item -Recurse -Force "$HOME/.hab/accepted-licenses"
    Remove-Item -Recurse -Force "/hab/accepted-licenses"

    It "version check without license works" {
        hab --version
        $LastExitCode | Should -Be 0
    }

    It "non-version check commands do not work without license" {
        $process = Start-Process "hab" -ArgumentList "--help" -PassThru
        {
            $process | Wait-Process -Timeout 1 -ErrorAction Stop
        } | Should -Throw "time-out"

        $process | Stop-Process -Force
    }

    hab license accept

    It "all commands work with license" {
        hab --version
        $LastExitCode | Should -Be 0

        hab --help
        $LastExitCode | Should -Be 0
    }
}