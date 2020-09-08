+++
title = "Airgapped Installation"

draft = false

[menu]
  [menu.automate]
    title = "Airgapped Installation"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/airgapped_installation.md Airgapped Installation"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/airgapped_installation.md)

## Overview

An airgapped host is one that has no direct inbound or outbound internet
traffic.
To install or upgrade Chef Automate on an airgapped host, you will need to create an Airgap Installation Bundle (`.aib`) on an internet-connected host and then transfer both the Airgap Installation Bundle and the `chef-automate` binary that you used to create it to the airgapped host for use.

## Get a License

To get a trial license for an airgapped host [contact Chef](https://www.chef.io/contact-us/).

## Create an Airgap Installation Bundle

On an internet-connected host, download the Chef Automate command-line tool and use it to
prepare an Airgap Installation Bundle.

### Get Chef Automate Installer and Admin Tool

Download the Chef Automate command-line tool from the `current` [release channel]({{< relref "install.md#release-channels" >}}).

```shell
curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate
```

### Prepare Airgap Installation Bundle

To download and bundle the software included in the most recent Chef Automate release, run:

```shell
./chef-automate airgap bundle create
```


To download and bundle the software included in a specific Chef Automate release, run

```shell
./chef-automate airgap bundle create --version VERSION
```

These commands produce an Airgap Installation Bundle named:

`automate-<timestamp>.aib`.

## Deploy the Airgap Installation Bundle

Transfer the `chef-automate` binary and the Airgap Installation Bundle to the airgapped host.
Save the Chef Automate command-line tool in a directory that is NOT in $PATH. The Chef Automate installation process puts a copy of `chef-automate` into `/bin` and manages it.

### Create Default Configuration

``` shell
sudo ./chef-automate init-config --upgrade-strategy none
```

creates a `config.toml` file with default values. Setting an upgrade strategy of `none`
prevents Chef Automate from checking its release channel for updates via the internet.

Edit `config.toml` to make changes to FQDN and other configuration settings. See
[Configuring Chef Automate]({{< relref "configuration.md" >}}) for more information on configuration settings.

### Deploy Chef Automate

```shell
sudo ./chef-automate deploy config.toml --airgap-bundle </path/to/airgap-install-bundle>
```

Deploying Chef Automate takes ten minutes for a clean install.
At the command prompt, accept the terms of service with a `y`, The installer then performs a series of pre-flight checks. Any
unsuccessful checks offer information for resolving issues or skipping the check.
After resolving any pre-flight issues, run the deploy command again.

At the end of the deployment process you will see:

```shell
Deploy complete
```

The deployment process writes login credentials to the `automate-credentials.toml` in your current working directory.

### Open Chef Automate

Navigate to `https://{{< example_fqdn "automate" >}}` in a browser and log in to Chef Automate with
the credentials provided in `automate-credentials.toml`.

Once you log in, Chef Automate prompts you for a license.

#### Configure Data Collection

To send data from your Chef Infra Server or Chef Infra Clients to Chef Automate 2, the process is the same as Chef Automate 1.
See [Configure Data Collection]({{< relref "data_collection.md" >}}) for more information.

### Upgrades

We've committed to ensuring the stability of the upgrade and to supporting Chef Automate's automatic upgrades.
To upgrade an airgapped install, you must supply an airgap bundle.

On an internet-connected host, follow the steps in [Create an Airgap
Installation Bundle]({{< relref "#create-an-airgap-installation-bundle" >}}) to upgrade your
Chef Automate command-line tool and prepare an Airgap Installation Bundle. Transfer the
bundle and Chef Automate command-line tool to the airgapped host and run:

```shell
sudo chef-automate upgrade run --airgap-bundle </path/to/bundle>
```

### Common Problems

If you are unable to open Chef Automate, check that the `config.toml` contains the host's public DNS name as the FQDN.

```shell
# This is a default Chef Automate configuration file. You can run
# 'chef-automate deploy' with this config file and it should
# successfully create a new Chef Automate instance with default settings.

[global.v1]
# The external fully qualified domain name.
# When the application is deployed you should be able to access 'https://<fqdn>/'
# to login.
fqdn = "<_Public DNS_name_>"
```

Once you correct and save the FQDN, run

```shell
sudo chef-automate config patch config.toml
```

and retry opening Chef Automate in your browser.
