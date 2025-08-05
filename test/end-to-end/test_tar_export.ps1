Remove-Item *.tar.gz

function Get-Ident($pkg, $tar) {
    $ident = tar --list --file $tar | Where-Object { $_ -like "hab/pkgs/core/$pkg/**/IDENT" }
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
        Get-Ident nginx $tar | Should -Not -Be $null
    }
    It "Includes hab" {
        Get-Ident hab $tar | Should -BeLike "core/hab/$version/*"
    }
    It "Includes supervisor" {
        Get-Ident hab-sup $tar | Should -BeLike "core/hab-sup/$version/*"
    }
    It "Includes launcher" {
        Get-Ident hab-launcher $tar | Should -Not -Be $null
    }
}
