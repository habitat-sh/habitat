# Scaffolding
Scaffolding is default implementations of the build phases and runtime hooks for your application. They also provide some default health check hooks where appropriate to ensure your application is functioning reliably. Customized Scaffolding can be created to facilitate re-usability of common patterns in your organization for developing, building, and running your applications.

## Getting Started
### New Projects
Simply run `hab plan init` to leverage the scaffolding. Habitat will look in your application code for known project types and add the appropriate scaffolding.

If you would like to specify the desired scaffolding to use, you can run `hab plan init --scaffolding core/java8-maven-tomcat`

The resulting plan will contain all the available callbacks and variables relevant to the scaffolding type.

### Existing Plans

To begin using Scaffolding, you will need to add the appropriate `pkg_scaffolding`.

```bash
pkg_name="my_java_app"
....
pkg_scaffolding="core/scaffolding-java8-maven-tomcat"
```

### Available Scaffolding

* `core/scaffolding-go17`
* `core/scaffolding-go18`
* `core/scaffolding-ruby-mri2.2.6-rails-unicorn`
* `core/scaffolding-ruby-mri2.3.4-rails-unicorn`
* `core/scaffolding-ruby-mri2.4.0-rails-unicorn`
* `core/scaffolding-java8-maven-tomcat`
* `core/scaffolding-java8-maven-jboss`
* `core/scaffolding-java8-gradle-tomcat`
* `core/scaffolding-java8-gradle-jboss`
* `core/scaffolding-java7-ant`
* `core/scaffolding-python-unicorn`

## Callbacks
Each scaffolding defines a set of callbacks which are unique to the scaffolding type. This allows you to complex projects which may need to leverage different scaffoldings as needed within your project. Each of these callbacks define overrides of base level callbacks.

### scaffolding_{flavor}_begin
Each scaffolding contains a begin callback which is executed immediately while the `scaffolding.sh` file is loaded.

### ruby _(This will likely live in the scaffolding README.md)_
#### do_scaffolding_bundler
#### do_scaffolding_rake
#### do_scaffolding_rack
#### do_scaffolding_more_ruby_stuff 

## Variables
These are variables which each scaffolding honors, allowing the plan author to consume the value to override a particular behavior.
### ruby
:warning: _This section will likely live in the Ruby scaffolding README.md and is here for feature clarification_ :warning:
* pkg_ruby_build_cmd - Overrides default binary and path.
* pkg_ruby_install_cmd
* pkg_ruby_bundle_cmd
* pkg_ruby_run_opts  - This functions similar to `pkg_svc_run` as the result of using this option will intelligently generate a hook file for you. However, you also can create the [hook file](#Run-Hooks) directly if you need to have more control over the resulting output.

## Run Hooks
#### Health Checks
Each scaffolding comes with a default health check for your application.

*TODO: Write the directions*

## Build Artifacts
### Auto-Ingestion
Each scaffolding defines a default value for `pkg_source` in [URI syntax](https://tools.ietf.org/html/rfc3986). You can override this value within your plan should you application source live in a non-standard location.

## Scaffolding Internals
Each scaffolding defines it's own `pkg_build_deps` and `pkg_deps` arrays with any required pckages and is merged into the TDEPS (transitive dependencies) as it would any other package.


