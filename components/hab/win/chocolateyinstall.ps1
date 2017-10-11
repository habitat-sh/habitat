$version = '$version$'
$url = "https://bintray.com/habitat/stable/download_file?file_path=windows%2Fx86_64%2Fhab-$version-x86_64-windows.zip"
$unzipLocation = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$checksum = '$checksum$'

Install-ChocolateyZipPackage "habitat" $url $unzipLocation -checksum $checksum -checksumType sha256
