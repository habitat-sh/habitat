Describe "ctl_gateway" {
    It "should NOT be able to issue a remote-sup call with the wrong secret key" {
        $env:HAB_CTL_SECRET=(hab sup secret generate)
        hab svc status --remote-sup "alpha.habitat.dev"
        $LASTEXITCODE | Should -Not -Be 0
    }
}
