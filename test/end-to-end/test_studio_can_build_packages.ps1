# `build` is a built-in helper function that maps to `hab pkg exec core/hab-plan-build` 
# rather than `hab pkg build` to avoid 'studio-in-studio' situations. Verify that the 
# command functions. We assume that if the build succeeds (exits 0) we've passed this 
# test, and leave more detailed testing to the build output to e2e tests for hab-plan-build

hab origin key generate $env:HAB_ORIGIN

Describe "Studio build" {
    It "builds a simple package" {
       hab pkg build test/fixtures/minimal-package
       $LASTEXITCODE | Should -Be 0
    }
}
