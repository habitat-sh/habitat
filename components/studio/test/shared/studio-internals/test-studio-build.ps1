$env:HAB_ORIGIN="ci"

$exit_code = 1
try { 
    hab origin key generate $HAB_ORIGIN

    build test/fixtures/empty-plan

    $exit_code = $LASTEXITCODE
} finally { 
    if($exit_code -eq 0) {
        Write-Host "[PASS]"
    } else {
        Write-Host "[FAIL]"
    }
    Write-Host "test finished"
}

exit $exit_code
