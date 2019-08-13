function Install-Habitat {
  if (get-command -Name hab -ErrorAction SilentlyContinue) {
      Write-Host "Using habitat version:`n$(hab --version)"
  } else {
      ."$PSScriptRoot\..\..\components\hab\install.ps1"
  }
}

function Get-Toolchain {
  "$(Get-Content $PSScriptRoot\..\..\rust-toolchain)"
}

function New-PathString([string]$StartingPath, [string]$Path) {
  if (-not [string]::IsNullOrEmpty($path)) {
      if (-not [string]::IsNullOrEmpty($StartingPath)) {
          [string[]]$PathCollection = "$path;$StartingPath" -split ';'
          $Path = ($PathCollection |
              Select-Object -Unique |
              Where-Object {-not [string]::IsNullOrEmpty($_.trim())} |
              Where-Object {test-path "$_"}
          ) -join ';'
      }
      $path
  }
  else {
      $StartingPath
  }
}

function Install-Rustup($Toolchain) {
  if(Test-Path $env:USERPROFILE\.cargo\bin) {
      $env:path = New-PathString -StartingPath $env:path -Path "$env:USERPROFILE\.cargo\bin"
  }

  if (get-command -Name rustup.exe -ErrorAction SilentlyContinue) {
      Write-Host "rustup is currently installed"
      rustup set default-host x86_64-pc-windows-msvc
      rustup default stable-x86_64-pc-windows-msvc
  } else {
      Write-Host "Installing rustup and $toolchain-x86_64-pc-windows-msvc Rust."
      [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
      invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
      ./rustup-init.exe -y --default-toolchain $toolchain-x86_64-pc-windows-msvc --no-modify-path
      $env:path += ";$env:USERPROFILE\.cargo\bin"
  }
}

function Install-RustToolchain($Toolchain) {
  rustup component list --toolchain $toolchain | Out-Null
  if ($LASTEXITCODE -ne 0) {
      Write-Host "Installing rust toolchain $toolchain"
      rustup toolchain install $toolchain
  } else {
      Write-Host "Rust toolchain $toolchain is already installed"
  }
}

# On buildkite, the rust binaries will be directly in C:
if($env:BUILDKITE) {
  # this will avoid a path length limit from the long buildkite working dir path
  $env:CARGO_TARGET_DIR = "c:\target"
}