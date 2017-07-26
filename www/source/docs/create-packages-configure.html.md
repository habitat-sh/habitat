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

Habitat supports the standard [built-in helpers](http://handlebarsjs.com/builtin_helpers.html):

* `if`
* `unless`
* `each`
* `with`
* `lookup`
* `>` ([partials](http://handlebarsjs.com/partials.html))
* `log`

> Note: Habitat also has a collection of [advanced helpers](#advanced-helpers) to assist in writing configuration and hook files.

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
      slaveof {{svc.leader.sys.ip}} {{svc.leader.cfg.port}}
    {/if}}

Here's an example using `each` to render multiple server entries:

    {{~#each cfg.servers as |server| }}
    server {
      host {{server.host}}
      port {{server.port}}
    }
    {{~/each}}

You would specify the corresponding values in a TOML file using an
[array of tables](https://github.com/toml-lang/toml#array-of-tables)
like this:

    [[servers]]
    host = "host-1"
    port = 4545

    [[servers]]
    host = "host-2"
    port = 3434

## Advanced Helpers

Habitat's templating flavour includes a number of useful helpers for writing configuration and hook files

* [`toLowercase`](#tolowercase-helper)
* [`toUppercase`](#touppercase-helper)
* [`strReplace`](#strreplace-helper)
* [`pkgPathFor`](#pkgpathfor-helper)
* [`eachAlive`](#eachalive-helper)
* [`toJson`](#tojson-helper)
* [`toToml`](#totoml-helper)
* [`toYaml`](#toyaml-helper)

### toLowercase Helper

Returns the lowercase equivalent of the given string literal.

    my_value={{toLowercase "UPPER-CASE"}}

### toUppercase Helper

Returns the uppercase equivalent of the given string literal.

    my_value={{toUppercase "lower-case"}}

### strReplace Helper

Replaces all matches of a pattern within the given string literal.

    my_value={{strReplace "this is old" "old" "new"}}

This sets `my_value` to "this is new".

### pkgPathFor Helper

Returns the absolute filepath to the package directory of the package best resolved from the given package identifier. The named package must exist in the `pkg_deps` of the plan from which the template resides. The helper will return a nil string if the named package is not listed in the `pkg_deps`. As result you will always get what you expect and the template won't leak to other packages on the system.

Example Plan Contents:

    pkg_deps=("core/jre8")

Example Template:

    export JAVA_HOME={{pkgPathFor "core/jre8"}}

### eachAlive Helper

Iterates over a collection of members and renders the template for members that are marked alive.

    {{~#eachAlive bind.backend.members as |member|}}
    server ip {{member.sys.ip}}:{{member.cfg.port}}
    {{~/eachAlive}}

### toJson Helper

To output configuration data as JSON, you can use the `toJson` helper.

Given a default.toml that looks like:

    [web]

    [[servers]]
    host = "host-1"
    port = 4545

    [[servers]]
    host = "host-2"
    port = 3434

and a template:

    {{toJson cfg.web}}

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

This can be useful if you have a configuration file that is in JSON format and
has the same structure as your TOML configuration data.

### toToml Helper

The `toToml` helper can be used to output TOML.

Given a default.toml that looks like:

    [web]

    port = 80

and a template:

    {{toToml cfg.web}}

when rendered, it will look like:

    port = 80

This can be useful if you have an app that uses TOML as its configuration file
format, but may have not been designed for Habitat, and you only need certain
parts of the configuration data in the rendered TOML file.

### toYaml Helper

The `toYaml` helper can be used to output [YAML](http://yaml.org/).

Given a default.toml that looks like:

    [web]

    port = 80

and a template:

    {{toYaml cfg}}

when rendered, it will look like:

    ---
    web:
      port: 80

The helper outputs a YAML document (with a line beginning with `---`), so it
must be used to create complete documents: you cannot insert a section of YAML
into an existing YAML document with this helper.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/create-packages-build">Build packages</a></li>
</ul>
