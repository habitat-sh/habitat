$supervisor = Start-Supervisor -Timeout 45

$pkg = "$EndToEndTestingOrigin/simple-hooks"

Describe "Simple hooks output" {
    $svc = Load-SupervisorService -PackageName $pkg -Timeout 20
    $pkgLogsPath = Join-Path $env:SystemDrive hab svc $svc logs

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
        $out = Receive-Job -Job $supervisor -ErrorAction SilentlyContinue
        $out | Should -Contain "$svc.default(O): run hook $svc"
    }

    It "Has correct 'post-run' hook stdout" { 
        $path = Join-Path $pkgLogsPath "post-run.stdout.log"
        Wait-PathHasContentUpdatedAfter $path $TestStartTime 10
        $path | Should -FileContentMatchExactly "post-run hook $svc"
    }

    $beforeUnloadTime = Get-Date
    Unload-SupervisorService -PackageName $pkg -Timeout 20

    It "Has correct 'post-stop' hook stdout" {
        $path = Join-Path $pkgLogsPath "post-stop.stdout.log"
        Wait-PathHasContentUpdatedAfter $path $beforeUnloadTime 10
        $path | Should -FileContentMatchExactly "post-stop hook $svc"
    }
}
