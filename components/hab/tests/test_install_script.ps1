# The older versions used below predate aarch64-windows packages.
$isArm64 = ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") -Or ($env:PROCESSOR_ARCHITEW6432 -eq "ARM64")

Describe "Install habitat using install.ps1" {
    It "can install the latest version of Habitat" {
        components/hab/install.ps1
        $LASTEXITCODE | Should -Be 0
        (Get-Command hab).Path | Should -Be "C:\ProgramData\Habitat\hab.exe"
    }

    It "can install a specific version of Habitat" -Skip:$isArm64 {
        components/hab/install.ps1 -v 0.90.6
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.90.6/*"
    }

    It "can install a specific version of legacy Habitat package" -Skip:$isArm64 {
        components/hab/install.ps1 -v 0.79.1
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.79.1/*"
    }

    It "ignores release when installing from packages.chef.io" -Skip:$isArm64 {
        components/hab/install.ps1 -v "0.90.6/20191112141314"
        $LASTEXITCODE | Should -Be 0

        $result = hab --version
        $result | Should -Match "hab 0.90.6/*"
    }
}

Describe "Installer archive selection" -Tag "unit" {
    BeforeAll {
        $installer = Join-Path $PSScriptRoot "..\install.ps1"
        $tokens = $null
        $parseErrors = $null
        $ast = [System.Management.Automation.Language.Parser]::ParseFile($installer, [ref]$tokens, [ref]$parseErrors)
        if ($parseErrors.Count -gt 0) { throw $parseErrors[0] }
        $function = $ast.Find({
            param($node)
            $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq "Get-Archive"
        }, $true)
        if (!$function) { throw "Get-Archive was not found in install.ps1" }
        . ([scriptblock]::Create($function.Extent.Text.Replace("Function Get-Archive", "Function script:Get-Archive")))
        function script:Get-File($url, $dst) {}
        $script:packagesChefioRootUrl = "https://packages.chef.io/files"
        $script:workdir = $TestDrive
        $source = Get-Content $installer -Raw
        $entryEnd = $source.IndexOf('Function Get-File')
        if ($entryEnd -lt 0) { throw "Installer entry point was not found" }
        $script:entryPoint = [scriptblock]::Create($source.Substring(0, $entryEnd) + "`n`$Target")
    }

    BeforeEach {
        $script:downloads = @()
        Mock Get-File {
            $script:downloads += $url
            if ($url -like "*.sha256sum") { Set-Content $dst "test-checksum  hab.zip" }
        }
    }

    It "downloads the latest ARM64 zip and checksum from the selected channel" {
        $archive = Get-Archive "dev" "latest" "aarch64-windows"
        $script:downloads[0] | Should -Be "https://packages.chef.io/files/dev/habitat/latest/hab-aarch64-windows.zip"
        $script:downloads[1] | Should -Be "https://packages.chef.io/files/dev/habitat/latest/hab-aarch64-windows.zip.sha256sum"
        $archive.shasum | Should -Be "test-checksum"
    }

    It "downloads the requested ARM64 version and ignores its release" {
        $null = Get-Archive "dev" "2.2.5/20261120120000" "aarch64-windows"
        $script:downloads[0] | Should -Be "https://packages.chef.io/files/habitat/2.2.5/hab-aarch64-windows.zip"
    }

    It "continues to download the x86_64 zip" {
        $null = Get-Archive "stable" "" "x86_64-windows"
        $script:downloads[0] | Should -Be "https://packages.chef.io/files/stable/habitat/latest/hab-x86_64-windows.zip"
    }

    It "accepts retired arguments but selects the host architecture" {
        $arch = $env:PROCESSOR_ARCHITECTURE
        $nativeArch = $env:PROCESSOR_ARCHITEW6432
        try {
            $env:PROCESSOR_ARCHITECTURE = "x86"
            $env:PROCESSOR_ARCHITEW6432 = "ARM64"
            $result = & $script:entryPoint -Target "x86_64-windows" -BldrUrl "https://example.invalid" -AuthToken "test-token" 3>&1
            $result[-1] | Should -Be "aarch64-windows"
            ($result | Out-String) | Should -Not -Match "test-token"
        } finally {
            $env:PROCESSOR_ARCHITECTURE = $arch
            $env:PROCESSOR_ARCHITEW6432 = $nativeArch
        }
    }
}
