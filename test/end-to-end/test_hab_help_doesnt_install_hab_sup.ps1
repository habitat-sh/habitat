# A simple test assertion that running `hab sup --help` will not
# attempt to install `chef/hab-sup` if that pkg is not present.

$env:FS_ROOT = (Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName()))
$env:HAB_SUP_BINARY = $null

Describe "hab sup --help" {
    hab sup --help | Out-Null

    It "runs successfully" {
        $LASTEXITCODE | Should -Be 0
    }
    It "does not install the supervisor package" {
        "$env:FS_ROOT/hab/pkgs/chef/hab-sup" | Should -Not -Exist
    }
}
