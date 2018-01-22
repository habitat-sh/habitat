$ErrorActionPreference = "Stop"

function info($message) {
    Write-Host -ForegroundColor Cyan -NoNewline "build-docker-image: "
    Write-Host $message
}

function Assert-Docker {
  if ((get-command docker -ErrorAction SilentlyContinue) -eq $null) {
      Write-Error "We require docker to push the image"
  }
}

function Push-Image {
    & "$PSScriptRoot\build-docker-image.ps1"
    if (!(Test-Path ./results/last_image.ps1)) {
      Write-error "Image build report ./results/last_image.ps1 missing, aborting"
  }
  . ./results/last_image.ps1

  try {
    info "Logging in to Bintray Docker repo"
    docker login -u="$env:BINTRAY_USER" -p="$env:BINTRAY_KEY" habitat-docker-registry.bintray.io
    info "Pushing ${docker_image}:$docker_image_version"
    docker push "${docker_image}:$docker_image_version"
    info "Pushing ${docker_image}:$docker_image_short_version tag for $docker_image_version"
    docker push "${docker_image}:$docker_image_short_version"
    info "Pushing latest tag for $docker_image_version"
    docker push "${docker_image}:latest"
  }
  finally {
      Remove-Item $HOME/.docker/config.json -Recurse -Force -ErrorAction SilentlyContinue
  }

  info
  info "Docker Image: docker pull $docker_image"
  info
}

if($env:BINTRAY_USER -eq $null) {
    Write-Error "Required environment variable: BINTRAY_USER"
}
if($env:BINTRAY_KEY -eq $null) {
    Write-Error "Required environment variable: BINTRAY_KEY"
}

Assert-Docker
Push-Image
