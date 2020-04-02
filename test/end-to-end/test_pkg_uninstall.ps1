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

    It "uninstall all but the three latest" {
        hab pkg uninstall --keep-latest=3 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "uninstall does nothing if keeping all" {
        hab pkg uninstall --keep-latest=10 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "uninstall all" {
        hab pkg uninstall --keep-latest=0 "$pkg"
        hab pkg list "$pkg" | Should -BeExactly @()
    }

    AfterAll {
        hab pkg uninstall --keep-latest=0 "$pkg"
    }
}
