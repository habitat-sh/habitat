---
title: Configure packages
---

# Add configuration to plans

Habitat allows you to templatize your application's native
configuration files using [Handlebars](http://handlebarsjs.com/)
syntax. The following sections describe how to create tunable
configuration elements for your application or service.

## Setting a config value

Template variables, also referred to as tags, are indicated by double
curly braces: `{{a_variable}}`. In Habitat, tunable config elements
are prefixed with `cfg.` to indicate that the value is user-tunable.

Here's an example of how to make a configuration element user-tunable. Assume that we have a native configuration file named `service.conf`. In `service.conf`, the following configuration element is defined:

    recv_buffer 128

We can make this user tunable like this:

    recv_buffer {{cfg.recv_buffer}}

Habitat can read values that it will use to render the templatize
config files in three ways:

1. `default.toml` - Each plan includes a `default.toml` file that specifies the default values to use in the absence of any user provided inputs. These files are written in [TOML](https://github.com/toml-lang/toml), a simple config format.
2. Environment variable - At start up, tunable config values can be passed to Habitat using environment variables.
3. At runtime - Users can alter config at runtime using `hab config
apply`. The input for this command also uses the TOML format.

Here's what we'd add to our project's `default.toml` file to provide a
default value for the `recv_buffer` tunable:

    recv_buffer = 128

## Branching and looping

You can use block expressions to add basic logic to your template such as checking if a
value exists or iterating through a list of items.

Block expressions use a helper function to perform the logic. The
syntax is the same for all block expressions and looks like this:

    {{#helper blockname}}
      {{expression}}
    {{/helper}}

Habitat supports the following helpers:

* each
* if
* with
* lookup
* partial
* block
* include >
* log

The most common block helpers that you will probably use are the `if` and `with` helpers.

The `if` helper evaluates conditional statements. The values `false`,
0, "", as well as undefined values all evaluate to false in `if`
blocks.

Here's an example that will only write out configuration for the
unixsocket tunable if a value was set by the user:

    {{~#if cfg.unixsocket}}
    unixsocket {{cfg.unixsocket}}
    {{~/if}}

> Note: The `~` indicates that whitespace should be omitted when rendering

TOML allows you to create sections (called [TOML tables](https://github.com/toml-lang/toml#table)) to better organize your configuration variables. For example, your `default.toml` or user defined TOML could have a `[repl]` section for variables controlling replication behavior. Here's what that looks like:

    [repl]
    backlog-size = 200
    backlog-ttl = 100
    disable-tcp-nodelay = no

When writing your template, you can use the `with` helper to reduce duplication:

    {{#with cfg.repl}}
      repl-backlog-size {{backlog-size}}
      repl-backlog-ttl {{backlog-ttl}}
      repl-disable-tcp-nodelay {{disable-tcp-nodelay}}
    {{/with}}


Helpers can also be nested and used together in block expressions. Here is another example from the redis.config file where the `if` and `with` helpers are used together to set up `core/redis` Habitat services  in a leader-follower topology.

    {{#if svc.me.follower}}
     {{#with svc.leader}}
      slaveof {{ip}} {{port}}
     {{/with}}
    {/if}}


Here's an example using `each` to render multiple server entries:

    {{~#each cfg.servers}}
    server {
      host {{host}}
      port {{port}}
    }
    {{~/each}}

You would specify the corresponding values in a TOML file using an
[array of tables](https://github.com/toml-lang/toml#array-of-tables)
like this:

    [["servers"]]
    host = host-1
    port = 4545

    [["servers"]]
    host = host-2
    port = 3434

## File format helpers

### JSON

To output configuration data as JSON, you can use the `json` helper.

Given a default.toml that looks like:

    [web]

    [["servers"]]
    host = "host-1"
    port = 4545

    [["servers"]]
    host = "host-2"
    port = 3434

and a template:

    {{ json cfg.web }}

when rendered, it will look like:

    {
      "servers": [
        {
          "host": "host-1",
          "port": 4545
        },
        {
          "host": "host-2",
          "port": 3434
        }
      ]
    }

This can be useful if you have a confugration file that is in JSON format and
has the same structure as your TOML configuration data.

### TOML

The `toml` helper can be used to output TOML.

Given a default.toml that looks like:

    [web]

    port = 00

and a template:

    {{ toml cfg.web }}

when rendered, it will look like:

    port = 80

This can be useful if you have an app that uses TOML as its configuration file
format, but may have not been designed for Habitat, and you only need certain
parts of the configuration data in the rendered TOML file.

## Further examples

For an example of how to templatize a configuration file and add it to your plan, see [Add configuration to your plan](/tutorials/getting-started-configure-plan) from the getting started tutorial.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/create-packages-build">Build packages</a></li>
</ul>
