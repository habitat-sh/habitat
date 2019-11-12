# This is a lightweight test to verify a studio can be created before merging a PR.
# This (hopefully) prevents us spending time building the first half of a release 
# only to hit a broken studio. 
# 
# Failure case: because this creates a studio from source, we don't exercise changes
# in our plan.sh, and could still end up with a bad studio build.

$ErrorActionPreference = "Stop"

$env:HAB_LICENSE = "accept-no-persist"

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
    & bin/hab-studio.bat new
    $exit_code = $LASTEXITCODE
} finally {
    # The test can exit before the Studio has closed all open 
    # handles to the following files/directories. This sleep 
    # gives those processes a chance to finish.  
    sleep 5
    Remove-Item "bin/7zip" -Recurse
    Remove-Item "bin/powershell" -Recurse
    Remove-Item "bin/hab" -Recurse
    Remove-Item "bin/environment.ps1"
    Remove-Item "bin/shared.ps1"
    Remove-Item "bin/hab-plan-build.ps1"
}
exit $exit_code
