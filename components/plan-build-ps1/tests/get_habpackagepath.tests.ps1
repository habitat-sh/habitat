. $PSScriptRoot\..\bin\shared.ps1

Describe "Get-HabPackagePath" {
    New-Item "TestDrive:\src" -ItemType Directory -Force | Out-Null
    $script:HAB_PKG_PATH = Join-Path (Get-PSDrive TestDrive).Root "hab\pkgs"
    New-Item -ItemType Directory $HAB_PKG_PATH
    $pkg_path = Join-Path $HAB_PKG_PATH "core\blah\0.1.0\111"
    $script:pkg_all_deps_resolved = @($pkg_path)

    It "finds path for origin/pkg" {
        Get-HabPackagePath "core/blah" | Should -Be $pkg_path
    }

    It "finds path for package name" {
        Get-HabPackagePath "blah" | Should -Be $pkg_path
    }

    It "finds path for package name/version" {
        Get-HabPackagePath "blah/0.1.0" | Should -Be $pkg_path
    }

    It "errors if there is no package found" {
        Get-HabPackagePath "blah/0.11.0" | Should -Be $null
    }
}
