# Test the event stream connection to a NATS server

Describe "event stream not connected to nats" {
    $env:RUST_LOG = "rants=trace"

    It "fails to start with --event-stream-connect-timeout set" {
        {
            $supLog = New-SupervisorLogFile("event_stream-fails_to_start_with_no_NATS_server_with_timeout")
            Start-Supervisor -LogFile $supLog -Timeout 3 -SupArgs @( `
                    "--event-stream-application=MY_APP", `
                    "--event-stream-environment=MY_ENV", `
                    "--event-stream-site=MY_SITE", `
                    "--event-stream-url='127.0.0.1:4222'", `
                    "--event-stream-token=blah", `
                    "--event-stream-connect-timeout=2" `
            )
        } | Should -Throw
    }
}

Describe "event stream connected to automate" {
    BeforeAll {
        $env:DOCKER_BUILDKIT=1
        try {
            Write-Host "Building automate image..."
            docker network ls
            $output = docker build --progress=plain --no-cache -t automate ./test/end-to-end/automate
            if ($LASTEXITCODE -ne 0) {
                throw "Docker build failed with exit code $LASTEXITCODE"
            }
        } catch {
            Write-Host "Error building automate image: $_"
            exit 1
        }
        Write-Host "starting automate container..."
        $script:cid = docker run --rm -d -p 4222:4222 automate
        Write-Host "Waiting for automate to get healthy..."
        docker exec $cid chef-automate status -w
        Write-Host "Automate is healthy!"
        $authToken = $(docker exec $cid chef-automate iam token create my_token --admin)
        Write-Host "Obtained token: $authToken"
        $cert = New-TemporaryFile
        docker exec $cid chef-automate external-cert show | Out-File $cert -Encoding utf8
        Write-Host "Retrieved server certificate to $cert"

        # Start the supervisor but do not require an initial event stream connection
        $supLog =  New-SupervisorLogFile("test_event_stream")
        Write-Host "Starting Supervisor..."
        Start-Supervisor -Timeout 45 -LogFile $supLog -SupArgs @( `
                "--event-stream-application=MY_APP", `
                "--event-stream-environment=MY_ENV", `
                "--event-stream-site=MY_SITE", `
                "--event-stream-url=localhost:4222", `
                "--event-stream-token=$authToken", `
                "--event-stream-server-certificate=$cert" `
        )
        Write-Host "Loading test-probe"
        Load-SupervisorService -PackageName "habitat-testing/test-probe"
        Write-Host "Service Loaded"
    }

    AfterAll {
        Unload-SupervisorService -PackageName "habitat-testing/test-probe" -Timeout 20
        Stop-Supervisor
        docker stop $script:cid
        docker rmi -f automate
    }

    It "connects and sends a health check" {
        # test-probe has a long init hook, and we want
        # to let the health-check hoo
        Start-Sleep -Seconds 20

        # Check that the output contains a connect message and that the server received a health check message
        $out = (docker exec $cid chef-automate applications show-svcs --service-name test-probe)
        $out[1] | Should -BeLike "*OK"
        # This change to index into an array is a response to a change in Automate (linked below)
        # where the header line is now written to stdout as opposed to stderr.  This resulted in an
        # array that needs to be navigated as opposed to a string that could be searched directly.
        # https://github.com/chef/automate/commit/5f5af20f562acb237668202992a76610c0a34896#diff-958adaffe8182cb66dec1ecbe75667e1052e051cc77b4e54f7d336ab427c1bfbL398
    }
}
