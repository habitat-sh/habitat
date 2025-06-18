+++
title = "Bootstrap Core Packages"
description = "Learn how to acquire a license key, sync core packages from Public Builder, and bootstrap your On-Prem Habitat Builder."
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "On-Prem Sync"
    identifier = "habitat/builder/on-prem-sync"
    parent = "habitat/builder"
    weight = 50
+++

# Bootstrap Core Packages

## Generate a Personal Access Token

Generate a Personal Access Token for bootstrapping the `core` packages, as well as for performing authenticated operations using the `hab` client.

Select your Gravatar icon on the top right corner of the Chef Habitat Builder on-prem web page, and then select **Profile**. This will take you to a page where you can generate your access token. Make sure to save it securely.

## Add a License Key

In order to download and sync official Chef-maintained packages from the Public Builder to your On-Prem Builder instance, a valid license key is required.

### Step 1: Obtain Your License Key

1. Log into your customer portal.
2. Copy the license key linked to your asset.
3. Enter the license key in Profile Page of Public Builder.

### Step 2: Add the License Key to Your On-Prem Account

You can enter your license key just below the Personal Access Token text field in order for your account to access official chef maintained packages.

## Enable Native Package Support

A couple of the new LTS supported packages include `native` packages. In order for an on-prem builder instance to host LTS packages, that builder instance must be configured to allow native package support. This is done by enabling the `nativepackages` feature and specifying `core` as an allowed native package origin. To do this, an on-prem builder's `/hab/user/builder-api/config/user.toml` file should be edited so that the `[api]` section looks as follows:

```
[api]
features_enabled = "nativepackages"
targets = ["x86_64-linux", "x86_64-linux-kernel2", "x86_64-windows"]
allowed_native_package_origins = ["core"]
```

## Bootstrap Builder with Habitat Packages

Chef Habitat Builder on-prem has no pre-installed package sets. You must populate your Builder instance by uploading packages.
To assist in bootstrapping an on-prem Builder instance with a set of core packages, you can install the habitat/pkg-sync package which will download packages from the public [SaaS Builder](https://bldr.habitat.sh) followed by a bulkupload to your on-prem Builder instance(s).

A valid license key must be registered to your account before attempting this step.

The following snippet illustrates how to bootstrap the on-prem Builder with a full set of stable core packages:

    ```bash
    sudo hab pkg install habitat/pkg-sync --channel LTS-2024
    hab pkg exec habitat/pkg-sync pkg-sync --bldr-url https://your-builder.tld --origin core --channel stable --private-builder-token <your_private_Builder_instance_token> --public-builder-token  <your_public_Builder_instance_token>
    ```

### Airgapped Environments

Airgapped builder instances must take an alternative approach because pkg-sync will not be able to transfer packages from the public internet to your instance. Instead you will use the `--generate-airgap-list` flag with pkg-sync to build a list of packages that need to be downloaded. Then you will use `hab pkg download` and `hab pkg upload` to download the packages from bldr.habitat.sh and upload them to your instance. Note that `pkg-sync` and `hab pkg download` must be used on a machine with access to the public internet. This will download a bundle you can archive and transfer to your instance. Finally you will use `hab pkg upload` locally on your builder instance to upload the packages into your instance.

The following section illustrates the steps required to bootstrap an airgapped on-prem Builder with a set of stable core packages:

1. Phase 1: download from a machine with internet connectivity

    ```bash
    sudo hab pkg install habitat/pkg-sync --channel LTS-2024
    hab pkg exec habitat/pkg-sync pkg-sync --generate-airgap-list --origin core --channel stable --public-builder-token  <your_public_Builder_instance_token>
    hab pkg download -u https://bldr.habitat.sh -z <your_public_Builder_instance_token> --target x86_64-linux --channel stable --file package_list_x86_64-linux.txt --download-directory builder_bootstrap
    hab pkg download -u https://bldr.habitat.sh -z <your_public_Builder_instance_token> --target x86_64-windows --channel stable --file package_list_x86_64-windows.txt --download-directory builder_bootstrap
    ```

    Archive the contents of `builder_bootstrap`. Copy and extract to the builder instance

1. Phase 2: bulkupload locally on the builder instance

    ```bash
    export HAB_AUTH_TOKEN=<your_private_Builder_instance_token>
    hab pkg bulkupload --url https://your-builder.tld --channel stable --auto-create-origins builder_bootstrap/
    ```

## Configuring a user workstation

Configuring a user's workstation to point to the Chef Habitat Builder on-prem should be fairly straightforward.

The following environment variables should be configured as needed:

1. `HAB_BLDR_URL` - this is the main (and most important) configuration. It should point to the instance of Chef Habitat Builder on-prem that you have set up. To invoke a Chef Automate-installed on-prem Builder from the command line, use:

```bash
export HAB_BLDR_URL=https://MY_ON_PREM_URL/bldr/v1/`
```

2. `HAB_AUTH_TOKEN` - this is the user's auth token that will be needed for private packages (if any), or for operations requiring privileges, for example, package uploads. The user will need to create their auth token and set/use it appropriately.
3. `SSL_CERT_FILE` - if the Chef Habitat Builder on-prem is configured with SSL and uses a self-signed or other certificate that is not in the trusted chain, then this environment variable can be used on the user's workstation to point the `hab` client to the correct certificate to use when connecting to Chef Habitat Builder on-prem.
