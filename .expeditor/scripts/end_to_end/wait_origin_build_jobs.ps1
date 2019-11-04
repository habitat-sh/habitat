<#

.SYNOPSIS
Wait until all queried jobs for an origin register as 'Complete'

.NOTES
The term "all queried jobs" is important. The script only verifies $JobLimit number of jobs. That
means potentially a job could still be running if there is a job with a creation time later
than the oldest job queried that has not yet completed.  

#>

param (
    [Parameter(Mandatory = $true)]
    [string]
    # The origin to check the build status of
    $Origin,
    
    [Parameter(Mandatory = $true)]
    [string]
    # The number of jobs to check the status of
    $JobLimit,
    
    [int]
    # The amount of time in seconds to wait between iterations of the loop
    $Sleep = 10,
    
    [int]
    # The amount of time in seconds before the script fails due to timeout
    $Timeout = 60 * 60,
    
    [string]
    # The builder url to query job statuses
    $BuilderUrl = $env:HAB_BLDR_URL
)

$ErrorActionPreference = "stop"

if (!$IsCoreCLR) {
    Write-Error "wait_origin_build_jobs_core is only compatiable with Powershell Core"
}

$startTime = Get-Date

do {
    $command = "hab bldr job status --origin $Origin --limit $JobLimit --url $BuilderUrl"
    Write-Output "Querying with command '$command'"
    $job = Wait-Job (hab bldr job status --origin $Origin --limit $JobLimit --url $BuilderUrl &)
    $output = Receive-Job $job
    
    # Cleanup the output and convert it to a table
    $output = $output[2..($output.Length - 2)] `
        -replace ("CREATED AT", "CREATED") `
        -replace ("GROUP ID", "GROUP") `
        -replace ("\s+", ",")
    $statusTable = ConvertFrom-Csv $output

    # Check if we have any not completed statuses
    $notCompleted = ($statusTable | Where-Object -Property "STATUS" -ne "Complete").Length
    if ($notCompleted -eq 0) {
        break    
    }

    # Check if we have timedout
    $timeTaken = [DateTime]::Now.Subtract($startTime)
    if ($timeTaken.TotalSeconds -ge $Timeout) {
        $TimeoutScript.Invoke()
        break
    }

    Write-Output "Waiting for $notCompleted jobs to complete"

    Start-Sleep 5
} while ($true) 
