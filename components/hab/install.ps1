<#
.SYNOPSIS
Installs the Habitat 'hab' program.

Authors: The Habitat Maintainers <humans@habitat.sh>

.DESCRIPTION
This script builds habitat components and ensures that all necesary prerequisites are installed.

.Parameter Channel
Specifies a channel. Defaults to $env:HAB_BLDR_CHANNEL, or "stable" if that is
not set.

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
    [string]$Channel,
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

if(!$Channel) {
    $Channel = if($env:HAB_BLDR_CHANNEL) { $env:HAB_BLDR_CHANNEL } else { "stable" }
}

if(!$Target) {
    # Use PROCESSOR_ARCHITECTURE/PROCESSOR_ARCHITEW6432 (set by Windows itself,
    # not .NET) instead of [System.Runtime.InteropServices.RuntimeInformation],
    # which requires .NET 4.7.1+ and is unavailable on older hosts such as
    # Windows Server 2012 / Windows 8.
    $isArm64 = ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") -Or ($env:PROCESSOR_ARCHITEW6432 -eq "ARM64")
    $Target = if($isArm64) { "aarch64-windows" } else { "x86_64-windows" }
}

if(!$BldrUrl) {
    $BldrUrl = if($env:HAB_BLDR_URL) { $env:HAB_BLDR_URL } else { $defaultBldrUrl }
}

# Habitat's Depot/.hart checksum is an unkeyed BLAKE2b-256 (32-byte) digest of
# the full raw file (see components/core/src/crypto/hash.rs). .NET has no
# built-in Blake2b support, so a minimal RFC 7693 implementation is embedded
# here to allow verifying the downloaded .hart before it is extracted/run.
$blake2bSource = @"
using System;
using System.IO;
using System.Text;

public static class HabBlake2b
{
    static readonly ulong[] IV = new ulong[]
    {
        0x6a09e667f3bcc908UL, 0xbb67ae8584caa73bUL, 0x3c6ef372fe94f82bUL, 0xa54ff53a5f1d36f1UL,
        0x510e527fade682d1UL, 0x9b05688c2b3e6c1fUL, 0x1f83d9abfb41bd6bUL, 0x5be0cd19137e2179UL
    };

    static readonly byte[,] SIGMA = new byte[12, 16]
    {
        {0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15},
        {14,10,4,8,9,15,13,6,1,12,0,2,11,7,5,3},
        {11,8,12,0,5,2,15,13,10,14,3,6,7,1,9,4},
        {7,9,3,1,13,12,11,14,2,6,5,10,4,0,15,8},
        {9,0,5,7,2,4,10,15,14,1,11,12,6,8,3,13},
        {2,12,6,10,0,11,8,3,4,13,7,5,15,14,1,9},
        {12,5,1,15,14,13,4,10,0,7,6,3,9,2,8,11},
        {13,11,7,14,12,1,3,9,5,0,15,4,8,6,2,10},
        {6,15,14,9,11,3,0,8,12,2,13,7,1,4,10,5},
        {10,2,8,4,7,6,1,5,15,11,9,14,3,12,13,0},
        {0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15},
        {14,10,4,8,9,15,13,6,1,12,0,2,11,7,5,3}
    };

    static ulong Rotr64(ulong x, int n) { return (x >> n) | (x << (64 - n)); }

    static void G(ulong[] v, int a, int b, int c, int d, ulong x, ulong y)
    {
        v[a] = v[a] + v[b] + x;
        v[d] = Rotr64(v[d] ^ v[a], 32);
        v[c] = v[c] + v[d];
        v[b] = Rotr64(v[b] ^ v[c], 24);
        v[a] = v[a] + v[b] + y;
        v[d] = Rotr64(v[d] ^ v[a], 16);
        v[c] = v[c] + v[d];
        v[b] = Rotr64(v[b] ^ v[c], 63);
    }

    static void Compress(ulong[] h, byte[] block, ulong t0, ulong t1, bool final)
    {
        ulong[] m = new ulong[16];
        for (int i = 0; i < 16; i++)
        {
            m[i] = BitConverter.ToUInt64(block, i * 8);
        }

        ulong[] v = new ulong[16];
        Array.Copy(h, v, 8);
        Array.Copy(IV, 0, v, 8, 8);
        v[12] ^= t0;
        v[13] ^= t1;
        if (final) { v[14] = ~v[14]; }

        for (int round = 0; round < 12; round++)
        {
            G(v, 0, 4, 8, 12, m[SIGMA[round, 0]], m[SIGMA[round, 1]]);
            G(v, 1, 5, 9, 13, m[SIGMA[round, 2]], m[SIGMA[round, 3]]);
            G(v, 2, 6, 10, 14, m[SIGMA[round, 4]], m[SIGMA[round, 5]]);
            G(v, 3, 7, 11, 15, m[SIGMA[round, 6]], m[SIGMA[round, 7]]);
            G(v, 0, 5, 10, 15, m[SIGMA[round, 8]], m[SIGMA[round, 9]]);
            G(v, 1, 6, 11, 12, m[SIGMA[round, 10]], m[SIGMA[round, 11]]);
            G(v, 2, 7, 8, 13, m[SIGMA[round, 12]], m[SIGMA[round, 13]]);
            G(v, 3, 4, 9, 14, m[SIGMA[round, 14]], m[SIGMA[round, 15]]);
        }

        for (int i = 0; i < 8; i++)
        {
            h[i] ^= v[i] ^ v[i + 8];
        }
    }

