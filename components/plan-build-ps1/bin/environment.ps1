$script:env = @{
    RunTime = @{}
    BuildTime = @{}
}
$script:provenance = @{
    RunTime = @{}
    BuildTime = @{}
}

# (I'm sure there are many other common variables we could add here;
# PRs welcome!)
$__well_known_aggregate_env_vars=@{
    # Shell
    INCLUDE=";"
    LIB=";"
    PATH=";"
    PATHEXT=";"
    PSModulePath=";"

    # Go
    GOPATH=";"

    # Java
    CLASSPATH=";"

    # NodeJS
    NODE_PATH=";"

    # Python
    PYTHONPATH=";"

    # Ruby
    BUNDLE_PATH=";"
    BUNDLE_WITHOUT=";"
    GEM_PATH=";"
    RUBYLIB=";"
    RUBYPATH=";"
}

function Invoke-SetupEnvironment {
}

function Invoke-SetupEnvironmentWrapper {
    Write-BuildLine "Setting up environment"
    Write-BuildLine "Populating runtime environment from dependencies"
    __populate_environment_from_deps "RunTime"
    Write-BuildLine "Populating buildtime environment from dependencies"
    __populate_environment_from_deps "BuildTime"

    Invoke-SetupEnvironment

    Write-BuildLine "Layering runtime environment on top of system environment"
    # Export everything from our collected runtime environment into
    # the real environment, except for PATH; for that, push the
    # runtime path onto the front of the system path
    foreach($k in $env["Runtime"].keys) {
        if(@("PATH", "LIB", "INCLUDE", "PSMODULEPATH") -contains $k) {
            $currentVal = ""
            if(Test-path env:\$k) {
                $currentVal = Get-Content env:\$k
            }
            $newValue = push_to_path (_Resolve-Paths $env["Runtime"][$k].Value) $currentVal
        } elseif ($env["Runtime"][$k].IsPath) {
            $newValue = _Resolve-Paths $env["Runtime"][$k].Value
        } else {
            $newValue = $env["Runtime"][$k].Value
        }
        New-Item -Name $k -Value $newValue -ItemType Variable -Path Env: -Force | Out-Null
        Write-BuildLine "Value of $k is $newValue"
    }

    Write-BuildLine "Layering buildtime environment on top of system environment"
    # Layer buildtime environment values into the system environment,
    # which has already had the runtime values merged in. This is a
    # stripped-down version of the logic used to layer environments
    # from dependencies in the first place.
    foreach($k in $env["Buildtime"].keys) {
        $val = $env["Buildtime"][$k].Value
        if(@("PATH", "LIB", "INCLUDE", "PSMODULEPATH") -contains $k -or ($env["Buildtime"][$k].IsPath)) {
            $val = _Resolve-Paths $val
        }

        if(Test-Path env:\$k) {
            # There was a previous value; need to figure out
            # how to proceed
            if((Get-Content -Path env:\$k) -eq $val) {
                # If the value is the same as what we've got,
                # there's nothing to do
                continue
            }

            if((__env_var_type $k) -eq "aggregate") {
                $val = push_to_path $val (Get-Content -Path env:\$k) (__env_aggregate_separator $k)
            }
            New-Item -Name $k -Value $val -ItemType Variable -Path Env: -Force | Out-Null
            Write-BuildLine "Value of $k is $val"
        } else {
            # There was no previous value; just set this one
            New-Item -Name $k -Value $val -ItemType Variable -Path Env: -Force | Out-Null
            Write-BuildLine "Value of $k is $val"
        }
    }
}

# Given that a variable is an aggregate (i.e., PATH-style) variable,
# return the separator character used to delimit items in the value.
function __env_aggregate_separator($VarName) {
    $hint_var = Get-Variable "HAB_ENV_$VarName_TYPE" -ErrorAction SilentlyContinue

    if($hint_var) {
        # Look for user-specified hints first
        $hint_var.Value
    } elseif($__well_known_aggregate_env_vars.ContainsKey($varname)) {
        # Look in our built-in map to see if we know anything about it
        $__well_known_aggregate_env_vars[$VarName]
    } else {
        # Just assume it's the default
        ';'
    }
}

# Read in the RUNTIME_ENVIRONMENT files from all direct dependencies
# (in `pkg_deps` / `pkg_build_deps` order!) and layer them as appropriate.
function __populate_environment_from_deps {
    param(
        [ValidateSet('BuildTime','RunTime')]
        [System.String]$Environment
    )

    $dep_array=$pkg_deps
    if($Environment -eq "BuildTime") {
        $dep_array = $pkg_build_deps
    }


    foreach($dep in $dep_array) {
        $path_to_dep = Get-HabPackagePath $dep.Split("/")[1]
        $dep_ident = (Get-Content "$path_to_dep/IDENT").Trim()
        __populate_environment_from_metafile $environment $path_to_dep $dep_ident
    }
}

