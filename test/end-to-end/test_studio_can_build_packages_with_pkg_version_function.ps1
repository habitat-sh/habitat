hab origin key generate $env:HAB_ORIGIN

Describe "package defining pkg_version function" {
    hab pkg build test/fixtures/pkg_version
    It "builds without error" {
        $LASTEXITCODE | Should -Be 0
    }
    It "builds the correct version" {
        # pkg_version hardcoces 5.5.5 as the version
        . results/last_build.ps1
        $pkg_version | Should -Be "5.5.5"
    }
}
