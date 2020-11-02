# Regression test to address https://github.com/habitat-sh/habitat/issues/4673
# Fixed in https://github.com/habitat-sh/habitat/pull/5365

Function Get-SocketFile {
    Get-ChildItem /tmp/rust-ipc-socket.* | ForEach-Object { $_.Name }
}

Describe "Supervisor shutdown" {
    $socket_files_before = New-TemporaryFile
    Get-SocketFile | Out-File $socket_files_before

    $supLog = New-SupervisorLogFile("test_socket_file_cleanup")
    $launcher_proc = Start-Supervisor -LogFile $supLog -Timeout 60
    hab sup term
    $launcher_proc.WaitForExit()

    $socket_files_after = New-TemporaryFile
    Get-SocketFile | Out-File $socket_files_after

    It "cleans up socket files" {
        Compare-Object $socket_files_before $socket_files_after -ExcludeDifferent | Should -Be $null
    }
}
