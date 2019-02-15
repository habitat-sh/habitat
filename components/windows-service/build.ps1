function Test-PullRequest {
    ($env:APPVEYOR_REPO_BRANCH -like 'master') -and
        (test-path env:/APPVEYOR_PULL_REQUEST_NUMBER) -and
        (-not [string]::IsNullOrEmpty($env:APPVEYOR_PULL_REQUEST_NUMBER))
}

if(Test-PullRequest) {
    $channel = "unstable"
}
else {
    $channel = "stable"
}

$bootstrapDir = "c:\habitat"
$url = "https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-$($env:hab_exe_version)-x86_64-windows.zip?bt_package=hab-x86_64-windows"
mkdir $bootstrapDir -Force
Write-Host "Download and install latest release of hab.exe from $url"
Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile hab.zip
Expand-Archive -Path hab.zip -DestinationPath $bootstrapDir -Force
Remove-Item hab.zip -Force
$habExe = (Get-Item "$bootstrapDir/*/hab.exe").FullName

$env:HAB_ORIGIN="core"
if($env:ORIGIN_KEY) {
    "SIG-SEC-1`ncore-20170318210306`n`n$($env:ORIGIN_KEY)" | & $habExe origin key import
}
else {
    Write-Host "Generating fake secret origin key for core..."
    & $habExe origin key generate core
}
if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

& $habExe pkg build . -w
if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

$hart = (Get-Item "results\*.hart")[-1]
& $habExe pkg install $hart.FullName
if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

$binPath = (Resolve-Path "/hab/pkgs/core/windows-service/*/*/bin").Path
$pathParts = $binPath.Split("\")
$versionStamp = "$($pathParts[-3])-$($pathParts[-2])"
Update-AppveyorBuild -Version $versionStamp

& $habExe pkg exec core/windows-service install
if(!(Get-Service Habitat -ErrorAction SilentlyContinue)) {
    Throw "The Habitat service was not installed!"
}
Start-Service Habitat
if((Get-Service Habitat).Status -ne "Running") {
    Throw "The Habitat service was unable to start!"
}

$retry = 0
while ($retry -lt 10 -and !((Test-NetConnection localhost -Port 9632).TcpTestSucceeded)) {
    Write-Host "Waiting for Supervisor to listen on 9632..."
    Start-Sleep -Seconds 1
    $retry += 1
}

Write-Host "Validating we can talk to a running supervisor"
& $habExe svc status
if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

if($env:HAB_AUTH_TOKEN) {
    & $habExe pkg upload $hart --channel $channel
    if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
}
