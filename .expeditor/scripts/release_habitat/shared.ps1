. $PSScriptRoot\..\shared.ps1

function Install-BuildkiteAgent() {
    # Though the Windows machine we're running on has to have the
    # buildkite-agent installed, by definition, if you need to use the
    # buildkite-agent inside a container running on that host (e.g., to
    # do artifact uploads, or to manipulate pipeline metadata), then
    # you'll need to install it in the container as well.
    Write-Host "--- Installing buildkite agent in container"
    # note that on 12/12/2023, the below script was broken by https://github.com/buildkite/agent/commit/8833bca9a204971218fad9baa2f9c26336eb8ce9
    # so we grab the previous commit for now until this is fixed.
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/buildkite/agent/dec511dfac662158fde6edab4cf8600fc22f2edd/install.ps1')) | Out-Null
}

function Install-LatestHabitat() {
    # Install latest hab from using install.ps1
    $env:HAB_LICENSE = "accept-no-persist"
    Write-Host "--- :habicat: Installing latest hab binary for $Env:HAB_PACKAGE_TARGET using install.ps1"
    Set-ExecutionPolicy Bypass -Scope Process -Force
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.ps1')) | Out-Null
    $baseHabExe="$Env:ProgramData\Habitat\hab.exe"

    $HabVersion=GetLatestPkgVersionFromChannel("hab")
    $StudioVersion=GetLatestPkgVersionFromChannel("hab-studio")

    if((-not [string]::IsNullOrEmpty($HabVersion)) -and `
        (-not [string]::IsNullOrEmpty($StudioVersion)) -and `
        ($HabVersion -eq $StudioVersion)) {

        Write-Host "-- Hab and studio versions match! Found hab: $HabVersion - studio: $StudioVersion. Upgrading :awesome:"
        Invoke-Expression "$baseHabExe pkg install chef/hab --binlink --force --channel $Env:HAB_BLDR_CHANNEL" | Out-Null
        Invoke-Expression "$baseHabExe pkg install chef/hab-studio --binlink --force --channel $Env:HAB_BLDR_CHANNEL" | Out-Null
        # This is weird. Why does binlinking go here but the install.ps1 go to ProgramData?
        $baseHabExe="C:\hab\bin\hab"
    } else {
        Write-Host "-- Hab and studio versions did not match. hab: $HabVersion - studio: $StudioVersion"
    }
    $baseHabExe
}

function GetLatestPkgVersionFromChannel($PackageName) {
    if($PackageName.Equals("")) {
        Write-Error "--- :error: Package name required"
    }
    $ReleaseChannel="habitat-release-$Env:BUILDKITE_BUILD_ID"
    try {
        $version=(Invoke-WebRequest "$Env:HAB_BLDR_URL/v1/depot/channels/core/$ReleaseChannel/pkgs/$PackageName/latest?target=$Env:BUILD_PKG_TARGET" -UseBasicParsing).Content | jq -r '.ident | .version'
        Write-Host "Found version of ${PackageName} - $version"
    } catch {
        Write-Host "No version found for $PackageName"
        Write-Host $_.ScriptStackTrace
    }
    $version
}

# Until we can reliably deal with packages that have the same
# identifier, but different target, we'll track the information in
# Buildkite metadata.
#
# Each time we put a package into our release channel, we'll record
# what target it was built for.
#
# The corresponding Linux function accepts a target, but this one is
# only ever going to be called on Windows, so we'll just hard-code
# that.
#
# Note that there is no corresponding `IdentHasTarget` function
# because *that* can be called from Linux hosts, so there's no need
# for a Windows-only implementation.
function Set-TargetMetadata {
    [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseSingularNouns', '')]
    param($PackageIdent)

    Invoke-Expression "buildkite-agent meta-data set $PackageIdent-x86_64-windows true"
}


function Get-ReleaseChannel {
    "habitat-release-$Env:BUILDKITE_BUILD_ID"
}
