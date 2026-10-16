#Linux docker studio does not support version command
if($IsWindows -Or !($env:DOCKER_STUDIO_TEST)) {
    Describe "Studio version" {
        BeforeAll {
            $cliVersion = ((hab --version) -split " ")[1]
            if($env:DOCKER_STUDIO_TEST) {
                $habVersionCmd = "hab studio version -D"
            } else {
                $habVersionCmd = "hab studio version"
            }
            hab origin key generate $env:HAB_ORIGIN

            # Download the studio before checking its version.
            Invoke-Expression $habVersionCmd
        }

        It "should match hab cli" {
            (Invoke-Expression $habVersionCmd) | Should -Match "hab-studio $(($cliVersion -split '/')[0])*"
        }
    }
}

# hab studio run is not yet implemented on macOS (run_studio function
# missing in hab-studio-darwin.sh), so skip this test on macOS.
if (!$IsMacOS) {
    Describe "Studio cli version" {
        BeforeAll {
            $cliVersion = ((hab --version) -split " ")[1]
            hab origin key generate $env:HAB_ORIGIN
        }

        It "should match hab cli" {
            (Invoke-StudioRun "hab --version")[-1] | Should -Be "hab $cliVersion"
        }
    }
}
