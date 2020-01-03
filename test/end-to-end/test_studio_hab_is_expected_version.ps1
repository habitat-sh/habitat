# This tests that the version of hab that we are releaseing is the same 
# version embedded in the studio package. Since the studio is built 
# with the previous version of `hab` this is useful to verify that the
# correct version was copied.
 
Describe "Studio hab cli version" {
    hab origin key generate "$env:HAB_ORIGIN"
    $expected_version = $(hab --version)

    It "matches the version of the studio" {
        $result = Invoke-StudioRun "hab --version"
        $result[-1] | Should -Be $expected_version
    }
}