function __populate_environment_from_metafile($environment, $path_to_dep, $dep_ident) {
  $envTable = __parse_metafile "$path_to_dep/${environment}_ENVIRONMENT"
  $envPathsTable = __parse_metafile "$path_to_dep/${environment}_ENVIRONMENT_PATHS"

  # vars in ENVIRONMENT_PATHS files are duplicated in ENVIRONMENT file
  # to support backward compat with older sup/cli
  # we will dedupe them here for the build
  foreach($key in $envPathsTable.keys) {
    $envTable.Remove($key)
  }

  __populate_environment_from_hashtable $environment $envTable $dep_ident
  __populate_environment_from_hashtable $environment $envPathsTable $dep_ident -IsPath
}

function __populate_environment_from_hashtable($environment, $table, $dep_ident, [switch]$IsPath) {
    foreach($key in $table.keys) {
      # Any values of `PATH`, `LIB`, and `INCLUDE` are skipped as we
      # will be computing these variables independently of the
      # RUNTIME_ENVIRONMENT metadata files. Additionally, this acts
      # as backwards compatibility for all `RUNTIME_ENVIRONMENT`
      # files that contain a `PATH` key.
      if(@("PATH", "LIB", "INCLUDE") -contains $key) {
        continue
      }

      if($env[$environment].ContainsKey($key)) {
          # There was a previous value; need to figure out
          # how to proceed

          # Where did the value come from originally?
          if($table[$key] -eq $env[$environment][$key].Value) {
              # If the value is the same as what we've got,
              # there's nothing to do
              continue
          }

          switch(__env_var_type $key) {
              "primitive" {
                  Write-Warning "Overwriting `$env:$($key) originally set from $($provenance[$environment][$key])"
                  __set_env $environment $key $table[$key] $dep_ident $IsPath
              }
              "aggregate" {
                  Write-Warning "Prepending to `$env:$($key) originally set from $($provenance[$environment][$key])"

                  # if aggregate, push to front with separator
                  __push_env $environment $key $table[$key] (__env_aggregate_separator $key) $dep_ident $IsPath
              }
          }
      }
      else {
          # There was no previous value; just add this one
          __set_env $environment $key $table[$key] $dep_ident $IsPath
      }
    }
}

function __parse_metafile($metafilePath) {
  $metafileTable = @{}
  if(Test-Path $metafilePath) {
    foreach($line in (Get-Content $metafilePath)) {
        $varval = $line.split("=")
        $metafileTable[$varval[0]] = $varval[1]
    }
  }
  $metafileTable
}

# Internal function implementing core "set" logic for environment variables.
function __set_env($Environment, $VarName, $VarValue, $ident, $IsPath){
    if($IsPath) { $VarValue = (_Get-UnrootedPath $VarValue) }
    $env[$Environment][$VarName] = @{
      Value = $VarValue
      IsPath = $IsPath
    }
    $provenance[$Environment][$VarName]=$ident
}

# Internal function implementing core "push" logic for environment variables.
function __push_env($Environment, $VarName, $VarValue, $separator, $ident, $IsPath) {
    # If there is no current value (that is, $current_value == ""), we
    # can still push onto that with no loss of generality. Because
    # push_to_path also dedupes the result, this allows us to take
    # $value inputs that are themselves paths, which may have
    # duplicate or blank entries (as is the case with some existing
    # Habitat metadata files) and this will effectively "clean" them
    # for us!
    if($env[$Environment][$VarName]) {
      $current_value=$env[$Environment][$VarName].Value
    }
    if($IsPath) { $VarValue = (_Get-UnrootedPath $VarValue) }
    $new_value=$(push_to_path $VarValue $current_value $Separator)
    $env[$Environment][$VarName] = @{
      Value = $new_value
      IsPath = $IsPath
    }

    $existing_provenance = $provenance[$Environment][$VarName]
    $provenance[$Environment][$VarName]=$(push_to_path $ident $existing_provenance)
}

# Pushes $ITEM onto $PATH (using optional $SEPARATOR) and then
# deduplicates entries.
#
# push_to_path "foo" "bar:foo:baz"
#   -> "foo:bar:baz"
#
# push_to_path "foo" ""
#   -> "foo"
#
# push_to_path "foo" "bar;baz" ";"
#   -> "foo;bar;baz"
#
function push_to_path($item, $path, $separator = ";") {
    if(!$path -or ($path -eq "")) {
        $temp=$item
    }
    else {
        $temp="$item$separator$path"
    }
    dedupe_path $temp $separator
}

