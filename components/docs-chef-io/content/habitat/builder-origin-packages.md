+++
title = "Upload and Promote Packages"
description = "Upload and Promote packages on Chef Habitat Builder enables automated package rebuilds and increases collaboration"

[menu]
  [menu.habitat]
    title = "Origin Packages"
    identifier = "habitat/builder/origin-packages"
    parent = "habitat/builder"
    weight = 40

+++

While you can build and run Chef Habitat packages without sharing them on [Chef Habitat Builder](https://bldr.habitat.sh), uploading them there enables greater collaboration and automated package rebuilds as underlying dependencies or your connected GitHub repository are updated.

> Note: Chef Habitat Builder can only build Linux based plans (`plan.sh`) at this time.

Setting up Chef Habitat Builder is easily done on the website: these steps take you through connecting your local Studio development experience with Builder.

You interact with Chef Habitat Builder by:

* Creating an account.
* Creating an origin, or being invited to join an origin that already exists.
* Setting up `hab` to authenticate with Builder.
* Uploading the private and public keys for that origin.
* Connecting your Github repositories and opting into rebuilds.

Chef Habitat Builder supports both public and private origins, packages, and Github repositories.

## Create a Builder Account

If you haven't created an account yet, see the [Create a Builder Account](/using-builder/builder-account) section above.

## Create or Join an Existing Origin

You can create your own origin in Builder or be invited to join an existing one. If you already built some Chef Habitat packages on your local computer prior to signing up for an account, you must rename your local packages' `pkg_origin` if the origin you want already exists.

## Set up Chef Habitat to Authenticate to Builder

When you upload a package to Builder, you are required to supply an auth token as part of the `hab pkg upload` subcommand. You can generate a Chef Habitat personal access token via the Builder site [Profile page](https://bldr.habitat.sh/#/profile) for use with the `hab` command-line utility.

Once you have this token, you can set the `HAB_AUTH_TOKEN` [environment variable](/reference#environment-variables) to this value, so that any commands requiring authentication will use it.

## Create an Origin Key Pair

After finishing the basic account creation steps, you need to create your origin key pair. Habitat will use the private origin key to sign the artifacts (`.hart` files) created by building your plan and verify the integrity of your artifacts with the public origin key.

You can create an origin key pair by running `hab cli setup` from your host machine, or by running `hab origin key generate <ORIGIN>` from either the host machine or from within the studio.

Your public and private origin keys are located at `~/.hab/cache/keys` on your host machine and at `/hab/cache/keys` inside the studio environment.

## Upload Your Origin Keys

If you created a new Habitat origin from your host machine or from the Studio, Builder will not have either of the origin keys corresponding to your artifact. Builder will not accept uploaded artifacts without first having the correct public origin key.

You can upload keys for the origin through the web interface for Builder, or by using the `hab origin key upload` command. You must have the access token for authentication, as described earlier, before you can upload keys.

## Upload Packages to Builder

As long as you are already a member of the Habitat origin, once Builder possesses at least the public origin key, then you may upload one or more artifacts to that origin with the `hab pkg upload` command. After Habitat validates the cryptographic integrity of the artifact, it is then uploaded and stored on Builder. Uploading artifacts is a privileged operation for which you must have the access token.

## Promote Packages

<%= partial "/partials/global/channel-overview" %>

By default, newly uploaded packages are placed in the `unstable` channel. However, the default package that is downloaded is the latest `stable` version of a package, unless overridden in commands such as `hab sup run`, `hab svc load`, and `hab pkg install`. If you want to promote your package to the `stable` channel, run the `hab pkg promote` command as follows:

```bash
$ hab pkg promote -z <TOKEN> origin/package/version/release stable
```

> **Note** You can also promote packages to the `stable` channel using the *promote to stable* button in the web app.

For more information on how to use channels, see [Continuous Deployment Using Channels](/using-habitat/continuous-deployment).

### Running Packages from Builder

> **Note:** When running private packages from Builder, it's necessary to add your [Chef Habitat access token](/using-builder/builder-token) to the machine where you intend to deploy the package, via `export HAB_AUTH_TOKEN=<token>`.

You can instruct the Supervisor to download and run packages from Builder by using the `hab sup` and `hab svc` commands, for example:

```bash
$ hab sup run
$ hab svc load core/postgresql
```

If the Supervisor does not have the `core/postgresql` package in its local cache, it will contact Builder, retrieve the latest version and the public key for the `core` origin, verify the cryptographic integrity of the package, and then start it.

You may also supply a `--channel` argument to instruct the Supervisor to use a different channel for the purposes of continuous deployment:

```bash
$ hab svc load core/postgresql --channel unstable
```

### Running Packages from Exported Tarballs

An exported tarball package contains the Chef Habitat client/binary as well as dependencies specified by your artifact.

After deploying the tarball to your target server, extract the contents to the root filesystem (`/`):

```bash
$ tar zxf core-nginx-1.11.10-20170616000025.tar.gz --directory /
```

You can instruct the Supervisor to run packages from an exported tarball:

```bash
$ /hab/bin/hab svc start core/nginx
```

Note: On a clean server, this will download additional packages to satisfy the Supervisor dependencies. You will also require a `hab` group and `hab` user on the system for most services.

## Building Packages with Multiple Plans

If you have a GitHub repository with multiple components inside, you will most likely also have individual plans for those components that are located inside of component subfolders. By default, Builder will only look for a package plan in either the root of the repository, or in a `habitat` subfolder at the root. If it does not find a plan file in those locations, it will not automatically issue builds when it detects file changes in the repository.

In order to tell Builder about the location of the individual plan files, and in order provide more fine-grained control over when component packages are built, you can programmatically customize how and when Builder will build your plans by specifying build behavior in a `.bldr.toml` file at the root of the repository that you connect to Builder.

Using this file, Builder only builds packages when source files or directories are updated in paths specified in `.bldr.toml`. This allows you to configure the building, publishing, and post-processing phases of a plan build in Builder.

To enable this functionality, do the following:

1. Create a `.bldr.toml` in the root of your repository.

2. Open it and add an entry for each component package that you want to build.

    The `.bldr.toml` file is in TOML format, so create a TOML table specifying the `$pkg_name` value for that plan and then add a `plan_path` field specifying the path to your `plan.sh` file (you do not need to include plan.sh explicitly in the path). If all the files related to the plan are under the plan path, then you are done. Otherwise, you will need an additional 'paths' field specifying Unix-style path globs to files that are associated with the plan you specified in the 'plan_path'. File or directory changes made in these path locations determine which packages will be rebuilt. Basically, when a file is committed, Builder will check to see whether it falls underneath the `plan_path` hierarchy, or matches one of the globs in the `paths` field if it was specified - if the answer is yes, then Builder will issue a build for that commit.

    It's important to note that the entries for `plan_path` and `paths` do not behave the same. If you have something like `plan_path = "habitat"`, that behaves as if you had written `plan_path = "habitat/*"` - that is, it will automatically check every file under the `habitat` directory. However, if you have something like `paths = [ "src" ]`, that is _not_ automatically expanded to `src/*`. That line will only watch for changes to a file called `src`. If you're wanting to watch for changes to any file inside the `src` directory, then you must explicitly specify the glob, like so: `paths = [ "src/*" ]`.

    For example, in the Chef Habitat repository itself, this TOML states that the `hab-launcher`, `hab-studio`, and `hab-sup` packages will be rebuilt if there are any changes in any of the specified `components` sub-directories. Note that `hab-studio` does not need to specify a `path` because all of it's files are within the `plan_path` hierarchy, but that is not the case for the other projects.

    ```toml
    # .bldr.toml
    [hab-launcher]
    plan_path = "components/launcher/habitat"
    paths = [
      "components/launcher/*",
      "components/launcher-protocol/*",
      "support/ci/builder-base-plan.sh",
    ]

    [hab-studio]
    plan_path = "components/studio"

    [hab-sup]
    plan_path = "components/sup"
    paths = [
      "components/sup/*",
      "components/eventsrv-client/*",
      "components/launcher-client/*",
      "components/butterfly/*",
      "components/core/*",
      "components/builder-depot-client/*",
    ]
    ```

    Notice that in order to specify that we're interested in all files inside of the directories in our `paths` entries, we had to add the `/*` glob to the end manually.

    It's also worth pointing out that there are multiple wildcard characters you can use when specifying path components.

* `?` will match any single character.
* `*` will match any (possibly empty) sequence of characters
* `**` matches the current directory and arbitrary subdirectories. This sequence must form a single path component, so both `**a` and `b**` are invalid. More than two consecutive `*` characters is also invalid.
* `[...]` matches any character inside the brackets. You can also specify a range, such as `[0-9]` to match any digit or `[a-z]` to match any lowercase letter.
* `[!...]` is the negation of `[...]` so it will match any character *not* in the brackets.

    Note that while the above set of rules bears a remarkable resemblance to regular expressions, we do not support full regular expression syntax. Only what's shown here is supported. Here is an example.

    ```toml
    # .bldr.toml
    [hab-sup]
    plan_path = "components/sup"          # automatically checks every file inside the 'sup' directory
    paths = [
      "components/sup/?",                 # matches any file with a single character file name inside the 'sup' directory
      "components/eventsrv-client/*",     # matches any file inside the 'eventsrv-client' directory
      "components/launcher-client/**/*",  # matches any file inside the 'launcher-client' directory and also any of its sub-directories
      "components/butterfly/[0-9]*"       # matches any file inside the 'butterfly' directory that begins with a number
    ]
    ```

## Automated Builds

By connecting a plan file in <a href="https://bldr.habitat.sh/#/sign-in" class="link-external" target="_blank">Chef Habitat Builder</a>, you can trigger both manual (via the web UI, or via the `hab` command line) as well as automated package rebuilds whenever a change is merged into the `master` branch of the repository containing your Chef Habitat plan, or when a dependent package updates (rebuilds).

### Connect a Plan

To connect a plan to Builder, view one of your origins (while signed in), click the **Connect a plan file** button, and complete the following steps:

  - Install the Builder GitHub App
  - Choose the GitHub organization and repository containing your Chef Habitat plan
  - Choose a privacy setting for the package
  - Specify container-registry publishing settings (optional)
  - Specify auto-build option (default is off)

### Auto-build Option

The auto-build option controls whether or not your package will get automatically re-built. This option is a useful capability to have - for example, if you have a demo app that doesn't need to be kept constantly up to date when some underlying dependency updates. Auto-build encompasses both builds that are triggered by Github web hooks (on commits to master), as well as builds that are triggered by a dependency updating.

By default, new plan connections will have auto-build turned off.
