---
title: Building packages
---

# Build packages

Habitat packages are cryptographically-signed tarballs with a .hart extension that are built from plans. You can build a package in two ways: interactively from inside a studio, and non-interactively.

In both scenarios, you'll first need to have a secret origin key to sign your package. The origin key name should either match the `pkg_origin` value defined inside your plan, or match the overridden value specified with the `HAB_ORIGIN` environment variable.

## Create origin keys
As part of building a package, it needs to be signed with a secret origin key at buildtime. On your host machine, if you want to generate an origin key pair manually, or you used the `hab setup` and simply need another origin key pair, run the following command:

    hab origin key generate originname

The `hab-origin` subcommand will place originname-_timestamp_.sig.key and originname-_timestamp_.pub files (the origin key pair) in the `$HOME/.hab/cache/keys` directory. If you're creating origin keys in the studio container or you are running as root on a Linux machine, your keys will be stored in `/hab/cache/keys`.

Because the secret key is used to sign your package, it should not be shared freely; however, if anyone wants to download and use your package, then they must have your public key (.pub) installed in their local `$HOME/.hab/cache/keys` or `/hab/cache/keys` directory. Public keys will be downloaded from the depot by the supervisor, if needed.

### Passing origin keys into the studio
When you enter the studio environment, your origin keys are not automatically shared into it. This is to keep the studio environment as clean as possible. However, because you need to reference a secret origin key to sign your package, you can do this in three ways:

* Set `HAB_ORIGIN` to the name of the secret origin key you intend to use before entering the studio like `export HAB_ORIGIN=originname`.
* Set `HAB_ORIGIN_KEYS` to one or more key names, separated by commas like `export HAB_ORIGIN_KEYS=originname-internal,originname-test,originname`
* Use the `-k` flag (short for “keys”) which accepts one or more key names separated by commas with `hab studio -k originname-internal,originname-test enter`

The first way overrides the `HAB_ORIGIN` environment variable to import public and secret keys into the studio environment and override any `pkg_origin` values in the packages that you build. This is useful if you want to not only build your package, but also you can use this to build your own versions of other packages, such as `originname/node` or `originname/glibc`.

The second and third way import multiple secret keys that must match the origin names for the plans you intend to build.

After you create or receive your secret origin key, you can start up the studio and build your package.

## Interactive Build

An interactive build is one in which you enter a Habitat studio to perform the build. Doing this allows you to examine the build environment before, during, and after the build.

The directory where your plan is located is known as the plan context.

1. Change to the parent directory of the plan context.
2. Create an enter a new Habitat studio and pass the origin key into it. We'll assume your origin key is named `yourname`.

       hab studio -k yourname enter

       > Note: Same note as above applies when entering into a studio environment.

3. The directory you were in is now mounted as `/src` inside the studio. Enter the following command to create the package.

       build /src/planname

4. If the package builds successfully, it is placed into a `results` directory at the same level as your plan.

## Non-Interactive Build

A non-interactive build is one in which Habitat creates a studio for you, builds the package inside it, and then destroys the studio, leaving the resulting `.hart` on your computer. Use a non-interactive build when you are sure the build will succeed, or in conjunction with a continuous integration system.

1. Change to the parent directory of the plan context.
2. Build the package in an unattended fashion, passing the name of the origin key to the command.

        hab pkg build yourpackage -k yourname

3. The resulting package is inside a directory called `results`, along with any build logs and a build report (`last_build.env`) that includes machine-parseable metadata about the build.

By default, the studio is reset to a clean state after the package is built; however you can reuse a previous studio when building your package by specifying the `-R` option when calling the `hab pkg build` subcommand.

For more information on how to define a plan and build a package, how to create origin signing keys, and how to run a Habitat service, see the [getting started tutorial](/tutorials/getting-started-overview).

For information on the contents of an installed package, see [Package contents](/docs/reference/package-contents).

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/create-packages-debugging">Debug plans</a></li>
</ul>
