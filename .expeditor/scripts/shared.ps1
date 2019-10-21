function Get-RustfmtToolchain {
  # It turns out that every nightly version of rustfmt has slight tweaks from the previous version.
  # This means that if we're always using the latest version, then we're going to have enormous
  # churn. Even PRs that don't touch rust code will likely fail CI, since master will have been
  # formatted with a different version than is running in CI. Because of this, we're going to pin
  # the version of nightly that's used to run rustfmt and bump it when we do a new release.
  #
  # Note that not every nightly version of rust includes rustfmt. Sometimes changes are made that
  # break the way rustfmt uses rustc. Therefore, before updating the pin below, double check
  # that the nightly version you're going to update it to includes rustfmt. You can do that
  # using https://mexus.github.io/rustup-components-history/x86_64-unknown-linux-gnu.html
  Get-Content "$PSScriptRoot\..\..\RUSTFMT_VERSION"
}

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
      ./rustup-init.exe -y --default-toolchain $toolchain-x86_64-pc-windows-msvc --no-modify-path --profile=minimal
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

function Wait-Supervisor($Timeout=1) {
  Write-Host "Waiting up to $Timeout seconds for Supervisor to start..."
  $startTime = [DateTime]::Now
  $success = (Test-NetConnection -ComputerName localhost -Port 9631).TcpTestSucceeded
  while(!$success) {
      Start-Sleep -Seconds 1
      $timeTaken = [DateTime]::Now.Subtract($startTime)
      if($timeTaken.TotalSeconds -ge $Timeout) {
          Write-Error "Timed out waiting $Timeout seconds for Supervisor to start"
          break
      }
      $success = (Test-NetConnection -ComputerName localhost -Port 9631).TcpTestSucceeded
  }
  if($success) { Write-Host "Supervisor is now running." }
}

# On buildkite, the rust binaries will be directly in C:
if($env:BUILDKITE) {
  # this will avoid a path length limit from the long buildkite working dir path
  $env:CARGO_TARGET_DIR = "c:\target"
}
