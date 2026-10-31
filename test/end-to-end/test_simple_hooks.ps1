# Initialize shared setup values in the current scope for sibling Pester blocks.
Describe "Simple hooks output" {
    $setup = {
        Set-Variable -Name TestStartTime -Value (Get-Date) -Scope Local
        $supLog = New-SupervisorLogFile("test_simple_hooks")
        Start-Supervisor -LogFile $supLog -Timeout 45 | Out-Null

        $pkg = "$(Get-EndToEndTestingOrigin)/simple-hooks"
        $svc = Load-SupervisorService -PackageName $pkg
        Set-Variable -Name pkgLogsPath -Value (Join-Path -Path $env:SystemDrive -ChildPath hab -AdditionalChildPath @("svc", $svc, "logs")) -Scope Local
    }
    if ($useWindowsPester5) {
        BeforeAll $setup
    } else {
        . $setup
    }

    It "Has correct 'install' hook stdout" {
        $path = Join-Path $pkgLogsPath "install.stdout.log"
        Wait-PathHasContentUpdatedAfter $path $TestStartTime 10
        $path | Should -FileContentMatchExactly "install hook $svc"
    }

    It "Has correct 'init' hook stdout" {
        $path = Join-Path $pkgLogsPath "init.stdout.log"
        Wait-PathHasContentUpdatedAfter $path $TestStartTime 10
        $path | Should -FileContentMatchExactly "init hook $svc"
    }

    It "Has correct 'run' hook stdout" {
        $out = Get-Content $supLog
        $out | Should -Contain "$svc.default(O): run hook $svc"
    }

    It "Has correct 'post-run' hook stdout" {
        $path = Join-Path $pkgLogsPath "post-run.stdout.log"
        Wait-PathHasContentUpdatedAfter $path $TestStartTime 10
        $path | Should -FileContentMatchExactly "post-run hook $svc"
    }

    $orderedTests = {
        $setup = {
            Set-Variable -Name beforeUnloadTime -Value (Get-Date) -Scope Local
            Unload-SupervisorService -PackageName $pkg -Timeout 20
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "Has correct 'post-stop' hook stdout" {
            $path = Join-Path $pkgLogsPath "post-stop.stdout.log"
            Wait-PathHasContentUpdatedAfter $path $beforeUnloadTime 10
            $path | Should -FileContentMatchExactly "post-stop hook $svc"
        }
    }
    if ($useWindowsPester5) {
        Context "after unloading the service" $orderedTests
    } else {
        . $orderedTests
    }
}
