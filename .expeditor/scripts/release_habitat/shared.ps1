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
    # Bootstrap hab from the release channel unless the target requires a
    # channel that already contains a package for that architecture.
    $env:HAB_LICENSE = "accept-no-persist"
    $BuildChannel = $Env:HAB_BLDR_CHANNEL
    $BootstrapChannel = if ($Env:HAB_BOOTSTRAP_CHANNEL) { $Env:HAB_BOOTSTRAP_CHANNEL } else { "stable" }
    $HabitatPackageChannel = if ($Env:HAB_PACKAGE_TARGET -eq "aarch64-windows") { "unstable" } else { $BuildChannel }
    Write-Host "--- :habicat: Installing bootstrap hab for $Env:HAB_PACKAGE_TARGET from $BootstrapChannel"
    Install-Habitat -HabChannel $BootstrapChannel | Out-Null
    $Env:HAB_BLDR_CHANNEL = $BuildChannel
    $baseHabExe="C:\hab\bin\hab"

    $HabVersion = GetLatestPkgVersionFromChannel -PackageName "hab" -Channel $HabitatPackageChannel
    $StudioVersion = GetLatestPkgVersionFromChannel -PackageName "hab-studio" -Channel $HabitatPackageChannel

    if((-not [string]::IsNullOrEmpty($HabVersion)) -and `
        (-not [string]::IsNullOrEmpty($StudioVersion)) -and `
        ($HabVersion -eq $StudioVersion)) {

        Write-Host "-- Hab and studio versions match on $HabitatPackageChannel! Found hab: $HabVersion - studio: $StudioVersion. Upgrading :awesome:"
        Invoke-Expression "$baseHabExe pkg install chef/hab --binlink --force --channel $HabitatPackageChannel" | Out-Null
        Invoke-Expression "$baseHabExe pkg install chef/hab-studio --binlink --force --channel $HabitatPackageChannel" | Out-Null
        # This is weird. Why does binlinking go here but the install.ps1 go to ProgramData?
    } else {
        Write-Host "-- Hab and studio versions did not match on $HabitatPackageChannel. hab: $HabVersion - studio: $StudioVersion"
    }
    $baseHabExe
}

function GetLatestPkgVersionFromChannel {
    param(
        $PackageName,
        $Channel = $null
    )

    if($PackageName.Equals("")) {
        Write-Error "--- :error: Package name required"
    }
    $Channel = if ($Channel) {
        $Channel
    } elseif ($Env:HAB_BLDR_CHANNEL) {
        $Env:HAB_BLDR_CHANNEL
    } elseif ($Env:BUILDKITE_BUILD_ID) {
        "habitat-release-$Env:BUILDKITE_BUILD_ID"
    } else {
        throw "HAB_BLDR_CHANNEL or BUILDKITE_BUILD_ID must be set"
    }
    try {
        $response = Invoke-RestMethod "$Env:HAB_BLDR_URL/v1/depot/channels/chef/$Channel/pkgs/$PackageName/latest?target=$Env:BUILD_PKG_TARGET" -UseBasicParsing
        $version = $response.ident.version
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
# The target is recorded explicitly so the same package ident can be
# tracked separately for each Windows target.
#
# Note that there is no corresponding `IdentHasTarget` function
# because *that* can be called from Linux hosts, so there's no need
# for a Windows-only implementation.
function Set-TargetMetadata {
    [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseSingularNouns', '')]
    param(
        $PackageIdent,
        $Target = $Env:BUILD_PKG_TARGET
    )

    if ([string]::IsNullOrWhiteSpace($Target)) {
        throw "Target is required when setting Buildkite package metadata"
    }

    Invoke-Expression "buildkite-agent meta-data set $PackageIdent-$Target true"
}


function Get-ReleaseChannel {
    "habitat-release-$Env:BUILDKITE_BUILD_ID"
}
