Describe "gossiping new config" {
    # Always test the current port (whatever it happens to be)
    It "probe service should bind to redis initial port" {
        Load-SupervisorService "core/redis" -Remote "alpha.habitat.dev"
        Load-SupervisorService "habitat-testing/test-probe" -Bind "thing_with_a_port:redis.default" -Remote "beta.habitat.dev"

        # Wait for gossip configuration to settle before detecting initial state
        Start-Sleep 10

        # Check initial port - might be either default (6379) or from persistent gossip config
        $initial_port = (Invoke-WebRequest "http://beta.habitat.dev:8000/context" | ConvertFrom-Json).bind.thing_with_a_port.first.cfg.port
        Write-Host "Initial Redis port detected: $initial_port"

        $current_port = (Invoke-WebRequest "http://beta.habitat.dev:8000/context" | ConvertFrom-Json).bind.thing_with_a_port.first.cfg.port
        $current_port | Should -Be $initial_port
    }

    Context "Apply config change" {
        It "probe service should bind to changed redis port" {
            # Choose a different port for the change test
            $new_port = if ($initial_port -eq 1234) { 5678 } else { 1234 }

            Set-Content redis_config.toml -Value "port = $new_port`nprotected-mode = `"no`""
            hab config apply `
                redis.default `
            ([DateTime]::Now.Ticks) `
                redis_config.toml `
                --remote-sup=bastion.habitat.dev
            Start-Sleep 40 # Long, because test-probe has long init and post-stop hooks

            $current_port = (Invoke-WebRequest "http://beta.habitat.dev:8000/context" | ConvertFrom-Json).bind.thing_with_a_port.first.cfg.port
            $current_port | Should -Be $new_port
        }
    }
}
