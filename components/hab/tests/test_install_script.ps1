Describe "Install habitat using install.ps1" {
    It "can install the latest version of Habitat" {
        components/hab/install.ps1
        $LASTEXITCODE | Should -Be 0
        (Get-Command hab).Path | Should -Be "C:\ProgramData\Habitat\hab.exe"
    }

    It "can install a specific version of Habitat" {
        components/hab/install.ps1 -v 0.90.6
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.90.6/*"
    }

    It "can install a specific version of Habitat from Bintray" {
        components/hab/install.ps1 -v 0.79.1
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.79.1/*"
    }
}
