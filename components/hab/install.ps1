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

Set-Variable packagesChefioRootUrl -Option ReadOnly -value "https://packages.chef.io/files"

Function Get-BintrayVersion($version, $channel) {
    $jsonFile = Join-Path ($workdir) "version.json"

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
  $parent = [System.IO.Path]::GetTempPath()
  [string] $name = [System.Guid]::NewGuid()
  New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

# Downloads the requested archive from packages.chef.io
Function Get-PackagesChefioArchive($channel, $version) {
    $url = $packagesChefioRootUrl
    if(!$version -Or $version -eq "latest") {
      $hab_url="$url/$channel/habitat/latest/hab-x86_64-windows.zip"
    } else {
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
    }
    catch {
        Write-Warning "No shasum exists for $version. Skipping validation."
    }
    $result
}

Function Get-BintrayArchive($channel, $version) {
    $url = "https://api.bintray.com/content/habitat/$channel/windows/x86_64/hab-$version-x86_64-windows.zip"
    $query = "?bt_package=hab-x86_64-windows"

    $hab_url="$url$query"
    $sha_url="$url.sha256sum$query"
    $hab_dest = (Join-Path ($workdir) "hab.zip")
    $sha_dest = (Join-Path ($workdir) "hab.zip.shasum256")

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
    $folder = (Get-ChildItem (Join-Path ($workdir) "hab-*"))
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
    $dest = $workdir
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

Function Test-UsePackagesChefio($version) {
    # The $_patch may contain the /release string as well.
    # This is fine because we only care about major/minor for this 
    # comparison. 

    $_major,$_minor,$_patch = $version -split ".",3,"SimpleMatch"
    $v1 = New-Object -TypeName Version -ArgumentList $_major,$_minor
    $v2 = New-Object -TypeName Version -ArgumentList "0.89"
    !$version -Or ($v1 -ge $v2)
}

Write-Host "Installing Habitat 'hab' program"

$workdir = Get-WorkDir
New-Item $workdir -ItemType Directory -Force | Out-Null
try {
    if(Test-UsePackagesChefio($Version)) { 
      $archive = Get-PackagesChefioArchive $channel $version
    } else {
      $Version = Get-BintrayVersion $Version $Channel
      $archive = Get-BintrayArchive $channel $version
    }
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
