Param(
    [Parameter(Mandatory=$true)]
    [string]$TestName,
    [string]$Channel="dev-v1.6"
)

$ErrorActionPreference="stop"

docker run `
    --rm `
    --interactive `
    --tty `
    --env-file="$(Get-Location)/e2e_env" `
    --volume="$(Get-Location):c:\workdir" `
    --workdir=/workdir `
    chefes/buildkite-windows powershell.exe .\.expeditor\scripts\end_to_end\run_e2e_test.ps1 $Channel $TestName
