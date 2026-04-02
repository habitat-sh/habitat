# macOS-specific external binaries tests.
# Tests that hab can find and execute binaries from installed packages.
# Container and Tar exporters are not compiled for macOS, so we test
# hab pkg exec instead, which is the underlying mechanism for running
# external package binaries.

$channel = "aarch64-darwin"

Describe "`hab` correctly executes external binaries" {
    BeforeAll {
        hab pkg install core/gzip --channel $channel
        hab pkg install core/less --channel $channel
    }

    It "`hab pkg exec` runs gzip from the package" {
        $out = hab pkg exec core/gzip gzip --version 2>&1 | Select-Object -First 1
        $LASTEXITCODE | Should -Be 0
        $out | Should -BeLike "*gzip*"
    }

    It "`hab pkg exec` runs less from the package" {
        $out = hab pkg exec core/less less --version 2>&1 | Select-Object -First 1
        $LASTEXITCODE | Should -Be 0
        $out | Should -BeLike "*less*"
    }

    It "`hab pkg exec` with nonexistent package fails" {
        hab pkg exec core/nonexistent-pkg-12345 some-binary 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "`hab pkg exec` with nonexistent binary fails" {
        hab pkg exec core/gzip nonexistent-binary-12345 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "`hab pkg export` with bad exporter fails gracefully" {
        hab pkg export a_bad_exporter --help 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }
}
