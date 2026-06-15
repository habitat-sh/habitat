#!/usr/bin/env powershell
# grype_scan.ps1
#
# Installs the Habitat seed packages from the dev channel and runs a grype
# vulnerability scan on C:\hab, writing Critical and High findings to a text
# file that is saved as a Buildkite artifact.

#Requires -Version 5

$ErrorActionPreference = "Stop"

. $PSScriptRoot\shared.ps1

$env:HAB_BLDR_URL = $env:PIPELINE_HAB_BLDR_URL
$env:HAB_LICENSE  = "accept-no-persist"
$env:HAB_PACKAGE_TARGET = "x86_64-windows"

# ---------------------------------------------------------------------------
# Clear the Habitat root to ensure a clean install of only the dev packages
# ---------------------------------------------------------------------------
Write-Host "--- :broom: Clearing C:\hab"
if (Test-Path "C:\hab") { Remove-Item -Recurse -Force "C:\hab" }

# ---------------------------------------------------------------------------
# Install hab
# ---------------------------------------------------------------------------
Write-Host "--- :habicat: Installing hab"
Install-Habitat "dev"
$habExe = "C:\hab\bin\hab"

# ---------------------------------------------------------------------------
# Install grype
# ---------------------------------------------------------------------------
Write-Host "--- :mag: Installing grype"
$release    = (Invoke-RestMethod "https://api.github.com/repos/anchore/grype/releases/latest" -UseBasicParsing).tag_name
$version    = $release.TrimStart("v")
$grypeUrl   = "https://github.com/anchore/grype/releases/download/${release}/grype_${version}_windows_amd64.zip"
Write-Host "Downloading grype ${version}"
Invoke-WebRequest -Uri $grypeUrl -OutFile "grype.zip" -UseBasicParsing
Expand-Archive "grype.zip" -DestinationPath "grype-bin" -Force
$env:PATH = "$(Resolve-Path 'grype-bin')$([IO.Path]::PathSeparator)$env:PATH"
grype version

# ---------------------------------------------------------------------------
# Install seed packages from the dev channel
# ---------------------------------------------------------------------------
Write-Host "--- :habicat: Installing seed packages from the dev channel"
$seeds = @(
    "chef/hab",
    "chef/hab-sup",
    "chef/hab-launcher",
    "chef/hab-studio",
    "chef/hab-pkg-export-tar",
    "chef/hab-pkg-export-container",
    "chef/hab-plan-build-ps1",
    "chef/windows-service"
)
foreach ($pkg in $seeds) {
    Write-Host "  Installing $pkg..."
    & $habExe pkg install --channel dev $pkg
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  WARNING: $pkg unavailable for x86_64-windows, skipping"
    }
}

# ---------------------------------------------------------------------------
# Run grype scan and write Critical/High findings to the report file
# ---------------------------------------------------------------------------
$scanDir = "C:\hab"
$report  = "grype-scan-x86_64-windows.txt"
$rawFile = "grype-raw-x86_64-windows.json"

Write-Host "--- :shield: Scanning $scanDir for Critical and High vulnerabilities"
grype "dir:$scanDir" --output json --quiet | Out-File -FilePath $rawFile -Encoding utf8
$rawJson = Get-Content $rawFile -Raw | ConvertFrom-Json

$allFindings = @(
    $rawJson.matches | Where-Object {
        $_.vulnerability.severity -in @("Critical", "High")
    }
)
$critCount = @($allFindings | Where-Object { $_.vulnerability.severity -eq "Critical" }).Count
$highCount = @($allFindings | Where-Object { $_.vulnerability.severity -eq "High" }).Count

$lines = @(
    "Grype scan: x86_64-windows",
    "Directory:  $scanDir",
    "Date:       $([System.DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ'))",
    "Critical:   $critCount",
    "High:       $highCount",
    ""
)

if ($allFindings.Count -eq 0) {
    $lines += "No Critical or High vulnerabilities found."
} else {
    $lines += "{0,-10} {1,-22} {2,-35} {3,-18} {4,-20} {5}" -f "SEVERITY", "CVE", "PACKAGE", "VERSION", "FIX", "HAB_PKG_PATH"
    $lines += "{0,-10} {1,-22} {2,-35} {3,-18} {4,-20} {5}" -f "--------", "---", "-------", "-------", "---", "------------"
    $sorted = $allFindings | Sort-Object { $_.vulnerability.severity }, { $_.artifact.name }
    foreach ($m in $sorted) {
        $fix = if ($m.vulnerability.fix.versions -and $m.vulnerability.fix.versions.Count -gt 0) {
            $m.vulnerability.fix.versions -join ", "
        } else { "none" }
        $firstPath = if ($m.artifact.locations -and $m.artifact.locations.Count -gt 0) {
            $loc = $m.artifact.locations[0]
            if ($loc.path)       { $loc.path }
            elseif ($loc.realPath)   { $loc.realPath }
            elseif ($loc.accessPath) { $loc.accessPath }
            else { "" }
        } else { "" }
        $habPkgPath = if ($firstPath -match "((?:(?:/opt)?/hab/)?pkgs/[^/]+/[^/]+/[^/]+/[^/]+)") {
            $Matches[1]
        } elseif ($firstPath -match "((?:(?:[A-Za-z]:\\)?(?:opt\\)?hab\\)?pkgs\\[^\\]+\\[^\\]+\\[^\\]+\\[^\\]+)") {
            $Matches[1]
        } elseif ($firstPath) {
            $firstPath
        } else { "no-location" }
        $lines += "{0,-10} {1,-22} {2,-35} {3,-18} {4,-20} {5}" -f `
            $m.vulnerability.severity, $m.vulnerability.id, `
            $m.artifact.name, $m.artifact.version, $fix, $habPkgPath
    }
}

$lines | Out-File -FilePath $report -Encoding utf8
Write-Host "Results saved to $report"
Get-Content $report
