pushd c:\projects\habitat

function Test-ComponentChanged ($path) {
    $component = $path -replace 'components/(\w+-*\w*)/.*$', '$1'
    ($env:HAB_COMPONENTS -split ';') -contains $component
}

function Test-SourceChanged {
    $BuildFiles = "appveyor.yml", "build.ps1", "support/ci/appveyor.ps1", "support/ci/appveyor.bat", 
                  "Cargo.toml", "Cargo.lock"
    (git diff master --name-only | 
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


if (Test-SourceChanged -or (test-path env:HAB_FORCE_TEST)) {
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
        else {
            Write-Warning "Unsupported Build Action: $BuildAction."
        }  
    }

    ## Prep artifact for publishing.
    $HabArchiveParams = @{
        Path = "./target/release/hab.exe", 
                "C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin\archive.dll", 
                "C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin\libeay32.dll",
                "C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin\ssleay32.dll",
                "C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin\zlib.dll",    
                "C:/Windows/System32/vcruntime140.dll"
        DestinationPath = "c:/projects/habitat/windows/x86_64/hab-$env:APPVEYOR_BUILD_VERSION-x86_64-windows.zip"
    }
    mkdir "c:/projects/habitat/windows/x86_64" -Force
    Compress-Archive  @HabArchiveParams
    Compress-Archive ./windows -DestinationPath "./hab-$env:APPVEYOR_BUILD_VERSION-x86_64-windows.zip"

    Write-Host "Created artifact: "
    ls $HabArchiveParams.DestinationPath
}
else {
    Write-Host "Nothing changed in Windows ported crates."
}
