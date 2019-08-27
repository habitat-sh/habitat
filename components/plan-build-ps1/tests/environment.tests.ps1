. $PSScriptRoot\..\bin\shared.ps1
. $PSScriptRoot\..\bin\environment.ps1

Describe "Invoke-SetupEnvironmentWrapper" {
    New-Item "TestDrive:\src" -ItemType Directory -Force | Out-Null
    $env:FS_ROOT = (Get-PSDrive TestDrive).Root
    $envvars = @{}

    Mock New-Item { $envvars[$name] = $value } -ParameterFilter {$Path -eq "Env:"}
    
    $script:HAB_PKG_PATH = Join-Path $env:FS_ROOT "hab\pkgs"
    $script:originalPath = "TestDrive:\src"
    $script:pkg_origin = "testorigin"
    $script:pkg_name = "testpkg"
    $script:pkg_version = "0.1.0"
    $script:pkg_release = "30300101010000"
    $script:pkg_prefix = "$HAB_PKG_PATH\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"
    $unrooted = "\hab\pkgs\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"

    Context "Unrooted values" {
        function Invoke-SetupEnvironment {
            Set-RuntimeEnv -IsPath "test_set_run_var" "$unrooted\test_set_run_var"
            Set-BuildtimeEnv -IsPath "test_set_build_var" "$unrooted\test_set_build_var"
            Push-RuntimeEnv -IsPath "test_push_run_var" "$unrooted\test_push_run_var1"
            Push-BuildtimeEnv -IsPath "test_push_build_var" "$unrooted\test_push_build_var1"
            Push-RuntimeEnv -IsPath "test_push_run_var" "$unrooted\test_push_run_var2"
            Push-BuildtimeEnv -IsPath "test_push_build_var" "$unrooted\test_push_build_var2"
        }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should set runtime var to the rooted value" {
            $envvars["test_set_run_var"] | Should -Be "$pkg_prefix\test_set_run_var"
        }
        It "Should set build time var to the rooted value" {
            $envvars["test_set_build_var"] | Should -Be "$pkg_prefix\test_set_build_var"
        }
        It "Should push runtime var to the rooted value" {
            $envvars["test_push_run_var"] | Should -Be "$pkg_prefix\test_push_run_var2;$pkg_prefix\test_push_run_var1"
        }
        It "Should push build time var to the rooted value" {
            $envvars["test_push_build_var"] | Should -Be "$pkg_prefix\test_push_build_var2;$pkg_prefix\test_push_build_var1"
        }
        It "Should store runtime var to the unrooted value" {
            $env["runtime"]["test_set_run_var"].Value | Should -Be "$unrooted\test_set_run_var"
        }
        It "Should store build time var to the urooted value" {
            $env["buildtime"]["test_set_build_var"].Value | Should -Be "$unrooted\test_set_build_var"
        }
        It "Should store pushed runtime var to the urooted value" {
            $env["runtime"]["test_push_run_var"].Value | Should -Be "$unrooted\test_push_run_var2;$unrooted\test_push_run_var1"
        }
        It "Should store pushed build time var to the urooted value" {
            $env["buildtime"]["test_push_build_var"].Value | Should -Be "$unrooted\test_push_build_var2;$unrooted\test_push_build_var1"
        }
    }

    Context "Rooted values" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }

        function Invoke-SetupEnvironment {
            Set-RuntimeEnv -IsPath "test_set_run_var" "$pkg_prefix\test_set_run_var"
            Set-BuildtimeEnv -IsPath "test_set_build_var" "$pkg_prefix\test_set_build_var"
            Push-RuntimeEnv -IsPath "test_push_run_var" "$pkg_prefix\test_push_run_var1"
            Push-BuildtimeEnv -IsPath "test_push_build_var" "$pkg_prefix\test_push_build_var1"
            Push-RuntimeEnv -IsPath "test_push_run_var" "$pkg_prefix\test_push_run_var2"
            Push-BuildtimeEnv -IsPath "test_push_build_var" "$pkg_prefix\test_push_build_var2"
        }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should set runtime var to the rooted value" {
            $envvars["test_set_run_var"] | Should -Be "$pkg_prefix\test_set_run_var"
        }
        It "Should set build time var to the rooted value" {
            $envvars["test_set_build_var"] | Should -Be "$pkg_prefix\test_set_build_var"
        }
        It "Should push runtime var to the rooted value" {
            $envvars["test_push_run_var"] | Should -Be "$pkg_prefix\test_push_run_var2;$pkg_prefix\test_push_run_var1"
        }
        It "Should push build time var to the rooted value" {
            $envvars["test_push_build_var"] | Should -Be "$pkg_prefix\test_push_build_var2;$pkg_prefix\test_push_build_var1"
        }
        It "Should store runtime var to the unrooted value" {
            $env["runtime"]["test_set_run_var"].Value | Should -Be "$unrooted\test_set_run_var"
        }
        It "Should store build time var to the urooted value" {
            $env["buildtime"]["test_set_build_var"].Value | Should -Be "$unrooted\test_set_build_var"
        }
        It "Should store pushed runtime var to the urooted value" {
            $env["runtime"]["test_push_run_var"].Value | Should -Be "$unrooted\test_push_run_var2;$unrooted\test_push_run_var1"
        }
        It "Should store pushed build time var to the urooted value" {
            $env["buildtime"]["test_push_build_var"].Value | Should -Be "$unrooted\test_push_build_var2;$unrooted\test_push_build_var1"
        }
    }

    Context "Non path values" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }

        function Invoke-SetupEnvironment {
            Set-RuntimeEnv "test_set_run_var" "$pkg_prefix\test_set_run_var"
            Set-BuildtimeEnv "test_set_build_var" "$pkg_prefix\test_set_build_var"
            Push-RuntimeEnv "test_push_run_var" "$pkg_prefix\test_push_run_var1"
            Push-BuildtimeEnv "test_push_build_var" "$pkg_prefix\test_push_build_var1"
            Push-RuntimeEnv "test_push_run_var" "$pkg_prefix\test_push_run_var2"
            Push-BuildtimeEnv "test_push_build_var" "$pkg_prefix\test_push_build_var2"
        }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should set runtime var to the given value" {
            $envvars["test_set_run_var"] | Should -Be "$pkg_prefix\test_set_run_var"
        }
        It "Should set build time var to the given value" {
            $envvars["test_set_build_var"] | Should -Be "$pkg_prefix\test_set_build_var"
        }
        It "Should push runtime var to the given value" {
            $envvars["test_push_run_var"] | Should -Be "$pkg_prefix\test_push_run_var2;$pkg_prefix\test_push_run_var1"
        }
        It "Should push build time var to the given value" {
            $envvars["test_push_build_var"] | Should -Be "$pkg_prefix\test_push_build_var2;$pkg_prefix\test_push_build_var1"
        }
        It "Should store runtime var to the given value" {
            $env["runtime"]["test_set_run_var"].Value | Should -Be "$pkg_prefix\test_set_run_var"
        }
        It "Should store build time var to the given value" {
            $env["buildtime"]["test_set_build_var"].Value | Should -Be "$pkg_prefix\test_set_build_var"
        }
        It "Should store pushed runtime var to the given value" {
            $env["runtime"]["test_push_run_var"].Value | Should -Be "$pkg_prefix\test_push_run_var2;$pkg_prefix\test_push_run_var1"
        }
        It "Should store pushed build time var to the given value" {
            $env["buildtime"]["test_push_build_var"].Value | Should -Be "$pkg_prefix\test_push_build_var2;$pkg_prefix\test_push_build_var1"
        }
    }

    Context "Dependency ENVIRONMENT_PATHS" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:pkg_all_deps_resolved = @()

        $script:pkg_deps = @("core/run-dep")
        $script:pkg_build_deps = @("core/build-dep")
        
        ($pkg_deps + $pkg_build_deps) | % {
            $pkg_path = Join-path $HAB_PKG_PATH "$_\0.1.0\111"
            $unrooted_pkg_path = Join-path "\hab\pkgs" "$_\0.1.0\111"
            $pkg_all_deps_resolved += $pkg_path
            mkdir $pkg_path -Force | Out-Null
            $dep_name = ($_ -split "/")[1]
            "$_/0.1.0/111" | Out-File "$pkg_path\IDENT"
            # We will duplicate ENVIRONMENT and ENVIRONMENT_PATHS to simulate actual behavior
            "${dep_name}_path=$unrooted_pkg_path\run" | Out-File "$pkg_path\RUNTIME_ENVIRONMENT" -Append
            "${dep_name}_path=$unrooted_pkg_path\run" | Out-File "$pkg_path\RUNTIME_ENVIRONMENT_PATHS" -Append
            "${dep_name}_build_path=$unrooted_pkg_path\build" | Out-File "$pkg_path\BUILDTIME_ENVIRONMENT" -Append
            "${dep_name}_build_path=$unrooted_pkg_path\build" | Out-File "$pkg_path\BUILDTIME_ENVIRONMENT_PATHS" -Append
        }
        function Invoke-SetupEnvironment {
            Push-RuntimeEnv -IsPath "run-dep_path" "$pkg_prefix\run_dir"
        }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should set rooted runtime env from dep and push setup value" {
            $envvars["run-dep_path"] | Should -Be "$pkg_prefix\run_dir;$HAB_PKG_PATH\core\run-dep\0.1.0\111\run"
        }
        It "Should set rooted build time env from dep" {
            $envvars["build-dep_build_path"] | Should -Be "$HAB_PKG_PATH\core\build-dep\0.1.0\111\build"
        }
    }

    Context "Dependency unrooted ENVIRONMENT" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:pkg_all_deps_resolved = @()

        $script:pkg_deps = @("core/run-dep")
        $script:pkg_build_deps = @("core/build-dep")
        
        ($pkg_deps + $pkg_build_deps) | % {
            $pkg_path = Join-path $HAB_PKG_PATH "$_\0.1.0\111"
            $unrooted_pkg_path = Join-path "\hab\pkgs" "$_\0.1.0\111"
            $pkg_all_deps_resolved += $pkg_path
            mkdir $pkg_path -Force | Out-Null
            $dep_name = ($_ -split "/")[1]
            "$_/0.1.0/111" | Out-File "$pkg_path\IDENT"
            "${dep_name}_path=$unrooted_pkg_path\run" | Out-File "$pkg_path\RUNTIME_ENVIRONMENT" -Append
            "${dep_name}_build_path=$unrooted_pkg_path\build" | Out-File "$pkg_path\BUILDTIME_ENVIRONMENT" -Append
        }
        function Invoke-SetupEnvironment { }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should set unrooted runtime env from dep" {
            $envvars["run-dep_path"] | Should -Be "\hab\pkgs\core\run-dep\0.1.0\111\run"
        }
        It "Should set unrooted build time env from dep" {
            $envvars["build-dep_build_path"] | Should -Be "\hab\pkgs\core\build-dep\0.1.0\111\build"
        }
    }

    Context "PSModulePath" {
        Mock Get-Content { $envvars["PSModulePath"] } -ParameterFilter {$Path -eq "env:\PSModulePath"}
        Mock Test-Path { $envvars.ContainsKey("PSModulePath") } -ParameterFilter {$Path -eq "env:\PSModulePath"}

        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:pkg_all_deps_resolved = @()

        $script:pkg_deps = @("core/run-dep")
        $script:pkg_build_deps = @("core/build-dep")
        
        ($pkg_deps + $pkg_build_deps) | % {
            $pkg_path = Join-path $HAB_PKG_PATH "$_\0.1.0\111"
            $unrooted_pkg_path = Join-path "\hab\pkgs" "$_\0.1.0\111"
            $pkg_all_deps_resolved += $pkg_path
            mkdir $pkg_path -Force | Out-Null
            $dep_name = ($_ -split "/")[1]
            "$_/0.1.0/111" | Out-File "$pkg_path\IDENT"
            "PSModulePath=$unrooted_pkg_path\modules" | Out-File "$pkg_path\RUNTIME_ENVIRONMENT" -Append
            "PSModulePath=$unrooted_pkg_path\build_modules" | Out-File "$pkg_path\BUILDTIME_ENVIRONMENT" -Append
        }
        function Invoke-SetupEnvironment {
            Push-RuntimeEnv -IsPath "PSModulePath" "$unrooted\modules"
            Push-BuildtimeEnv -IsPath "PSModulePath" "$unrooted\build_modules"
        }
        
        Invoke-SetupEnvironmentWrapper
    
        It "Should layer all rooted assignments" {
            $envvars["PSModulePath"] | Should -Be "$pkg_prefix\build_modules;$HAB_PKG_PATH\core\build-dep\0.1.0\111\build_modules;$pkg_prefix\modules;$HAB_PKG_PATH\core\run-dep\0.1.0\111\modules"
        }
    }
}

