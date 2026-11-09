BeforeAll {
    Set-Variable -Name HAB_ORIGIN -Value ("core") -Scope Local
}

Describe "bldr channel list" {
    It "Lists the non-sandbox channels for the origin" {
        $channels = @(hab bldr channel list $HAB_ORIGIN)
        $LASTEXITCODE | Should -Be 0
        $channels | Should -Contain "stable"
        $channels | Where-Object { $_.StartsWith("bldr-") } | Should -BeNullOrEmpty
    }
}

Describe "bldr channel list --sandbox" {
    It "Lists all the channels including sandbox for the origin" {
        $success = $false
        foreach($ch in hab bldr channel list --sandbox $HAB_ORIGIN) {
            if($ch.StartsWith("bldr-")) {
                $success = $true
                break
            }
        }
        $success | Should -BeTrue
    }
}

Describe "bldr channel list --filter" {
    It "Only lists channels whose names contain the filter string" {
        $channels = @(hab bldr channel list --filter "stable" $HAB_ORIGIN)
        $LASTEXITCODE | Should -Be 0
        $channels | Should -Contain "stable"
        $channels | Where-Object { -not $_.Contains("stable") } | Should -BeNullOrEmpty
    }

    It "Applies the filter to sandbox channels" {
        $channels = @(hab bldr channel list --sandbox --filter "bldr-" $HAB_ORIGIN)
        $LASTEXITCODE | Should -Be 0
        $channels | Should -Not -BeNullOrEmpty
        $channels | Where-Object { -not $_.StartsWith("bldr-") } | Should -BeNullOrEmpty
    }

    It "Reports when no channels match the filter" {
        $output = hab bldr channel list --filter "no-such-channel-zzz" $HAB_ORIGIN
        $LASTEXITCODE | Should -Be 0
        "$output" | Should -Be "No channels found matching 'no-such-channel-zzz'"
    }

    It "Rejects a blank filter" {
        hab bldr channel list --filter " " $HAB_ORIGIN 2>&1 | Out-Null
        $LASTEXITCODE | Should -Not -Be 0
    }
}
