#!/usr/bin/env powershell

#Requires -Version 5

function is_fake_release() {
  buildkite-agent meta-data exists fake-release
  if($LastExitCode -eq 0) {
    return $true
  }
  return $false
}

$ErrorActionPreference="stop" 
$ReleaseChannel = & buildkite-agent meta-data get release-channel

Write-Host "--- Building the Windows Docker Studio"

& $PSScriptRoot/build_docker_image.ps1 -ReleaseChannel $ReleaseChannel

Write-Host "--- Publishing the Windows Docker Studio"
. ./results/last_image.ps1

docker login -u="$env:DOCKER_LOGIN_USER" -p="$env:DOCKER_LOGIN_PASSWORD"

if(is_fake_release) {
  Write-Host "This is a fake release."
  Write-Host "Not uploading $docker_image"
} else {
  try {
    Write-Host "Pushing ${docker_image}:$docker_base_image_tag-$docker_image_version"
    docker push "${docker_image}:$docker_base_image_tag-$docker_image_version"
    Write-Host "Pushing ${docker_image}:$docker_base_image_tag-$docker_image_short_version tag for $docker_image_version"
    docker push "${docker_image}:$docker_base_image_tag-$docker_image_short_version"
    if($docker_base_image_tag -eq "ltsc2016") {
      Write-Host "Pushing latest tag for $docker_image_version"
      docker push "${docker_image}:latest"
    }
  }
  finally {
      Remove-Item $HOME/.docker/config.json -Recurse -Force -ErrorAction SilentlyContinue
  }
}
