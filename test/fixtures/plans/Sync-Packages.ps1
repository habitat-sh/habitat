<#

.SYNOPSIS
Sync the packages in a specified directory with packages stored in builder

.DESCRIPTION
Sync-Packages recursivly looks for plan files in `PlansPath. For each plan the following is done:

* Extract the package name and origin from the plan file to determine the package identifier
* Fetch the last time the package was updated from `<BuilderUrl>/v1/depot/pkgs/<package-identifier>/latest`
* Compare the most recent write time of files in the plans directory with the packages last update
  time. If the package update time is more recent, the package is already up to date and we can
  skip this package.
* Build the package
* Install the package
* Upload the package to `BuilderUrl` with an optional `Channel`
* Verify that the new package update time is more recent than the file modification time

The packages are built and installed from most to least nested. This allows packages to depend on
packages that are nested below it.

#>

[CmdletBinding()]
Param (
    [string]
    # The builder URL to upload packages to 
    $BuilderUrl = "https://bldr.acceptance.habitat.sh",
    
    [Parameter(Mandatory = $true)]
    [string]
    # The token to authenticate with when accessing the builder instance
    $BuilderAuthToken,
    
    [string]
    # The channel to tag packages with when uploading
    $Channel,
    
    [Parameter(Mandatory = $true)]
    [string]
    # The path to the directory containing the plans to build and to sync with builder
    $PlansPath
)

$ErrorActionPreference = "Stop"

###################################################################################################

function Get-PackageUpdateTime {
    Param([string]$PackageIdentifier)

    $uri = "$($BuilderUrl)/v1/depot/pkgs/$($PackageIdentifier)/latest"
    $json = try { 
        (Invoke-WebRequest -Uri $uri -ErrorAction Stop).Content
    }
    catch {
        $statusCode = $_.Exception.Response.StatusCode.value__
        if ($statusCode -eq 404) {
            return $null
        }
        else {
            throw("'$($uri)' returned unexpected status code '$($statusCode)'")
        }
    } 
    $packageUpdated = ($json | ConvertFrom-Json).updated_at
    if (!$packageUpdated) {
        throw ("No 'updated_at' field found on json")
    }
    $packageUpdated
}

###################################################################################################

# Get the directory containing the plans to build
$plansDirectory = Get-Item $PlansPath

# Based on OS determine the plan file name
$planFileName = if ($IsLinux) {
    "plan.sh"
}
elseif ($IsWindows) {
    "plan.ps1"
}
else {
    $(throw "Must be run on Linux or Windows")
}

# Get all plan files sorted by most to least nested. We build packages in order
# of nesting level allowing packages to depend on packages nested under the current level.
$planFiles = Get-ChildItem -Recurse -Filter $planFileName $plansDirectory |
    Sort-Object -Descending { $_.FullName.length }

# Directory that stores the built hart files
$resultsDirectory = Join-Path $plansDirectory results

# Process each plan
foreach ($planFile in $planFiles) {
    $directory = $planFile.Directory

    # Get the package identifier by searching the contents of the plan file
    $matches = (Select-String -Path $planFile -Pattern 'pkg_name="(.+)"').Matches
    if ($matches.Count -eq 0) {
        throw ("Unable to find 'pkg_name' in '$($planFile)'")
    }
    $packageName = $matches[0].Groups[1].Value
    $matches = (Select-String -Path $planFile -Pattern 'pkg_origin="(.+)"').Matches
    if ($matches.Count -eq 0) {
        throw ("Unable to find 'pkg_origin' in '$($planFile)'")
    }
    $packageOrigin = $matches[0].Groups[1].Value
    $packageIdentifier = "$($packageOrigin)/$($packageName)"

    # Check if this package can be skipped because it is already up to date
    $directoryModified = (Get-ChildItem -Recurse $directory |
        Sort-Object LastWriteTime |
        Select-Object -Last 1).LastWriteTimeUtc
    $packageUpdated = Get-PackageUpdateTime $packageIdentifier
    if ($null -ne $packageUpdated -AND $packageUpdated -gt $directoryModified) {
        Write-Output "Skipping '$($packageIdentifier)' package was last updated '$($packageUpdated.ToString("yyyy-MM-ddTHH:MM:ssZ"))' directory modified '$($directoryModified.ToString("yyyy-MM-ddTHH:MM:ssZ"))'"
        continue
    }

    Write-Output "Building plan '$($planFile)' with package identifier '$($packageIdentifier)'"

    # Build the package
    $contextPath = ($directory.FullName -replace "$($plansDirectory.FullName)").substring(1)
    $env:HAB_ORIGIN = $packageOrigin
    hab pkg build --reuse --src $plansDirectory --keys $packageOrigin $contextPath 2>&1 | Out-Null

    # Find the most recently built package
    $hart = Get-ChildItem -Recurse -Filter *.hart $resultsDirectory |
        Sort-Object LastWriteTimeUtc |
        Select-Object -Last 1
    if (!$hart) {
        throw ("Failed to find hart file in '$($resultsDirectory)'")
    }

    # Install the package so other packages can depend on it
    hab pkg install $hart

    # Upload the package
    if (!$Channel) {
        hab pkg upload --url $BuilderUrl --auth $BuilderAuthToken --force $hart
    }
    else {
        hab pkg upload --url $BuilderUrl --auth $BuilderAuthToken --channel $Channel --force $hart
    }

    # Verify that the package is now up to date
    $packageUpdated = Get-PackageUpdateTime $packageIdentifier
    if ($null -eq $packageUpdated -OR $packageUpdated -lt $directoryModified) {
        throw ("Expected '$($packageIdentifier)' to be up to date! Package was last updated '$($packageUpdated.ToString("yyyy-MM-ddTHH:MM:ssZ"))' directory modified '$($directoryModified.ToString("yyyy-MM-ddTHH:MM:ssZ"))'.")
    }
}
