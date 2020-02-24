# Test the at-once service update strategy

$supLog = New-TemporaryFile
Start-Supervisor -LogFile $supLog -Timeout 45 | Out-Null
$testChannel="at-once-$([DateTime]::Now.Ticks)"
$pkg="habitat-testing/nginx"
$initialRelease="habitat-testing/nginx/1.17.4/20191115184838"
$updatedRelease="habitat-testing/nginx/1.17.4/20191115185517"

Describe "at-once update" {
    hab pkg promote $initialRelease $testChannel
    Load-SupervisorService $pkg -Strategy "at-once" -Channel $testChannel

    It "loads initial release" {
        Wait-Release -Ident $initialRelease
    }

    Context "promote update" {
        hab pkg promote $updatedRelease $testChannel

        It "updates release" {
            Wait-Release -Ident $updatedRelease
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
        Unload-SupervisorService -PackageName $pkg -Timeout 20
        Stop-Supervisor
    }
}
