Describe "hab file upload" {

    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
    switch ($arch) {
        'X64' {
            $script:redisRelease="core/redis/4.0.14/20200421191514"
        }
        'Arm64' {
            $script:redisRelease="core/redis/4.0.14/20230621153438"
        }
        Default {
            throw "Unsupported architecture: $arch"
        }
    }
    Load-SupervisorService $redisRelease -Remote "alpha.habitat.dev"
    Wait-Release -Ident $redisRelease -Remote "alpha"

    $message = "Hello from Habitat!"
    Set-Content message.txt -Value $message
    hab file upload `
        redis.default `
    ([DateTime]::Now.Ticks) `
        message.txt `
        --remote-sup=bastion.habitat.dev
    Start-Sleep 5

    It "should upload the file to a Supervisor running the service" {
        $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_alpha_1" cat /hab/svc/redis/files/message.txt
        $uploadedMessage | Should -Be $message
    }

    It "should NOT upload the file to a Supervisor not running the service" {
        docker exec "${env:COMPOSE_PROJECT_NAME}_beta_1" cat /hab/svc/redis/files/message.txt
        $LASTEXITCODE | Should -Not -Be 0
    }

    Context "loading service on a new Supervisor" {
        # Now load the service on another supervisor... the file should be
        # present now, as well
        Load-SupervisorService $redisRelease -Remote "beta.habitat.dev"
        Wait-Release -Ident $redisRelease -Remote "beta"

        It "should write the previously-uploaded service file to disk on the new Supervisor" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_beta_1" cat /hab/svc/redis/files/message.txt
            $uploadedMessage | Should -Be $message
        }

    }
}
