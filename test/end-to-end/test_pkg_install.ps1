# dep-pkg-1 has a simple install hook.
# dep-pkg-2 depends on 1 and its install hook will exit 1
# if dep-pkg-1's install hook did not run succesfully.
# dep-pkg-3 depends on 2 and its install hook will exit 1
# if dep-pkg-2's install hook did not run succesfully.
Describe "pkg install" {
    BeforeEach {
        hab origin key generate $env:HAB_ORIGIN

        Invoke-BuildAndInstall dep-pkg-1
        Invoke-BuildAndInstall dep-pkg-2
        Invoke-BuildAndInstall dep-pkg-3
        Invoke-BuildAndInstall dep-pkg-4

        hab pkg uninstall $env:HAB_ORIGIN/dep-pkg-3
        hab pkg uninstall $env:HAB_ORIGIN/dep-pkg-4
    }

    It "installs all dependencies and executes all install hooks" {
        $cached = Get-Item "/hab/cache/artifacts/$env:HAB_ORIGIN-dep-pkg-3*"
        hab pkg install $cached.FullName
        $LASTEXITCODE | Should -Be 0
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-1)/INSTALL_HOOK_STATUS" | Should -Be "0"
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-2)/INSTALL_HOOK_STATUS" | Should -Be "0"
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-3)/INSTALL_HOOK_STATUS" | Should -Be "0"
    }

    # dep-pkg-4 depends on 1 and 2 and its install hook will exit 1
    # if dep-pkg-2's install hook did not run succesfully. Because 2 depends on one,
    # the dep on 1 here is unnecessary but this test ensures that 1 will be
    # installed before 2
    It "installs all dependencies and executes all install hooks in the correct order" {
        $cached = Get-Item "/hab/cache/artifacts/$env:HAB_ORIGIN-dep-pkg-4*"
        hab pkg install $cached.FullName
        $LASTEXITCODE | Should -Be 0
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-1)/INSTALL_HOOK_STATUS" | Should -Be "0"
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-2)/INSTALL_HOOK_STATUS" | Should -Be "0"
        Get-Content "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-4)/INSTALL_HOOK_STATUS" | Should -Be "0"
    }
}
