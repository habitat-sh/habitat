Describe "Finding peers from watch file" {
    It "finds all three peers" {
        $testScript = { (Invoke-WebRequest "http://bastion.habitat.dev:9631/census" | ConvertFrom-Json).last_membership_counter -eq 3 }
        $timeoutScript = { Write-Error "Timed out waiting 45 seconds for all members to reach bastion" }
        Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45
        $json = (Invoke-WebRequest "http://bastion.habitat.dev:9631/census" | ConvertFrom-Json)
        $json.last_membership_counter | Should -Be 3
    }
}
