$ErrorActionPreference = "Stop"

$env:HAB_LICENSE = "accept-no-persist"

$env:studio_command = "bin/hab-studio.ps1"

hab pkg install core/powershell
hab pkg install core/7zip
hab pkg install core/hab 
hab pkg install core/hab-plan-build-ps1 

mkdir "bin/powershell" | Out-Null
mkdir "bin/hab" | Out-Null
mkdir "bin/7zip" | Out-Null

Copy-Item "$(hab pkg path core/powershell)/bin/*" "bin/powershell" -Recurse
Copy-Item "$(hab pkg path core/hab)/bin/*" "bin/hab"
Copy-Item "$(hab pkg path core/7zip)/bin/*" "bin/7zip"
Copy-Item "$(hab pkg path core/hab-plan-build-ps1)/bin/*" "bin/"

try {
    & test/shared/test-all.ps1 -studio_command "bin/hab-studio.ps1"
    $exit_code = $LASTEXITCODE
} finally {
    # The tests can exit before the Studio or Await have closed all open 
    # handles to the following files/directories. This sleep gives those 
    # processes a chance to finish.  
    sleep 5
    Remove-Item "bin/7zip" -Recurse
    Remove-Item "bin/powershell" -Recurse
    Remove-Item "bin/hab" -Recurse
    Remove-Item "bin/*"
}
exit $exit_code
