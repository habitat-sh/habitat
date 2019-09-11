$ErrorActionPreference="stop" 

Param(
   [Parameter(Mandatory=$true)]
   [string[]]$TestArgs
)


# These are the commands that will be sequentially run in the
# container. The final one will be the test itself, which is supplied
# as the arguments of the script.
$commands=@(".\.expeditor\scripts\end_to_end\setup_environment.ps1 DEV" ) + $TestArgs

# Add a `;` after every command, for feeding into the container. This
# allows them to run in sequence.
$commandSequence = ($commands -join ";")

docker run `
       --rm `
       --interactive `
       --tty `
       --env-file="$(pwd)/e2e_env" `
       --volume="$(pwd):c:\workdir" `
       --workdir=/workdir `
       chefes/buildkite-windows powershell.exe $commandSequence
