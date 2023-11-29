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

    It "installs any missing transitive dependency that may have been removed" {
        $dep3_artifact = Get-Item "/hab/cache/artifacts/$env:HAB_ORIGIN-dep-pkg-3*"

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        $dep1_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-1)"
        $dep1_install_hook_status = "$dep1_install/INSTALL_HOOK_STATUS"

        $dep2_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-2)"
        $dep2_install_hook_status = "$dep2_install/INSTALL_HOOK_STATUS"

        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"

        # remove dep 1
        Remove-Item -Recurse -Force -Confirm:$false "$dep1_install"

        # dep 1 hook status will not exist
        { Get-Content  "$dep1_install_hook_status" } | Should -Throw
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        # removed dep 1 packages are reinstalled by habitat
        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"
    }

    It "installs any missing direct dependency that may have been removed" {
        $dep3_artifact = Get-Item "/hab/cache/artifacts/$env:HAB_ORIGIN-dep-pkg-3*"

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        $dep1_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-1)"
        $dep1_install_hook_status = "$dep1_install/INSTALL_HOOK_STATUS"

        $dep2_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-2)"
        $dep2_install_hook_status = "$dep2_install/INSTALL_HOOK_STATUS"

        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"

        # remove dep 2
        Remove-Item -Recurse -Force -Confirm:$false "$dep2_install"

        # dep 2 hook status will not exist
        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        { Get-Content  "$dep2_install_hook_status" } | Should -Throw

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        # removed dep 1 packages are reinstalled by habitat
        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"
    }


    It "installs any missing transitive and direct dependencies that may have been removed" {
        $dep3_artifact = Get-Item "/hab/cache/artifacts/$env:HAB_ORIGIN-dep-pkg-3*"

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        $dep1_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-1)"
        $dep1_install_hook_status = "$dep1_install/INSTALL_HOOK_STATUS"

        $dep2_install = "$(hab pkg path $env:HAB_ORIGIN/dep-pkg-2)"
        $dep2_install_hook_status = "$dep2_install/INSTALL_HOOK_STATUS"

        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"

        # remove dep 1 and dep 2
        Remove-Item -Recurse -Force -Confirm:$false "$dep1_install"
        Remove-Item -Recurse -Force -Confirm:$false "$dep2_install"

        # dep 1 and dep 2 hook status will not exist
        { Get-Content  "$dep1_install_hook_status" } | Should -Throw
        { Get-Content  "$dep2_install_hook_status" } | Should -Throw

        hab pkg install $dep3_artifact
        $LASTEXITCODE | Should -Be 0

        # removed dep 1 and dep 2 packages are reinstalled by habitat
        Get-Content  "$dep1_install_hook_status" | Should -Be "0"
        Get-Content  "$dep2_install_hook_status" | Should -Be "0"
    }
}
