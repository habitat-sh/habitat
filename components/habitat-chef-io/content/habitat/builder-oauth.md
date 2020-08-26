
+++
title = "Builder On-Prem and OAuth"
description = "Authenticating Chef Habitat Builder on-prem with OAuth services, Azure AD (OpenId Connect), GitHub, GitLab (OpenId Connect), Okta (OpenId Connect) and Atlassian Bitbucket"

[menu]
  [menu.habitat]
    title = "OAuth"
    identifier = "habitat/builder-on-prem/oauth"
    parent = "habitat"

+++
{{< note >}}

Chef Automate has the `./chef-automate deploy --product builder --product automate` command, which offers the fastest and easiest Chef Habitat Builder on-prem installation and authentication path. See the [Install Chef Habitat on-prem with Chef Automate Guide](https://docs.chef.io/automate/on_prem_builder/) for more information.

{{< /note >}

## Pre-Requisites

Prior to starting the installation, please ensure you have reviewed all the items
in the Requirements section, and have a location for the installation that
meets all the requirements.

Note that the initial install will require _outgoing_ network connectivity.

Your Chef Habitat Builder on-prem instance will need to have the following _inbound_ port open:

* Port 80 (or 443 if you plan to enable SSL)

You may need to work with your enterprise network admin to enable the appropriate firewall rules.

### OAuth Application

We currently support Chef Automate v2, Azure AD (OpenId Connect), GitHub, GitLab (OpenId Connect), Okta (OpenId Connect) and Atlassian Bitbucket (cloud) OAuth providers for authentication. You will need to set up an OAuth application for the instance of the Chef Habitat Builder on-prem you are setting up.

Refer to the steps that are specific to your OAuth provider to create and configure your OAuth application. The below steps illustrate setting up the OAuth application using Github as the identity provider:

1. Create a new OAuth Application in your OAuth Provider - for example, [GitHub](https://github.com/settings/applications/new)
1. Set the homepage url value of `APP_URL` to `http://${BUILDER_HOSTNAME_OR_IP}/`, or `https://${BUILDER_HOSTNAME_OR_IP}/` if you plan to enable SSL.
1. Set the callback url value of `OAUTH_REDIRECT_URL` to `http://${BUILDER_HOSTNAME_OR_IP}/` (The trailing `/` is *important*). Specify `https` instead of `http` if you plan to enable SSL.
1. Record the the Client Id and Client Secret. These will be used for the `OAUTH_CLIENT_ID` and `OAUTH_CLIENT_SECRET` environment variables in the section below.

For the configuration below, you will also need to know following *fully qualified* end-points:

* Authorization Endpoint (example: `https://github.com/login/oauth/authorize`)
* Token Endpoint (example: `https://github.com/login/oauth/access_token`)
* API Endpoint (example: `https://api.github.com/user`)

For more information, please refer to the developer documentation of these services:

* [Azure Active Directory](https://docs.microsoft.com/azure/active-directory/develop/active-directory-protocols-oauth-code)
* [GitHub](https://developer.github.com/apps/building-oauth-apps/authorization-options-for-oauth-apps/)
* [GitLab](https://docs.gitlab.com/ee/integration/oauth_provider.html)
* [Okta](https://developer.okta.com/authentication-guide/implementing-authentication/auth-code)
* [BitBucket](https://confluence.atlassian.com/bitbucket/oauth-on-bitbucket-cloud-238027431.html)

For further information on OAuth endpoints, see the Internet Engineering Task Force (IETF) RFC 6749, [The OAuth 2.0 Authorization Framework](https://tools.ietf.org/html/rfc6749), page 21.

### Preparing your filesystem (Optional)

Since substantial storage may be required for holding packages, please ensure you have an appropriate amount of free space on your filesystem.

The package artifacts will be stored in your Minio instance by default, typically at the following location: `/hab/svc/builder-minio/data`

If you need to add additional storage, it is recommended that you create a mount at `/hab` and point it to your external storage. This is not required if you already have sufficient free space.

*Note*: If you would prefer to use Artifactory instead of Minio for the object storage, please see the [Artifactory](artifactory.md) documentation.

### Procuring SSL certificate (Recommended)

By default, the Chef Habitat Builder on-prem will expose the web UI and API via http. Though it allows for easier setup and is fine for evaluation purposes, for a secure and more permanent installation it is recommended that you enable SSL on the Chef Habitat Builder endpoints.

In order to prepare for this, you should procure a SSL certificate. If needed, you may use a self-signed certificate - however if you do so, you will need to install the certificate in the trusted chain on client machines (ones that will use the Chef Habitat Builder UI or APIs). You may use the `SSL_CERT_FILE` environment variable to also point to the certificate on client machines when invoking the `hab` client, for example:

```bash
SSL_CERT_FILE=ssl-certificate.crt hab pkg search -u https://localhost <search term>
```

Below is a sample command to generate a self-signed certificate with OpenSSL:

```bash
sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/ssl-certificate.key -out /etc/ssl/certs/ssl-certificate.crt
```

*Important*: Make sure that the certificate files are named exactly `ssl-certificate.key` and `ssl-certificate.crt`. If you have procured the certificate from a different source, rename them to the prescribed filenames, and ensure that they are located in the same folder as the `install.sh` script. They will get uploaded to the Chef Habitat supervisor during the install.

### Prerequisite Tasks for an Airgapped Installation (Required if applicable)

In order to install the on-prem Chef Habitat Builder in an airgapped (no direct Internet access) environment, the following preparatory steps are required.

> Note: Unless otherwise noted, the tasks are intended to be completed on a Non-Airgapped environment with Internet connectivity

1. Download the [Zip archive](https://github.com/habitat-sh/on-prem-builder/archive/master.zip) of the on-prem-builder repo

    ```bash
    curl -LO https://github.com/habitat-sh/on-prem-builder/archive/master.zip
    ```

1. Download the Chef Habitat [cli tool](https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux)

    ```bash
    curl -Lo hab.tar.gz https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux
    ```

1. Create the Habitat Builder package bundle from the [Builder Seed List](https://github.com/habitat-sh/on-prem-builder/blob/master/package_seed_lists/builder_x86_64-linux_stable) package seed list and download it

     ```bash
     git clone https://github.com/habitat-sh/on-prem-builder.git
     export DOWNLOAD_DIR=/some/base/download/directory
     cd on-prem-builder
     hab pkg download --target x86_64-linux --channel stable --file package_seed_lists/builder_x86_64-linux_stable --download-directory ${DOWNLOAD_DIR}/builder_packages
     ```

1. Create any additional package bundles to upload to Builder from package seed lists as documented in the [Bootstrap Builder](https://github.com/habitat-sh/on-prem-builder/blob/master/README.md#bootstrap-builder-with-habitat-packages-new) section of this README. You can specify `--download-directory ${DOWNLOAD_DIR}/builder_bootstrap` argument to the download command in order to consolidate all bootstrap packages in a single directory
1. Zip up all the above content, transfer and unzip on the Linux system where Builder will be deployed in the Airgapped environment

> Note: The following tasks are intended to be completed on the Airgapped system where Builder will be deployed, in advance of the [Installation](https://github.com/habitat-sh/on-prem-builder/blob/master/README.md#Installation).

1. From the zip archive, install the `hab` binary somewhere in $PATH and ensure it has execute permissions:

     ```bash
     sudo chmod 755 /usr/bin/hab
     sudo hab # read the license and accept if in agreement, as the root user
     ```

1. Import the public package signing keys from the downloaded Builder package bundle:

     ```bash
     export UNZIP_DIR=/some/base/unzip/directory
     for file in $(ls ${UNZIP_DIR}/builder_packages/keys/*pub); do cat $file | sudo hab origin key import; done
     ```

1. Create a Habitat artifact cache directory, place the Builder `*.hart` packages into that directory and then pre-install the Builder Services:

     ```bash
     sudo mkdir -p /hab/cache/artifacts
     sudo mv ${UNZIP_DIR}/builder_packages/artifacts/*hart /hab/cache/artifacts
     sudo hab pkg install /hab/cache/artifacts/habitat-builder*hart
     ```

1. Pre-install the Habitat Supervisor and its dependencies:

     ```bash
     sudo hab pkg install --binlink --force /hab/cache/artifacts/core-hab-*hart
     ```

## Setup

1. Clone this repo (or unzip the zip archive you previously downloaded from the Github release page) at the desired machine where you will stand up the Chef Habitat Builder on-prem
1. `cd ${SRC_ROOT}`
1. `cp bldr.env.sample bldr.env`
1. Edit `bldr.env` with a text editor and replace the values appropriately. Consider helping us to improve Chef Habitat as well by changing the `ANALYTICS_ENABLED` setting to `true` and providing an optional company name.

## Installation

> Note: If the on-prem Builder system is in an Airgapped (non-Internet connected) environment, you must first complete the [prerequisite](https://github.com/habitat-sh/on-prem-builder/blob/master/README.md#prerequisite-tasks-for-an-airgapped-installation-required-if-applicable) tasks detailed earlier.

1. `./install.sh`
1. `sudo systemctl restart hab-sup`

If everything goes well, you should see output similar to the following showing that the Chef Habitat Builder on-prem services are loaded:

```output
hab-sup(AG): The habitat/builder-datastore service was successfully loaded
hab-sup(AG): The habitat/builder-minio service was successfully loaded
hab-sup(AG): The habitat/builder-memcached service was successfully loaded
hab-sup(AG): The habitat/builder-api service was successfully loaded
hab-sup(AG): The habitat/builder-api-proxy service was successfully loaded
```

Do a `hab svc status` to check the status of all the services. They may take a few seconds to all come up.

If things don't work as expected (eg, if all the services are not in the `up` state), please see the Troubleshooting section below.

## Minio Web UI

The Chef Habitat Builder on-prem stores package artifacts in [Minio](https://github.com/minio/minio). By default, the Minio instance will be available on port 9000 (or whatever port you specified in your `bldr.env`). Please confirm that the Minio UI is available, and that you can log in with the credentials that were specified in your `bldr.env` file. There should already be a bucket created in which to host the artifacts.

## Chef Habitat Builder on-prem Web UI

Once the services are running successfully, the Chef Habitat Builder on-prem UI will become available at the configured hostname or IP address.

Navigate to `http://${BUILDER_HOSTNAME_OR_IP}/#/sign-in` to access the Chef Habitat Builder on-prem UI.

At that point you should be able to log in using your configured OAuth provider.

## Next Steps

[Bootstrap Core Origin](/docs/bootstrap-core.md)
