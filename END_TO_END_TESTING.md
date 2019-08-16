# End-to-End Testing

The [end-to-end scripts](./test/end-to-end) are run automatically in CI
with each merge to master. They are intended to run on a bare system,
and thus should not be run directly on a developer workstation, as
false positives, false negatives, and unintended consequences will be
the result.

To mimic the CI environment of the tests, the
[e2e_local.sh](./e2e_local.sh) script is provided. Developers can use
this to iterate locally on an individual test scenario.

```sh
./e2e_local.sh test/end-to-end/${YOUR_TEST_FILE}
```

Ideally, test files will take take no arguments, but can respond to
environment variables. The variables used for local testing are
currently found in [e2e_env](./e2e_env).

Also see the [end-to-end pipeline definition](./.expeditor/end_to_end.pipeline.yml) for additional background.
