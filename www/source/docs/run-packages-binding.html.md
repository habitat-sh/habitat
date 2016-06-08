---
title: Running packages with runtime binding
---

# Runtime Binding

*Runtime binding* in Habitat refers to the ability for service groups to be named generically in plans, and hence Habitat packages, and for those generic service group names to be resolved to actual service group names at runtime.

For example, you might have a web application `foo` that depends on the value of the leader of a database service group. Rather than hardcoding the name of the service group in `foo`'s plan, which would limit its portability, you can _bind_ the name `database`, for example, to the service group name `postgresql.production` in a production environment, and `postgresql.development` in a development environment.

## Example

The [Ruby on Rails sample plan](https://github.com/habitat-sh/habitat/tree/master/plans/ruby-rails-sample) illustrates this exact situation. The [configuration template](https://github.com/habitat-sh/habitat/blob/master/plans/ruby-rails-sample/config/database.yml) for the database has content like this:

```
{{#if bind.database}}
{{#with bind.database.members}}  host: {{ip}}{{/with}}
{{/if}}
```

`database` is a generic name which will be substituted with the real name
using the `--bind` parameter to the supervisor, for example:

       hab start core/ruby-rails-sample --bind database:postgresql.qa

which would bind `database` to the `postgresql.qa` service group.

You can declare bindings to multiple service groups in your templates. The arguments to `--bind` are separated by commas.

The supervisor will throw an error if you have declared bindings but failed to resolve all of them with `--bind` when starting the package.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-security">Security</a></li>
</ul>
