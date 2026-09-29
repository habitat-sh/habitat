<#
.SYNOPSIS
Installs the Habitat 'hab' program.

Authors: The Habitat Maintainers <humans@habitat.sh>

.DESCRIPTION
This script builds habitat components and ensures that all necesary prerequisites are installed.

.Parameter Channel
Specifies a channel

.Parameter Version
Specifies a version (ex: 0.75.0, 0.75.0/20190219232208)

.Parameter Target
Specifies the package target to install (ex: x86_64-windows, aarch64-windows).
Defaults to x86_64-windows, unless running on an ARM64 host, in which case it
defaults to aarch64-windows.

.Parameter BldrUrl
Only used when Target is aarch64-windows (or any target not published to
packages.chef.io). Specifies the Builder Depot instance to download hab
directly from (ex: https://bldr.acceptance.habitat.sh). Defaults to
$env:HAB_BLDR_URL, or https://bldr.habitat.sh if that is not set.

.Parameter AuthToken
Only used when downloading directly from a Builder Depot instance. A bearer
token to use when the Depot requires authentication (ex: for a private
origin). Defaults to $env:HAB_AUTH_TOKEN.
#>

param (
    [Alias("c")]
    [string]$Channel="stable",
    [Alias("v")]
    [string]$Version,
    [Alias("t")]
    [string]$Target,
    [Alias("u")]
    [string]$BldrUrl,
    [string]$AuthToken=$env:HAB_AUTH_TOKEN
)

$ErrorActionPreference="stop"

Set-Variable packagesChefioRootUrl -Option ReadOnly -Value "https://packages.chef.io/files"
Set-Variable defaultBldrUrl -Option ReadOnly -Value "https://bldr.habitat.sh"

if(!$Target) {
    $isArm64 = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64
    $Target = if($isArm64) { "aarch64-windows" } else { "x86_64-windows" }
}

if(!$BldrUrl) {
    $BldrUrl = if($env:HAB_BLDR_URL) { $env:HAB_BLDR_URL } else { $defaultBldrUrl }
}

Function Get-File($url, $dst) {
    Write-Host "Downloading $url"
    # Can't use [System.Net.SecurityProtocolType]::Tls12 on older .NET versions
    # Need to use 3072. Un patched older versions of windows will fail even on 3072
    try {
        [System.Net.ServicePointManager]::SecurityProtocol = [Enum]::ToObject([System.Net.SecurityProtocolType], 3072)
    } catch {
        Write-Error "TLS 1.2 is not supported on this operating system. Upgrade or patch your Windows installation."
    }
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($url, $dst)
}

Function Get-WorkDir {
    $parent = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

# Downloads the requested archive from packages.chef.io
Function Get-Archive($channel, $version) {
    $url = $packagesChefioRootUrl
    if(!$version -Or $version -eq "latest") {
        $hab_url="$url/$channel/habitat/latest/hab-x86_64-windows.zip"
    } else {
        $version,$_release = $version -split "/",2,"SimpleMatch"
        if($null -ne $_release) {
            Write-Warning "packages.chef.io does not support 'version/release' format. Using $version for the version"
        }

        $hab_url="$url/habitat/${version}/hab-x86_64-windows.zip"
    }
    $sha_url="$hab_url.sha256sum"
    $hab_dest = (Join-Path ($workdir) "hab.zip")
    $sha_dest = (Join-Path ($workdir) "hab.zip.shasum256")

    Get-File $hab_url $hab_dest
    $result = @{ "zip" = $hab_dest }

    # Note that this will fail on versions less than 0.71.0
    # when we did not upload shasum files to bintray.
    # NOTE: This is left in place because, while we don't ship <0.71.0
    # from s3 today, the intent is to move old releases over
    try {
        Get-File $sha_url $sha_dest
        $result["shasum"] = (Get-Content $sha_dest).Split()[0]
    } catch {
        Write-Warning "No shasum exists for $version. Skipping validation."
    }
    $result
}

# Resolves and downloads the requested hab package directly from a Builder
# Depot instance (used for targets such as aarch64-windows that are not yet
# published to packages.chef.io).
Function Get-ArchiveFromBuilder($bldrUrl, $channel, $version, $target, $token) {
    $origin = "chef"
    $identPath = "depot/channels/$origin/$channel/pkgs/hab"
    if($version -And $version -ne "latest") {
        $ver,$release = $version -split "/",2,"SimpleMatch"
        $identPath += "/$ver"
        if($release) { $identPath += "/$release" }
    }
    $identPath += "/latest"

    $metaUrl = "$bldrUrl/v1/$identPath`?target=$target"
    Write-Host "Resolving hab package via $metaUrl"

    $headers = @{}
    if($token) { $headers["Authorization"] = "Bearer $token" }

    $metaJson = Invoke-RestMethod -Uri $metaUrl -Headers $headers -UseBasicParsing
    $ident = $metaJson.ident

    $downloadUrl = "$bldrUrl/v1/depot/pkgs/$($ident.origin)/$($ident.name)/$($ident.version)/$($ident.release)/download`?target=$target"
    $hartDest = (Join-Path ($workdir) "hab.hart")

    Write-Host "Downloading $downloadUrl"
    $webRequestHeaders = @{}
    if($token) { $webRequestHeaders["Authorization"] = "Bearer $token" }
    Invoke-WebRequest -Uri $downloadUrl -Headers $webRequestHeaders -OutFile $hartDest -UseBasicParsing

    @{ "hart" = $hartDest; "ident" = $ident; "checksum" = $metaJson.checksum }
}

# Strips the ASCII header off a .hart file (5 lines: format version,
# name-with-rev, hash type, signature, blank line) leaving the raw
# xz-compressed tar payload, then extracts it using the native tar.exe
# (bundled with Windows since 10 1803 / Server 2019, with built-in xz support).
Function Expand-Hart($hartPath) {
    $dest = $workdir
    $bytes = [System.IO.File]::ReadAllBytes($hartPath)
    $newlineCount = 0
    $offset = -1
    for ($i = 0; $i -lt $bytes.Length; $i++) {
        if ($bytes[$i] -eq 10) {
            $newlineCount++
            if ($newlineCount -eq 5) { $offset = $i + 1; break }
        }
    }
    if($offset -lt 0) {
        Write-Error "Unable to parse .hart file header in $hartPath"
    }

    $payloadPath = Join-Path $dest "hab.tar.xz"
    [System.IO.File]::WriteAllBytes($payloadPath, $bytes[$offset..($bytes.Length - 1)])

    $tarExe = Get-Command "tar.exe" -ErrorAction SilentlyContinue
    if(!$tarExe) {
        Write-Error "tar.exe is required to expand .hart archives but was not found on the PATH. It ships with Windows 10 (1803+) and Windows Server 2019+."
    }

    & tar.exe -xf $payloadPath -C $dest
    if($LASTEXITCODE -ne 0) {
        Write-Error "Failed to expand .hart payload with tar.exe (exit code $LASTEXITCODE)"
    }
}

function Get-SHA256Converter {
    if($PSVersionTable.PSEdition -eq 'Core') {
        [System.Security.Cryptography.SHA256]::Create()
    } else {
        New-Object -TypeName Security.Cryptography.SHA256Managed
    }
}

Function Get-Sha256($src) {
    $converter = Get-SHA256Converter
    try {
        $bytes = $converter.ComputeHash(($in = (Get-Item $src).OpenRead()))
        return ([System.BitConverter]::ToString($bytes)).Replace("-", "").ToLower()
    } finally {
        # Older .Net versions do not expose Dispose()
        if($PSVersionTable.PSEdition -eq 'Core' -Or ($PSVersionTable.CLRVersion.Major -ge 4)) {
            $converter.Dispose()
        }
        if ($null -ne $in) { $in.Dispose() }
    }
}

Function Assert-Shasum($archive) {
    Write-Host "Verifying the shasum digest matches the downloaded archive"
    $actualShasum = Get-Sha256 $archive.zip
    if($actualShasum -ne $archive.shasum) {
        Write-Host "Expected: $($archive.shasum)"
        Write-Host "Actual:   $actualShasum"
        Write-Error "Checksum '$($archive.shasum)' invalid."
    }
}

Function Install-Habitat($sourceDir, $fullIdent) {
    $habPath = Join-Path $env:ProgramData Habitat
    if(Test-Path $habPath) { Remove-Item $habPath -Recurse -Force }
    New-Item $habPath -ItemType Directory | Out-Null
    Copy-Item "$sourceDir\*" $habPath
    $env:PATH = New-PathString -StartingPath $env:PATH -Path $habPath
    $currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    if($currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        $machinePath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
        $machinePath = New-PathString -StartingPath $machinePath -Path $habPath
        [System.Environment]::SetEnvironmentVariable("PATH", $machinePath, "Machine")
    } else {
        Write-Warning "Not running with Administrator privileges. Unable to add $habPath to PATH!"
        Write-Warning "Either rerun as Administrator or manually add $habPath to your PATH in order to run hab in another shell session."
    }
    $fullIdent
}

Function New-PathString([string]$StartingPath, [string]$Path) {
    if (-not [string]::IsNullOrEmpty($path)) {
        if (-not [string]::IsNullOrEmpty($StartingPath)) {
            [string[]]$PathCollection = "$path;$StartingPath" -split ';'
            $Path = ($PathCollection |
                    Select-Object -Unique |
                    Where-Object {-not [string]::IsNullOrEmpty($_.trim())}
            ) -join ';'
        }
        $path
    } else {
        $StartingPath
    }
}

Function Expand-Zip($zipPath) {
    $dest = $workdir
    try {
        # Works on .Net 4.5 and up (as well as .Net Core)
        # Yes on PS v5 and up we have Expand-Archive but this works on PS v4 too
        [System.Reflection.Assembly]::LoadWithPartialName("System.IO.Compression.FileSystem") | Out-Null
        [System.IO.Compression.ZipFile]::ExtractToDirectory($zipPath, $dest)
    } catch {
        try {
            # Works on all GUI enabled versions. Will fail
            # On Server Core editions
            $shellApplication = New-Object -com shell.application
            $zipPackage = $shellApplication.NameSpace($zipPath)
            $destinationFolder = $shellApplication.NameSpace($dest)
            $destinationFolder.CopyHere($zipPackage.Items())
        } catch{
            Write-Error "Unable to unzip files on this OS"
        }
    }
}

Function Assert-Habitat($ident, $target) {
    Write-Host "Checking installed hab version $ident"
    $orig = $env:HAB_LICENSE
    $env:HAB_LICENSE = "accept-no-persist"
    try {
        $actual = hab --version
        if (!$actual -or ("hab $ident" -ne "$($actual.Replace('/', '-'))-$target")) {
            Write-Error "Unable to verify Habitat was succesfully installed"
        }
    } finally {
        $env:HAB_LICENSE = $orig
    }
}

Write-Host "Installing Habitat 'hab' program for target $Target"

$workdir = Get-WorkDir
New-Item $workdir -ItemType Directory -Force | Out-Null
try {
    if($Target -eq "x86_64-windows") {
        $archive = Get-Archive $channel $version
        if($archive.shasum) {
            Assert-Shasum $archive
        }
        Expand-zip $archive.zip
        $folder = (Get-ChildItem (Join-Path ($workdir) "hab-*"))
        $fullIdent = Install-Habitat $folder.FullName $folder.Name.Replace("hab-","")
    } else {
        # Targets not yet published to packages.chef.io (ex: aarch64-windows)
        # are installed directly from a Builder Depot instance instead.
        Write-Warning "$Target is not published to packages.chef.io. Downloading directly from $BldrUrl instead."
        $archive = Get-ArchiveFromBuilder $BldrUrl $channel $version $Target $AuthToken
        Write-Warning "Checksum verification is not yet supported for Builder-direct downloads (Blake2b checksum $($archive.checksum)). Skipping validation."
        Expand-Hart $archive.hart
        $ident = $archive.ident
        $binDir = Join-Path $workdir "hab\pkgs\$($ident.origin)\$($ident.name)\$($ident.version)\$($ident.release)\bin"
        $fullIdent = Install-Habitat $binDir "$($ident.version)/$($ident.release)"
    }
    Assert-Habitat $fullIdent $Target

    Write-Host "Installation of Habitat 'hab' program complete."
} finally {
    try { Remove-Item $workdir -Recurse -Force } catch {
        Write-Warning "Unable to delete $workdir"
    }
}
