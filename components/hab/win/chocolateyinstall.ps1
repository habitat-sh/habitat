$version = '$version$'
$url = "https://packages.chef.io/files/habitat/$version/hab-x86_64-windows.zip"
$unzipLocation = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$checksum = '$checksum$'
$has_tls12 = [Enum]::GetNames([Net.SecurityProtocolType]) -contains 'Tls12'
Write-Host "has tls? -- $has_tls12"
if(!$has_tls12) {
    throw "I have not the required tls"
}
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Install-ChocolateyZipPackage "habitat" $url $unzipLocation -checksum $checksum -checksumType sha256
