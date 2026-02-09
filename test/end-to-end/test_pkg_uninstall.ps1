$pkg = "core/redis"
$pkgs = @(
    "$pkg/3.2.3/20161102201135",
    "$pkg/3.2.4/20161104175435",
    "$pkg/3.2.4/20161210004233",
    "$pkg/3.2.4/20161215055911",
    "$pkg/3.2.4/20170103160441",
    "$pkg/3.2.4/20170106011058",
    "$pkg/4.0.10/20180801003001",
    "$pkg/4.0.10/20190116005049",
    "$pkg/4.0.14/20200319184753",
    "$pkg/4.0.14/20200319200053"
)
$nginxPkg = "core/nginx"
$env:HAB_NOCOLORING="true"

hab origin key generate $env:HAB_ORIGIN

# Start the supervisor and load nginx
$job = Start-Job { hab sup run }
Wait-Supervisor -Timeout 120

Describe "pkg uninstall" {
    It "installs core/redis" {
        foreach ($p in $pkgs) {
            Write-Host "Installing package $p"
            Write-Host (hab pkg install "$p" | Out-String)
        }
        hab pkg list "$pkg" | Should -BeExactly $pkgs
    }

    It "uninstall a single package" {
        hab pkg uninstall "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[0..8]
    }

    It "uninstall all but the two latest of version 3.2.4" {
        Write-Host (hab pkg uninstall --keep-latest=2 "$pkg/3.2.4" | Out-String)
        hab pkg list "$pkg" | Should -BeExactly $pkgs[,0+4..8]
    }

    It "dry run should do nothing" {
        hab pkg uninstall -d "$pkg"
        hab pkg uninstall -d --keep-latest=1 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[,0+4..8]
    }

    It "uninstall with a fully qualified ident" {
        hab pkg uninstall --keep-latest=3 "$pkg/3.2.3/20161102201135" | Should -Contain "… Skipping Only 1 packages installed"
        hab pkg uninstall --keep-latest=0 "$pkg/3.2.3/20161102201135"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[4..8]
    }

    It "uninstall all but the three latest" {
        hab pkg uninstall --keep-latest=3 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "uninstall does nothing if keeping all" {
        hab pkg uninstall --keep-latest=10 "$pkg" | Should -Contain "… Skipping Only 3 packages installed"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "cannot uninstall a package loaded by the supervisor or any of its dependencies" {
        # Install nginx
        hab pkg install $nginxPkg --channel stable

        # Get list of nginx dependencies before loading
        $initialDeps = @(hab pkg dependencies $nginxPkg --transitive)

        # Load nginx service
        hab svc load $nginxPkg
        Wait-SupervisorService nginx -Timeout 20

        # Attempt to uninstall nginx
        Write-Host (hab pkg uninstall $nginxPkg | Out-String)

        # Verify nginx is still installed
        hab pkg list $nginxPkg | Should -Not -BeNullOrEmpty

        # Verify all nginx dependencies are still installed
        foreach($dep in $initialDeps) {
            Write-Host "Checking dependency $dep"
            hab pkg list $dep | Should -Not -BeNullOrEmpty
        }
    }

    It "uninstall all" {
        hab pkg uninstall --keep-latest=0 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly @()
        hab svc unload $nginxPkg
        Wait-SupervisorServiceUnload nginx -Timeout 20
        hab pkg uninstall --keep-latest=0 "$nginxPkg"
        hab pkg list "$nginxPkg" | Should -BeExactly @()
        # we know this is a dep of nginx and no other reverse dependencies
        hab pkg list "core/libedit" | Should -BeExactly @()
    }

    It "uninstalls all package deps and transitive deps" {
        Invoke-BuildAndInstall dep-pkg-1
        Invoke-BuildAndInstall dep-pkg-2
        Invoke-BuildAndInstall dep-pkg-4

        hab pkg uninstall $env:HAB_ORIGIN/dep-pkg-4

        hab pkg list "$env:HAB_ORIGIN/dep-pkg-4" | Should -BeExactly @()
        hab pkg list "$env:HAB_ORIGIN/dep-pkg-2" | Should -BeExactly @()
        hab pkg list "$env:HAB_ORIGIN/dep-pkg-1" | Should -BeExactly @()
    }

    It "Leaves package with reverse deps that are not being uninstalled" {
        Invoke-BuildAndInstall dep-pkg-1
        Invoke-BuildAndInstall dep-pkg-2
        Invoke-BuildAndInstall dep-pkg-3
        Invoke-BuildAndInstall dep-pkg-4

        hab pkg uninstall $env:HAB_ORIGIN/dep-pkg-4

        hab pkg list "$env:HAB_ORIGIN/dep-pkg-4" | Should -BeExactly @()
        hab pkg list "$env:HAB_ORIGIN/dep-pkg-3" | Should -Not -BeNullOrEmpty
        hab pkg list "$env:HAB_ORIGIN/dep-pkg-2" | Should -Not -BeNullOrEmpty
        hab pkg list "$env:HAB_ORIGIN/dep-pkg-1" | Should -Not -BeNullOrEmpty
    }

    AfterAll {
        Stop-Job -Job $job
        Remove-Job -Job $job

        hab pkg uninstall --keep-latest=0 "$pkg"
        hab pkg uninstall --keep-latest=0 "$nginxPkg"
    }
}
