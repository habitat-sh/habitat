---
title: Configure packages
---

# Add configuration to plans
Habitat allows you to templatize the native configuration file for your application or service using the [Handlebars-rust]((https://github.com/sunng87/handlebars-rust)) port of [Handlebars.js](http://handlebarsjs.com/) and [TOML](https://github.com/toml-lang/toml) files. If you are unfamiliar with Handlebars.js, it is an extension of the [Mustache templating language](http://mustache.github.io/) used to build semantic templates for HTML, configuration files, source code, and so on. The general differences between Handlebars.js and Mustache are covered [here](https://github.com/wycats/handlebars.js#differences-between-handlebarsjs-and-mustache) and the Handlebars.js features supported in Handlebars-rust are covered [here](https://github.com/sunng87/handlebars-rust#handlebars-js-features-supported-in-handlebars-rust).

The following sections describe how you may use Handlebars to create tunable configuration elements for your application or service.

## Expressions
Because Handlebars.js is an extension of Mustache, the basic Mustache tag format `{{}}` is also used, but in Handlebars, it is referred to as an expression. And in Habitat, the way to specify a tunable configuration element is to prefix any variable in your expression with `cfg` to indicate this is a user-defined element. These elements can have their values set in three ways:

* In the default.toml file included in the plan for that package
* By an environment variable at start up
* At runtime through a configuration TOML file that is sent to all services in a service group by the `hab` command-line interface (CLI) tool

Here's a simple example of how to make a configuration element configurable using an expression. Assume that we have a native configuration file named "service.conf". In service.conf, the following configuration element is defined:

    recv_buffer 128

If we want to make that element configurable, we would replace the value with an expression.

    recv_buffer={{cfg.recv_buffer}}

You must include a default.toml file in your plan directory to specify the default values for any configurable elements, so in this example, the default.toml file would have the following entry:

    recv_buffer = 128

> Note: Unlike Handlebars.js, Habitat does not support escaping HTML from within expressions.

## Block expressions
Block expressions are sections of a Handlebars template that use Handlebars helpers to perform logic on that section, such as checking if a value exists or iterating through a list of items.

To use a block helper in a block expression, use the following general format:

    {{#helper blockname}}
      {{expression}}
    {{/helper}}

Because Habitat supports the helpers defined in Handlebars-rust, the following built-in helpers may be used:

* each
* if
* with
* lookup
* partial
* block
* include >
* log  

The most common block helpers that you will probably use are the `if` and `with` helpers.

The `if` helper evaluates conditional statements, which are typically used in Habitat to determine whether a specific variable has a defined value at runtime. The following example checks if the min_buffer_size for the service has been set to a value greater than 1 or is set to "true". If the conditional statement evaluates to false, the block expression does not get executed.

    {{#if cfg.min_buffer_size}}
      min_buffer_size {{cfg.min_buffer_size}}
    {{/if}}

The `with` helper is used when referencing configuration elements within a category of elements.

    {{#with cfg.section}}
      section {{expression1}} {{expression2}} {{expressionN}}
    {{/with}}

The following example from the redis.config file located in the `core/redis` plan, shows how to use the `with` helper to reference a set of values.

    {{#with cfg.save}}
      save {{sec}} {{keys}}
    {{/with}}

> Note: The corresponding TOML is shown in the next section.

Helpers can also be nested and used together in block expressions. Here is another example from the redis.config file where the `if` and `with` helpers are used together to set up `core/redis` Habitat services who are in a leader-follower topology.

    {{#if svc.me.follower}}
     {{#with svc.leader}}
      slaveof {{ip}} {{port}}
     {{/with}}
    {/if}}

For general information on block helpers, see http://handlebarsjs.com/block_helpers.html.

## TOML tables
[TOML tables](https://github.com/toml-lang/toml#table) can also be used if you have configuration elements that are grouped together into categories within your configuration file. To do so, use the following notation in your default.toml file.

    [table]
    key = value

The following examples from the `core/nginx` show how to create a TOML table that corresponds to a configuration element within a category.

**default.toml**

    [events]
    # worker_connections: Connections per Worker Process.  Default = 1024
    worker_connections = 1024

**nginx.conf**

    events {
        worker_connections  {{cfg.events.worker_connections}};
    }

[Arrays of tables](https://github.com/toml-lang/toml#array-of-tables) can also be used when you have more complex configuration elements to templatize. Here is an example of how that is done in the default.toml file in the `core/redis` plan.

    [[save]]
    sec = 900
    keys = 1

    [[save]]
    sec = 300
    keys = 10

    [[save]]
    sec = 60
    keys = 10000

## Further examples
For an example of how to templatize a configuration file and add it to your plan, see [Add configuration to your plan](/tutorials/getting-started-configure-plan) from the getting started tutorial.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/create-packages-binary-only">Binary-only packages</a></li>
</ul>
