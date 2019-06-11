# Depends on https://www.powershellgallery.com/packages/Await/0.8
param(
    [String]$studio_command,
    [String]$test_case
)

$env:HAB_NOCOLORING = "true"
# Await has trouble parsing non-ascii glyphs
$env:HAB_GLYPH_STYLE = "ascii"  
$exit_code = 0
$studio_name = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath("/hab/studios/studio-internals-test")
try { 
    Start-AwaitSession 
    Send-AwaitCommand "$studio_command enter -o $studio_name" 
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
    Receive-AwaitResponse

    # Await won't block on "Waiting for supervisor to finish..." 
    # Copy the studio shutdown behavior and block for a bit until the supervisor has finished
    $retry = 0
    while(($retry -lt 5) -and (Test-Path "$studio_name\hab\sup\default\LOCK")) {
      $retry += 1
      Write-Host "Waiting for Supervisor to finish..."
      Start-Sleep -Seconds 5
    }

    Send-AwaitCommand "$studio_command rm -o $studio_name"
    Receive-AwaitResponse -Stream

    # Add the same behavior for `hab studio rm` to ensure that the studio is fully cleaned up before 
    # stopping the Await session.
    $retry = 0
    while(($retry -lt 10) -and (Test-Path "$studio_name")) {
      $retry += 1
      Write-Host "Waiting for Studio to exit..."
      Start-Sleep -Seconds 5
    }

    Stop-AwaitSession
}

exit $exit_code
