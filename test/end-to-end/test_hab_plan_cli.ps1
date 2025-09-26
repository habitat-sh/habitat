Describe 'hab plan render' {

    BeforeAll {
        $tempdir = New-TemporaryDirectory

        $output = hab plan render ./test/fixtures/render/consul/config/consul_config.json `
            --default-toml ./test/fixtures/render/consul/default.toml `
            --mock-data ./test/fixtures/render/consul/override.json `
            --render-dir $tempdir `
            --user-toml ./test/fixtures/render/consul/user.toml `
            --print 2>&1
        $output | Out-Null # for PSScriptAnalyzer which can't see actual usage

        $outfile = Join-Path -Path $tempdir consul_config.json
        $outfile | Out-Null # for PSScriptAnalyzer which can't see actual usage
    }

    AfterAll { Remove-Item -Path $tempdir -Recurse -Force }

    It '--print should work' {
        $output | Should -Not -BeNullOrEmpty
    }

    It '--render-dir should work' {
        $outfile | Should -Exist
    }

    It '--default-toml should work' {
        $outfile | Should -FileContentMatch 'IN_DEFAULT_TOML'
    }

    It '--mock-data should work' {
        $outfile | Should -FileContentMatch 'IN_OVERRIDE_JSON'
    }

    It '--user-toml should work' {
        $outfile | Should -FileContentMatch 'IN_USER_TOML'
    }
}

Describe 'hab plan init' {

    BeforeAll {
        $testroot = Get-Location
        $testroot | Out-Null # for PSScriptAnalyzer which can't see actual usage
        if ($IsWindows) {
            $plan_file = 'plan.ps1'
        } else {
            $plan_file = 'plan.sh'
        }
        $plan_file | Out-Null # for PSScriptAnalyzer which can't see actual usage
        # This always be just hab for committed code but if you are working
        # locally to extend or debug these tests then you can swap the
        # variables to use your locally built hab binary
        # $hab = Join-Path -Path $testroot target debug hab
        $hab = 'hab'
        $hab | Out-Null # for PSScriptAnalyzer which can't see actual usage
    }

    BeforeEach {
        $tempdir = New-TemporaryDirectory
        Set-Location $tempdir
        $plan_path = Join-Path -Path $tempdir bar $plan_file
        $plan_path | Out-Null # for PSScriptAnalyzer which can't see actual usage
    }

    AfterEach {
        Set-Location $testroot
        Remove-Item -Path $tempdir -Recurse -Force
    }

    # NOTE about the tests that follow.  When testing the scaffolding there was
    # some weirdness with string interpolation where $ needed to be escaped even
    # inside of a single quoted string and $ is Windows specific which we want
    # to smooth over anyway.  So the scaffolding asserts intentionally drop '#'
    # and '$' to just stay away from any weirdness at all.  In addition, there
    # was an issue in which the code was defaulting to "core/scaffolding-ruby"
    # even when the --scaffolding option wasn't specified so the explicit
    # denial of "core/scaffolding-ruby" is about making sure that defaulting
    # is not occuring somehow and testing "core/scaffolding-go" is about making
    # sure the explicit choice was exercised.

    It 'normal plan file (scaffolding placeholder)' {
        Invoke-NativeCommand "$hab" plan init --origin foo bar 2>&1 | Out-Null
        $plan_path | Should -Exist
        $plan_path | Should -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-ruby"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-go"'
    }

    It 'minimal plan file (scaffolding placeholder)' {
        Invoke-NativeCommand "$hab" plan init --origin foo --min bar 2>&1 | Out-Null
        $plan_path | Should -Exist
        $plan_path | Should -Not -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-ruby"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-go"'
    }

    It 'minimal plan file with scaffolding specified' {
        Invoke-NativeCommand "$hab" plan init --origin foo --min --scaffolding go bar 2>&1 | Out-Null
        $plan_path | Should -Exist
        $plan_path | Should -Not -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-ruby"'
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="core/scaffolding-go"'
    }
}