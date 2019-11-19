Habitat Multi-Supervisor Testing Scenarios
==========================================

Each directory in `testcases` is an individual test. Each test defines
its own isolated Docker network of peered Habitat Supervisors. One
container in the network is not running a Supervisor, but is instead a
"test container"; consider this to be a workstation within the network
that is issuing commands to the Supervisors and making assertions on
their behavior.

To create a new testcase, run the following:

``` sh
new_testcase.sh <NAME_OF_TESTCASE>
```

This will generate everything needed to create a new (failing!)
testcase in `testcases/<NAME_OF_TESTCASE>`.

To run all test cases:

``` sh
./run_all.sh <NAME_OF_CHANNEL>
```

This will build all the containers containing `core/hab`,
`core/hab-sup`, and `core/hab-launcher` packages from the specified
channel in https://bldr.habitat.sh

For additional background, read the comments in the files in this
directory.

The `CTL_SECRET` file is fine to include in the repository. It just
defines a shared secret that all test containers can use to interact
with all the Supervisor test containers. It is not used in any "real"
infrastructure anywhere.
