+++
title = "Runtime Binds"
description = "Define runtime binds in your plan file"

[menu]
  [menu.habitat]
    title = "Runtime Binds"
    identifier = "habitat/packages/pkg-binds"
    parent = "habitat/packages"

+++

## Runtime Binds and Exports

*Runtime binding* in Chef Habitat refers to the ability for one service group to connect to another, forming a producer-consumer relationship where the consumer service can use the producer service's current configuration in order to configure itself at runtime. When the producer's configuration change, the consumer is notified and can reconfigure itself as needed.

With runtime binding, a consumer service can use a "binding name" of their choosing in their configuration and lifecycle hook templates as a kind of handle to refer to the configuration values they need from the producer service. This name isn't inherently tied to any particular package or service group name. Instead, when the service is run, users associate a service group with that binding name, which gives Chef Habitat all the information it needs to wire the producer and consumer services together.

Let's look at how we set up this relationship in detail.

### Defining the Producer Contract

A producer service defines its contract by "exporting" a subset of its runtime configuration. This is done by defining values in the `pkg_exports` associative array defined in your package's `plan.sh`. For example, a database server named `amnesia` might define the following exports:

```bash
pkg_exports=(
  [port]=network.port
  [ssl-port]=network.ssl.port
)
```

Note that Powershell plans use hashtables where Bash plans use associative arrays. A `plan.ps1` would declare its exports as:

```powershell
$pkg_exports=@{
  port="network.port"
  ssl-port="network.ssl.port"
}
```

This will export the runtime values of its `network.port` and `network.ssl.port` configuration entries publicly as `port` and `ssl-port`, respectively. All configuration entries in `pkg_exports` must have a default value in `default.toml`, but the actual exported values will change at runtime to reflect the producer's current configuration. When values change (such as when an operator uses `hab config apply`), the consumer service will be notified that its producer service configuration has changed. We'll see how to use this on the consumer in the sections below.

Producer services export only the *subset* of their configuration that is defined through `pkg_exports` and not the entire thing. Consumer services see only what the producer service exports, and nothing more. This is important, because it means that configuration that must remain secret--such as passwords--are not shared _unless_ they are explicitly defined in `pkg_exports`.

Additionally, the internal structure of the producer's configuration is independent of the exported interface it presents. In our example, `ssl-port` originally comes from a deeply-nested `network.ssl.port` value. However, the exported interface is _flat_, effectively a non-nested set of key-value pairs.

### Defining the Consumer Contract

The consumer service defines a "binding name" as a handle to refer to a service group from which it receives configuration data. However, it must do more than just name the bind, it must also state the configuration values it expects from the service group. Chef Habitat will make sure that whatever service group is bound actually exports the expected values to the consumer service.

As an example, let's say we have an application server, called `session-server`, that needs to connect to a database service, and needs both a "port" and an "ssl-port" in order to make that connection. We can describe this relationship in our `plan.sh` file like so:

```bash
pkg_binds=(
  [database]="port ssl-port"
)
```

Here, `pkg_binds` is an associative array. The key ("database") is the binding name, while the value ("port ssl-port") is a space-delimited list of the exported configuration the binding requires. A consumer can specify multiple binds; each would be an individual entry in this associative array. Judging from this, the producer we described above would be a good candidate for this application server to bind to, because it exports both a "port" and an "ssl-port".

A bound service group may export additional values, but they cannot export less and still satisfy the contract.

Chef Habitat only matches services up at the syntactic, not semantic, level of this contract. If you bind to a service that exports a "port", Chef Habitat only knows that the service exports something called "port"; it could be the port for a PostgreSQL database, or it could be the port of an application server. You will need to ensure that you connect the correct services together; Chef Habitat's binds provide the means by which you express these relationships. You are, however, free to create bind names and export names that are meaningful for you.

#### The Difference Between _pkg\_binds_ and _pkg\_binds\_optional_

In addition to the `pkg_binds` array, Plan authors may also specify `pkg_binds_optional`. It has exactly the same structure as `pkg_binds`, but, as the name implies, these bindings are _optional_; however, it is worth examining exactly what is meant by "optional" in this case.

In order to load a service into the Supervisor, each bind defined in `pkg_binds` *must* be mapped to a service group; if any of these binds are not mapped, then the Supervisor will refuse to load the service.

Binds defined in `pkg_binds_optional`, on the other hand, *may* be mapped when loading a service. If a service group mapping is not defined at load time, the Supervisor will load the service without question. As an extreme example, a service could have no `pkg_binds` entries, and five `pkg_binds_optional` entries; such a service could be loaded with no binds mapped, one bind mapped, all the way to mapping all five binds.

