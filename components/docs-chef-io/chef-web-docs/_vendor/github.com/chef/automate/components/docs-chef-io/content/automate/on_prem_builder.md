+++
title = "Install Chef Habitat Builder On-prem"

date = 2019-11-19T14:10:15-08:00

draft = false

[menu]
  [menu.automate]
    title = "Install Chef Habitat Builder On-prem"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/on_prem_builder.md Install Chef Habitat Builder On-prem"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/on_prem_builder.md)

This guide details how to install Chef Automate and deploy Chef Habitat Builder on-prem together. Enterprise customers may wish to set up an on-premises Chef Habitat Builder depot to store Chef Habitat packages for use by their own Chef Habitat Studios and Supervisors.

This guide covers setting up Chef Automate and Chef Habitat Builder on-prem, and bootstrapping Chef Habitat Builder on-prem with curated core seed lists from the Chef Habitat public Builder.

The Chef installer includes everything necessary to get started with Chef Automate and Chef Habitat Builder on-prem.
Bootstrapping Chef Habitat Builder requires:

* An outward bound HTTPS connection
* An existing Chef Habitat [public Builder](https://bldr.habitat.sh) account.

## System requirements

This guide demonstrates the ease of authenticating between Chef Automate and Chef Habitat Builder on-prem by installing both components on the same host.
Outside the boundaries of this proof-of-concept, we recommend against running installations of Chef Automate and Chef Habitat Builder on the same host. Please contact your Chef representative before using this implementation in production

### Hardware Requirements

The minimum with which Chef Automate and Chef Habitat Builder can be deployed on a single host is:

* 16 GB of RAM.
* 130 GB of disk space, available to /hab.
* 4 vCPUs.
* Inbound network connectivity from LAN (HTTP/HTTPS) is required for internal clients to access the on-premises Chef Habitat Builder.

For deployments that are expected to see production-scale workload, we recommend:

* 64 GB of RAM.
* 200 GB of diskspace, available to /hab.
* 16 vCPUs.

Roughly 80 GB of the disk space is designated for Chef Automate; the rest is used for
Chef Habitat Builder and the artifacts it stores. The current implementation uses Minio for
Chef Habitat artifact storage; we do not support using Artifactory for artifact storage.

### Operating System

Chef Automate and Chef Habitat Builder require:

* a Linux kernel of version 3.2 or greater
* `systemd` as the init system
* `useradd`
* `curl` or `wget`
* The shell that starts Chef Automate should have a max open files setting of at least 65535
* Run the installation and bootstrapping procedures as the superuser or use `sudo` at the start of each command.

### Unsupported Topologies

* high-availability/DR/multinode Builder

## Get Started with Chef Automate and Chef Habitat On-prem

### Download the Chef Automate Installer

Download and unzip the installer:

```shell
curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate
```

### Deploy Chef Automate and Chef Habitat Builder On-prem

Deploying Chef Habitat Builder with Chef Automate requires a Chef Automate license.
If you already have a Chef Automate license, you may use it for the deployment.
Otherwise, you can accept the 30-day trial license when you first sign in to Chef Automate.

If you are deploying Chef Habitat Builder with Chef Automate in an airgapped environment, follow [the documentation on building an airgap bundle]({{< relref "airgapped_installation.md" >}}).

You can deploy Chef Habitat Builder either with a full Chef Automate installation or with the Chef Automate auth stack only.

#### Deploy Chef Automate and Chef Habitat Builder

To deploy Chef Automate and Chef Habitat Builder, specify both the `builder` and `automate` products on the command line.
For example:

```shell
 ./chef-automate deploy --product builder --product automate
```

Accept the license with `y`.

#### Deploy Chef Habitat Builder with Chef Automate Auth

To deploy Chef Habitat Builder with the Chef Automate UI and Auth services only, specify the `builder` product on the command line.
For example:

```shell
 ./chef-automate deploy --product builder
```

Accept the license with `y`.

Please note that the Chef Automate UI will only support managing Users, Groups, and Authorization policies when deployed without the full Chef Automate stack.
If you wish to enable all of Chef Automate at a later time you can update the product configuration to include the complete Chef Automate stack. For example:

1. Create a new patch configuration toml as `config.toml` and update the products
  to include both `builder` and `automate`:

    ```toml
    [deployment.v1.svc]
      products = ["builder", "automate"]
    ```

1. Patch the configuration to deploy the rest of the Chef Automate services:

    ```shell
    ./chef-automate config patch config.toml
    ```

    You should see output similar to:

    ```output
    Updating deployment configuration

    Applying deployment configuration
      Installed automate-elasticsearch
      Installed automate-es-gateway
      Installed event-service
      Installed es-sidecar-service
      Installed event-feed-service
      Installed secrets-service
      Installed applications-service
      Installed notifications-service
      Installed nodemanager-service
      Installed compliance-service
      Installed ingest-service
      Installed config-mgmt-service
      Installed data-feed-service
      Installed event-gateway
      Started automate-elasticsearch
      Started automate-es-gateway
      Started event-service
      Started es-sidecar-service
      Started event-feed-service
      Started secrets-service
      Started applications-service
      Started notifications-service
      Started nodemanager-service
      Started compliance-service
      Started ingest-service
      Started config-mgmt-service
      Started data-feed-service
      Started event-gateway
      Started automate-load-balancer
    Success: Configuration patched
    ```

#### Add Chef Habitat Builder to a Chef Automate Installation

Patch an existing Chef Automate installation to add Chef Habitat Builder:

1. Create a `patch.toml` file to add `builder` to the list of products to deploy:

    ```toml
       [deployment.v1.svc]
       products=["automate", "builder"]
    ```

1. Apply the patch to the Chef Automate installation:

    ```shell
       sudo chef-automate config ./patch.toml
    ```

    The command output shows the Chef Habitat Builder services being added:

    ```shell
       Updating deployment configuration

       Applying deployment configuration
         Installed automate-minio
         Installed automate-builder-memcached
         Installed automate-builder-api
         Installed automate-builder-api-proxy
         Started automate-minio
         Started automate-builder-memcached
         Started automate-builder-api
         Started automate-builder-api-proxy
         Started automate-load-balancer
       Success: Configuration patched
    ```

### Sign in to Chef Automate and the Chef Habitat Builder

1. View your login credentials in the terminal with:

   ```bash
   cat automate-credentials.toml
   ```

    You should see output similar to:

   ```output
   url = "https://{{< example_fqdn "automate" >}}"
   username = "admin"
   password = "abcdefgh1234567890PASSWORDSTRING"
   ```

1. Navigate to `https://{{< example_fqdn "automate" >}}` in your browser. If you cannot access the site in Google Chrome, try Mozilla Firefox or Microsoft Edge.
1. Sign in to Chef Automate using the [credentials provided during deployment]({{< relref
   "install.md#open-chef-automate" >}}).
1. Select **Applications** in the top navigation bar
1. Select **Chef Habitat Builder** in the left sidebar, which redirects the browser to the Chef Habitat Builder login site
1. Select **Sign in with Chef Automate**
1. Sign into Chef Habitat Builder using the same credentials used with Chef Automate

The Chef Automate-deployed Chef Habitat Builder supports authentication with local users only. We plan on adding more authentication methods in future releases.

### Generate a Chef Habitat on-prem Builder Personal Access Token

You need a Personal Access Token for Chef Habitat on-prem in order to bootstrap packages and to perform authenticated operations with the `hab` client.

Select your Gravatar icon on the top right corner of the Chef Habitat Builder web page, and then select Profile. This will take you to a page where you can generate your access token. Make sure to save it securely.

### Create the Core Origin

Once you are signed in to the Chef Habitat Builder UI, select the **New Origin** button and enter `core` as the name for the origin.

## Access Chef Habitat Builder On-prem With Chef Habitat Command-Line Tools

Use the `https://{{< example_fqdn "automate" >}}/bldr/v1` URL when accessing your Chef Habitat Builder installation with the Chef Habitat command-line tools.
The Chef Habitat command-line tools recognize the [`HAB_BLDR_URL` environment variable](https://www.habitat.sh/docs/reference/), which you can set on the command line with:

```bash
export HAB_BLDR_URL=https://{{< example_fqdn "automate" >}}/bldr/v1/
```

## Bootstrap Chef Habitat Builder with Core Packages (Optional)

Prerequisites:

* HTTPS connection
* GitHub account
* Public Chef Habitat Builder account
* Public Chef Habitat Builder personal access token

Use [seed lists](https://github.com/habitat-sh/on-prem-builder/blob/master/package_seed_lists/README.md) to populate your on-premises Chef Habitat Builder installation with the packages required by your builds.
[Sample seed lists](https://github.com/habitat-sh/on-prem-builder/tree/master/package_seed_lists) exist for the following scenarios:

* Full `core`: the full contents of the upstream `core` origin. The x86_64 Linux set expands to 12GB, the Linux kernel2 set to 1GB, and the Windows set to 3.5GB.
* Core dependencies: a subset of `core` consisting of commonly-used buildtime dependencies.
* Effortless: packages used to start with the [Effortless pattern](https://github.com/chef/effortless). A complete Effortless implementation requires the contents of both the `stable` and the `unstable` channel.

### Clone the Chef Habitat Builder On-prem Repository

To access the curated seed lists for bootstrapping Chef Habitat Builder on-prem, you will need to clone the Chef Habitat Builder on-prem repository using https.

```shell
git clone https://github.com/habitat-sh/on-prem-builder.git
```

Once the repository is successfully cloned, move into the `on-prem-builder` repository:

```shell
cd on-prem-builder
```

The Chef Automate installer uses a self-signed certificate. Copy the SSL public key certificate chain from Chef Automate into `/hab/cache/ssl` with this command:

```shell
cp /hab/svc/automate-load-balancer/data/{{< example_fqdn "automate" >}}.cert /hab/cache/ssl/{{< example_fqdn "automate" >}}.cert
```

### Download Seed List Packages from the Public Chef Habitat Builder

Your host must have access to the internet to download the curated seed list packages from the **public** [Chef Habitat Builder](https://bldr.habitat.sh).
If you have not already done so, create a user account and personal access token on the **public** [Chef Habitat Builder](https://bldr.habitat.sh/).

Use the `hab pkg download` command with a seed list `</path/to/seed_list>` to download packages for
your desired architecture `<arch>` from a channel `<channel>` to a directory `<artifact-dir>`:

```shell
HAB_AUTH_TOKEN=<your_public_builder_personal_access_token> hab pkg download --target <arch> --channel <channel> --file </path/to/seed_list> --download-directory <artifact-dir>
```

For example, to use the Effortless seed list to download `x86_64-linux` packages from the
`stable` channel to the `builder_bootstrap` directory:

```shell
HAB_AUTH_TOKEN=<your_public_builder_personal_access_token> hab pkg download --target x86_64-linux --channel stable --file package_seed_lists/effortless_x86_64-linux_stable --download-directory builder_bootstrap
```

### Bulk-Upload Seed List Packages to Chef Habitat Builder on-prem

Run the `bulkupload` command to upload artifacts from `<artifact-dir>` to the `<channel>` channel in the on-premises Chef Habitat Builder using the [Builder API endpoint]({{< ref "on_prem_builder.md#access-chef-habitat-builder-on-prem-with-chef-habitat-command-line-tools" >}}):

```shell
HAB_AUTH_TOKEN=<your_on-prem_Builder_personal_access_token> hab pkg bulkupload --url https://{{< example_fqdn "automate" >}}/bldr/v1 --channel <channel> <artifact-dir> --auto-create-origins
```

For example,

```shell
HAB_AUTH_TOKEN=<your_on-prem_Builder_personal_access_token> hab pkg bulkupload --url https://{{< example_fqdn "automate" >}}/bldr/v1 --channel stable builder_bootstrap/ --auto-create-origins
```

Should produce the output:

```output
Preparing to upload artifacts to the 'stable' channel on https://{{< example_fqdn "automate" >}}/bldr/v1
Using builder_bootstrap/artifacts for artifacts and builder_bootstrap/keys signing keys
Found 46 artifact(s) for upload.
Discovering origin names from local artifact cache
Missing origin 'chef'
Origin 'core' already exists
Missing origin 'effortless'
Creating origin chef.
Created origin chef.
Creating origin effortless.
Created origin effortless.
75 B / 75 B | [===========================================] 100.00 % 1.31 MB/s d
Uploading public origin key chef-20160614114050.pub
...
```

The `--auto-create-origins` flag creates each origin listed in the
`<artifact-dir>/artifacts` directory. If you omit the `--auto-create-origins` flag,
use the Chef Habitat Builder UI to create the necessary origins before running the
`bulkupload` command.

To finish up, return to your Chef Habitat Builder on-prem installation and view the packages that you've added to your `core` origin at `https://{{< example_fqdn "automate" >}}/bldr/#/origins/core/packages`.

## Using Chef Habitat Builder

Because you are using an on-prem installation of Chef Habitat Builder, you must specify the [Builder API endpoint of your installation]({{< ref "on_prem_builder.md#access-chef-habitat-builder-on-prem-with-habitat-command-line-tools" >}}) when following the [Habitat Builder documentation](https://www.habitat.sh/docs/using-builder/).
This documentation covers [using origin keys](https://www.habitat.sh/docs/using-builder/#using-origin-secrets), [using origin secrets](https://www.habitat.sh/docs/using-builder/#using-origin-secrets), and [uploading and promoting packages](https://www.habitat.sh/docs/using-builder/#upload-and-promote-packages).

## Operating Chef Habitat Builder

Chef Habitat Builder uses the same mechanisms that Chef Automate does for [backups]({{< relref "backup.md" >}}), [log management]({{< relref "log_management.md" >}}), and [uninstalling]({{< relref "troubleshooting.md#uninstalling-chef-automate" >}}).

### Logging errors

To change the log level for Chef Habitat Builder only, create a TOML file that contains the partial configuration below. Uncomment and change settings as needed, and then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

```toml
[builder_api.v1.sys.log]
level = "debug"
scoped_levels = ["tokio_core=error", "tokio_reactor=error", "zmq=error", "hyper=error" ]
```

## Setting up Automate as an OAuth Provider for Habitat Builder (Deprecated)

{{< warning >}}
These instructions have been deprecated in favor of using the Chef Automate installer to deploy Chef Habitat on-prem.
{{< /warning >}}

To configure Chef Automate as an OAuth Provider for Chef Habitat Builder, create a TOML file with the partial configuration below.
Run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

`bldr_client_id` and `bldr_client_secret` simply need to match what you configured for the corresponding
values in Chef Habitat Builder (see below). However, we strongly recommend those values follow
[best practices](https://www.oauth.com/oauth2-servers/client-registration/client-id-secret/)
for `client_id` and `client_secret` in the Oauth2 standard.

```toml
[session.v1.sys.service]
bldr_signin_url = "<your Builder fqdn>" # for example, "http://builder.test/"

# This needs to match what you configured OAUTH_CLIENT_ID as when you configured Habitat Builder.
bldr_client_id = "<your Habitat Builder Oauth2 Client ID>"

# This needs to match what you configured OAUTH_CLIENT_SECRET as when you configured Habitat Builder.
bldr_client_secret = "<your Habitat Builder Oauth2 Client Secret>"
```

In addition, add Chef Automate's TLS certificate to Builder's list of accepted certificates.
Locate Chef Automate's default self-signed certificate by running `cat /hab/svc/automate-load-balancer/data/{{< example_fqdn "automate" >}}.cert`, copy this default certificate, and then add it to your Builder instance's list of accepted certificates.

```text
-----BEGIN CERTIFICATE-----
MIIDfDCCAmSgAcaSldKaf...
-----END CERTIFICATE-----
```

If you are using a certificate signed by a trusted certificate authority instead of the default certificate,
you can provide Builder with the root certificate authority for the signed certificate.

For more information, check out this further explanation on how to [configure Builder to authenticate via Chef Automate](https://github.com/habitat-sh/on-prem-builder).
