# This endures that given a 'hab' user, a plan with no pkg_svc_user will
# not produce a service that will be run by the hab user. This used to be
# the case but we are no longer using the hab account by default on windows.
# see https://github.com/habitat-sh/habitat/issues/6847
$username = "hab"
$password = "Pass@word1"
net user /add $username $password
net localgroup administrators $username /add
Add-Type -TypeDefinition (Get-Content ".expeditor\scripts\end_to_end\LsaWrapper.cs" | Out-String)
$lsa_wrapper = New-Object -type LsaWrapper
$lsa_wrapper.SetRight($username, "SeServiceLogonRight")

$env:HAB_ORIGIN = "ci"
hab origin key generate ci

hab pkg install core/windows-service
Start-Service Habitat
Wait-Supervisor -Timeout 45

Describe "with no svc_user" {
    hab pkg build test/fixtures/windows_plans/dummy
    . .\results\last_build.ps1
    hab pkg install .\results\$pkg_artifact
    $loadOut = hab svc load ci/dummy
    Start-Sleep -Seconds 5

    It "does not create a SVC_USR metafile" {
        Test-Path c:\hab\pkgs\$pkg_ident\SVC_USER | Should -Be $false
    }
    It "Succesfully loads service" {
        $loadOut | Should -Be "The ci/dummy service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/dummy/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "runs hook as current user" {
        # the dummy run hook simply runs ping continuously
        $proc = Get-Process ping -IncludeUserName
        $proc.UserName | Should -Be "NT AUTHORITY\SYSTEM"
    }
    AfterAll {
        hab svc unload ci/dummy
        Start-Sleep -Seconds 5 # ping needs to be forcefully shutdown
    }
}

Describe "with svc_user" {
    hab pkg build test/fixtures/windows_plans/dummy_hab_svc_user
    . .\results\last_build.ps1
    hab pkg install .\results\$pkg_artifact
    $loadOut = hab svc load ci/dummy-hab-user --password $password
    Start-Sleep -Seconds 5

    It "does create a SVC_USR metafile" {
        Test-Path c:\hab\pkgs\$pkg_ident\SVC_USER | Should -Be $true
    }
    It "Succesfully loads service" {
        $loadOut | Should -Be "The ci/dummy-hab-user service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/dummy-hab-user/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "runs hook as current user" {
        # the dummy run hook simply runs ping continuously
        $proc = Get-Process ping -IncludeUserName
        $proc.UserName | Should -Be "$env:computername\hab"
    }
}
