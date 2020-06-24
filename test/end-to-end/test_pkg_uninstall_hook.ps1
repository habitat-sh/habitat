$env:HAB_NOCOLORING="true"

Describe "pkg uninstall with uninstall hook" {
    $log = Join-Path -Path $env:SystemDrive -ChildPath hab -AdditionalChildPath @("svc", "uninstall-hook", "logs", "uninstall.stdout.log")
    BeforeEach {
        hab origin key generate $env:HAB_ORIGIN
        Invoke-BuildAndInstall uninstall-hook1
        Invoke-BuildAndInstall uninstall-hook2
    }

    It "does not run uninstall hook when only removing latest package" {
        hab pkg uninstall $env:HAB_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -Not -Exist
    }

    It "does not run uninstall hook when ignoring uninstall hook" {
        hab pkg uninstall --keep-latest=0 --ignore-uninstall-hook $env:HAB_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -Not -Exist
    }

    It "runs latest uninstall hook when uninstalling all packages" {
        hab pkg uninstall --keep-latest=0 $env:HAB_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -FileContentMatchExactly "uninstalling 0.2.0"
    }

    It "exits with exit code in hook" {
        Invoke-BuildAndInstall uninstall-hook3
        hab pkg uninstall --keep-latest=0 $env:HAB_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 1
        $log | Should -FileContentMatchExactly "uninstalling 0.3.0"
    }

    AfterEach {
        hab pkg uninstall --keep-latest=0 $env:HAB_ORIGIN/uninstall-hook
        Remove-Item $log -ErrorAction SilentlyContinue
    }
}
