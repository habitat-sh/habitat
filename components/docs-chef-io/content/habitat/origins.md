+++
title = "Create an Origin"
description = "Create an Origin on Chef Habitat Builder"

[menu]
  [menu.habitat]
    title = "Create an Origin"
    identifier = "habitat/origins Create an Origin"
    parent = "habitat/origins"
    weight = 10

+++

An origin is a space on Chef Habitat Builder where you can store, share, and build packages. It is a unique namespace within Chef Habitat Builder, and while you can delete or transfer an origin, you can't rename an origin after it is created. One example of an origin is the "core" origin, which is the set of foundational packages managed and versioned by the core Chef Habitat maintainers.

You can join existing origins by invitation and you can create your own origins.
For more on invitations, see [origin membership and RBAC]({{< relref "origin-rbac.md#origin-membership" >}}).

### Create an Origin

![Chef Habitat Builder without origins](/images/habitat/create-origin.png)

To create an origin, select the **Create origin** button on the _My Origins_ page which opens the _Create New Origin_ form. (Chef Habitat Builder > My Origins )

![Creating an origin](/images/habitat/create-origin-form.png)

First, enter a unique name that you want to associate with your packages.  Chef Habitat will only let you create an origin with a unique name. Some examples that you'll see in Chef Habitat Builder are team names, user names, and abstract concepts.

Next, choose a privacy setting to set as the default for new packages. You can override this setting when uploading individual packages from the CLI or by connecting a plan file that declares a package as private. The difference between public and private packages is:

- Anyone can find and use public packages
- Only users with origin membership can find and use private packages

When you select **Save and Continue**, Chef Habitat Builder:

1. Creates your origin
1. Creates an [origin key pair]({{< relref "origin-keys.md" >}})
1. Redirects Chef Habitat Builder to the origin page

![Origin successfully created](/images/habitat/create-origin-done.png)

#### Create an Origin with the Chef Habitat CLI

Use the [hab origin]({{< relref "habitat-cli.md#hab-origin" >}}) commands to manage your origins from the command line.

Create an origin from the command line with the [hab origin create]({{< relref "habitat-cli.md#hab-origin-create" >}}) command

```bash
hab origin create <origin>
```

The results of this command differ slightly from creating an origin on the Chef Habitat Builder site. The CLI command:

1. Creates an origin on the Chef Habitat Builder site
1. Does _not_ generate an origin key pair

For more information, see the [`hab origin create`]({{< relref "habitat-cli.md#hab-origin-create" >}}) CLI documentation.
