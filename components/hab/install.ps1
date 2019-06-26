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
#>

param (
    [Alias("c")]
    [string]$Channel="stable",
    [Alias("v")]
    [string]$Version
)

$ErrorActionPreference="stop"

Function Get-Version($version, $channel) {
    $jsonFile = Join-Path (Get-WorkDir) "version.json"

    # bintray expects a '-' to separate version and release and not '/'
    $version = $version.Replace("/", "-")
    
    if(!$version) {
        $version = "%24latest"
    } elseif([System.Reflection.Assembly]::LoadWithPartialName("System.Web.Extensions")) {
        # Using .Net json serializer instead of Convert-FromJson (in PS v3 (win 2012) and greater)
        # because the .Net strategy will work on any system with .Net 3.5 and up
        Write-Host "Determining fully qualified version of package for '$version'"
        Get-File "https://api.bintray.com/packages/habitat/$channel/hab-x86_64-windows" $jsonFile
        $ser = New-Object System.Web.Script.Serialization.JavaScriptSerializer
        $versions = $ser.DeserializeObject((Get-Content $jsonFile)).versions
        $rev = $versions | ? { $_.StartsWith($version) }
        if(!$rev) {
            $e =  "Version '$version' could not used or version doesn't exist."
            $e += " Please provide a simple version like: '0.15.0'"
            $e += " or a fully qualified version like: '0.15.0/20161222203215'."
            Write-Error $e
        } else {
            Write-Host "Using fully qualified Bintray version string of: $rev"
            $version = $rev
        }
    } else {
        # Must have an older .Net installation
        if($version -match "^\d+\.\d+\.\d+-\d{12}`$") {
            Write-Warning "Validating the version is not supported without at least .Net 3.5"
            Write-Warning "We will make a best effort to retrieve $version"
        }
        else {
            Write-Warning "Validating the version is not supported without at least .Net 3.5"
            Write-Error "You must supply a fully qualified version (ex: 0.75.0/20190219232208)"
        }
    }

    $version
}

Function Get-File($url, $dst) {
    Write-Host "Downloading $url"
    # Can't use [System.Net.SecurityProtocolType]::Tls12 on older .NET versions
    # Need to use 3072. Un patched older versions of windows will fail even on 3072
    try {
        [System.Net.ServicePointManager]::SecurityProtocol = [Enum]::ToObject([System.Net.SecurityProtocolType], 3072)
    }
    catch {
        Write-Error "TLS 1.2 is not supported on this operating system. Upgrade or patch your Windows installation."
    }
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($url, $dst)
}

Function Get-WorkDir {
    Join-Path $env:temp "hab.XXXX"
}

Function Get-Archive($channel, $version) {
    $url = "https://api.bintray.com/content/habitat/$channel/windows/x86_64/hab-$version-x86_64-windows.zip"
    $query = "?bt_package=hab-x86_64-windows"

    $hab_url="$url$query"
    $sha_url="$url.sha256sum$query"
    $hab_dest = (Join-Path (Get-WorkDir) "hab.zip")
    $sha_dest = (Join-Path (Get-WorkDir) "hab.zip.shasum256")

    Get-File $hab_url $hab_dest
    $result = @{ "zip" = $hab_dest }

    # Note that this will fail on versions less than 0.71.0
    # when we did not upload shasum files to bintray
    try {
        Get-File $sha_url $sha_dest
        $result["shasum"] = (Get-Content $sha_dest).Split()[0]
    }
    catch {
        Write-Warning "No shasum exists for $version. Skipping validation."
    }
    $result
}

function Get-SHA256Converter {
  if($PSVersionTable.PSEdition -eq 'Core') {
    [System.Security.Cryptography.SHA256]::Create()
  }
  else {
    New-Object -TypeName Security.Cryptography.SHA256Managed
  }
}

Function Get-Sha256($src) {
    $converter = Get-SHA256Converter
    try {
        $bytes = $converter.ComputeHash(($in = (Get-Item $src).OpenRead()))
        return ([System.BitConverter]::ToString($bytes)).Replace("-", "").ToLower()
    }
    finally {
        # Older .Net versions do not expose Dispose()
        try { $converter.Dispose() } catch {}
        if ($in -ne $null) { $in.Dispose() }
    }
}

Function Assert-Shasum($archive) {
    Write-Host "Verifying the shasum digest matches the downloaded archive"
    $actualShasum = Get-Sha256 $archive.zip
    if($actualShasum -ne $archive.shasum) {
        Write-Error "Checksum '$($archive.shasum)' invalid."
    }
}

Function Install-Habitat {
    $habPath = Join-Path $env:ProgramData Habitat
    if(Test-Path $habPath) { Remove-Item $habPath -Recurse -Force }
    New-Item $habPath -ItemType Directory | Out-Null
    $folder = (Get-ChildItem (Join-Path (Get-WorkDir) "hab-*"))
    Copy-Item "$($folder.FullName)\*" $habPath
    $env:PATH = New-PathString -StartingPath $env:PATH -Path $habPath
    $machinePath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
    $machinePath = New-PathString -StartingPath $machinePath -Path $habPath
    [System.Environment]::SetEnvironmentVariable("PATH", $machinePath, "Machine")
    $folder.Name.Replace("hab-","")
}

Function New-PathString([string]$StartingPath, [string]$Path) {
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

Function Expand-Zip($zipPath) {
    $dest = Get-WorkDir
    try {
        # Works on .Net 4.5 and up (as well as .Net Core)
        # Yes on PS v5 and up we have Expand-Archive but this works on PS v4 too
        [System.Reflection.Assembly]::LoadWithPartialName("System.IO.Compression.FileSystem") | Out-Null
        [System.IO.Compression.ZipFile]::ExtractToDirectory($zipPath, $dest)
    }
    catch {
        try {
            # Works on all GUI enabled versions. Will fail
            # On Server Core editions
            $shellApplication = new-object -com shell.application
            $zipPackage = $shellApplication.NameSpace($zipPath)
            $destinationFolder = $shellApplication.NameSpace($dest)
            $destinationFolder.CopyHere($zipPackage.Items())
        }
        catch{
            Write-Error "Unable to unzip files on this OS"
        }
    }
}

Function Assert-Habitat($ident) {
    Write-Host "Checking installed hab version $ident"
    $orig = $env:HAB_LICENSE
    $env:HAB_LICENSE = "accept-no-persist"
    try {
        $actual = hab --version
        if (!$actual -or ("hab $ident" -ne "$($actual.Replace('/', '-'))-x86_64-windows")) {
            Write-Error "Unable to verify Habitat was succesfully installed"
        }
    }
    finally {
        $env:HAB_LICENSE = $orig
    }
}

Write-Host "Installing Habitat 'hab' program"

$workdir = Get-WorkDir
New-Item $workdir -ItemType Directory -Force | Out-Null
try {
    $Version = Get-Version $Version $Channel
    $archive = Get-Archive $channel $version
    if($archive.shasum) {
        Assert-Shasum $archive
    }
    Expand-zip $archive.zip
    $fullIdent = Install-Habitat
    Assert-Habitat $fullIdent

    Write-Host "Installation of Habitat 'hab' program complete."
}
finally {
    try { Remove-Item $workdir -Recurse -Force } catch {
        Write-Warning "Unable to delete $workdir"
    }
}
