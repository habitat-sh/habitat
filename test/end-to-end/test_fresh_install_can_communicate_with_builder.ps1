# Given a fresh install of Habitat we should be able to install packages from builder
# Ensure that we don't have any ssl certificates cached that might influence our ability
# to connect. 
# Note: since we're testing pre-release this won't be a "pure" fresh install, as we 
# have to curlbash install stable first, in order to get the pre-release version. 

Describe "Clean hab installation" {
    It "has no root ssl cache" {
        Test-Path /hab/cache/ssl | Should -Be $false
    }
    It "has no user ssl cache" {
        su hab -c "test ! -d ~/.hab/cache/ssl"
        $LASTEXITCODE | should -Be 0
    }
    It "can talk to builder" {
        hab pkg install core/redis --channel stable
        $LASTEXITCODE | should -Be 0
    }
}
