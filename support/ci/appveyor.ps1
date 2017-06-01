function Test-ReleaseBuild {
    $env:APPVEYOR_REPO_TAG_NAME -eq "$(Get-Content c:/projects/habitat/VERSION)" -and (!$env:APPVEYOR_REPO_TAG_NAME.EndsWith("dev"))
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

pushd "c:/projects/habitat"
Write-Host "Configuring build environment"
./build.ps1 -Configure -SkipBuild

write-host "TAG: $env:APPVEYOR_REPO_TAG_NAME"
Write-Host "VERSION: $(Get-Content VERSION)"
if (($env:APPVEYOR_REPO_TAG_NAME -eq "$(Get-Content VERSION)") -or (Test-SourceChanged) -or (test-path env:HAB_FORCE_TEST)) {
    if(Test-ReleaseBuild) {
        # appveyor.yml sets this to acceptance
        $env:HAB_DEPOT_URL=$null
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
                pushd "c:/projects/habitat/components/$component"
                Write-Host "Testing $component"
                Write-Host ""
                cargo test
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
                popd
            }
        }
        elseif ($BuildAction -like 'package') {
            Write-Host "Download and install latest release of hab.exe"
            $bootstrapDir = "c:\habitat"
            $url = "https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-%24latest-x86_64-windows.zip?bt_package=hab-x86_64-windows"
            mkdir $bootstrapDir -Force
            # download a hab binary to build hab from source in a studio
            Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile hab.zip
            Expand-Archive -Path hab.zip -DestinationPath $bootstrapDir
            $habExe = (Get-Item "$bootstrapDir/*/hab.exe").FullName
            
            # This will override plan's CARGO_TARGET_DIR so we do not have to build each clean
            $env:HAB_CARGO_TARGET_DIR = "c:\projects\habitat\target"

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
                & $habExe pkg build components/$component
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
                
                $hart = Get-Item "C:\hab\studios\projects--habitat\src\components\$component\results\*.hart"
                Write-Host "Copying $hart to artifacts directory..."
                Copy-Item $hart.FullName results
                & $habExe pkg install $hart.FullName
                if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

                if($env:HAB_AUTH_TOKEN -and (!Test-PullRequest)) {
                    & $habExe pkg upload $hart
                    if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
                }

                # Install and extract hab cli bin files for zip
                if ($component -eq "hab") {
                    Write-Host "Packaging HAB cli zip file"
                    Write-Host ""
                    $zip = "hab-$env:APPVEYOR_BUILD_VERSION-x86_64-windows.zip"
                    $zipDir = $zip.Replace(".zip", "")
                    $stagingZipDir = "c:/projects/habitat/windows/x86_64"
                    mkdir $zipDir -Force
                    Copy-Item "/hab/pkgs/core/hab/*/*/bin/*" $zipDir

                    mkdir $stagingZipDir -Force
                    Compress-Archive -Path $zipDir -DestinationPath "$stagingZipDir/$zip" 
                    if(Test-ReleaseBuild) {
                        mkdir "results/prod" -Force
                        Compress-Archive -Path ./windows -DestinationPath "results/prod/$zip"
                    }
                    Compress-Archive -Path ./windows -DestinationPath "results/$zip"
                }
                if ($component -eq "studio") {
                    # Now that we have built the studio we can use current hab and studio bits
                    Copy-Item "/hab/pkgs/core/hab/*/*/bin/*" $bootstrapDir -Force
                }
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
