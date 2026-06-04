Remove-Item *.tar.gz

$IsDarwinArm64 = $IsLinux -eq $false -and $IsWindows -eq $false -and ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64)

# On aarch64-darwin, packages live under opt/hab; everywhere else under hab
$HabRoot = if ($IsDarwinArm64) { "opt/hab" } else { "hab" }

function Get-Ident($pkg, $tar) {
    $ident = tar --list --file $tar | Where-Object { $_ -like "$HabRoot/pkgs/$pkg/**/IDENT" }
    if ($null -ne $ident) {
        tar --extract --to-stdout --file $tar $ident
    }
}

Describe "hab pkg export tar core/nginx" {
    hab pkg export tar core/nginx --base-pkgs-channel $env:HAB_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    $version = ((((hab --version) -split " ")[1]) -split "/")[0]
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Includes hab" {
        Get-Ident chef/hab $tar | Should -BeLike "chef/hab/$version/*"
    }
    # On aarch64-darwin, supervisor and launcher are never included
    if ($IsDarwinArm64) {
        It "Does not include supervisor" {
            Get-Ident chef/hab-sup $tar | Should -Be $null
        }
        It "Does not include launcher" {
            Get-Ident chef/hab-launcher $tar | Should -Be $null
        }
    } else {
        It "Includes supervisor" {
            Get-Ident chef/hab-sup $tar | Should -BeLike "chef/hab-sup/$version/*"
        }
        It "Includes launcher" {
            Get-Ident chef/hab-launcher $tar | Should -Not -Be $null
        }
    }
}

Describe "hab pkg export tar core/nginx --no-hab-bin" {
    hab pkg export tar core/nginx --no-hab-bin --base-pkgs-channel $env:HAB_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Does not include hab binary directory" {
        $habBinDir = tar --list --file $tar | Where-Object { $_ -like "$HabRoot/bin/*" }
        $habBinDir | Should -Be $null
    }
    # On aarch64-darwin, supervisor and launcher are never included
    if ($IsDarwinArm64) {
        It "Does not include supervisor" {
            Get-Ident chef/hab-sup $tar | Should -Be $null
        }
        It "Does not include launcher" {
            Get-Ident chef/hab-launcher $tar | Should -Be $null
        }
    } else {
        It "Includes supervisor" {
            Get-Ident chef/hab-sup $tar | Should -Not -Be $null
        }
        It "Includes launcher" {
            Get-Ident chef/hab-launcher $tar | Should -Not -Be $null
        }
    }
}

Context "hab pkg export tar core/nginx --no-hab-sup" {
    # --no-hab-sup is not available on aarch64-darwin (supervisor is always excluded there)
    if (-not $IsDarwinArm64) {
        hab pkg export tar core/nginx --no-hab-sup --base-pkgs-channel $env:HAB_INTERNAL_BLDR_CHANNEL
        $tar = Get-Item core-nginx-*.tar.gz
        It "Creates tarball" {
            $tar | Should -Not -Be $null
        }
        It "Includes nginx" {
            Get-Ident core/nginx $tar | Should -Not -Be $null
        }
        It "Includes hab binary directory" {
            $habBinDir = tar --list --file $tar | Where-Object { $_ -like "$HabRoot/bin/*" }
            $habBinDir | Should -Not -Be $null
        }
        It "Does not include supervisor" {
            Get-Ident chef/hab-sup $tar | Should -Be $null
        }
        It "Does not include launcher" {
            Get-Ident chef/hab-launcher $tar | Should -Be $null
        }
    }
}
