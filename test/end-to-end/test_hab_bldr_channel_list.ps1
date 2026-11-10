BeforeAll {
    Set-Variable -Name HAB_ORIGIN -Value ("core") -Scope Local

    # hab prints a "Determining channels for <origin>." status line, styled with
    # ANSI escapes, that can be captured alongside the channel names, so drop it.
    function Get-ChannelOutput {
        param([string[]]$ListArgs)
        @(hab bldr channel list @ListArgs 2>$null) |
            ForEach-Object { $_ -replace "\x1b\[[0-9;]*m", "" } |
            Where-Object { $_ -notmatch "Determining channels for" }
    }
}

Describe "bldr channel list" {
    It "Lists the non-sandbox channels for the origin" {
        $channels = @(Get-ChannelOutput -ListArgs $HAB_ORIGIN)
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
        $channels = @(Get-ChannelOutput -ListArgs "--filter", "stable", $HAB_ORIGIN)
        $LASTEXITCODE | Should -Be 0
        $channels | Should -Contain "stable"
        $channels | Where-Object { -not $_.Contains("stable") } | Should -BeNullOrEmpty
    }

    It "Applies the filter to sandbox channels" {
        $channels = @(Get-ChannelOutput -ListArgs "--sandbox", "--filter", "bldr-", $HAB_ORIGIN)
        $LASTEXITCODE | Should -Be 0
        $channels | Should -Not -BeNullOrEmpty
        $channels | Where-Object { -not $_.StartsWith("bldr-") } | Should -BeNullOrEmpty
    }

    It "Reports when no channels match the filter" {
        $output = Get-ChannelOutput -ListArgs "--filter", "no-such-channel-zzz", $HAB_ORIGIN
        $LASTEXITCODE | Should -Be 0
        "$output" | Should -Be "No channels found matching 'no-such-channel-zzz'"
    }

    It "Rejects a blank filter" {
        hab bldr channel list --filter " " $HAB_ORIGIN 2>&1 | Out-Null
        $LASTEXITCODE | Should -Not -Be 0
    }
}
