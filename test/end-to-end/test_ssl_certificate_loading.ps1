$ErrorActionPreference="stop"
Context "Habitat SSL Cache" {
    BeforeAll {
        $coreChannel = "base"
        Write-Host "--- Generating a signing key"
        Invoke-NativeCommand hab origin key generate "$env:HAB_ORIGIN"
    }

    BeforeEach {
        if (Test-Path c:\hab\cache\ssl) {
            Remove-Item c:\hab\cache\ssl -Recurse -Force
        }
    }

    Describe "Fresh Install" {
        # Note this isn't a pure fresh install, as we had to bootstrap this machine with
        # The previous release in order to test the current build.  However, the BeforeEach
        # block above ensures we don't have any cached certs that this may be pulling in.
        It "Can install packages" {
            hab pkg install core/7zip --channel $coreChannel
            $LASTEXITCODE | Should -Be 0
        }
    }

    Describe "Custom Certificates" {
        It "Can install packages when an invalid certificate is present" {
            New-Item -Type Directory -Path c:\hab\cache\ssl
            New-Item -Type File -Path c:\hab\cache\ssl\invalid-certifcate.pem
            Add-Content -Path c:\hab\cache\ssl\invalid-certificate.pem "I AM NOT A CERTIFICATE"

            # Resolve against Builder even if the earlier test installed 7zip.
            hab pkg install core/7zip --channel $coreChannel --ignore-local
            $LASTEXITCODE | Should -Be 0
        }

        It "Loads custom certificates" {
            New-Item -Type Directory -Path c:\hab\cache\ssl
            New-TestCertificate -Path c:\hab\cache\ssl\custom-certificate.pem

            $env:RUST_LOG="debug"
            Start-Process hab -ArgumentList "pkg search core/lessmsi" -RedirectStandardError err.log -Wait
            $env:RUST_LOG="info"
            Get-Content -Raw -Path err.log | Should -Match "Processing cert file: C:\\hab/cache/ssl\\custom-certificate.pem"
        }
    }

    Describe "Studio" {
        It "Makes custom certificates available in the studio" {
            $customCertFile = "c:\hab\cache\ssl\custom-certificate.pem"
            # We can use unix style pathing here to get the real root of the studio
            # That way we don't have to worry about the studio name if our
            # CWD ever changes from 'workdir'
            $studioCertFile = "/hab/cache/ssl/custom-certificate.pem"
            New-Item -Type Directory -Path c:\hab\cache\ssl
            New-TestCertificate -Path $customCertFile

            $result = hab studio run "(Test-Path $studioCertFile).ToString()"
            Write-Host $result
            $result[-1] | Should -Be "True"
        }
    }
}
