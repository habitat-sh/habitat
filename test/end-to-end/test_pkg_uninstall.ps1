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

# Start the supervisor and load nginx
$job = Start-Job { hab sup run }
Wait-Supervisor -Timeout 120

Describe "pkg uninstall" {
    It "installs core/redis" {
        foreach ($p in $pkgs) {
            hab pkg install "$p"
        }
        hab pkg list "$pkg" | Should -BeExactly $pkgs
    }

    It "uninstall a single package" {
        hab pkg uninstall "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[0..8]
    }

    It "uninstall all but the two latest of version 3.2.4" {
        hab pkg uninstall --keep-latest=2 "$pkg/3.2.4"
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
        hab pkg uninstall $nginxPkg

        # Verify nginx is still installed
        hab pkg list $nginxPkg | Should -Not -BeNullOrEmpty

        # Verify all nginx dependencies are still installed
        foreach($dep in $initialDeps) {
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

    AfterAll {
        Stop-Job -Job $job
        Remove-Job -Job $job

        hab pkg uninstall --keep-latest=0 "$pkg"
        hab pkg uninstall --keep-latest=0 "$nginxPkg"
    }
}
