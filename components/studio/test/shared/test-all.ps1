param( 
    [String]$studio_command
)

$ErrorActionPreference = "Stop"

$studio_internals="./test/shared/studio-internals"

foreach($test_case in Get-ChildItem test/shared/studio-internals -Filter "test-studio-*.ps1") {
  Write-Host "--- Running $test_case"
  & ./test/shared/studio-enter.ps1 -studio_command "$studio_command" -test_case "$studio_internals/$test_case"
}

Write-Host "--- Testing studio run `"write-host 'i said `"hello world`"'"
$out = & $studio_command run "write-host 'i said """"hello world""""'"
Write-Host $out
if($out[-1] -ne "i said `"hello world`"") {
  Write-Host "Unexpected output from hab studio run"
  Write-Host "Expected i said `"hello world`""
  exit 1
}
