param (
    [string]$HookPath,
    [string]$PipeName
)

function Restore-Environment($origEnv) {
    # Ensure all original variables get set to their original values
    $origEnv.keys | % {
        New-Item -ItemType Variable -Path Env: -Name $_ -Value $origEnv[$_] -Force | Out-Null
    }
    # Delete any added variables that were not present originally
    Get-ChildItem env:\ | ? { !$origEnv.ContainsKey($_.name) } | % {
        Remove-Item -Path (Join-Path Env: $_.Name) | Out-Null
    }
}

Write-Host "Starting PS pipe server for $PipeName with PID $PID"
# hooks do not have an extension but the execution operator below
# will fail if the file does not have a ps1 extension
Copy-Item $HookPath "${HookPath}.ps1" -Force

# Get a handle for this service's parent (should be the supervisor)
$parent_pid = (Get-CimInstance -Class Win32_Process -Filter "ProcessID=$PID").ParentProcessId
$parent = Get-Process -Id $parent_pid

try {
    $np = new-object System.IO.Pipes.NamedPipeServerStream($PipeName, [System.IO.Pipes.PipeDirection]::InOut)
    Write-Host "Named pipe created. Waiting for connection..."
    $running = $true
 
    # take a snapshot of all environment variables before we run a hook
    $origEnv = @{}
    Get-ChildItem env:\ | % {
        $origEnv[$_.name] = $_.Value
    }

    while($running) {
        $connectionWait = $np.WaitForConnectionAsync()
        # wait for a client to connect
        while(!$connectionWait.IsCompleted) {
            # ideally if the client goes out of scope in the supervisor
            # its drop implementation will send a signal (0) to cleanly shut
            # down the named pipe. However if the parent is forcefully terminated,
            # we may need to clean up after ourself
            if($parent.HasExited) {
                $running = $false
                break
            }
            Start-Sleep -Milliseconds 10
        }

        if($np.IsConnected) {
            $byte = $np.ReadByte()

            # A 1 signal is a request to eun the hook
            if($byte -eq 1) {
                # exit unknown (3) if the hook throws an exception
                try {
                    & "${HookPath}.ps1"
                    $exit_code = $LASTEXITCODE
                } catch {
                    $exit_code = 3
                }
                # If a hook does not use the exit command, we assume 0
                if(!$exit_code) { $exit_code = 0 }
                $bytes = [System.BitConverter]::GetBytes($exit_code)
                $np.Write($bytes, 0, $bytes.Length)
                $np.Flush()

                # Restore all environment variables to their previous state
                Restore-Environment $origEnv
            }
            else {
                Write-Host "Quitting $PipeName pipe."
                $running = $false
            }
            $np.WaitForPipeDrain()
            $np.Disconnect()
        }
    }
} finally {
    if($np) { $np.Dispose() }
    Write-Host "exiting $PID"
}

