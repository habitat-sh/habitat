Describe "hab file upload" {
    hab pkg install core/redis
    Load-SupervisorService "core/redis" -Remote "alpha.habitat.dev"
    Load-SupervisorService "core/redis" -Remote "beta.habitat.dev"
    
    $message = "Hello from Habitat!"
    Set-Content message.txt -Value $message
    hab file upload `
        redis.default `
        ([DateTime]::Now.Ticks) `
        message.txt `
        --remote-sup=bastion.habitat.dev
    start-sleep 5

    @("alpha", "beta") | % {
       It "should upload the file to $_" {
         $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_${_}_1" cat /hab/svc/redis/files/message.txt
         $uploadedMessage | Should -Be $message
       }
    }
 }