    public static string HashBytes(byte[] data, int digestLength)
    {
        ulong[] h = new ulong[8];
        Array.Copy(IV, h, 8);
        h[0] ^= (0x01010000UL | (ulong)digestLength);

        int totalLen = data.Length;
        int offset = 0;
        ulong bytesCompressed = 0;
        byte[] block = new byte[128];

        if (totalLen == 0)
        {
            Compress(h, new byte[128], 0, 0, true);
        }
        else
        {
            while (offset < totalLen)
            {
                int remaining = totalLen - offset;
                int chunk = remaining < 128 ? remaining : 128;
                Array.Clear(block, 0, 128);
                Array.Copy(data, offset, block, 0, chunk);
                offset += chunk;
                bytesCompressed += (ulong)chunk;
                bool isFinal = offset >= totalLen;
                Compress(h, block, bytesCompressed, 0, isFinal);
            }
        }

        byte[] result = new byte[64];
        for (int i = 0; i < 8; i++)
        {
            BitConverter.GetBytes(h[i]).CopyTo(result, i * 8);
        }

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < digestLength; i++)
        {
            sb.Append(result[i].ToString("x2"));
        }
        return sb.ToString();
    }

    public static string HashFile(string path, int digestLength)
    {
        return HashBytes(File.ReadAllBytes(path), digestLength);
    }
}
"@
if (-not ([System.Management.Automation.PSTypeName]"HabBlake2b").Type) {
    Add-Type -TypeDefinition $blake2bSource -ErrorAction Stop
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
    $fullyQualified = $false
    if($version -And $version -ne "latest") {
        $ver,$release = $version -split "/",2,"SimpleMatch"
        $identPath += "/$ver"
        if($release) {
            $identPath += "/$release"
            $fullyQualified = $true
        }
    }
    if(!$fullyQualified) {
        $identPath += "/latest"
    }

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

Function Assert-HartChecksum($archive) {
    Write-Host "Verifying the Blake2b checksum matches the downloaded .hart"
    $actual = [HabBlake2b]::HashFile($archive.hart, 32)
    if($actual -ne $archive.checksum) {
        Write-Host "Expected: $($archive.checksum)"
        Write-Host "Actual:   $actual"
        Write-Error "Checksum '$($archive.checksum)' invalid. The downloaded .hart may be corrupted or tampered with; refusing to extract or install it."
    }
}

# Strips the ASCII header off a .hart file (5 lines: format version,
# name-with-rev, hash type, signature, blank line) leaving the raw
# xz-compressed tar payload, then extracts it using the native tar.exe
# (bundled with Windows since 10 1803 / Server 2019, with built-in xz support).
Function Expand-Hart($hartPath) {
    $dest = $workdir
    $payloadPath = Join-Path $dest "hab.tar.xz"

    $reader = [System.IO.File]::OpenRead($hartPath)
    try {
        # Scan for the end of the 5-line header a byte at a time. The header
        # is always tiny (well under 1KB), so this never buffers more than
        # that; the (potentially large) payload after it is never read into
        # memory as a whole.
        $newlineCount = 0
        $pos = 0
        $offset = -1
        $b = $reader.ReadByte()
        while ($b -ge 0) {
            if ($b -eq 10) {
                $newlineCount++
                if ($newlineCount -eq 5) { $offset = $pos + 1; break }
            }
            $pos++
            $b = $reader.ReadByte()
        }
        if ($offset -lt 0) {
            Write-Error "Unable to parse .hart file header in $hartPath"
        }

        $reader.Seek($offset, [System.IO.SeekOrigin]::Begin) | Out-Null
        $writer = [System.IO.File]::Create($payloadPath)
        try {
            $reader.CopyTo($writer)
        } finally {
            $writer.Dispose()
        }
    } finally {
        $reader.Dispose()
    }

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
        $fullIdent = Install-Habitat -sourceDir $folder.FullName -fullIdent $folder.Name.Replace("hab-","")
    } else {
        # Targets not yet published to packages.chef.io (ex: aarch64-windows)
        # are installed directly from a Builder Depot instance instead.
        Write-Warning "$Target is not published to packages.chef.io. Downloading directly from $BldrUrl instead."
        $archive = Get-ArchiveFromBuilder -bldrUrl $BldrUrl -channel $channel -version $version -target $Target -token $AuthToken
        Assert-HartChecksum $archive
        Expand-Hart $archive.hart
        $ident = $archive.ident
        $binDir = Join-Path $workdir "hab\pkgs\$($ident.origin)\$($ident.name)\$($ident.version)\$($ident.release)\bin"
        $fullIdent = Install-Habitat -sourceDir $binDir -fullIdent "$($ident.version)-$($ident.release)-$Target"
    }
    Assert-Habitat -ident $fullIdent -target $Target

    Write-Host "Installation of Habitat 'hab' program complete."
} finally {
    try { Remove-Item $workdir -Recurse -Force } catch {
        Write-Warning "Unable to delete $workdir"
    }
}
