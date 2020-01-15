# Test that SSL_CERT_FILE is persisted into the studio and 
# set to the correct internal path. 
$ErrorActionPreference="stop" 

function Cleanup-CachedCertificate {
  $hab_ssl_cache="/hab/cache/ssl"
  Remove-Item -Force "$hab_ssl_cache/*" -ErrorAction SilentlyContinue
}

hab origin key generate "$env:HAB_ORIGIN"

$tempdir = New-TemporaryDirectory
$e2e_certname = "e2e-ssl.pem"
hab pkg install core/openssl 
hab pkg exec core/openssl openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out (Join-Path $tempdir $e2e_certname)

if($IsLinux) {
    $sslCertFileCheck = "test -f `$SSL_CERT_FILE"
    $sslCertFilePrint = "echo `$SSL_CERT_FILE"
    $sslCacheCertFileCheck = "test -f '/hab/cache/ssl/$e2e_certname'"
    $sslCertFileNotSetCheck = "test ! -v SSL_CERT_FILE"
} else {
    $sslCertFileCheck = "exit (!(Test-Path `$env:SSL_CERT_FILE))"
    $sslCertFilePrint = "`$env:SSL_CERT_FILE.Replace('\','/')"
    $sslCacheCertFileCheck = "exit (!(Test-Path '/hab/cache/ssl/$e2e_certname'))"
    $sslCertFileNotSetCheck = "exit (Test-Path `$env:SSL_CERT_FILE)"
}

Context "SSL_CERT_FILE is passed into the studio" {
    BeforeEach { 
        hab studio rm
        Cleanup-CachedCertificate
    }

    Describe "SSL_CERT_FILE is a valid certificate" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir $e2e_certname)
        It "Sets env:SSL_CERT_FILE in the studio"  {
            $expected = "/hab/cache/ssl/$e2e_certname"
            $result = Invoke-StudioRun $sslCertFilePrint
            $result[-1] | Should -BeLike "*$expected"
        }

        It "Copies the certificate described by SSL_CERT_FILE into the studio" {
            Invoke-StudioRun $sslCertFileCheck
            $LASTEXITCODE | Should -Be 0
        }

        It "Can search builder for packages when SSL_CERT_FILE is set" {
            $result = Invoke-StudioRun "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "Custom SSL cert" {
        if($IsLinux) {
            sudo cp $(Join-Path $tempdir $e2e_certname) /hab/cache/ssl/
        } else {
            Copy-Item (Join-Path $tempdir $e2e_certname) /hab/cache/ssl/
        }

        It "is available in studio" {
            Invoke-StudioRun $sslCacheCertFileCheck
            $LASTEXITCODE | Should -Be 0
        }
    }

    Describe "SSL_CERT_FILE is an invalid certificate" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "invalid_certificate")
        Set-Content -Path $env:SSL_CERT_FILE -Value "I am not a certificate"

        It "Can still search packages on builder" {
            $result = Invoke-StudioRun "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is an invalid cached certificate" {
        if($IsLinux) {
            sudo sh -c 'echo "I am not a certificate" > /hab/cache/ssl/invalid-ssl-cert.pem'
        } else {
            Set-Content -Path /hab/cache/ssl/invalid-ssl-cert.pem -Value "I am not a certificate"
        }

        It "Can still search packages on builder" {
            $result = Invoke-StudioRun "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is a directory" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "cert-as-directory")
        New-Item -ItemType Directory -Force -Path $env:SSL_CERT_FILE

        It "Should not set SSL_CERT_FILE" {
            Invoke-StudioRun $sslCertFileNotSetCheck
            $LASTEXITCODE | Should -Be 0
        }

        It "Should not copy the directory into the studio" {
            if($isLinux) {
              Invoke-StudioRun "test -e /hab/cache/ssl/cert-as-directory"
            } else { 
              Invoke-StudioRun $sslCertFileCheck
            }

            $LASTEXITCODE | Should -Be 1
        }

        It "Can still search packages on builder" {
            $result = Invoke-StudioRun "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }

    Describe "SSL_CERT_FILE is a non-existant-file" {
        $env:SSL_CERT_FILE = (Join-Path $tempdir "non-existant-file")
        if (Test-Path $env:SSL_CERT_FILE) {
            Remove-Item -Path $env:SSL_CERT_FILE -Force
        }

        It "Should not copy the file into the studio" {
            if($isLinux) {
              Invoke-StudioRun "test -e /hab/cache/ssl/non-existant-file"
            } else { 
              Invoke-StudioRun $sslCertFileCheck
            }
            $LASTEXITCODE | Should -Be 1
        }

        It "Should not set SSL_CERT_FILE" {
            Invoke-StudioRun $sslCertFileNotSetCheck
            $LASTEXITCODE | Should -Be 0
        }

        It "Can still search packages on builder" {
            $result = Invoke-StudioRun "hab pkg search core/nginx"
            $result | Should -Contain "core/nginx"
        }
    }
}

Write-Host "--- Testing SSL_CERT_FILE is not set"

Context "SSL_CERT_FILE isn't set" {
    BeforeEach { 
        Cleanup-CachedCertificate
        # Ensure SSL_CERT_FILE isn't set
        Remove-Item Env:\SSL_CERT_FILE
        hab studio rm
        hab pkg uninstall core/hab-studio
    }

    Describe "Studio is auto-installed on first run" {
        It "Should not set SSL_CERT_FILE in the studio" {
            Invoke-StudioRun $sslCertFileNotSetCheck
            $LASTEXITCODE | Should -Be 0
        }
    }
}
