#!/usr/bin/env powershell

#Requires -Version 5

. $PSScriptRoot/shared.ps1

$ErrorActionPreference="stop"
$ReleaseChannel=Get-ReleaseChannel

Write-Host "--- Building the Windows Docker Studio"

& $PSScriptRoot/build_docker_image.ps1 -ReleaseChannel $ReleaseChannel -BldrUrl $Env:PIPELINE_HAB_BLDR_URL

Write-Host "--- Publishing the Windows Docker Studio"
. ./results/last_image.ps1

docker login -u="$env:DOCKER_LOGIN_USER" -p="$env:DOCKER_LOGIN_PASSWORD"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to login to docker hub"
}

try {
    Write-Host "Pushing ${docker_image}:$docker_base_image_tag-$docker_image_version"
    docker push "${docker_image}:$docker_base_image_tag-$docker_image_version"
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to upload image"
    }
    Write-Host "Pushing ${docker_image}:$docker_base_image_tag-$docker_image_short_version tag for $docker_image_version"
    docker push "${docker_image}:$docker_base_image_tag-$docker_image_short_version"
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to remotely tag image"
    }
    if($docker_base_image_tag -eq "ltsc2016") {
        Write-Host "Pushing latest tag for $docker_image_version"
        docker push "${docker_image}:latest"
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to remotely tag image"
        }
    }
}
finally {
    Remove-Item $HOME/.docker/config.json -Recurse -Force -ErrorAction SilentlyContinue
}

exit $LASTEXITCODE
