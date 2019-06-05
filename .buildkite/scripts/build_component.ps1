#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop" 

# Import shared functions
. "$PSScriptRoot\shared.ps1"

if($Component.Equals("")) {
    Write-Error "--- :error: Component to build not specified, please use the -Component flag"
}


# install buildkite agent because we are in a container :(
Write-Host "--- Installing buildkite agent in container"
$Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
Invoke-Expression (Invoke-WebRequest https://raw.githubusercontent.com/buildkite/agent/master/install.ps1 -UseBasicParsing).Content

Write-Host "--- Installing latest stable Habitat binary"
$baseHabExe = [HabShared]::install_base_habitat_binary("latest", "stable")
Write-Host "--- Using hab executable at $baseHabExe"

Write-Host "--- Importing Keys"
[HabShared]::import_keys($baseHabExe)

Write-Host "--- Moving build folder to new location"
New-Item -ItemType directory -Path C:\build
# Adding -Force to ensure we take the .git directory over as well
Copy-Item -Path C:\workdir\* -Destination C:\build -Recurse -Force

Push-Location "C:\build"
    $ReleaseChannel = & buildkite-agent meta-data get release-channel
    Write-Host "--- Setting HAB_BLDR_CHANNEL channel to $ReleaseChannel"
    $Env:HAB_BLDR_CHANNEL="$ReleaseChannel"
    Write-Host "--- Running hab pkg build for $Component"
    Invoke-Expression "$baseHabExe pkg build components\$Component --keys core"
    . "results\last_build.ps1"

    Write-Host "Running hab pkg upload for $Component to channel $ReleaseChannel"
    Invoke-Expression "$baseHabExe pkg upload results\$pkg_artifact --channel=$ReleaseChannel"
    Invoke-Expression "buildkite-agent meta-data set ${pkg_ident}-x86_64-windows true"



    If ($Component -eq 'hab') {
        Write-Host "--- :buildkite: Recording metadata $pkg_ident"
        Invoke-Expression "buildkite-agent meta-data set 'hab-version-x86_64-windows' '$pkg_ident'"
        Invoke-Expression "buildkite-agent meta-data set 'hab-release-x86_64-windows' '$pkg_release'"
        Invoke-Expression "buildkite-agent meta-data set 'hab-artifact-x86_64-windows' '$pkg_artifact'"
    } Elseif ($component -eq 'studio') {
        Write-Host "--- :buildkite: Recording metadata for $pkg_ident"
        Invoke-Expression "buildkite-agent meta-data set 'studio-version-x86_64-windows' $pkg_ident"       
    } Else {
        Write-Host "Not recording any metadata for $pkg_ident, none required."
    }
    Invoke-Expression "buildkite-agent annotate --append --context 'release-manifest' '<br>* ${pkg_ident} (x86_64-windows)'"
Pop-Location

exit $LASTEXITCODE
