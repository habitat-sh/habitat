$pkg_name="habitat"
$pkg_origin="core"
$pkg_version="$(Get-Content $PLAN_CONTEXT/../VERSION)"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_license=@("Apache-2.0")
$pkg_source="https://s3-us-west-2.amazonaws.com/habitat-win-deps/hab-win-deps.zip"
$pkg_shasum="00b34fb983ebc43bfff9e8e2220d23db200cb45494a4971a5e2e733f1d73d04b"
$pkg_bin_dirs=@("bin")
$pkg_deps=@("core/powershell")
$pkg_build_deps=@("core/powershell", "core/hab", "core/hab-plan-build-ps1", "core/7zip",
    "core/visual-cpp-redist-2013", "core/rust", "core/cacerts")

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    }
    else {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CACHE_SRC_PATH/$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:PLAN_VERSION               = "$pkg_version/$pkg_release"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:SODIUM_LIB_DIR             = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:LIBARCHIVE_INCLUDE_DIR     = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBARCHIVE_LIB_DIR         = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBZMQ_PREFIX              = "$HAB_CACHE_SRC_PATH/$pkg_dirname"
}

function Invoke-Unpack {
    Expand-Archive -Path "$HAB_CACHE_SRC_PATH/hab-win-deps.zip" -DestinationPath "$HAB_CACHE_SRC_PATH/$pkg_dirname"
}

function Invoke-Build {
    Invoke-Build-Supervisor
    Invoke-Build-Ctl
    Invoke-Build-Studio
}

function Invoke-Install {
    Invoke-Install-Supervisor
    Invoke-Install-Ctl
    Invoke-Install-Studio
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2013")/bin/*" "$pkg_prefix/bin"
}

function Invoke-Build-Supervisor {
    Push-Location "$PLAN_CONTEXT/../components/sup"
    try {
        cargo build --release --no-default-features
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    }
    finally { Pop-Location }
}

function Invoke-Build-Studio {
    Get-Content "$PLAN_CONTEXT/../components/studio/bin/hab-studio.ps1" | % {
		    $_.Replace("@author@", $pkg_maintainer).Replace("@version@", $pkg_version)
    } | Add-Content -Path hab-studio.ps1
    Copy-Item $PLAN_CONTEXT/../components/studio/bin/hab-studio.bat hab-studio.bat
}

function Invoke-Build-Ctl {
    Push-Location "$PLAN_CONTEXT/../components/hab"
    try {
        cargo build --release --verbose
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    }
    finally { Pop-Location }
}

function Invoke-Install-Supervisor {
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab-sup.exe" "$pkg_prefix/bin/hab-sup.exe"
    Copy-Item "$HAB_CACHE_SRC_PATH/$pkg_dirname/bin/*" "$pkg_prefix/bin"
}

function Invoke-Install-Studio {
    mkdir "$pkg_prefix/bin/powershell"
    mkdir "$pkg_prefix/bin/hab"
    mkdir "$pkg_prefix/bin/7zip"

    Copy-Item hab-studio.ps1 "$pkg_prefix/bin/hab-studio.ps1"
    Copy-Item hab-studio.bat "$pkg_prefix/bin/hab-studio.bat"

    Copy-Item "$(Get-HabPackagePath powershell)/bin/*" "$pkg_prefix/bin/powershell" -Recurse
    Copy-Item "$(Get-HabPackagePath hab)/bin/*" "$pkg_prefix/bin/hab"
    Copy-Item "$(Get-HabPackagePath 7zip)/bin/*" "$pkg_prefix/bin/7zip"
    Copy-Item "$(Get-HabPackagePath hab-plan-build-ps1)/bin/*" "$pkg_prefix/bin"
}

function Invoke-Install-Ctl {
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab.exe" "$pkg_prefix/bin/hab.exe"
    Copy-Item "$HAB_CACHE_SRC_PATH/$pkg_dirname/bin/*" "$pkg_prefix/bin"
}
