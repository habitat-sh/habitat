hab origin key generate $env:HAB_ORIGIN

Function Invoke-WindowsPlanBuild($package) {
    Invoke-NativeCommand hab pkg build test/fixtures/windows_plans/$package -R | Out-Null
    . results/last_build.ps1
    @{ Artifact = $pkg_artifact; Ident = $pkg_ident }
}

Describe "package using scaffolding" {
    $dummy = Invoke-WindowsPlanBuild dummy
    $dummyHabSvcUser = Invoke-WindowsPlanBuild dummy_hab_svc_user
    $scaffolding = Invoke-WindowsPlanBuild scaffolding
    $consumer = Invoke-WindowsPlanBuild use_scaffolding
    It "inherits scaffolding dependencies" {
        hab pkg install "results/$($dummy.Artifact)"
        hab pkg install "results/$($dummyHabSvcUser.Artifact)"
        hab pkg install "results/$($scaffolding.Artifact)"
        hab pkg install "results/$($consumer.Artifact)"
        # scaffolding has dummy as runtime and dummy_hab_svc_user as build time deps
        
        "/hab/pkgs/$($consumer.Ident)/DEPS" | Should -FileContentMatch "habitat-testing/dummy"
        "/hab/pkgs/$($consumer.Ident)/BUILD_DEPS" | Should -FileContentMatch "habitat-testing/dummy-hab-user"
    }
}
