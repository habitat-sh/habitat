Describe "Finding peers from watch file" {
    It "finds all three peers" {
        $testScript = { (Invoke-WebRequest "http://bastion.habitat.dev:9631/census" | ConvertFrom-Json).last_membership_counter -eq 3 }
        $timeoutScript = { Write-Error "Timed out waiting 45 seconds for all members to reach bastion" }
        Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45
        $json = (Invoke-WebRequest "http://bastion.habitat.dev:9631/census" | ConvertFrom-Json)
        $json.last_membership_counter | Should -Be 2
    }

    It "adds beta to the peer watch file and finds it as a peer" {
        Add-Content -Path "./testcases/peer_watcher/PEERS" -Value "beta.habitat.dev"
        Start-Sleep -Seconds 5 # give butterfly some time to detect the change and update the census
        $json = (Invoke-WebRequest "http://bastion.habitat.dev:9631/census" | ConvertFrom-Json)
        $json.last_membership_counter | Should -Be 3
    }
}