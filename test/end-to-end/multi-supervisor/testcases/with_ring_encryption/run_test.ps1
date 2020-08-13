# In this test, the Supervisors use ring encryption to secure
# Supervisor-to-Supervisor communication with each other.
#
# However, the `alpha` Supervisor is not configured to participate
# with encryption. It will start up, but none of the gossip will get
# to it.
#
# To test this, we upload a file to the bastion and verify that it
# gets to the Supervisor that does participate with encryption
# (`beta`), but no the one that doesn't (`alpha`).
Describe "ring encryption works" {
    Load-SupervisorService "core/redis" -Remote "alpha.habitat.dev"
    Load-SupervisorService "core/redis" -Remote "beta.habitat.dev"

    $message = "Hello from Habitat!"
    Set-Content message.txt -Value $message
    hab file upload `
        redis.default `
    ([DateTime]::Now.Ticks) `
        message.txt `
        --remote-sup=bastion.habitat.dev
    Start-Sleep 5

    It "should NOT upload the file to alpha" {
        $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_alpha_1" cat /hab/svc/redis/files/message.txt
        $uploadedMessage | Should -Be $null
    }

    It "SHOULD upload the file to beta" {
        $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_beta_1" cat /hab/svc/redis/files/message.txt
        $uploadedMessage | Should -Be $message
    }
}
