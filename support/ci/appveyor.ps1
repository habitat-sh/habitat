pushd c:\projects\habitat

$RunTests = (git diff master --name-only | 
                where-object {$_ -like 'components/*'} | 
                foreach-object { $_ -replace 'components/(\w+-*\w*)/.*$', '$1'} | 
                sort-object -unique | 
                where-object {($env:HAB_COMPONENTS -split ';') -contains $_}
            ).count -ge 1

foreach ($BuildAction in ($env:hab_build_action -split ';')) {
    if (($RunTests -or (test-path env:HAB_FORCE_BUILD)) -and ($BuildAction -like 'build')) {
        Write-Host "Building hab..."
        pushd "c:/projects/habitat/components/hab"
        cargo build
        if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
        popd
        ./target/debug/hab.exe --version
        if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}            
    }
    elseif (($RunTests -or (test-path env:HAB_FORCE_TEST)) -and ($BuildAction -like 'test')) {
        foreach ($component in ($env:hab_components -split ';')) {
            pushd "c:/projects/habitat/components/$component"
            cargo test
            if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
            popd
        }
    }
    else {
        Write-Host "Nothing changed in ported crates. Skipping $BuildAction."
    }
}
