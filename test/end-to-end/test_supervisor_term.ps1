# Initialize shared setup values in the current scope for sibling Pester blocks.
$setup = {
    Set-Variable -Name tempFile -Value (Join-Path ([System.IO.Path]::GetTempPath()) "testpkgstophook.out") -Scope Local
    $supLog = New-SupervisorLogFile("test_supervisor_term")
    Set-Variable -Name launcherProc -Value (Start-Supervisor -LogFile $supLog -Timeout 45) -Scope Local
}
if ($usePester5) {
    BeforeAll $setup
} else {
    . $setup
}

Describe "hab sup term" {
    BeforeAll {
        hab origin key generate $env:HAB_ORIGIN
        Invoke-BuildAndInstall testpkgstophook
        Load-SupervisorService "$env:HAB_ORIGIN/testpkgstophook"
        hab sup term
        $supProc = Get-Process "hab-sup"
        $supProc.WaitForExit(5000)
        $launcherProc.WaitForExit(10000)
    }

    It "should terminate launcher" {
        $launcherProc.HasExited | Should -Be $true
    }
    It "should gracefully terminate service run hook" {
        $tempFile | Should -FileContentMatch "run hook is terminating"
    }
    It "should fire stop hook" {
        $tempFile | Should -FileContentMatch "post-stop hook has fired"
    }
}
