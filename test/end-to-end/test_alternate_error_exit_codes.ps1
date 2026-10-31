# Initialize shared setup values in the current scope for sibling Pester blocks.
$setup = {
    $PkgName = "custom-hook-exit-code"
    $PkgOrigin = "habitat-testing"
    Set-Variable -Name PkgIdent -Value ("$PkgOrigin/$PkgName") -Scope Local
    $Env:HAB_ORIGIN = $PkgOrigin
    hab origin key generate $PkgOrigin
    Invoke-Build $PkgName
    . ./results/last_build.ps1
}
if ($useWindowsPester5) {
    BeforeAll $setup
} else {
    . $setup
}

Describe "install and uninstall hook error codes are propogated" {
    It "`hab pkg install` exits with install hook exit code" {
        $Env:CUSTOM_HOOK_EXIT_CODE = 42
        hab pkg install ./results/$pkg_artifact
        $LastExitCode | Should -Be $Env:CUSTOM_HOOK_EXIT_CODE
    }

    It "`hab pkg install` succeeds" {
        $Env:CUSTOM_HOOK_EXIT_CODE = ""
        hab pkg install ./results/$pkg_artifact
        $LastExitCode | Should -Be 0
    }

    It "`hab pkg uninstall` exits with uninstall hook exit code" {
        $Env:CUSTOM_HOOK_EXIT_CODE = 97
        hab pkg uninstall $PkgIdent
        $LastExitCode | Should -Be $Env:CUSTOM_HOOK_EXIT_CODE
    }

    It "`hab pkg uninstall` succeeds" {
        $Env:CUSTOM_HOOK_EXIT_CODE = ""
        hab pkg uninstall $PkgIdent
        $LastExitCode | Should -Be 0
    }

    AfterAll {
        hab pkg uninstall --keep-latest 0 $PkgIdent
    }
}
