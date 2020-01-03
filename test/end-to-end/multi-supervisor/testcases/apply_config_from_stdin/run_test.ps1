Describe "apply config from stdin" {
    hab pkg install core/redis
    Load-SupervisorService "core/redis" -Remote "alpha.habitat.dev"
    Load-SupervisorService "core/redis" -Remote "beta.habitat.dev"
    
    $new_port=1234
    "port = $new_port`nprotected-mode = `"no`"" | hab config apply `
        redis.default `
        ([DateTime]::Now.Ticks) `
        --remote-sup=bastion.habitat.dev
    start-sleep 5

    @("alpha", "beta") | % {
       It "should call redis cli SET on applied port on $_" {
         hab pkg exec core/redis redis-cli -h "$_.habitat.dev" -p $new_port SET from_stdin_port $new_port
         $LASTEXITCODE | Should -Be 0
       }
    }

    @("alpha", "beta") | % {
        It "should call redis cli GET on applied port on $_" {
          hab pkg exec core/redis redis-cli -h "$_.habitat.dev" -p $new_port GET from_stdin_port
          $LASTEXITCODE | Should -Be 0
        }
     }
 }
