$version = '$version$'
$url = "https://packages.chef.io/files/habitat/$version/hab-x86_64-windows.zip"
$unzipLocation = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$checksum = '$checksum$'

Install-ChocolateyZipPackage "habitat" $url $unzipLocation -checksum $checksum -checksumType sha256
