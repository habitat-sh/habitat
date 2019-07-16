# We assume the BUILDKITE_AGENT_ACCESS_TOKEN is set
function Install-BuildkiteAgent() {
  # install buildkite agent because we are in a container :(
  Write-Host "--- Installing buildkite agent in container"
  $Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
  # We have to do this because everything that comes from vault is quoted on windows.
  # TODO: This can be removed when we go live!
  $Rawtoken=$Env:ACCEPTANCE_HAB_AUTH_TOKEN
  $Env:HAB_AUTH_TOKEN=$Rawtoken.Replace("`"","")
  iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/buildkite/agent/master/install.ps1')) | Out-Null
}

function Install-LatestHabitat() {
  # Install latest hab from using install.ps1
  $env:HAB_LICENSE = "accept-no-persist"
  Write-Host "--- :habicat: Installing latest hab binary for $Env:HAB_PACKAGE_TARGET using install.ps1"
  Set-ExecutionPolicy Bypass -Scope Process -Force
  iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.ps1')) | Out-Null
  $baseHabExe="$Env:ProgramData\Habitat\hab.exe"

  $HabVersion=GetLatestPkgVersionFromChannel("hab")
  $StudioVersion=GetLatestPkgVersionFromChannel("hab-studio")

  if((-not [string]::IsNullOrEmpty($HabVersion)) -and `
     (-not [string]::IsNullOrEmpty($StudioVersion)) -and `
     ($HabVersion -eq $StudioVersion)) {
    
    Write-Host "-- Hab and studio versions match! Found hab: $HabVersion - studio: $StudioVersion. Upgrading :awesome:"
    Invoke-Expression "$baseHabExe pkg install core/hab --binlink --force --channel $Env:HAB_BLDR_CHANNEL" | Out-Null
    Invoke-Expression "$baseHabExe pkg install core/hab-studio --binlink --force --channel $Env:HAB_BLDR_CHANNEL" | Out-Null
    # This is weird. Why does binlinking go here but the install.ps1 go to ProgramData?
    $baseHabExe="C:\hab\bin\hab" 
    $NewVersion=Invoke-Expression "$baseHabExe --version"
  }
  else {
    write-host "-- Hab and studio versions did not match. hab: $HabVersion - studio: $StudioVersion"
  }
  $baseHabExe
}

function GetLatestPkgVersionFromChannel($PackageName) {
  if($PackageName.Equals("")) {
    Write-Error "--- :error: Package name required"
  }
  $ReleaseChannel="habitat-release-$Env:BUILDKITE_BUILD_ID"
  try {
    $version=(Invoke-Webrequest -UseBasicParsing "$Env:HAB_BLDR_URL/v1/depot/channels/core/$ReleaseChannel/pkgs/$PackageName/latest?target=$Env:BUILD_PKG_TARGET").Content | jq -r '.ident | .version'
    Write-Host "Found version of ${PackageName} - $version"
  }
  catch {
    Write-Host "No version found for $PackageName"
    Write-Host $_.ScriptStackTrace
  }
  $version
}