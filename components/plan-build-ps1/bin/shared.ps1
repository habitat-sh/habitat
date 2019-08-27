function _Exit-With {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$True)]
        [string]
        $Message,

        [Parameter(Mandatory=$True)]
        [int32]
        $ExitCode
    )

    process {
        Write-Error "$Message"
        exit $ExitCode
    }
}

function Write-BuildLine {
    <#
    .SYNOPSIS
    Print a line of build output
    .DESCRIPTION
    Takes a string as its only argument.
    #>
    [CmdletBinding()]
    param(
        # A message to display
        [string]
        $Message
    )

    process {
        Write-Host "   ${pkg_name}: " -ForegroundColor Cyan -NoNewline
        Write-Host "$Message" -ForegroundColor White
    }
}

function Get-HabPackagePath {
<#
.SYNOPSIS
Returns the path for the desired build or runtime direct package dependency
on stdout from the resolved dependency set.

.PARAMETER Identity
The package identity of the path to retrieve.

.EXAMPLE
Get-HabPackagePath "acme/nginx"
# /hab/pkgs/acme/nginx/1.8.0/20150911120000

.EXAMPLE
Get-HabPackagePath "zlib"
# /hab/pkgs/acme/zlib/1.2.8/20151216221001

.EXAMPLE
Get-HabPackagePath "glibc/2.22"
# /hab/pkgs/acme/glibc/2.22/20151216221001
#>
    param($Identity)

    foreach($e in $pkg_all_deps_resolved) {
        if((Resolve-HabPkgPath $e).Contains("/$Identity/")) {
          return $e
        }
    }
    Write-Error "Get-HabPackagePath '$Identity' did not find a suitable installed package`nResolved package set: ${pkg_all_deps_resolved}"
}

function Resolve-HabPkgPath($unresolved) {
    $unresolved.Replace("$(Resolve-Path $HAB_PKG_PATH)\", "").Replace("\", "/")
}

# Returns the path with the studio directory stripped.
# So c:\hab\studios\my-studio\hab\pkgs would unroot to 
# \hab\pkgs
function _Get-UnrootedPath($path) {
    # Make sure $path is absolute and cannonicalized
    Push-Location $originalPath
    try {
      $path = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($path)
    }
    finally { Pop-Location }
  
    # Find the Studio directory
    $prefixDrive = (Resolve-Path $originalPath).Drive.Root
  
    # Strip the studio directory
    $strippedPrefix = $path
    if($path.StartsWith($prefixDrive)) {
      $strippedPrefix = $path.Substring($prefixDrive.length)
    }
    if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }
    $strippedPrefix
}
  