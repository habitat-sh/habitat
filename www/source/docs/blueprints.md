# Blueprints
Blueprints are default implementations of the build phases and runtime hooks for your application. They also provide some default health check hooks where appropriate to ensure your application is functioning reliably. Customized Blueprints can be created to facilitate re-usability of common patterns in your organization for developing, building, and running your applications.

## Getting Started
### New Projects
Simply run `hab plan init` to leverage blueprints. Habitat will look in your application code for known project types and the appropriate `pkg_blueprint`.

If you would like to specify the desired blueprint to use, you can run `hab plan init --blueprint core/java8-maven-tomcat`

The resulting plan will contain all the available callbacks and variables relevant to the blueprint type.

### Existing Plans

To begin using Blueprints, you will need to add the appropriate `pkg_blueprints` 

```bash
pkg_name="my_java_app"
....
blueprint="core/blueprint-java8-maven-tomcat"
```

### Available Blueprints

* `core/blueprint-golang`
* `core/blueprint-ruby-mri2.2.6-rails-unicorn`
* `core/blueprint-ruby-mri2.3.4-rails-unicorn`
* `core/blueprint-ruby-mri2.4.0-rails-unicorn`
* `core/blueprint-java8-maven-tomcat`
* `core/blueprint-java8-maven-jboss`
* `core/blueprint-java8-gradle-tomcat`
* `core/blueprint-java8-gradle-jboss`
* `core/blueprint-java7-ant`
* `core/blueprint-python-unicorn`

## Callbacks
Each blueprint defines a set of callbacks which are unique to the blueprint type. This allows you to complex projects which may need to leverage different blueprints as needed within your project. Each of these callbacks define overrides of base level callbacks.

### ruby _(This will likely live in the blueprint README.md)_
#### do_blueprint_bundler
#### do_blueprint_rake
#### do_blueprint_rack
#### do_blueprint_more_ruby_stuff 

## Variables
These are variables which each blueprint honors, allowing the plan author to consume the value to override a particular behavior. 
### ruby
:warning: _This section will likely live in the Ruby blueprint README.md and is here for feature clarification_ :warning:
* pkg_ruby_build_cmd - Overrides default binary and path.
* pkg_ruby_install_cmd
* pkg_ruby_bundle_cmd
* pkg_ruby_run_opts  - This functions similar to `pkg_svc_run` as the result of using this option will intelligently generate a hook file for you. However, you also can create the [hook file](#Run-Hooks) directly if you need to have more control over the resulting output.

## Run Hooks
#### Health Checks
Each blueprint comes with a default health check for your application.

*TODO: Write the directions*

## Build Artifacts
### Auto-Ingestion
Each blueprint defines a default value for `pkg_source` in [URI syntax](https://tools.ietf.org/html/rfc3986). You can override this value within your plan should you application source live in a non-standard location.

## Blueprints Internals
Each Blueprint defines it's own `pkg_build_deps` and `pkg_deps` arrays with any required pckages and is merged into the TDEPS (transitive dependencies) as it would any other package.


