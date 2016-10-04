# Habitat testing

## Running all tests

In it's first incarnation, the testing framework must be run inside the Habitat dev shell and from the `./test` directory. If you clone the [Habitat repo](https://github.com/habitat-sh/habitat), you can run:

	make shell

First, ensure that all Habitat related environment variables are unset:


	env | grep HAB_

In Bash, you can use the `unset` command for each variable that you'd like to clean.

Next, run the entire test suite with:

	cd test
	./test.sh

If you'd like to generate additional debugging information, set the following env variable before running the tests:

	export HAB_TEST_DEBUG=1

Log files are located in the `./logs` directory.

If the test suite fails, a `cleanup.sh` file will be generated that you can use to clean your environment manually.

### Running individual Inspec tests

If you are writing a single Inspec control and would like to circumvent the rest of the test framework, you can run the test manually via `hab pkg exec ...`.

Inspec tests _must_ be run via `hab pkg exec`:

	hab pkg exec core/inspec inspec exec ./path/to/control

### Running RSpec tests

If you are writing a single spec and would like to circumvent the rest of the test framework, you can run the test manually via `hab pkg exec ...`.

Rspec tests _must_ be run via `hab pkg exec`:

	hab pkg exec core/inspec rspec ./spec/foo.rb

> NOTE: rspec is run from the core/inspec package.


## Adding a new test

Create a new spec in the `./test/spec/` directory, and then add the file basename to the `test.sh` `all_specs` array.

Here's a template you can use for a new spec.

```
# it's safest to use `require_relative` with `spec_helper`,
# as different modes of running the tests have different require
# behaviors. We can't fully rely on a `.rspec` file.
require_relative 'spec_helper'

describe "Foo" do
    before(:all) do
        ctx.common_setup()
    end

    after(:all) do
        ctx.common_teardown()
    end

	 # This method lets us prevent cleanup of the test environment
	 # in the event of a test failure.
    # You don't need this if you want the test environment to
    # cleanup all generated test directories and keys even
    # if your tests fail.
    after(:each) do |example|
        if example.exception
            puts "Detected failed examples, keeping environment"
            ctx.cleanup = false
        end
    end

    context "Some context" do
        it "awesome thing" do
				# rspec or inspec stuff!
        end
    end
end
```

### Using the platform context

The platform context is an object that's instantiated upon startup that contains platform dependent test methods. By default, it will create unique `origin`, `user`, and `ring` keys for every invocation, as well as set the `HAB_ORIGIN` environment variable for testing.

The platform context is available in your specs via the `ctx` object.

### Testing guidelines

- Try not to hardcode any platform specific paths.
	- Use `Pathname` to join paths if possible, so we can leverage tests in a Windows testing suite.

#### **Never use a `sleep()` in a test**
	- Use `ctx.wait_for` instead, it will make the tests more reliable and faster, as we don't have to wait for a full `sleep()` to check results. For example, if you normally wait 30 seconds for operation X to occur but it finishes in 5, `ctx.wait_for` will return after 5 seconds where `sleep()` is sleeping for 30.

```
ctx.wait_for("this block to return true", :max_retries => 5) do
	check_some_condition_that_returns_true_or_false
end
```

Additional options that can be passed to `wait_for` are as follows:

- `debug`: show additional debugging output. Default `false`
- `max_retries`: The maximum number of times the block will be called until it returns true. Default: `100`
- `max_wait_seconds`: The maximum numnber of seconds the `wait_for` method will run before raising an exception. Default: `30`
- `sleep_increment_seconds`: The number of seconds to sleep between each block eval iteration. Default: `1`
- `show_progress`: Show a `*` character for each eval of the block that's passed in. Default: `true`


#### Running `hab` or `hab-sup` commands

- The `ctx.cmd_expect` method runs a `hab` subcommand, which may be a long-running process, and looks for output.

There are 2 additional forms of this method

- `ctx.hab_cmd_expect` - used to call the compiled `hab` binary, not to be used for `hab-sup` subcommands.
- `ctx.sup_cmd_expect` - used to call the compiled `hab-sup` binary, not to be used for `hab` subcommands.


Additional params can be passed to the `cmd_expect` function:
- `:timeout_seconds` - the number of seconds to wait for expected output before failing
- `:kill_when_found` - If the desired output is found, send a KILL signal to this child and return.
- `:bin` - fully qualified path to the hab or hab-sup binary you'd like to use to run a command.


###### Which one do I use?

- If you want to test a `hab` subcommand, use `ctx.hab_cmd_expect`.
- If you want to test a `hab-sup` subcommand, use `ctx.sup_cmd_expect`.
- If you want something more flexible, use `ctx.cmd_expect` with any of the additional params defined above.

For example, to build a plan and wait for success:

```
result = ctx.hab_cmd_expect("studio build fixtures/simple_service",
                         "I love it when a plan.sh comes together",
                         :timeout_seconds => 60)
```

In this example, if building a plan takes longer than 60 seconds, the result will be a failure.


In the next example, we start a Habitat supervisor (which never exits), and look for the text `Shipping out to Boston`. If the text isn't detected in `ctx.cmd_timeout_seconds` (default: 30 seconds), then the result will indicate failure.

```
result = ctx.sup_cmd_expect("start #{ctx.hab_origin}/simple_service",
                                         "Shipping out to Boston",
                                         :kill_when_found => true)
```

#### Cleanup up after running tests

- If your tests create files or directories, consider registering them with the `ctx.register` method:

	ctx.register "some_file"

When the tests pass successfully, the files/directories will automatically be removed. Upon failure however, a `cleanup.sh` script will be generated that can be run manually.

## Running distributed tests

- TODO
