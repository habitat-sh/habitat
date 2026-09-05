<#
.SYNOPSIS
    Reproduces building `components/hab` and using the raw `hab-plan-build.ps1`
    script to build the `chef/hab-plan-build-ps1` package as a signed .hart,
    entirely outside of a Habitat Studio (needed because there is currently no
    windows-aarch64 studio package ecosystem).

.NOTES
    - Run this script from an elevated (Administrator) PowerShell.
    - Designed for a clean Windows ARM64 machine. Adjust `-arch`/target values
      below if running on x86_64 (use "x64" for vcvarsall and "x86_64-windows"
      for BUILD_PKG_TARGET, and skip the ARM64 VC/Clang components).
    - Idempotent: safe to re-run; each step checks whether it's already done.
#>

[CmdletBinding()]
param(
    [string]$RepoRoot = "C:\Users\azureuser\habitat",
    [string]$VsInstallPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools",
    [string]$VcVarsArch = "arm64",              # use "x64" on x86_64 hosts
    [string]$BuildPkgTarget = "aarch64-windows" # use "x86_64-windows" on x86_64 hosts
)

$ErrorActionPreference = "Stop"

function Write-Step($msg) {
    Write-Host "`n=== $msg ===" -ForegroundColor Cyan
}

# ------------------------------------------------------------------------
# 1. Install Rust (rustup) - pinned toolchain comes from repo's rust-toolchain
# ------------------------------------------------------------------------
Write-Step "Installing rustup"
if (-not (Get-Command rustup -ErrorAction SilentlyContinue) -and
    -not (Test-Path "$env:USERPROFILE\.cargo\bin\rustup.exe")) {
    winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements --silent
} else {
    Write-Host "rustup already installed, skipping"
}
$cargoExe = "$env:USERPROFILE\.cargo\bin\cargo.exe"

# ------------------------------------------------------------------------
# 2. Install VS Build Tools 2022: MSVC (host arch) + Clang/LLVM toolset
#    (Clang is required because aws-lc-sys ships GNU-style .S assembly files
#    that MSVC's cl.exe cannot assemble; clang-cl handles them.)
# ------------------------------------------------------------------------
Write-Step "Installing VS Build Tools 2022 (MSVC + Clang)"
if (-not (Test-Path $VsInstallPath)) {
    winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
        --accept-source-agreements --accept-package-agreements --silent `
        --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.ARM64 --add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset --includeRecommended"
} else {
    Write-Host "VS Build Tools already installed at $VsInstallPath, ensuring required components are present"
    $installer = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\setup.exe"
    $argStr = "--quiet --norestart modify --installPath `"$VsInstallPath`" " +
              "--add Microsoft.VisualStudio.Workload.VCTools " +
              "--add Microsoft.VisualStudio.Component.VC.Tools.ARM64 " +
              "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang " +
              "--add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset"
    Start-Process -FilePath $installer -ArgumentList $argStr -Wait -PassThru | Out-Null
}
$vcvarsall = Join-Path $VsInstallPath "VC\Auxiliary\Build\vcvarsall.bat"
if (-not (Test-Path $vcvarsall)) { throw "vcvarsall.bat not found at $vcvarsall - VS Build Tools install failed" }

# ------------------------------------------------------------------------
# 3. Install protoc (protobuf compiler) - required by habitat-sup-protocol
# ------------------------------------------------------------------------
Write-Step "Installing protoc"
$protocDir = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\Google.Protobuf_*" -Directory -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $protocDir) {
    winget install --id Google.Protobuf -e --accept-source-agreements --accept-package-agreements --silent
    $protocDir = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\Google.Protobuf_*" -Directory | Select-Object -First 1
} else {
    Write-Host "protoc already installed, skipping"
}
$protocExe = (Get-ChildItem "$($protocDir.FullName)\bin\protoc.exe").FullName

