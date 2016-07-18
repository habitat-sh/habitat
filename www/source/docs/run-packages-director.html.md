---
title: Run multiple packages using the director
---

# Run multiple packages using the director
The director is a supervisor that can quickly start up and manage multiple services on _one machine_ using a config.toml file, and when run in a supervisor process (`hab-sup`) itself, the director can be reconfigured at runtime like any other Habitat service. Reconfiguring the director causes all child processes to be restarted.

## Defining the services

The config.toml file used by the director contains one or more service definitions. Service definitions are combination of a package identifier, service group, and CLI arguments. They are specified as a dot-separated list in a [TOML table](https://github.com/toml-lang/toml#table) name.

    [services.<origin>.<name>.<group>.<organization>]

The following example corresponds to the `core/redis` package in the redis.somegroup service group.

    [services.core.redis.somegroup]

This example corresponds to the `core/redis` package in the redis.somegroup service group which is in the someorg organization.

    [services.core.redis.somegroup.someorg]

> Note: All services must be described as children of the services TOML table. When the TOML is rendered, the values for services will be
located under `cfg.services.*` .

A service definition can additionally specify a start key/value under
the service table definition:

    # Start core/redis with --group somegroup and --org someorg
    # Additionally, pass in --permanent-peer to the start command
    [services.core.redis.somegroup.someorg]
    start = "--permanent-peer"

    [services.core.rngd.foo.someorg]
    start = "--permanent-peer --foo=bar"

> Note:  CLI arguments specified in config.toml are split on whitespace.


Services can provide environment variables in the form of a TOML table which follows the following format:

	[services.<origin>.<name>.<group>.<organization>.env]
	ENV1="some value"
	ENV2="some other value" 

> Note: Environment variables MUST be specified as valid TOML strings. 

For example:

	# Specify custom JAVA_HOME and CLASSPATH environment variables
	[services.core.java_app.somegroup.someorg]
	start = "--permanent-peer"
	[services.core.java_app.somegroup.someorg.env]
	JAVA_HOME="/some/path/"
	CLASSPATH="/some/classpath/foo.jar"
	
	[services.core.rngd.foo.someorg]
	start = "--permanent-peer --foo=bar"
	[services.core.rngd.foo.someorg.env]
	JAVA_HOME="/a/different/path/"
	# we don't specify CLASSPATH here, so it won't be set for core/rngd

> Note: We current don't support global environment variables.

## Using the director
When run in a supervisor, the director can be started using the `hab start` command.

    hab start core/hab-director

You must also pass in the config.toml file containing your service definitions. This can be done at runtime dynamically by using the `hab config apply` subcommand.

    hab config apply hab-director.default --peer 172.17.0.2 1 /path/to/config.toml

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-export">Export packages</a></li>
</ul>
