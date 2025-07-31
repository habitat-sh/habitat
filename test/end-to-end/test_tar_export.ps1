Remove-Item *.tar.gz

function Get-Ident($pkg, $tar) {
    $ident = tar --list --file $tar | Where-Object { $_ -like "hab/pkgs/$pkg/**/IDENT" }
    tar --extract --to-stdout --file $tar $ident
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
    It "Includes supervisor" {
        Get-Ident chef/hab-sup $tar | Should -BeLike "chef/hab-sup/$version/*"
    }
    It "Includes launcher" {
        Get-Ident chef/hab-launcher $tar | Should -Not -Be $null
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
        $habBinDir = tar --list --file $tar | Where-Object { $_ -like "hab/bin/*" }
        $habBinDir | Should -Be $null
    }
    It "Includes supervisor" {
        Get-Ident chef/hab-sup $tar | Should -Not -Be $null
    }
    It "Includes launcher" {
        Get-Ident chef/hab-launcher $tar | Should -Not -Be $null
    }
}

Context "hab pkg export tar core/nginx --no-hab-sup" {
    hab pkg export tar core/nginx --no-hab-sup --base-pkgs-channel $env:HAB_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Includes hab binary directory" {
        $habBinDir = tar --list --file $tar | Where-Object { $_ -like "hab/bin/*" }
        $habBinDir | Should -Not -Be $null
    }
    It "Does not include supervisor" {
        Get-Ident chef/hab-sup $tar | Should -Be $null
    }
    It "Does not include launcher" {
        Get-Ident chef/hab-launcher $tar | Should -Be $null
    }
}
