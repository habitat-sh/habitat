# Basic set of tests for the hab pkg download command
#
# There are a number of pieces of this which are fragile, and could be
# implemented in a more clever fashion. There are many opportunites
# for cleaner code and more fine grained tests. However, they are a
# bit of a pain to program in bash. This is intended to provide
# minimal testing pending our figuring out the best approach for
# command line testing.
#
# Assumptions:
# 1. ${CACHE_DIR} can be set to a writable location on the filesystem


# Test the Habitat package downloader.
#
# Uses the `HAB_INTERNAL_BLDR_CHANNEL` environment variable to control
# the base packages channel for the exporter.
#
# Developers most likely want to run:
# HAB_TEST_CMD=./target/debug/hab test/end-to-end/test_pkg_download.sh
#
$cacheDir = "test-cache"

function Test-IdentDownloaded($FilePrefix) {
    $path = Join-Path $cacheDir "artifacts" "$FilePrefix-*"
    if(!(Test-Path $path)) {
        Write-Error "$path was not found."
    }
}

function Test-GzipIdents {
    Test-IdentDownloaded "core-gzip"
    Test-IdentDownloaded "core-glibc"
    Test-IdentDownloaded "core-gcc-libs"
    Test-IdentDownloaded "core-grep"
    Test-IdentDownloaded "core-linux-headers"
    Test-IdentDownloaded "core-pcre"
    Test-IdentDownloaded "core-less"
    Test-IdentDownloaded "core-ncurses"

    if((Get-ChildItem (Join-Path $cacheDir "artifacts") -File).Count -ne 8) {
        Write-Error "did not find 8 gzip artifacts"
    }
}

function Test-RustIdents {
    Test-IdentDownloaded "core-rust"
    Test-IdentDownloaded "core-visual-cpp-redist-2015"
    Test-IdentDownloaded "core-visual-cpp-build-tools-2015"

    if((Get-ChildItem (Join-Path $cacheDir "artifacts") -File).Count -ne 3) {
        Write-Error "did not find 3 gzip artifacts"
    }
}

Describe "hab pkg download" {
    $identFile = "ident_file"
    $fixtures = "$PSScriptRoot/fixtures/pkg_download"

    BeforeEach {
        if(Test-Path $cacheDir) {
            Remove-Item $cacheDir -Recurse -Force
        }
    }

    It "'hab pkg download --channel stable --download-directory $cacheDir core/gzip' succeeds" {
        hab pkg download --channel stable --download-directory $cacheDir core/gzip
        Test-GzipIdents
    }
    It "'hab pkg download --channel stable --download-directory $cacheDir --file $identFile' succeeds" {
        Set-Content $identFile -Value "core/gzip"
        hab pkg download --channel stable --download-directory $cacheDir --file $identFile
        Test-GzipIdents
    }
    It "'hab pkg download --channel stable --download-directory $cacheDir --file $identFile' succeeds with comments and empty lines" {
        Set-Content $identFile -Value @"
# this is a series
# of comments, followed by empty lines and whitespaces

 core/gzip 
"@
        hab pkg download --channel stable --download-directory $cacheDir --file $identFile
        Test-GzipIdents
    }
    It "'hab pkg download --channel stable --download-directory $cacheDir core/rust --target=x86_64-windows' succeeds" {
        hab pkg download --channel stable --download-directory $cacheDir core/rust --target=x86_64-windows
        Test-RustIdents
    }
    It "fails when package is invalid" {
        hab pkg download --download-directory $cacheDir arglebargle
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when no package is provided" {
        hab pkg download --download-directory $cacheDir
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when invalid package is provided in file" {
        Set-Content $identFile -Value "arglebargle"
        hab pkg download --channel stable --download-directory $cacheDir --file $identFile
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when package does not exist" {
        hab pkg download --download-directory $cacheDir "core/half_life_4"
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when cache dir cannot be created" {
        New-Item $cacheDir
        hab pkg download --download-directory $cacheDir "core/gzip"
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when target is invalid" {
        hab pkg download --download-directory $cacheDir "core/gzip" --target=6502-commodore
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when token is invalid" {
        hab pkg download --download-directory $cacheDir "core/gzip" --auth asdfa
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when url is invalid" {
        hab pkg download --download-directory $cacheDir "core/gzip" --url https://www.example.org
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when channel is invalid" {
        hab pkg download --download-directory $cacheDir "core/gzip" --channel number_5
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "succeeds with valid toml" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/happy_path.toml"
        $LASTEXITCODE | Should -Be 0
    }
    It "succeeds with valid toml without a header" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/no_header.toml"
        $LASTEXITCODE | Should -Be 0
    }
    It "fails with toml that has a bad header" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/bad_header.toml"
        $LASTEXITCODE | Should -Be 1
    }
    It "fails with toml that has a bad ident" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/bad_ident.toml"
        $LASTEXITCODE | Should -Be 1
    }
    It "fails with toml that has a bad target" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/bad_target.toml"
        $LASTEXITCODE | Should -Be 1
    }
    It "fails with toml that has no target" {
        hab pkg download --download-directory $cacheDir --file "$fixtures/no_target.toml"
        $LASTEXITCODE | Should -Be 1
    }
}
