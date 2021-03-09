$version = '$version$'
$url = "https://packages.chef.io/files/habitat/$version/hab-x86_64-windows.zip"
$unzipLocation = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$checksum = '$checksum$'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Install-ChocolateyZipPackage "habitat" $url $unzipLocation -checksum $checksum -checksumType sha256
