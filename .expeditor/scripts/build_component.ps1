#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop" 

# Import shared functions
. ".buildkite\scripts\shared.ps1"

if($Component.Equals("")) {
    Write-Error "--- :error: Component to build not specified, please use the -Component flag"
}

# install buildkite agent because we are in a container :(
Write-Host "--- Installing buildkite agent in container"
$Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
Invoke-Expression (Invoke-WebRequest https://raw.githubusercontent.com/buildkite/agent/master/install.ps1 -UseBasicParsing).Content

# We have to do this because everything that comes from vault is quoted on windows.
$Rawtoken=$Env:ACCEPTANCE_HAB_AUTH_TOKEN
$Env:HAB_AUTH_TOKEN=$Rawtoken.Replace("`"","")

$Env:HAB_BLDR_URL=$Env:ACCEPTANCE_HAB_BLDR_URL
$Env:HAB_PACKAGE_TARGET=$Env:BUILD_PKG_TARGET

# For viewability
$Channel = "habitat-release-$Env:BUILDKITE_BUILD_ID"
Write-Host "--- Channel: $Channel - bldr url: $Env:HAB_BLDR_URL"

# Get the latest version available from bintray -  we want the latest stable version to start with
$current_protocols = [Net.ServicePointManager]::SecurityProtocol
$latestVersionURI = ""
$downloadUrl = ""
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $response = Invoke-WebRequest "https://bintray.com/habitat/stable/hab-x86_64-windows/_latestVersion" -UseBasicParsing -ErrorAction Stop
    $latestVersionURI = ($response).BaseResponse.ResponseUri.AbsoluteUri
}
finally {
    [Net.ServicePointManager]::SecurityProtocol = $current_protocols
}
  
$uriArray = $latestVersionURI.Split("/")
$targetVersion = $uriArray[$uriArray.Length-1]
Write-Host "--- Latest version is $targetVersion"
$downloadUrl = "https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-$targetVersion-x86_64-windows.zip?bt_package=hab-x86_64-windows"
# We want a short-ish path to install our latest hab
$bootstrapDir = "C:\hab-latest"

# Download the binary now
Write-Host "--- Downloading from $downloadUrl"
$current_protocols = [Net.ServicePointManager]::SecurityProtocol
try {
  [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
  Invoke-WebRequest -UseBasicParsing -Uri "$downloadUrl" -OutFile hab.zip
}
finally {
  [Net.ServicePointManager]::SecurityProtocol = $current_protocols
}

Write-Host "--- Extracting to $bootstrapDir"
New-Item -ItemType directory -Path $bootstrapDir -Force
Expand-Archive -Path hab.zip -DestinationPath $bootstrapDir
Remove-Item hab.zip -Force
$baseHabExe = (Get-Item "$bootstrapDir\hab-$targetVersion-x86_64-windows\hab.exe").FullName

# Accept license
Invoke-Expression "$baseHabExe license accept"

# Get keys
Write-Host "--- :key: Downloading 'core' public keys from Builder"
Invoke-Expression "$baseHabExe origin key download core"
Write-Host "--- :closed_lock_with_key: Downloading latest 'core' secret key from Builder"
Invoke-Expression "$baseHabExe origin key download core --auth $Env:HAB_AUTH_TOKEN --secret"
$Env:HAB_CACHE_KEY_PATH = "C:\hab\cache\keys"
$Env:HAB_ORIGIN = "core"

# Run a build!
Write-Host "--- Setting HAB_BLDR_CHANNEL channel to $Channel"
$Env:HAB_BLDR_CHANNEL="$Channel"
Write-Host "--- Running hab pkg build for $Component"
Invoke-Expression "$baseHabExe pkg build components\$Component --keys core"
. results\last_build.ps1

Write-Host "--- Running hab pkg upload for $Component to channel $Channel"
Invoke-Expression "$baseHabExe pkg upload results\$pkg_artifact --channel=$Channel"
Write-Host "--- Running hab pkg promote for $pkg_ident to channel $Channel"
Invoke-Expression "$baseHabExe pkg promote $pkg_ident $Channel $Env:BUILD_PKG_TARGET"
Invoke-Expression "buildkite-agent meta-data set $pkg_ident-x86_64-windows true"

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

exit $LASTEXITCODE