function dedupe_path($path, $separator = ";"){
    $pathArray = $path.Split($separator)
    $pathArray = $pathArray | Select -Unique
    [String]::Join($separator, $pathArray)
}

function __env_var_type($VarName) {
    $hint_var = Get-Variable "HAB_ENV_${VarName}_TYPE" -ErrorAction SilentlyContinue

    if($hint_var) {
        # Look for user-specified hints first
        $hint_var.Value
    }
    elseif($__well_known_aggregate_env_vars.ContainsKey($varname)) {
        # Look in our built-in map to see if we know anything about it
        'aggregate'
    }
    else {
        # We know nothing about it; treat it as a primitive
        Write-Warning "Treating `$$varName as a primitive type. If you would like to change this, add `"HAB_ENV_${VarName}_TYPE='aggregate'`" to your plan."
        'primitive'
    }
}

function Set-BuildtimeEnv(
  $VarName,
  $VarValue = $(throw "Must provide a value to Set-BuildtimeEnv for key '$VarName'"),
  [switch]$force,
  [switch]$IsPath) {
    set_env "BuildTime" @PSBoundParameters
}

function Set-RuntimeEnv(
  $VarName,
  $VarValue = $(throw "Must provide a value to Set-RuntimeEnv for key '$VarName'"),
  [switch]$force,
  [switch]$IsPath) {
    set_env "RunTime" @PSBoundParameters
}

function set_env($Environment, $VarName, $VarValue, [switch]$force, [switch]$IsPath) {
    __fail_on_protected_env_var_manipulation $VarName

    if($env[$Environment].ContainsKey($VarName)) {
        if(!$force) {
            _Exit-With "Already have a value for `$$VarName, set by $($provenance[$Environment][$VarName]): $($env[$Environment][$VarName].Value). If you really wish to overwrite this value, pass the '-force' option when setting it." 1
        } else {
            Write-Warning "Already have a value for `$$VarName, set by $($provenance[$Environment][$VarName]): $($env[$Environment][$VarName].Value). Overwriting value because the '-Force' flag was passed"
        }
    }

    __set_env $Environment $VarName $VarValue "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}" $IsPath
}

function __fail_on_protected_env_var_manipulation($VarName) {
    $protected=@{
        PATH="pkg_bin_dirs"
        LIB="pkg_lib_dirs"
        LD_RUN_PATH="pkg_lib_dirs"
        LDFLAGS="pkg_lib_dirs"
        INCLUDE="pkg_include_dirs"
        CFLAGS="pkg_include_dirs"
        CPPFLAGS="pkg_include_dirs"
        CXXFLAGS="pkg_include_dirs"
        PKG_CONFIG_PATH="pkg_pconfig_dirs"
    }
    foreach($p in $protected.Keys) {
        if($VarName -eq $p) {
            _Exit-With "Cannot directly manipulate environment variable $VarName! Add appropriate entries to the '$($protected[$VarName])' variable in plan.ps1 instead!"
        }
    }
}

function Push-BuildtimeEnv($VarName, $VarValue, [switch]$IsPath) {
    Write-BuildLine "PUSH $VarName TO BUILD"
    do_push_env "BuildTime" @PSBoundParameters
}

function Push-RuntimeEnv($VarName, $VarValue, [switch]$IsPath) {
    Write-BuildLine "PUSH $VarName TO RUN"
    do_push_env "RunTime" @PSBoundParameters
}

function do_push_env($Environment, $VarName, $VarValue, [switch]$IsPath) {
    __fail_on_protected_env_var_manipulation $VarName

    __push_env $Environment $VarName $VarValue (__env_aggregate_separator $VarName) "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}" $IsPath
}

function _Resolve-Paths($paths) {
    $path_part = $null
    Push-Location $originalPath
    try {
        foreach($path in $paths.split(";")) {
            $data = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($path)
            if (!$path_part) {
            $path_part = $data
            }
            else {
            $path_part += ";$data"
            }
        }
    }
    finally { Pop-Location }
    $path_part
}

