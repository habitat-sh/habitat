
function Get-RepoRoot {
    (Resolve-Path "$PSScriptRoot\..\..\").Path
}
function Test-ReleaseBuild {
    $env:APPVEYOR_REPO_TAG_NAME -eq (Get-Content "$(Get-RepoRoot)/VERSION") -and (!$env:APPVEYOR_REPO_TAG_NAME.EndsWith("dev"))
}

function Test-ComponentChanged ($path) {
    $component = $path -replace 'components/(\w+-*\w*)/.*$', '$1'
    ($env:HAB_COMPONENTS -split ';') -contains $component
}

function Test-PullRequest() {
    ($env:APPVEYOR_REPO_BRANCH -like 'master') -and
        (test-path env:/APPVEYOR_PULL_REQUEST_NUMBER) -and
        (-not [string]::IsNullOrEmpty($env:APPVEYOR_PULL_REQUEST_NUMBER))
}

function Test-SentinelBuild() {
    $env:APPVEYOR_REPO_BRANCH -like 'sentinel*'
}

function Test-SourceChanged {
    $files = if (Test-PullRequest -or Test-SentinelBuild) {
        # for pull requests or sentinel builds diff
        # against master
        git diff master --name-only
    } else {
        # for master builds, check against the last merge
        git show :/^Merge --pretty=format:%H -m --name-only
    }

    $BuildFiles = "appveyor.yml", "build.ps1", "support/ci/appveyor.ps1", "support/ci/appveyor.bat",
                  "Cargo.toml", "Cargo.lock"
    ($files |
        where-object {
            ($BuildFiles -contains $_ ) -or
            (($_ -like 'components/*') -and
                (Test-ComponentChanged $_))
        }
    ).count -ge 1
}

$ErrorActionPreference="stop"

pushd (Get-RepoRoot)
Write-Host "Configuring build environment"
./build.ps1 -Configure -SkipBuild
$version = $(Get-Content VERSION)

write-host "TAG: $env:APPVEYOR_REPO_TAG_NAME"
Write-Host "VERSION: $version"
if (($env:APPVEYOR_REPO_TAG_NAME -eq $version) -or (Test-SourceChanged) -or (test-path env:HAB_FORCE_TEST)) {
    if(Test-ReleaseBuild -and $env:hab_components -ne "launcher") {
        $channel = "rc-$version"
    }
    else {
        $channel = "unstable"
    }

    foreach ($BuildAction in ($env:hab_build_action -split ';')) {
        if ($BuildAction -like 'build') {

            Write-Host "Building hab..."
            Write-Host ""

            ./build.ps1 -Path "components/hab" -Release
            if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
            ./target/release/hab.exe --version
            if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

        }
        elseif ($BuildAction -like 'test') {
            foreach ($component in ($env:hab_components -split ';')) {
                pushd "$(Get-RepoRoot)/components/$component"
                Write-Host "Testing $component"
                Write-Host ""
                cargo test --verbose
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
                popd
            }
        }
        elseif ($BuildAction -like 'package') {
            Write-Host "Download and install latest release of hab.exe"
            $bootstrapDir = "c:\habitat"
            $url = "https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-$($env:hab_exe_version)-x86_64-windows.zip?bt_package=hab-x86_64-windows"
            mkdir $bootstrapDir -Force
            # download a hab binary to build hab from source in a studio
            Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile hab.zip
            Expand-Archive -Path hab.zip -DestinationPath $bootstrapDir -Force
            Remove-Item hab.zip -Force
            $habExe = (Get-Item "$bootstrapDir/*/hab.exe").FullName

            # This will override plan's CARGO_TARGET_DIR so we do not have to build each clean
            $env:HAB_CARGO_TARGET_DIR = "$(Get-RepoRoot)\target"

            $env:HAB_ORIGIN="core"
            if($env:ORIGIN_KEY) {
                "SIG-SEC-1`ncore-20170318210306`n`n$($env:ORIGIN_KEY)" | & $habExe origin key import
            }
            else {
                Write-Host "Generating fake secret origin key for core..."
                & $habExe origin key generate core
            }
            if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

            mkdir results -Force
            foreach ($component in ($env:hab_components -split ';')) {
                Write-Host "Building plan for $component"
                Write-Host ""
                if($(& $habExe --version) -lt "hab 0.63") {
                    & $habExe pkg build components/$component -w
                } else {
                    & $habExe pkg build components/$component -w -R
                }
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

                $hart = (Get-Item "$(Get-RepoRoot)\components\$component\results\*.hart")[-1]
                Write-Host "Copying $hart to artifacts directory..."
                Copy-Item $hart.FullName results
                & $habExe pkg install $hart.FullName
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

                if($env:HAB_AUTH_TOKEN -and (!(Test-PullRequest))) {
                    & $habExe pkg upload $hart --channel $channel
                    if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
                }

                # Install and extract hab cli bin files for zip
                if ($component -eq "hab") {
                    Write-Host "Packaging HAB cli zip file"
                    Write-Host ""
                    $binPath = (Resolve-Path "/hab/pkgs/core/hab/*/*/bin").Path
                    $pathParts = $binPath.Split("\")
                    $versionStamp = "$($pathParts[-3])-$($pathParts[-2])"
                    Update-AppveyorBuild -Version $versionStamp
                    $zip = "hab-$versionStamp-x86_64-windows.zip"
                    $zipDir = $zip.Replace(".zip", "")
                    $stagingZipDir = "$(Get-RepoRoot)/windows/x86_64"
                    mkdir $zipDir -Force
                    Copy-Item "/hab/pkgs/core/hab/*/*/bin/*" $zipDir

                    mkdir $stagingZipDir -Force
                    Compress-Archive -Path $zipDir -DestinationPath "$stagingZipDir/$zip"
                    if(Test-ReleaseBuild) {
                        mkdir "results/prod" -Force
                        Compress-Archive -Path ./windows -DestinationPath "results/prod/$zip"
                        $nuspec_version = $versionStamp.substring(0, $versionStamp.IndexOf('-'))
                        $checksum = (Get-FileHash "$stagingZipDir/$zip" -Algorithm SHA256).Hash
                        $choco_install = "$(Get-RepoRoot)/components/hab/win/chocolateyinstall.ps1"

                        (Get-Content $choco_install) |
                            % {$_.Replace('$version$', $versionStamp) } | 
                            Set-Content $choco_install

                        (Get-Content $choco_install) |
                            % {$_.Replace('$checksum$', $checksum) } |
                            Set-Content $choco_install

                        choco pack "$(Get-RepoRoot)/components/hab/win/habitat.nuspec" --version $nuspec_version --output-directory "results/prod"
                    }
                    Compress-Archive -Path ./windows -DestinationPath "results/$zip"
                    Remove-Item $zipDir -Recurse -Force
                    Remove-Item $stagingZipDir -Recurse -Force
                }
                if ($component -eq "studio") {
                    # Now that we have built the studio we can use current hab and studio bits
                    Copy-Item "/hab/pkgs/core/hab/*/*/bin/*" (Split-Path $habExe -Parent) -Force
                }
            }
            if(!(Test-PullRequest)) {
                $env:HAB_BLDR_CHANNEL = $channel
                & $habExe pkg exec core/hab-bintray-publish publish-studio
                $env:HAB_BLDR_CHANNEL = $null
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
            }
        }
        else {
            Write-Warning "Unsupported Build Action: $BuildAction."
        }
    }
}
else {
    Write-Host "Nothing changed in Windows ported crates."
}
