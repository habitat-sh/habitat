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