There are several scenarios where optional binds may be useful:

 * A service may have some default functionality which may be overridden at load-time by mapping an optional binding. Perhaps you have some kind of artifact repository service that, in the absence of a "remote-store" bind stores data on the local filesystem. However, if `remote-store` is bound to an appropriate S3 API-compatible service, such as [Minio](https://www.minio.io), it could modify its behavior to store data remotely.

 * A service can be optionally bind to a service to unlock additional features. For example, say you have an application that may run with or without a caching layer. You can model this using an optional bind named (say), "cache". If you wish to run without the caching functionality enabled, you can start the service without specifying a service group mapping for the "cache" bind. Since the bind is optional, it is not needed for Chef Habitat to run your service. However, if you do wish to run with the caching enabled, you can specify a service group mapping, e.g. `hab svc load acme/my-app --bind=cache:redis.prod`. In this scenario, your service's configuration can pull configuration values from the `redis.prod` service group, enabling it to use Redis as a caching layer.

* A service may can optionally bind one of several services; if bind "X" is mapped, operate _this_ way; if "Y" is mapped, operate _that_ way. An application that could use either a Redis backend or a PostgreSQL backend, depending on the deployment scenario, could declare optional "redis" and "postgresql" bindings, and pick which one to map at service load-time. If this is your use case, Chef Habitat does not have a way to encode the fact that "one and only one of these optional bindings should be mapped", so you will have to manage that on your own.

### Service Start-up Behavior

Prior to Chef Habitat 0.56.0, if the service group that you bound to was not present in the Supervisor network census, or had no live members, your service would not start until the group was present with live members. While this can be desirable behavior in some cases, as with running certain legacy applications, it is not always desirable, particularly for modern microservice applications, which should be able to gracefully cope with the absence of their networked dependencies.

With 0.56.0, however, this behavior can be modified using the new runtime service option `--binding-mode`. By setting `--binding-mode=relaxed` when loading a service, that service can start immediately, whether there are any members of a bound service group present or not. (Setting `--binding-mode=strict` will give you the previous, start-only-after-all-bound-groups-are-present behavior. This is also the current default, though `relaxed` will be the eventual default for Chef Habitat 1.0.0.). Such a service should have configuration and lifecycle hook templates written in such a way that the service can remain operational (though perhaps with reduced functionality) when there are no live members of a bound service group present in the network census.

#### The Difference Between Required Binds, Optional Binds, and Binding Mode

While there is a bit of overlap in these concepts, they are distinct. It's best to think of required and optional binds as defining "how applications can be wired together" (specifically, which "wires" must be connected in order to provide the minimal amount of information needed to run a service). Binding mode, on the other hand, defines how the application's start-up behavior is affected the presence or absence of its networked dependencies.

Another useful thing to keep in mind when thinking about required and optional binds is that service group mappings currently cannot be dynamically changed at runtime. They can only be changed by stopping a service, reloading the service with a new set of options, and then starting it up again. This constraint (which may change in future versions of Chef Habitat) may help guide your choice between what should be a required bind, and what should be optional, particularly when using the relaxed binding mode.

### Using Runtime Binds with Consumer Services

Once you've defined both ends of the contract, you can leverage the bind in any of your package's hooks or configuration files. Given the two example services above, a section of a configuration file for `session-server` might look like this:

```handlebars
{{~#each bind.database.members as |member|}}
  database = "{{member.sys.ip}}:{{member.cfg.port}}"
  database-secure = "{{member.sys.ip}}:{{member.cfg.ssl-port}}"
{{~/each}}
```

Here, `bind.<BINDING_NAME>` will be "truthy" (and can thus be used in boolean expressions) only if the bind has been satisfied, and `bind.<BINDING_NAME>.members` will be an array of only active members.

(Prior to Chef Habitat 0.56.0, `bind.<BINDING_NAME>` was always present, and `bind.<BINDING_NAME>.members` had _all_ members, even ones that had left the Supervisor network long ago. This necessitated using the `eachAlive` helper function, instead of just `each`.)

### Starting a Consumer Service

Since your application server defined `database` as a required bind, you'll need to provide the name of a service group running a package which fulfills the contract using the `--bind` parameter to the Supervisor. For example, running the following:

```bash
hab svc load <ORIGIN>/<NAME> --bind database:amnesia.default
```

would create a bind aliasing `database` to the `amnesia` service in the `default` service group.

The service group passed to `--bind database:{service}.{group}` doesn't *need* to be the service `amnesia`. This bind can be any service as long as they export a configuration key for `port` and `ssl-port`.

You can declare bindings to multiple service groups in your templates by using the `--bind` option multiple times on the command line. Your service will not start if your package has declared a required bind and a value for it was not specified by `--bind`.

