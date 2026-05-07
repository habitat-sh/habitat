[Diagnostics.CodeAnalysis.SuppressMessage("PSUseCorrectCasing", '')]
param ()


#  We assume that if the build succeeds (exits 0) we've passed this
# test, and leave more detailed testing to the build output to e2e tests for hab-plan-build
hab origin key generate $env:HAB_ORIGIN

Describe "Studio build" {
    It "builds package" {
        hab pkg build test/fixtures/plan-in-root -D
        $LASTEXITCODE | Should -Be 0
    }
}
