[Diagnostics.CodeAnalysis.SuppressMessage("PSUseCorrectCasing", '')]
param ()

# `build` is a built-in helper function that maps to `hab pkg exec core/hab-plan-build`
# rather than `hab pkg build` to avoid 'studio-in-studio' situations. Verify that the
# command functions. We assume that if the build succeeds (exits 0) we've passed this
# test, and leave more detailed testing to the build output to e2e tests for hab-plan-build
hab origin key generate $env:HAB_ORIGIN

Describe "Studio build" {
    It "does not build plan-in-root-and-habitat" {
        hab pkg build test/fixtures/plan-in-root-and-habitat
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "does not build plan-in-none" {
        Copy-Item "components/studio/bin/hab-studio.sh" "/hab/pkgs/core/hab-studio/1.6.587/20220930210438/bin/hab-studio"
        write-host $(hab pkg build test/fixtures/plan-in-none)
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "builds plan in target if also in root" {
        write-host $(Invoke-Build plan-in-root-and-target)
        . ./results/last_build.ps1

        $pkg_name | Should -Be "target_plan"
    }
}
