# macOS-specific external binaries tests.
# Tests that hab can find and execute binaries from installed packages.
# Container and Tar exporters are not compiled for macOS, so we test
# hab pkg exec instead, which is the underlying mechanism for running
# external package binaries.

$channel = "aarch64-darwin"

Describe "`hab` correctly executes external binaries" {
    BeforeAll {
        hab pkg install core/gzip --channel $channel
    }

    It "`hab pkg exec` runs gzip from the package" {
        # Cannot use --version/--help/-V because clap intercepts those flags
        # before they reach the actual binary via execvp().
        # Instead, compress a file and verify the output exists.
        "hello habitat" | Set-Content -Path /tmp/hab_test_gzip_input.txt
        hab pkg exec core/gzip gzip /tmp/hab_test_gzip_input.txt
        $LASTEXITCODE | Should -Be 0
        "/tmp/hab_test_gzip_input.txt.gz" | Should -Exist
        Remove-Item -Force /tmp/hab_test_gzip_input.txt.gz
    }

    It "`hab pkg exec` can decompress data" {
        # Verify gzip decompression works via pipe (gzip -d reads stdin)
        "test data" | Set-Content -Path /tmp/hab_test_gzip2.txt
        hab pkg exec core/gzip gzip /tmp/hab_test_gzip2.txt
        hab pkg exec core/gzip gzip -d /tmp/hab_test_gzip2.txt.gz
        $LASTEXITCODE | Should -Be 0
        "/tmp/hab_test_gzip2.txt" | Should -Exist
        Get-Content /tmp/hab_test_gzip2.txt | Should -Be "test data"
        Remove-Item -Force /tmp/hab_test_gzip2.txt
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
