#!/bin/bash

# Test that SSL_CERT_FILE is persisted into the studio and 
# set to the correct internal path. 
$ErrorActionPreference="stop" 

$studio_flags = ""
if( $env:DOCKER_STUDIO_TEST -eq $true) {
    studio_flags = "-D"
}

function Cleanup-CachedCertificate {
  $hab_ssl_cache="$env:SYSTEM_DRIVE\hab\cache\ssl"
  Remove-Item -Force "$hab_ssl_cache\*" -ErrorAction SilentlyContinue
}

function New-TemporaryDirectory {
  $parent = [System.IO.Path]::GetTempPath()
  [string] $name = [System.Guid]::NewGuid()
  New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

Write-Host "--- Generating a signing key"
hab origin key generate "$env:HAB_ORIGIN"

Write-Host "--- Generating self-signed ssl certificate"

$tempdir = New-TemporaryDirectory
$e2e_certname = "e2e-ssl.pem"
hab pkg install core/openssl 
hab pkg exec core/openssl openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out (Join-Path $tempdir $e2e_certname)

Write-Host "--- Testing valid SSL_CERT_FILE in the studio"

Context "SSL_CERT_FILE is passed into the studio" {
    BeforeEach { 
        $result = hab studio rm
        Cleanup-CachedCertificate
    }

    Describe "SSL_CERT_FILE is a valid certificate" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir $e2e_certname)
        It "Sets env:SSL_CERT_FILE in the studio"  {
            $expected = "\hab\cache\ssl\$e2e_certname"
            $result = hab studio run '(Get-ChildItem env:SSL_CERT_FILE).Value'
            $result[-1] | Should -BeLike "*$expected"
        }

        It "Copies the certificate described by SSL_CERT_FILE into the studio" {
            $result = hab studio run 'Write-Host $env:SSL_CERT_FILE'
            $result
            $result = hab studio run '(Test-Path $env:SSL_CERT_FILE).ToString()'
            $result[-1] | Should -Be "True"
        }

        It "Can search builder for packages when SSL_CERT_FILE is set" {
            $result = hab studio run "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is an invalid certificate" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "invalid_certificate")
        Set-Content -Path $env:SSL_CERT_FILE -Value "I am not a certificate"

        It "Can still search packages on builder" {
            $result = hab studio run "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is a directory" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "cert-as-directory")
        New-Item -ItemType Directory -Force -Path $env:SSL_CERT_FILE

        It "Should not copy the directory into the studio" {
            $result = hab studio run '(Test-Path $env:SSL_CERT_FILE).ToString()'
            $result[-1] | Should -Be "False"
        }

        It "Can still search packages on builder" {
            $result = hab studio run "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is a non-existant-file" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "non-existant-file")
        if (Test-Path $env:SSL_CERT_FILE) {
            Remove-Item -Path $env:SSL_CERT_FILE -Force
        }

        It "Should not copy the file into the studio" {
            $result = hab studio run '(Test-Path $env:SSL_CERT_FILE).ToString()'
            $result[-1] | Should -Be "False"
        }

        It "Can still search packages on builder" {
            $result = hab studio run "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }
}
