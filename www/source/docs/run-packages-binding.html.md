---
title: Running packages with runtime binding
---

# Runtime Binding

*Runtime binding* in Habitat refers to the ability for one service group to connect to another forming a producer/consumer relationship where the consumer can use the producer's publicly available configuration to configure it's services at runtime. We form a [polymorphic relationship](https://en.wikipedia.org/wiki/Polymorphism_(computer_science)) between these two service groups defined by a [dynamically-typed](https://en.wikipedia.org/wiki/Duck_typing) contract where the consumer specifies a generic name of the service it is consuming along with the configuration keys it expects the producer to export.

For example, you might have a web application `app-server` that depends on the value of the leader of a database service group. Rather than hardcoding the name of the service group or package identifier in `app-servers`'s plan, which would limit its portability, you can _bind_ the name `database`, for example, to the `default` service group running PostgreSQL. If you have multiple service groups for PostgreSQL - perhaps you have a production and development environment - you could bind `database` to `postgresql.production` or `postgresql.development`. If `app-server` supports multiple different database backends you could even bind `database` to another, such as `redis.default` or `mysql.default`.

## Producer Contract

The producer defines their contract by "exporting" configuration publicly to consumers. This is done by setting keys in the `pkg_exports` associative array defined in your package's `plan.sh`. For example, a database server named `amnesia` might define the exports:

    pkg_exports=(
      [port]=network.port
      [ssl-port]=network.ssl.port
    )

This will export the value of `network.port` and `transport.ssl.port` defined in it's `default.toml` publicly as `port` and `ssl-port` respectively. All `pkg_exports` must define a default value in `default.toml` but their values may change at runtime by an operator configuring the service group. If this happens, the consumer will be notified that their producer's configuration has changed. We'll see how to leverage this on the consumer in the sections below.

## Consumer Contract

Consumers defines their half of the contract by specifying required and optional "binds". These are also represented by key/value pairs in an associative array called `pkg_binds` and `pkg_binds_optional` where the values are the exported keys defined by the producer. For example, an application server named `session-server` that depends on a database might define the following binds:

    pkg_binds=(
      [database]="port ssl-port"
    )

This says that `session-server` needs to bind to a service aliased as `database` and that service must export a configuration key for both `port` and `ssl-port`. This would make this application service compatible with the producer we defined above for a database called `amnesia` since it does export a value for both of these keys.

## Consumer's Configuration Example

Once you've defined both ends of the contract you can leverage the bind in any of your package's hooks or configuration files. Given the two example services above, a section of a configuration file for `session-server` might look like this:

~~~
{{#if bind.has_database }}
{{~#each bind.database.members as |member| }}
{{~#if member.alive }}
  database = "{{member.sys.ip}}:{{member.cfg.port}}"
  database-secure = "{{member.sys.ip}}:{{member.cfg.ssl-port}}"
{{~/if}}
{{~/each}}
{{~/if}}
~~~

## Starting A Consumer

Since your application server defined `database` as a required bind, you'll need to provide the name of a service group running a package which fulfills the contract using the `--bind` parameter to the supervisor. For example, running the following:

    hab start my-origin/app-server --bind database:amnesia.default

would create a bind aliasing `database` to the `amnesia` service in the `default` service group.

The service group passed to `--bind database:{service}.{group}` doesn't *need* to be the service `amnesia`. This bind can be any service as long as they export a configuration key for `port` and `ssl-port`.

You can declare bindings to multiple service groups in your templates by using the `--bind` option multiple times on the command line. Your service will not start if your package has declared a required bind and a value for it was not specified by `--bind`.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-update-strategy">Update strategy</a></li>
</ul>
