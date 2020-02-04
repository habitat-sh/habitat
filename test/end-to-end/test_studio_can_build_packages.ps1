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

Describe "working after success callback" {
    $result = hab pkg build test/fixtures/after-success-plan
    $exit = $LASTEXITCODE
    It "exits 0" {
        $exit | Should -Be 0
    }

    It "fires after success callback" {
        $result | Should -Contain "I am a success"
    }

    It "does not fire after failure callback" {
        $result | Should -Not -Contain "I am a failure"
    }
}

Describe "failing after success callback" {
    $result = hab pkg build test/fixtures/broken-after-success-plan
    $exit = $LASTEXITCODE
    It "exits 0" {
        $exit | Should -Be 0
    }

    It "outputs success callback error" {
        ($result | Out-String) | Should -BeLike "*success' callback failed*"
    }

    It "does not fire after failure callback" {
        $result | Should -Not -Contain "I am a failure"
    }
}

Describe "working after failure callback" {
    $result = hab pkg build test/fixtures/after-failure-plan
    $exit = $LASTEXITCODE
    It "exits 1" {
        $exit | Should -Be 1
    }

    It "fires after failure callback" {
        $result | Should -Contain "I am a failure"
    }

    It "does not fire after success callback" {
        $result | Should -Not -Contain "I am a success"
    }
}

Describe "failing after failure callback" {
    $result = hab pkg build test/fixtures/broken-after-failure-plan
    $exit = $LASTEXITCODE
    It "exits 1" {
        $exit | Should -Be 1
    }

    It "outputs failure callback error" {
        ($result | Out-String) | Should -BeLike "*failure' callback failed*"
    }

    It "does not fire after success callback" {
        $result | Should -Not -Contain "I am a success"
    }
}
