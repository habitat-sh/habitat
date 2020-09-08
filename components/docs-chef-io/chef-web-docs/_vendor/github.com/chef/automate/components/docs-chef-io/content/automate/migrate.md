+++
title = "Migrate from Chef Automate 1"

draft = false
[menu]
  [menu.automate]
    title = "Migrate from Chef Automate 1"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/migrate.md Migrate from Chef Automate 1"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/migrate.md)

Chef Automate versions 1.0.0-1.8.96 reached end-of-life on December 31, 2019 and are no longer supported. For more information and for help upgrading your system, please contact your Chef account representative.
The current Chef Automate versions includes significant architectural and technical improvements to the core product platform.
This guide shows you how to migrate your existing Chef Automate installation to the current Chef Automate.

In this guide, we call Chef Automate versions 1.0.0-1.8.96 "Chef Automate 1" and the current version "Chef Automate 2".

## Overview

The Chef Automate migration process performs the following steps, in order:

1. Runs preflight checks to ensure the system is suitable for Chef Automate 2, your Chef Automate 1 installation can be migrated safely, and that the upgrade process will be able to migrate your data.
1. Analyzes your Chef Automate 1 configuration files and migrates the relevant settings to a configuration file for Chef Automate 2. If incompatibilities are detected, the migration process fails and emits a description of the problem. You will have an opportunity to make any necessary changes to the generated Chef Automate 2 configuration.
1. Downloads Automate 2. Chef Automate 2 is distributed via [Habitat](https://www.habitat.sh/) packages that are installed early in the process to minimize the downtime required for the migration.
1. Puts your Chef Automate 1 installation into maintenance mode, waits for queued data to be processed, and then backs up all Chef Automate 1 data. This ensures that data will not be lost in the migration process and that you will be able to recover to a working state should an unforeseen error occur.
1. Creates a local snapshot of Chef Automate 1 data for import into Chef Automate 2.
1. Shuts down Chef Automate 1.
1. Imports the Chef Automate 1 snapshot into Chef Automate 2.
1. Starts Chef Automate 2. On startup, each component service of Chef Automate 2 detects imported data. Once all component services perform this step, Chef Automate 2 is ready to use and can accept new data.
1. Migrates imported historical data in the background. You will be notified when the migration completes.

{{< warning >}}
Chef Automate 2 migrates only the LDAP configuration and local users (also referred to as "internal users").
SAML configuration is not migrated.
{{< /warning >}}

## Prerequisites

Before you start the migration process, fulfill the requirements detailed in this section.

### Command Line Tool

You will need the `chef-automate` command line tool to initiate your upgrade.

1. Download the latest version of the Chef Automate CLI:

    ```shell
    wget https://packages.chef.io/files/current/automate/latest/chef-automate_linux_amd64.zip
    ```

2. Unzip the package:

    ```shell
    unzip chef-automate_linux_amd64.zip
    ```

3. Move the unzipped `chef-automate` binary to `/usr/local/bin`:

    ```shell
    mv chef-automate /usr/local/bin
    ```

### Internet Access

The Chef Automate upgrade process currently requires connectivity to the internet to install the Chef Automate 2 binaries. The standard Automate installation requires current versions for Chrome, Edge, or Firefox browsers. If you filter internet access via proxy or by other means, you must ensure the following domains are accessible:

* `packages.chef.io`
* `licensing.chef.io`
* `raw.githubusercontent.com`
* `api.bintray.com`
* `bldr.habitat.sh`
* `akamai.bintray.com`
* `dl.bintray.com`
* `bintray.com`

#### Proxies

The Chef Automate 2 installer respects the following environment variables:

* `HTTPS_PROXY`/`https_proxy`
* `HTTP_PROXY`/`http_proxy`
* `NO_PROXY`/`no_proxy`

If you use a proxy to manage outbound HTTP(S) connections, ensure these variables are set when running the migration.

### Chef Automate 1 Version

Recent versions of Chef Automate 1 contain enhancements that the migration process relies upon to ensure your data is safely migrated. Currently, Chef Automate 1.8.38 or greater is required.

### Systemd

Chef Automate 2 requires the systemd init system. If you're currently using Chef Automate 1 on an operating system that makes use of a different init system, we recommend consulting Customer Support for the appropriate migration strategy.

## Considerations

While we've taken care to make the migration process as smooth as possible, this section outlines some caveats to consider before you proceed.

### Plan for Downtime

The Chef Automate 2 migration process puts your Chef Automate 1 installation into maintenance mode, shuts it down, and starts Chef Automate 2. During the downtime, the migration process takes a backup of your Chef Automate 1 data and exports some of its data to a local snapshot, which is imported into Chef Automate 2.

To minimize this downtime, we recommended that you create an online backup of Chef Automate 1 just prior to the upgrade. Historical information such as Chef Infra Client run data and compliance scan data is backed up incrementally, which means that the upgrade only needs to transfer data that has been added since the last backup.

By default, the Chef Automate 2 upgrade process will not proceed if your Chef Automate 1 installation does not have backups configured. Invoke the migration using the `--skip-backup-check` flag to avoid this check.

To configure Chef Automate 1 backups, see the [Chef Automate 1 Documentation](https://docs.chef.io/delivery_server_backup/).

### Unsupported Features and Topologies

Chef Automate 2 includes significant architectural and technical improvements to the core product platform.
If you rely on any of the capabilities listed below, we recommend you continue to using your existing Chef Automate installation.

* **Chef Manage:** Chef Automate 2, unlike Automate 1, cannot serve as a SAML auth proxy
* **FIPS:** Chef Automate 2 cannot currently operate in FIPS mode
* **Disaster Recovery:** Chef Automate 2 cannot currently operate in a primary/standby mode
* **Custom Kibana dashboards:** Chef Automate 2 does not include Kibana in its technology stack
* **SAML config migration:** Chef Automate 2 supports SAML integration, however due to configuration incompatibilities Chef Automate 2 cannot migrate Chef Automate 1 SAML settings to Chef Automate 2 as part of the upgrade. After the upgrade is completed, you may follow [these configuration instructions]({{< relref "configuration.md#saml" >}}) to set up SAML.

Should you wish to migrate to Chef Automate 2 without these features, invoke the migration with the appropriate flags:

* `--skip-fips-check`
* `--skip-disaster-recovery-check`
* `--skip-saml-check`

These flags enable you to migrate by skipping preflight checks for unsupported features.

### External Elasticsearch cluster

The Chef Automate 2 migration process requires manual intervention to migrate a Chef Automate 1 installation that uses external Elasticsearch.

To migrate an external Elasticsearch cluster, please reach out to a Customer Success or Customer Support representative for assistance.

### New Data Paths

Chef Automate 2 stores its data in directories named `/hab/svc/$service-name/data`. In particular:

* Elasticsearch data is stored in `/hab/svc/automate-elasticsearch/data/`
* PostgreSQL data is stored in `/hab/svc/automate-postgresql/data/`

If you use dedicated disks or partitions for either of these applications in Chef Automate 1, you must modify your disk mount configuration to make these disks/partitions available to Chef Automate 2.

### Workflow

Follow the instructions in [Upgrade Workflow]{{< relref "workflow_install.md" >}}
The migration process will stop if it detects that you used the Workflow component of Chef Automate 1.
To use Workflow with Chef Automate 2 specify the `--enable-workflow` option to enable the Workflow component. You can enable the Workflow component after upgrading with `chef-automate deploy --enable-workflow`.

### Chef Automate 2 License

Login to Chef Automate to start a trial. The trial provides you with a 60-day license.
Requesting a trial license requires internet connectivity in your Chef Automate 2 instance (only at the time of the license request).

If you are migrating an [airgapped Chef Automate installation](https://docs.chef.io/install_chef_air_gap/#chef-automate),
contact your Chef account representative for a Chef Automate 2 license.

## Migrate

1. Create a backup your Chef Automate 1 installation:

    ```shell
    automate-ctl create-backup
    ```

2. Once the backup has completed, initiate the migration process. If your host is internet-connected,
   run the command:

    ```shell
    ./chef-automate migrate-from-v1 --channel current
    ```

    If your host is airgapped, run the command:

    ```shell
    ./chef-automate migrate-from-v1 --airgap-bundle </path/to/bundle>
    ```

After the migration runs the preflight checks and analyzes your Chef Automate 1 configuration, it asks for confirmation to continue. Review the generated configuration file and if it is correct, type `yes` to continue.

The migration process backs up your Chef Automate 1 data, shuts down Chef Automate 1, imports your data to Chef Automate 2, then starts Chef Automate 2.
At this point, you can use your existing Chef Automate 1 user credentials to login to Chef Automate 2.
If you've been using LDAP for authenticating users, that configuration will have been migrated as well, and you can use your LDAP credentials to login.
Historical data will be migrated in the background.

### Upgrades
Chef Automate 2 handles upgrades differently than Chef Automate 1 did. The [Installation]({{< relref "install.md#upgrade" >}}) documentation and [Airgapped Installation]({{< relref "airgapped_installation.md#upgrade" >}}) documentation provide further detail.
