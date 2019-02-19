#!/usr/bin/env powershell

#Requires -Version 5

param (
  # The builder channel to install packages from. Defaults to unstable
  [string]$ReleaseChannel="unstable",
  # The docker image name. Defaults to "habitat-docker-registry.bintray.io/win-studio-x86_64-windows
  [string]$imageName = "habitat-docker-registry.bintray.io/win-studio-x86_64-windows"
)

$ErrorActionPreference = "Stop"

$startDir="$pwd"
$tmpRoot = mkdir (Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName()))
Push-Location $tmpRoot
try {
    $env:FS_ROOT="$tmpRoot/rootfs"
    # Ensure that no existing `HAB_BINLINK_DIR` environment variable is present,
    # like it would if executed in a Studio instance.
    $env:HAB_BINLINK_DIR = $null
    
    Write-Host "Installing and extracting initial Habitat packages"
    $InstallHarts = @(
        "core/hab-studio",
        "core/hab-sup",
        "core/windows-service"
    )
    $InstallHarts | % {
        Invoke-Expression "hab pkg install $_ --channel=$ReleaseChannel"
        if ($LASTEXITCODE -ne 0) {
            Write-Error "hab install failed for $_, aborting"
        }
    }
    $studioPath = hab pkg path core/hab-studio
    if ($LASTEXITCODE -ne 0) {
      Write-Error "core/hab-studio must be installed, aborting"
    }
    
    Write-Host "Purging container hab cache"
    Remove-Item "$env:FS_ROOT/hab/cache" -Recurse -Force
    
    $pathParts = $studioPath.Replace("\", "/").Split("/")
    $ident = [String]::Join("/", $pathParts[-4..-1])
    $shortVersion = $pathParts[-2]
    $version = "$($pathParts[-2])-$($pathParts[-1])"
    
@"
FROM microsoft/windowsservercore
MAINTAINER The Habitat Maintainers <humans@habitat.sh>
ADD rootfs /
WORKDIR /src
RUN /hab/pkgs/$ident/bin/hab/hab.exe pkg exec core/windows-service install
ENTRYPOINT ["/hab/pkgs/$ident/bin/powershell/pwsh.exe", "-ExecutionPolicy", "bypass", "-NoLogo", "-file", "/hab/pkgs/$ident/bin/hab-studio.ps1"]
"@ | Out-File "$tmpRoot/Dockerfile" -Encoding ascii
    
Write-Host "Building Docker image ${imageName}:$version'"
docker build --no-cache -t ${imageName}:$version .
if ($LASTEXITCODE -ne 0) {
    Write-Error "docker build failed, aborting"
}

Write-Host "Tagging latest image to ${imageName}:$version"
docker tag ${imageName}:$version ${imageName}:latest

Write-Host "Tagging latest image to ${imageName}:$shortVersion"
docker tag ${imageName}:$version ${imageName}:$shortVersion

# Ensure the results directory exists before writing to it
New-Item -ItemType directory -Path "$startDir/results" -Force
@"
`$docker_image="$imageName"
`$docker_image_version="$version"
`$docker_image_short_version="$shortVersion"
"@ | Out-File "$startDir/results/last_image.ps1" -Encoding ascii
    
    Write-Host "Docker Image: ${imageName}:$version"
    Write-Host "Build Report: $startDir/results/last_image.ps1"
}
finally {
    Pop-Location
    $env:FS_ROOT = $null
    Remove-Item $tmpRoot -Recurse -Force
}
