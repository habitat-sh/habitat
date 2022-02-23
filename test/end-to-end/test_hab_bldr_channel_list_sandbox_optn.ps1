$HAB_ORIGIN = "core"

Describe "bldr channel list --sandbox" {
    It "Lists all the channels including sandbox for the origin" {
        $success = $false
        foreach($ch in hab bldr channel list --sandbox $HAB_ORIGIN) {
            if($ch.StartsWith("bldr-")) {
                $success = $true
                break
            }
        }
        $success | Should be $true
    }
}
