+++
title = "Bootstrap Core Packages"
description = "Using Artifactory with Chef Habitat Builder on-prem"

[menu]
  [menu.habitat]
    title = "Bootstrap Core Packages"
    identifier = "habitat/builder-on-prem/bootstrap-core"
    parent = "habitat"

+++

Create a `core` origin for an initial set of base packages. Uploads will fail unless you first populate your Chef Habitat Builder on-prem with the upstream `core` upstream origin.

Once you are logged in to the Chef Habitat Builder on-prem UI, select the `New Origin` button and enter in `core` as the origin name.

## Generate a Personal Access Token

Next, generate a Personal Access Token for bootstrapping the `core` packages, as well as for performing authenticated operations using the `hab` client.

Select your Gravatar icon on the top right corner of the Chef Habitat Builder on-prem web page, and then select **Profile**. This will take you to a page where you can generate your access token. Make sure to save it securely.

## Bootstrap Builder with Habitat Packages (**New**)

Chef Habitat Builder on-prem has no pre-installed package sets. You must populate your Builder instance by uploading packages.
With Habitat *0.88.0*, two new commands were introduced to assist in bootstrapping an on-prem Builder instance with a set of stable packages:

1. *hab pkg download*
1. *hab pkg bulkupload*

As you can see from the commands above, the package Bootstrap flow is comprised of two main phases: a download from the public [SaaS Builder](https://bldr.habitat.sh) followed by a bulkupload to your on-prem Builder instance(s). Historically, we bootstrapped on-prem-builders by downloading all the packages in 'core' for all targets. That amounted to ~15GB and was both too much and too little, in that many of the packages weren't needed, and for many patterns (effortless) other origins were needed.

The [new bootstrap process flow](https://forums.habitat.sh/t/populating-chef-habitat-builder-on-prem/1228) allows you to easily customize your Bootstrap package set or use pre-populated [Package Seed Lists](https://github.com/habitat-sh/on-prem-builder/blob/master/package_seed_lists/README.md) files.

The following section illustrates the steps required to bootstrap the on-prem Builder with the [Effortless Linux](https://github.com/habitat-sh/on-prem-builder/blob/master/package_seed_lists/effortless_x86_64-linux_stable) package seed list. Simply repeat the following download/bulkupload flow for any other package seed lists you think you will need to have in your on-prem Builder, or even create your own custom package seed list file:

1. Phase 1: download

    ```bash
    export HAB_AUTH_TOKEN=<your_public_Builder_instance_token>
    cd on-prem-builder
    hab pkg download --target x86_64-linux --channel stable --file package_seed_lists/effortless_x86_64-linux_stable --download-directory builder_bootstrap
    ```

    > Note: If the on-prem Builder is Airgapped, this phase must be completed on a system with Internet connectivity. The downloaded content will need to be zipped and then transferred to the Builder system for phase 2.

1. Phase 2: bulkupload

     **Important**: Inspect the contents of the `builder_bootstrap/artifacts` directory created from the download command above. For each of the origins (`core`, `effortless`, etc),  create the origin name if one doesn't exist already in the on-prem Builder UI before starting the bulkupload.

    > Note: If your on-prem builder's SSL certificate was issued from an internal Public Key Infrastructure and not from a Publicly Trusted Certificate Authority, you will need to copy the SSL public key cert chain into `/hab/cache/ssl` locally on the system that is uploading packages to the on-prem Builder. This is described in more detail [here](https://blog.chef.io/chef-habitat-product-announcement-improved-tls-certificate-management/)

    ```bash
    export HAB_AUTH_TOKEN=<your_on-prem_Builder_instance_token>
    hab pkg bulkupload --url https://your-builder.tld --channel stable builder_bootstrap/
    ```

## Bootstrap `core` packages (**Deprecated**)

*Important*: This methodology is now deprecated in favor of the download/bulkupload flow described above.

*Important*: Create a `core` origin before starting this process. The process will fail without first having a `core` origin.

Chef Habitat Builder on-prem has no pre-installed packages. To bootstrap a set of stable `core` origin packages (refer to the [core-plans repo](https://github.com/habitat-sh/core-plans)), you can do the following:

1. Export your Personal Access Token as `HAB_AUTH_TOKEN` to your environment

    ```bash
    export HAB_AUTH_TOKEN=<your token>
    ```

1. Run the population script, passing the root URL of your new Chef Habitat Builder on-prem as the last argument (Replace `http` with `https` in the URL if SSL is enabled)

    ```bash
    sudo -E ./scripts/on-prem-archive.sh populate-depot http://${BUILDER_HOSTNAME_OR_IP}`
    ```

This is quite a lengthy process, so be patient. It will download a *large* (~ 14GB currently) archive of the latest stable core plans, and then install them to your Chef Habitat Builder on-prem.

Please ensure that you have plenty of free disk space available for hosting the `core` packages as well as for managing your own packages. Updated packages install without deleting any existing packages, so plan disk space accordingly.

## Synchronizing 'core' packages from an upstream (**Deprecated**)

*Important*: This methodology is now deprecated in favor of the download/bulkupload flow described above.

*Important*: Create a `core` origin before starting this process. The process will fail without first having a `core` origin.

It is possible to also use the 'on-prem-archive.sh' script to synchronize the Chef Habitat Builder on-prem using the public Chef Habitat Builder site as an 'upstream'.

This allows new stable core packages from the upstream to get created in the Chef Habitat Builder on-prem instance automatically.

If your Chef Habitat Builder on-prem instance will have continued outgoing internet connectivity, you may wish to periodically run the script to check for updates.

1. Export your Personal Access Token as `HAB_AUTH_TOKEN` to your environment (e.g, `export HAB_AUTH_TOKEN=<your token>`)
1. `sudo -E ./scripts/on-prem-archive.sh sync-packages http://${BUILDER_HOSTNAME_OR_IP} base-plans`, passing the root URL of your new Chef Habitat Builder on-prem as the last argument. Replace `http` with `https` in the URL if SSL is enabled.

The 'base-plans' parameter restricts the sync to a smaller subset of the core packages. If you wish to synchronize all core packages, omit the 'base-plans' parameter from the script. Note that it will take much longer for the synchronization of all packages. Generally, it will only take a few minutes for base packages to synchronize.

You can also run the sync-packages functionality to initially populate the local Chef Habitat Builder on-prem.

*NOTE*: This functionality is being provided as an alpha - please log any issues found in the on-prem-builder repo.

## Configuring a user workstation

Configuring a user's workstation to point to the Chef Habitat Builder on-prem should be fairly straightforward.

The following environment variables should be configured as needed:

1. `HAB_BLDR_URL` - this is the main (and most important) configuration. It should point to the instance of Chef Habitat Builder on-prem that you have set up.
2. `HAB_AUTH_TOKEN` - this is the user's auth token that will be needed for private packages (if any), or for operations requiring privileges, for example, package uploads. The user will need to create their auth token and set/use it appropriately.
3. `SSL_CERT_FILE` - if the Chef Habitat Builder on-prem is configured with SSL and uses a self-signed or other certificate that is not in the trusted chain, then this environment variable can be used on the user's workstation to point the `hab` client to the correct certificate to use when connecting to Chef Habitat Builder on-prem.

