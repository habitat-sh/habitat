# Depends on https://www.powershellgallery.com/packages/Await/0.8
param(
    [String]$studio_command,
    [String]$test_case
)

$env:HAB_NOCOLORING = "true"
# Await has trouble parsing non-ascii glyphs
$env:HAB_GLYPH_STYLE = "ascii"  
$exit_code = 0

try { 
    Start-AwaitSession 
    Send-AwaitCommand "$studio_command enter" 
    $prompt = Wait-AwaitResponse "[HAB-STUDIO] Habitat:\src>"
    Write-Host $prompt

    Send-AwaitCommand "$test_case" 
    $result = Wait-AwaitResponse "test finished"

    Write-Host $result
    if($result -match "\[FAIL\]") {
        $exit_code = 1
    }

} finally { 
    Write-Host "Cleaning up"
    Send-AwaitCommand "exit"
    
    # Await won't block on "Waiting for supervisor to finish..." 
    # and there's no output to match afterwords to wait on.
    # This sleep is required to give the studio a chance to finish 
    # exiting before we issue the `rm`.
    sleep 10

    Send-AwaitCommand "$studio_command rm"
    sleep 2
    Stop-AwaitSession
}

exit $exit_code