# ------------------------------------------------------------------------
# 4. Install 7-Zip - required by hab-plan-build.ps1 to create tar/xz artifacts
# ------------------------------------------------------------------------
Write-Step "Installing 7-Zip"
$sevenZipExe = "C:\Program Files\7-Zip\7z.exe"
if (-not (Test-Path $sevenZipExe)) {
    winget install --id 7zip.7zip -e --accept-source-agreements --accept-package-agreements --silent
} else {
    Write-Host "7-Zip already installed, skipping"
}
if (-not (Test-Path $sevenZipExe)) { throw "7z.exe not found after install" }

# ------------------------------------------------------------------------
# 5. Apply source fix: allow the `studio` subcommand to compile on
#    windows+aarch64 (it was previously gated to windows+x86_64 only).
#    Skip this if already applied/merged in your checkout.
# ------------------------------------------------------------------------
Write-Step "Patching cli_v4 studio cfg gates for windows+aarch64 (idempotent)"
$filesToPatch = @(
    "$RepoRoot\components\hab\src\cli_v4.rs",
    "$RepoRoot\components\hab\src\cli_v4\studio.rs"
)
foreach ($f in $filesToPatch) {
    $content = Get-Content -Raw $f
    $old = 'all(target_os = "windows", target_arch = "x86_64")'
    $new = 'all(target_os = "windows", any(target_arch = "x86_64", target_arch = "aarch64"))'
    if ($content -match [regex]::Escape($old)) {
        $content = $content.Replace($old, $new)
        Set-Content -Path $f -Value $content -NoNewline
        Write-Host "Patched $f"
    } else {
        Write-Host "$f already patched or pattern not found, skipping"
    }
}

# ------------------------------------------------------------------------
# 6. Build hab.exe
# ------------------------------------------------------------------------
Write-Step "Building components/hab"
$habExe = "$RepoRoot\target\debug\hab.exe"
$buildLog = "$env:TEMP\hab_build_log.txt"
$buildCmd = "call `"$vcvarsall`" $VcVarsArch >nul 2>&1 & " +
            "set PROTOC=$protocExe & " +
            "cd /d `"$RepoRoot`" & " +
            "`"$cargoExe`" build -p hab > `"$buildLog`" 2>&1"
& $env:ComSpec /c $buildCmd
if ($LASTEXITCODE -ne 0) {
    Get-Content $buildLog -Tail 80
    throw "cargo build -p hab failed (see $buildLog for full output)"
}
if (-not (Test-Path $habExe)) { throw "hab.exe not found at $habExe after build" }
Write-Host "Built $habExe"
& $habExe --version

# ------------------------------------------------------------------------
# 7. Generate the 'chef' origin signing key (matches plan.ps1's $pkg_origin)
# ------------------------------------------------------------------------
Write-Step "Generating chef origin signing key"
$env:HAB_LICENSE = "accept-no-persist"
$existingKeys = Get-ChildItem "C:\hab\cache\keys\chef-*.sig.key" -ErrorAction SilentlyContinue
if (-not $existingKeys) {
    & $habExe origin key generate chef
} else {
    Write-Host "chef origin key already present, skipping"
}

# ------------------------------------------------------------------------
# 8. Run hab-plan-build.ps1 directly against the plan-build-ps1 plan
#    (no studio involved)
# ------------------------------------------------------------------------
Write-Step "Building chef/hab-plan-build-ps1 via hab-plan-build.ps1 (no studio)"
$env:Path += ";$(Split-Path $habExe);C:\Program Files\7-Zip"
$env:BUILD_PKG_TARGET = $BuildPkgTarget

Push-Location $RepoRoot
try {
    pwsh -NoProfile -File ".\components\plan-build-ps1\bin\hab-plan-build.ps1" `
        -Context ".\components\plan-build-ps1"
} finally {
    Pop-Location
}

Write-Step "Done"
Get-ChildItem "$RepoRoot\results\*.hart" | Format-List Name, FullName, Length
