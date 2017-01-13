---
title: Running packages with runtime binding
---

# Runtime Binding

*Runtime binding* in Habitat refers to the ability for service groups to be named generically in plans, and hence Habitat packages, and for those generic service group names to be resolved to actual service group names at runtime.

For example, you might have a web application `foo` that depends on the value of the leader of a database service group. Rather than hardcoding the name of the service group in `foo`'s plan, which would limit its portability, you can _bind_ the name `database`, for example, to the service group name `postgresql.production` in a production environment, and `postgresql.development` in a development environment.

## Example

For an example look at the [haproxy](https://github.com/habitat-sh/core-plans/blob/master/haproxy/config/haproxy.conf) core-plan. These lines in the [haproxy.conf](https://github.com/habitat-sh/core-plans/blob/master/haproxy/config/haproxy.conf#L18-L23) illustrate how to reference a service binding. When a binding is enabled, the config will be rendered with a backend for each healthy instance in the service group.

~~~
{{#if bind.has_backend }}
{{~#each bind.backend.members}}
{{~#if alive }}
    server {{ip}} {{ip}}:{{port}}
{{~/if}}
{{~/each}}
~~~

`backend` is a generic name which will be substituted with the real name
using the `--bind` parameter to the supervisor, for example:

       hab start core/haproxy --bind backend:example-services

which would bind `backend` to the `example-services` service group.

You can declare bindings to multiple service groups in your templates. The arguments to `--bind` are separated by commas.

The supervisor will throw an error if you have declared bindings but failed to resolve all of them with `--bind` when starting the package.


<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-update-strategy">Update strategy</a></li>
</ul>
