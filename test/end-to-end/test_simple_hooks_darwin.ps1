# macOS-specific hooks tests.
# Tests that hooks actually execute during package install and produce
# the expected output.
# HAB_INTERPRETER_IDENT is set to core/coreutils in setup_environment_darwin.sh
# since core/busybox-static is not available for aarch64-darwin.

Describe "Hook execution during package install" {
    BeforeAll {
        hab origin key generate $env:HAB_ORIGIN
        Invoke-Build plans/simple-hooks
        . ./results/last_build.ps1
        $script:pkgIdent = $pkg_ident
        $script:pkgName = ($pkg_ident -split "/")[1]
        $script:pkgPrefix = "/hab/pkgs/$pkg_ident"
        # Install without --ignore-install-hook so the install hook runs
        hab pkg install ./results/$pkg_artifact
    }

    It "runs the install hook successfully" {
        "$script:pkgPrefix/INSTALL_HOOK_STATUS" | Should -Exist
        Get-Content "$script:pkgPrefix/INSTALL_HOOK_STATUS" | Should -Be "0"
    }

    It "install hook renders templates and produces stdout" {
        $logPath = "/hab/svc/$($script:pkgName)/logs/install.stdout.log"
        $logPath | Should -Exist
        Get-Content $logPath | Should -Contain "install hook $($script:pkgName)"
    }
}

Describe "Hook extension stripping" {
    BeforeAll {
        Invoke-Build hook-extension-plan
        . ./results/last_build.ps1
        $script:hookPkgPrefix = "/hab/pkgs/$pkg_ident"
    }

    It "strips .sh extension from hook files" {
        "$script:hookPkgPrefix/hooks/install" | Should -Exist
    }

    It "does not keep the original .sh file" {
        Test-Path "$script:hookPkgPrefix/hooks/install.sh" | Should -Be $false
    }
}

Describe "Bad hook extension plan" {
    It "fails when there are multiple extensions for the same hook" {
        hab pkg build test/fixtures/bad-hook-extension-plan
        $LASTEXITCODE | Should -Not -Be 0
    }
}
