# End-to-End Testing

The [end-to-end scripts](./test/end-to-end) are run automatically in CI
with each merge to master. They are intended to run on a bare system,
and thus should not be run directly on a developer workstation, as
false positives, false negatives, and unintended consequences will be
the result.

To mimic the CI environment of the tests, the
[e2e_local.sh](./e2e_local.sh) script is provided. Developers can use
this to iterate locally on an individual test scenario.  TEST_NAME is 
is the name of the test located in `test/end-to-end`, without the file 
extension.

```sh
./e2e_local.sh ${TEST_NAME}
```

Ideally, test files will take take no arguments, but can respond to
environment variables. The variables used for local testing are
currently found in [e2e_env](./e2e_env).

Also see the [end-to-end pipeline definition](./.expeditor/end_to_end.pipeline.yml) for additional background.

## Testing changes before they're built

We currently don't have a good mechanism for building a set of packages off of a branch. The release pipeline 
can be used for this purpose, however there is always the risk of change causing an unintended release. Additionally, 
if you need to build against `core` origin packages that have not been promoted to stable, you won't be able 
to use the release pipeline. The below snippet will produce a set of packages and upload them to a named channel,
allowing you to run the end-to-end tests against them locally. Substitute the builder channel containing unreleased
packages for `stable` below in `HAB_BLDR_CHANNEL` if that is required.

**CAUTION** Do not use `stable` for `pkg upload`. This will cause an accidental release.

```
env HAB_BLDR_CHANNEL=stable HAB_ORIGIN=core \
hab studio run "for component in hab plan-build backline studio launcher sup pkg-export-tar pkg-export-docker; do build components/\$component; done"
######################################################################
# Before uploading, ensure only your intended hart files are present #
######################################################################
hab pkg upload --channel=YOUR_TEST_CHANNEL results/*.hart
./e2e_local.sh TEST_NAME YOUR_TEST_CHANNEL
```

Once your tests are complete, it's a good idea to clean up your test channel
```
hab bldr channel delete YOUR_TEST_CHANNEL
```
