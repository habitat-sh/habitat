Habitat Integration Tests
=========================

Habitat uses [BATS](https://github.com/sstephenson/bats) for integration tests.

It also makes use of a number of additional BATS helper libraries:
* [bats-support](https://github.com/ztombol/bats-support)
* [bats-assert](https://github.com/ztombol/bats-assert)
* [bats-file](https://github.com/ztombol/bats-file)

These helpers currently are pulled in as Git submodules; to retrieve
them, run:

```sh
git submodule init
git submodule update
```

These tests exercise various aspects of the `hab` CLI tool, and makes
real requests against the [Builder](https://bldr.habitat.sh)
service. It also assumes full control of the local `/hab` directory
(not a mocked directory!) and regularly wipes its contents, so be
aware of this if you plan to run the tests directly on your local
workstation.

Tests only run for Linux at the moment.

# Cleanroom Test Environment

To easily run the tests, a `Dockerfile` is provided that creates a
minimal "cleanroom" that the tests can run in. A script in the
top-level of this repository, `run-bats.sh`, should be used to build
the container and then run the tests with the `hab` binaries from
`target/debug` mounted in as appropriate.

A top-level Makefile target `make bats` wraps all this up in a single
operation.
