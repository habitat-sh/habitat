## PR Verification Testing

All components of habitat are tested when PRs are created. The helper scripts are designed to run against a single component (such as `common`), in an environment that has habitat and rust preinstalled.

#### Linux
```
.expeditor/scripts/verify/run_cargo_test.sh <component>
```
*Parameters:*
`--test-options` - allows you to pass options such as `--test-threads=1` to the cargo test command.
`--features` - allows you to enable specific features when running tests.

#### Windows (powershell 5+)
```
.expeditor/scripts/verify/run_cargo_test.ps1 <component>
```
*Parameters:*
`--TestOptions` - allows you to pass options such as `--test-threads=1` to the cargo test command.
`--Features` - allows you to enable specific features when running tests.

####Examples
See [verify.pipeline.yml](.expeditor/verify.pipeline.yml) for a full list of what is run in the CI pipeline, and what parameters are used.