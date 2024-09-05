$pkg_name="dummy-scaffolding"
$pkg_origin=$env:HAB_ORIGIN
$pkg_version="0.1.0"

function Invoke-Install {
    Copy-Item "$PLAN_CONTEXT/lib" $pkg_prefix -Recurse -Force
}