Describe "Write-EnvironmentFiles" {
    New-Item "TestDrive:\src" -ItemType Directory -Force | Out-Null
    $env:FS_ROOT = (Get-PSDrive TestDrive).Root

    Mock New-Item { } -ParameterFilter {$Path -eq "Env:"}
    
    $script:HAB_PKG_PATH = Join-Path $env:FS_ROOT "hab\pkgs"
    $script:originalPath = "TestDrive:\src"
    $script:pkg_origin = "testorigin"
    $script:pkg_name = "testpkg"
    $script:pkg_version = "0.1.0"
    $script:pkg_release = "30300101010000"
    $script:pkg_prefix = "$HAB_PKG_PATH\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"
    $unrooted = "\hab\pkgs\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"

    $script:pkg_all_deps_resolved = @()
    $script:pkg_deps = @()
    $script:pkg_build_deps = @()

    Context "environment path values" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }
        mkdir $pkg_prefix -Force | Out-Null
        function Invoke-SetupEnvironment {
            Set-RuntimeEnv -IsPath "test_set_run_var" "$pkg_prefix\test_set_run_var"
            Set-BuildtimeEnv -IsPath "test_set_build_var" "$pkg_prefix\test_set_build_var"
        }
        
        Invoke-SetupEnvironmentWrapper
        Write-EnvironmentFiles

        It "Should write runtime var to RUNTIME_ENVIRONMENT" {
            Get-Content "$pkg_prefix\RUNTIME_ENVIRONMENT" | Should -Be "test_set_run_var=$unrooted\test_set_run_var"
        }
        It "Should write runtime var to RUNTIME_ENVIRONMENT_PATHS" {
            Get-Content "$pkg_prefix\RUNTIME_ENVIRONMENT_PATHS" | Should -Be "test_set_run_var=$unrooted\test_set_run_var"
        }
        It "Should write buildtime var to BUILDTIME_ENVIRONMENT" {
            Get-Content "$pkg_prefix\BUILDTIME_ENVIRONMENT" | Should -Be "test_set_build_var=$unrooted\test_set_build_var"
        }
        It "Should write runtime var to BUILDTIME_ENVIRONMENT_PATHS" {
            Get-Content "$pkg_prefix\BUILDTIME_ENVIRONMENT_PATHS" | Should -Be "test_set_build_var=$unrooted\test_set_build_var"
        }
    }

    Context "environment values" {
        $script:env = @{
            RunTime = @{}
            BuildTime = @{}
        }
        $script:provenance = @{
            RunTime = @{}
            BuildTime = @{}
        }
        mkdir $pkg_prefix -Force | Out-Null
        function Invoke-SetupEnvironment {
            Set-RuntimeEnv "test_set_run_var" "$pkg_prefix\test_set_run_var"
            Set-BuildtimeEnv "test_set_build_var" "$pkg_prefix\test_set_build_var"
        }
        
        Invoke-SetupEnvironmentWrapper
        Write-EnvironmentFiles

        It "Should write runtime var to RUNTIME_ENVIRONMENT" {
            Get-Content "$pkg_prefix\RUNTIME_ENVIRONMENT" | Should -Be "test_set_run_var=$pkg_prefix\test_set_run_var"
        }
        It "Should not write runtime var to RUNTIME_ENVIRONMENT_PATHS" {
            "$pkg_prefix\RUNTIME_ENVIRONMENT_PATHS" | Should -Not -Exist
        }
        It "Should write buildtime var to BUILDTIME_ENVIRONMENT" {
            Get-Content "$pkg_prefix\BUILDTIME_ENVIRONMENT" | Should -Be "test_set_build_var=$pkg_prefix\test_set_build_var"
        }
        It "Should not write runtime var to BUILDTIME_ENVIRONMENT_PATHS" {
            "$pkg_prefix\BUILDTIME_ENVIRONMENT_PATHS" | Should -Not -Exist
        }
    }
}
