Automated Tests
===============

We're building out a suite of automated tests for the studio and other Habitat products. For now, these haven't been wired up to an automated build process, but they will be shortly. In the meantime, you can run them manually.

# Prerequisites

We currently have tests written in [BATS](https://github.com/bats-core/bats-core) as well as [Expect](https://core.tcl.tk/expect/index). Additionally, suites of Expect-based tests can be run using the [DejaGNU](https://www.gnu.org/software/dejagnu/) test runner.

Tests of non-interactive functionality are written in BATS, whereas interactive testing is done in Expect. As we build out the tests more, this division may change; it's not carved in stone. Additionally, they will likely move to a higher level within the repository's directory structure.

On Ubuntu, these can be installed using `apt-get`:

```bash
sudo apt-get install bats expect dejagnu
```

# Running the Tests

## BATS

```bash
cd $HABITAT_REPO/components/rootless_studio/tests
bats ./bats
```

## Expect

To run an entire suite, use DejaGNU's `runtest` executable:

```bash
cd $HABITAT_REPO/components/rootless_studio/tests/dejagnu
runtest --tool=studio
```

Individual tests can also be executed by passing the test file as an argument:

```bash
cd $HABITAT_REPO/components/rootless_studio/tests/dejagnu
runtest --tool=studio secrets_are_passed.exp
```

This is most useful when you're working on a single test at a time.

We cannot currently use `expect` to execute test directly because we are using DejaGNU-specific testing calls `pass` and `fail` to allow the Expect scripts to actually behave as tests, without having to write a lot of additional boilerplate ourselves.
