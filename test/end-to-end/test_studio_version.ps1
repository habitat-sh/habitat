$cliVersion = ((hab --version) -split " ")[1]

if($env:DOCKER_STUDIO_TEST) {
    $habVersionCmd = "hab studio version -D"
} else {
    $habVersionCmd = "hab studio version"
}
hab origin key generate $env:HAB_ORIGIN

# call this first to download the studio
Invoke-Expression $habVersionCmd

#Linux docker studio does not support version command
if($IsWindows -Or !($env:DOCKER_STUDIO_TEST)) {
    Describe "Studio version" {
        It "should match hab cli" {
            (Invoke-Expression $habVersionCmd) | Should -Match "hab-studio $(($cliVersion -split '/')[0])*"
        }
    }
}

# hab studio run is not yet implemented on macOS (run_studio function
# missing in hab-studio-darwin.sh), so skip this test on macOS.
if (!$IsMacOS) {
    Describe "Studio cli version" {
        It "should match hab cli" {
            (Invoke-StudioRun "hab --version")[-1] | Should -Be "hab $cliVersion"
        }
    }
}
