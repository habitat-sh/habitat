---
title: Runtime settings
---

# Runtime settings
The following settings can be used during a Habitat service's lifecycle. This means that you can use these settings in any of the plan hooks, such as init, or run, and also in any templatized configuration file for your application or service.

These configuration settings are referenced using the [Handlebars.js](https://github.com/wycats/handlebars.js/) version of [mustache-style](https://mustache.github.io/mustache.5.html) tags. For an example on how these settings are used in plan hooks, see [Add Health Monitoring to a Plan](/tutorials/sample-app/mac/add-health-check-hook/) in the getting started tutorial.

## sys
These are service settings specified by Habitat and correspond to the network information of the running Habitat service.

**ip**
: The IP address of the running service.

**hostname**
: The hostname of the running service.

## pkg
These are package settings specified by Habitat and correspond to the the settings of the package when it was built and installed.

**origin**
: Denotes a particular upstream of a package. This value is pulled from the `pkg_origin` setting in a plan.

**name**
: The name of the package. This value is pulled from the `pkg_name` setting in a plan.

**version**
: The version of a package. This value is pulled from the `pkg_version` setting in a plan.

**release**
: The UTC datetime stamp when the package was built. This value is specified in _YYYYMMDDhhmmss_ format.

**ident**
: The fully-qualified identifier of a package that consists of origin/name/version/release.

**deps**
: An array of runtime dependencies for your package based on the pkg_deps setting in a plan.

**exposes**
: The port(s) to expose for an application or service. This value is pulled from the pkg_exposes setting in a plan.

**path**
: The location where the fully-qualified package is installed.

**svc_path**
: The root location of the source files for the Habitat service.

**svc\_config\_path**
: The location of any configuration files for the Habitat service.

**svc\_data\_path**
: The location of any data files for the Habitat service.

**svc\_files\_path**
: The location of any gossiped configuration files for the Habitat service.

**svc\_static\_path**
: The location of any static content for the Habitat service.

**svc\_var\_path**
: The location of any variable state data for the Habitat service.

**svc_user**
: The value of `pkg_svc_user` specified in a plan.

**svc_group**
: The value of `pkg_svc_group` specified in a plan.

**svc\_user\_default**
: The default user determined by the Habitat supervisor. `svc_user_default` will contain one of the following values, tested in order:

- `svc_user` if specified in the plan
- `hab` if the user exists
- the current user id

**svc\_group\_default**
: The default group determined by the Habitat supervisor. `svc_group_default` will contain one of the following values, tested in order:

- `svc_group` if specified in the plan
- `hab` if the group exists
- the effective group id

## cfg
These are settings defined in your templatized configuration file. The values for those settings are pulled from the `default.toml` file included in your package.