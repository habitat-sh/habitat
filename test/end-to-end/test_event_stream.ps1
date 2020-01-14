# Test the event stream connection to a NATS server

$natsPkg = "$EndToEndTestingOrigin/nats-event-stream-test"

$authToken = "my_token="

Describe "event stream connection to nats" {
    $env:HAB_FEAT_EVENT_STREAM = 1
    $env:RUST_LOG = "rants=trace"

    It "fails to start with no NATS server and --event-stream-connect-timeout set" { 
        { 
            Start-Supervisor -Timeout 3 -SupArgs @( `
                    "--event-stream-application=MY_APP", `
                    "--event-stream-environment=MY_ENV", `
                    "--event-stream-site=MY_SITE", `
                    "--event-stream-url='127.0.0.1:4222'", `
                    "--event-stream-token=$authToken", `
                    "--event-stream-connect-timeout=2" `
            )
        } | Should -Throw
    }

    # Start the supervisor but do not require an initial event stream connection
    $supLog = New-TemporaryFile
    Start-Supervisor -Timeout 45 -LogFile $supLog -SupArgs @( `
            "--event-stream-application=MY_APP", `
            "--event-stream-environment=MY_ENV", `
            "--event-stream-site=MY_SITE", `
            "--event-stream-url=127.0.0.1:4222", `
            "--event-stream-token=$authToken" `
    )

    # Start the NATS Server
    Load-SupervisorService -PackageName $natsPkg -Timeout 20 -HealthCheckInterval 1

    It "event stream connects and sends a health check" {
        # Wait for a few health checks to run
        Start-Sleep -Seconds 3

        # Check that the output contains a connect message and that the server received a health check message
        $out = (Get-Content $supLog) -join "`r`n"
        $out | Should -BeLike "*INFO rants] Transitioned to state 'Connecting(127.0.0.1:4222)' from 'Connecting(127.0.0.1:4222)'*"
        $out | Should -BeLike "*NATS server is healthy\n:\x00B\x02\b\x01`"]*"
    }

    Unload-SupervisorService -PackageName $natsPkg -Timeout 20
    Stop-Supervisor
}