# **Internal**  Build a `PATH` string suitable for entering into this package's
# `RUNTIME_PATH` metadata file. The ordering of this path is important as this
# value will ultimately be consumed by other programs such as the Supervisor
# when constructing the `PATH` environment variable before spawning a process.
#
# The path is constructed by taking all `PATH` metadata file entries from this
# package (in for the form of `$pkg_bin_dirs[@]`), followed by entries from the
# *direct* dependencies first (in declared order), and then from any remaining
# transitive dependencies last (in lexically sorted order). All entries are
# present only once in the order of their first appearance.
function _Assemble-RuntimePath() {
    # Contents of `pkg_xxx_dirs` are relative to the plan root;
    # prepend the full path to this release so everything resolves
    # properly once the package is installed.
    $strippedPrefix = _Get-UnrootedPath $pkg_prefix
  
    $paths = @()
  
    # Add element for each entry in `$pkg_bin_dirs[@]` first
    foreach($dir in $pkg_bin_dirs) {
      $paths += "$strippedPrefix\$dir"
    }
  
    # Iterate through all direct direct run dependencies following by all
    # remaining transitive run dependencies and for each, append each path entry
    # onto the result, assuming it hasn't already been added. In this way, all
    # direct dependencies will match first and any programs that are used by a
    # direct dependency will also be present on PATH, albeit at the very end of
    # the PATH. Additionally, any path entries that don't relate to the
    # dependency in question are filtered out to deal with a vintage of packages
    # which included more data in `PATH` and have since been addressed.
    foreach($dep_prefix in ($pkg_deps_resolved + $pkg_tdeps_resolved)) {
      if (Test-Path (Join-Path $dep_prefix "PATH")) {
        $data = (Get-Content (Join-Path $dep_prefix "PATH") | Out-String).Trim()
        foreach($entry in $data.split(";")) {
          $paths = @(_return_or_append_to_set $entry $paths)
        }
      } elseif (Test-Path (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT")) {
        # Backwards Compatibility: If `PATH` can't be found, then attempt to fall
        # back to looking in an existing `RUNTIME_ENVIRONMENT` metadata file for
        # a `PATH` entry. This is necessary for packages created using a release
        # of Habitat between 0.53.0 (released 2018-02-05) and up to including
        # 0.55.0 (released 2018-03-20).
        $strippedPrefix = _Get-UnrootedPath $dep_prefix
  
        foreach ($line in (Get-Content (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT"))) {
            $varval = $line.split("=")
            if ($varval[0] -eq "PATH") {
                foreach($entry in $varval[1].split(";")) {
                  # Filter out entries that are not related to the `$dep_prefix`
                  if ("$entry" -Like "$strippedPrefix\*") {
                    $paths = @(_return_or_append_to_set $entry $paths)
                  }
                }
                break;
            }
        }
      }
    }
  
    # Return the elements of the result, joined with a colon
    $paths -join ';'
}
  
function Write-EnvironmentFiles {
    $runtime_path = _Assemble-RuntimePath
    if ($runtime_path) {
      "$runtime_path" | Out-File "$pkg_prefix\RUNTIME_PATH" -Encoding ascii

      # Backwards Compatibility: Set the `PATH` key for the runtime environment
      # if a computed runtime path is necessary which will be used by Habitat
      # releases between 0.53.0 (released 2018-02-05) and up to including
      # 0.55.0 (released 2018-03-20). All future releases will ignore the
      # `PATH` entry in favor of using the `RUNTIME_PATH` metadata file.
      $env["RunTime"]["PATH"] = @{
        Value = "$runtime_path"
        IsPath = $true
      }
    }

    foreach ($var in $env.Runtime.GetEnumerator()) {
        "$($var.Key)=$($var.Value.Value)" | Out-File "$pkg_prefix\RUNTIME_ENVIRONMENT" -Encoding ascii -Append
    }

    $env.Runtime.GetEnumerator() | ? { $_.Value.IsPath } | % {
        "$($_.Key)=$($_.Value.Value)" | Out-File "$pkg_prefix\RUNTIME_ENVIRONMENT_PATHS" -Encoding ascii -Append
    }

    foreach ($var in $provenance.Runtime.GetEnumerator()) {
        "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\RUNTIME_ENVIRONMENT_PROVENANCE" -Encoding ascii -Append
    }

    foreach ($var in $env.Buildtime.GetEnumerator()) {
        "$($var.Key)=$($var.Value.Value)" | Out-File "$pkg_prefix\BUILDTIME_ENVIRONMENT" -Encoding ascii -Append
    }

    $env.Buildtime.GetEnumerator() | ? { $_.Value.IsPath } | % {
        "$($_.Key)=$($_.Value.Value)" | Out-File "$pkg_prefix\BUILDTIME_ENVIRONMENT_PATHS" -Encoding ascii -Append
    }

    foreach ($var in $provenance.Buildtime.GetEnumerator()) {
        "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\BUILDTIME_ENVIRONMENT_PROVENANCE" -Encoding ascii -Append
    }
}