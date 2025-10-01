Describe 'hab plan render' {

    BeforeAll {
        $tempdir = New-TemporaryDirectory

        $output = hab plan render ./test/fixtures/render/consul/config/consul_config.json `
            --default-toml ./test/fixtures/render/consul/default.toml `
            --mock-data ./test/fixtures/render/consul/override.json `
            --render-dir $tempdir `
            --user-toml ./test/fixtures/render/consul/user.toml `
            --print 2>&1

        $render_exitcode = $LASTEXITCODE
        $outfile = Join-Path -Path $tempdir.FullName consul_config.json

        # PSScriptAnalyzer can't see the actual usages so | Out-Null
        $output | Out-Null
        $render_exitcode | Out-Null
        $outfile | Out-Null
    }

    AfterAll {
        Remove-Item -Path $tempdir -Recurse -Force
    }

    It 'should succeed' {
        if ( $render_exitcode -ne 0 ) {
            $output | Write-Host
        }
        $render_exitcode | Should -Be 0
    }

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
        if ($IsWindows) {
            $plan_file = 'plan.ps1'
        } else {
            $plan_file = 'plan.sh'
        }
        $testroot | Out-Null
        # PSScriptAnalyzer can't see the actual usages so | Out-Null
        $plan_file | Out-Null
    }

    BeforeEach {
        $tempdir = New-TemporaryDirectory
        Set-Location $tempdir
        $plan_path = Join-Path -Path $tempdir bar $plan_file
        # PSScriptAnalyzer can't see the actual usages so | Out-Null
        $plan_path | Out-Null
    }

    AfterEach {
        Set-Location $testroot
        Remove-Item -Path $tempdir -Recurse -Force
    }

    It 'normal plan file (scaffolding placeholder)' {
        hab plan init --origin foo bar 2>&1 | Out-Null
        $LASTEXITCODE | Should -Be 0
        $plan_path | Should -Exist
        $plan_path | Should -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-.+"'
    }

    It 'minimal plan file (scaffolding placeholder)' {
        hab plan init --origin foo --min bar 2>&1 | Out-Null
        $LASTEXITCODE | Should -Be 0
        $plan_path | Should -Exist
        $plan_path | Should -Not -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="core/scaffolding-.+"'
    }

    It 'minimal plan file with scaffolding specified' {
        hab plan init --origin foo --min bar --scaffolding go 2>&1 | Out-Null
        $LASTEXITCODE | Should -Be 0
        $plan_path | Should -Exist
        $plan_path | Should -Not -FileContentMatch "# This file is the heart of your application's habitat."
        $plan_path | Should -Not -FileContentMatch 'pkg_scaffolding="some/scaffolding"'
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="core/scaffolding-go"'
        $plan_path | Should -FileContentMatch 'pkg_scaffolding="core/scaffolding-.+"'
    }
}