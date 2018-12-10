#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The base hab version to run the build with. Defaults to latest
    [string]$BaseHabVersion="latest",
    # The builder channel to pull from. Defaults to stable
    [string]$SourceChannel="stable",
    # The bintray channel/repo to push to. Defaults to unstable
    [string]$TargetChannel="unstable"
)

# Import shared functions
. "$PSScriptRoot\shared.ps1" -ErrorAction Stop

Write-Host "--- Setting source package channel to $SourceChannel"
$Env:HAB_BLDR_CHANNEL="$SourceChannel"

Write-Host "--- Installing base habitat binary version: $BaseHabVersion"
$baseHabExe = [HabShared]::install_base_habitat_binary($BaseHabVersion, $SourceChannel)
Write-Host "--- Using hab executable at $baseHabExe"

# install buildkite agent because we are in a container :(
Write-Host "--- Installing buildkite agent in container"
$Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
Invoke-Expression (Invoke-WebRequest https://raw.githubusercontent.com/buildkite/agent/master/install.ps1).Content

$HabArtifact = & buildkite-agent meta-data get hab-artifact-windows --job $Env:BUILDKITE_JOB_ID

Write-Host "--- :windows: Install core/hab-bintray-publish package"
Invoke-Expression "$baseHabExe pkg install --channel=$SourceChannel core/hab-bintray-publish"

Write-Host "--- :windows: Install core/hab package"
Invoke-Expression "$baseHabExe pkg install --channel=$SourceChannel core/hab"

Write-Host "--- :habicat: :windows: Uploading core/hab to Bintray"
$Env:HAB_BLDR_CHANNEL="$SourceChannel"
$Env:BINTRAY_USER="$Env:HABITAT_BINTRAY_USER"
$Env:BINTRAY_KEY="$Env:HABITAT_BINTRAY_KEY"
$Env:BINTRAY_PASSPHRASE="$Env:HABITAT_BINTRAY_PASSPHRASE"
Invoke-Expression "$baseHabExe pkg exec core/hab-bintray-publish publish-hab -s -r $TargetChannel C:\hab\cache\artifacts\$HabArtifact"


if($LASTEXITCODE -ne 0) { Write-Error "Something mysterious and unexpected happened!" } 