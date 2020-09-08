+++
title = "Installation Guide"

draft = false

[menu]
  [menu.automate]
    title = "Install Guide"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/install.md Install Guide"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/install.md)

Before beginning your installation, check the [System Requirements]({{< relref "system_requirements.md" >}}) for Chef Automate.

See [Airgapped Installation]({{< relref "airgapped_installation.md" >}}) for installing Chef Automate to a host with no inbound or outbound internet traffic.

## Download the Chef Automate Command-Line Tool

Download and unzip the Chef Automate command-line tool:

```shell
curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate
```

## Create Default Configuration

Create a `config.toml` file with default values for your Chef Automate installation:

```shell
sudo ./chef-automate init-config
```

You can customize your FQDN, login name, and other values, by changing the values in the `config.toml` in your editor.

If you have requirements around data size and/or redundancy, see [Configuring External
Data Stores]({{< relref "#configuring-external-data-stores" >}}) for information on
configuring Chef Automate to use an externally-deployed PostgreSQL database cluster
and/or Elasticsearch cluster. If you have requirements around a highly-available
deployment of Chef Automate, please reach out to a Customer Success or Professional
Services representative for assistance.

See [Configuring Chef Automate]({{< relref "configuration.md" >}}) for more information
on configuration settings.

## Deploy Chef Automate

```shell
sudo ./chef-automate deploy config.toml
```

Deployment takes a few minutes. The first step is accepting the terms of service in the command line, after which the installer performs a series of pre-flight checks;
any unsuccessful checks have information for resolving issues or skipping the check.
Run the deploy command again, after resolving any pre-flight issues.

At the end of the deployment process you will see:

```shell
Deploy complete
```

The deployment process writes login credentials to the `automate-credentials.toml` in your current working directory.

## Open Chef Automate

Navigate to `https://{{< example_fqdn "automate" >}}` in a browser and log in to Chef Automate with
the credentials provided in `automate-credentials.toml`.  Once you log in, Chef Automate
prompts you for a license.

When your Chef Automate instance is equipped with internet connectivity, you can get a 60-day trial license from there.
Alternatively, a license obtained by other means can be applied.

### Configure Data Collection

To send data from your Chef Infra Server or Chef Infra Clients to Chef Automate 2, the process is the same as Chef Automate 1.
See ["Configure Data Collection"]({{< relref "data_collection.md" >}}) for more information.

## Upgrades

By default, Chef Automate will automatically upgrade to the latest version available. These updates can be taken safely, as we've committed to ensuring the stability of the upgrade process - automatic updates will never introduce breaking changes.

### Release Channels

The Chef Automate upgrade process makes use of **release channels** to allow greater control over the automatic upgrades applied to your system. Chef Automate will always pull from the latest release within its specified release channel. We're initially shipping with the default `current` channel, but additional channels will be introduced in the future.

To change the release channel that is used for upgrades, modify the `channel` setting in your `config.toml` file:

```toml
channel = "current"
```

### Disable Automatic Upgrades

You can disable automatic upgrades by modifying the `upgrade_strategy` setting in your `config.toml` file:

```toml
upgrade_strategy = "none"
```

To manually initiate an upgrade, run

```shell
chef-automate upgrade run
```

This command upgrades Chef Automate to the latest version available from your release channel.

### Common Problems

If you are unable to open Chef Automate, check that the `config.toml` contains the public DNS as the FQDN.

```shell
# This is a default Chef Automate configuration file. You can run
# 'chef-automate deploy' with this config file and it should
# successfully create a new Chef Automate instance with default settings.

[global.v1]
# The external fully qualified domain name.
# When the application is deployed you should be able to access 'https://<fqdn>/'
# to login.
fqdn = "<_Public DNS_name>"
```

Once you correct and save the FQDN, run

```shell
sudo chef-automate config patch config.toml
```

and retry opening Chef Automate in your browser.

### Configuring External Data Stores

You can configure Chef Automate to use PostgreSQL and Elasticsearch clusters that are not
deployed via Chef Automate itself. The directions provided below are intended for use only
during initial deployment of Chef Automate. Please reach out to a Customer Support or
Customer Success representative for assistance with migrating from a standalone
installation of Chef Automate to one using externally-deployed data stores.

#### Configuring External Elasticsearch

Add the following to your config.toml:

```toml
[global.v1.external.elasticsearch]
  enable = true
  nodes = ["http://elastic1.example:9200", "http://elastic2.example:9200", "..." ]

# Uncomment and fill out if using external elasticsearch with SSL and/or basic auth
# [global.v1.external.elasticsearch.auth]
#   scheme = "basic_auth"
# [global.v1.external.elasticsearch.auth.basic_auth]
## Create this elasticsearch user before starting the Automate deployment;
## Automate assumes it exists.
#   username = "<admin username>"
#   password = "<admin password>"
# [global.v1.external.elasticsearch.ssl]
#  Specify either a root_cert or a root_cert_file
#  root_cert = """$(cat </path/to/cert_file.crt>)"""
#  server_name = "<elasticsearch server name>"

# Uncomment and fill out if using external elasticsearch that uses hostname-based routing/load balancing
# [esgateway.v1.sys.ngx.http]
#  proxy_set_header_host = "<your external es hostname>:1234"

# Uncomment and add to change the ssl_verify_depth for the root cert bundle
#  ssl_verify_depth = "2"
```

Because externally-deployed Elasticsearch nodes will not have access to Chef Automate's built-in backup storage services, you must configure Elasticsearch backup settings separately from Chef Automate's primary backup settings. You can configure backups to use either the local filesystem or S3.

##### Backup Externally-Deployed Elasticsearch to Local Filesystem

To configure local filesystem backups of Chef Automate data stored in an externally-deployed Elasticsearch cluster:

1. Ensure that the filesystems you intend to use for backups are mounted to the same path on all Elasticsearch master and data nodes.
1. Configure the Elasticsearch `path.repo` setting on each node as described in the [Elasticsearch documentation](https://www.elastic.co/guide/en/elasticsearch/reference/6.8/modules-snapshots.html#_shared_file_system_repository).
1. Add the following to your `config.toml`:

```toml
[global.v1.external.elasticsearch.backup]
enable = true
location = "fs"

[global.v1.external.elasticsearch.backup.fs]
# The `path.repo` setting you've configured on your Elasticsearch nodes must be
# a parent directory of the setting you configure here:
path = "/var/opt/chef-automate/backups"
```

##### Backup Externally-Deployed Elasticsearch to AWS S3

To configure AWS S3 backups of Chef Automate data stored in an externally-deployed Elasticsearch cluster:

1. Install the [`repository-s3` plugin](https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-s3.html) on all nodes in your Elasticsearch cluster.
1. If you wish to use IAM authentication to provide your Elasticsearch nodes access to the S3 bucket, you must apply the appropriate IAM policy to each host system in the cluster.
1. Configure each Elasticsearch node with a S3 client configuration containing the proper S3 endpoint, credentials, and other settings as [described in the Elasticsearch documentation](https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-s3-client.html).
1. Enable S3 backups by adding the following settings to your `config.toml`:

    ```toml
    [global.v1.external.elasticsearch.backup]
    enable = true
    location = "s3"

    [global.v1.external.elasticsearch.backup.s3]

      # bucket (required): The name of the bucket
      bucket = "<bucket name>"

      # base_path (optional):  The path within the bucket where backups should be stored
      # If base_path is not set, backups will be stored at the root of the bucket.
      base_path = "<base path>"

      # name of an s3 client configuration you create in your elasticsearch.yml
      # see https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-s3-client.html
      # for full documentation on how to configure client settings on your
      # Elasticsearch nodes
      client = "<client name>"

    [global.v1.external.elasticsearch.backup.s3.settings]
    ## The meaning of these settings is documented in the S3 Repository Plugin
    ## documentation. See the following links:
    ## https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-s3-repository.html

    ## Backup repo settings
    # compress = false
    # server_side_encryption = false
    # buffer_size = "100mb"
    # canned_acl = "private"
    # storage_class = "standard"
    ## Snapshot settings
    # max_snapshot_bytes_per_sec = "40mb"
    # max_restore_bytes_per_sec = "40mb"
    # chunk_size = "null"
    ## S3 client settings
    # read_timeout = "50s"
    # max_retries = 3
    # use_throttle_retries = true
    # protocol = "https"
    ```

##### Backup Externally-Deployed Elasticsearch to GCS

To configure Google Cloud Storage Bucket (GCS) backups of Chef Automate data stored in an externally-deployed Elasticsearch cluster:

1. Install the [`repository-gcs` plugin](https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-gcs.html) on all nodes in your Elasticsearch cluster.
1. Create a storage bucket and configure a service account to access it per the steps [described in the Elasticsearch documentation](https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-gcs-usage.html).
1. Configure each Elasticsearch node with a GCS client configuration that contains the proper GCS settings as [described in the Elasticsearch documentation](https://www.elastic.co/guide/en/elasticsearch/plugins/current/repository-gcs-client.html).
1. Enable GCS backups by adding the following settings to your `config.toml`:

    ```toml
    [global.v1.external.elasticsearch]
      enable = true
      nodes = ["https://my-es.cluster"]
      ## If multiple
      # nodes = ["https://my-es.node-1", "https://my-es.node-2", "etc..."]

    ## The following settings are required if you have Elasticsearch setup with basic auth
    #[global.v1.external.elasticsearch.auth]
    #  scheme = "basic_auth"
    #
    #[global.v1.external.elasticsearch.auth.basic_auth]
    #  username = "everyuser"
    #  password = "pass123"

    [global.v1.external.elasticsearch.backup]
      enable = true
      location = "gcs"

    [global.v1.external.elasticsearch.backup.gcs]
      bucket = "<bucket name>"
      # Client name is normally default, but can be set here if you have generated service
      # account credentials with a different client name
      client = "default"

    ## GCS Bucket Settings:
    # type = nearline
    # access control = uniform
    ```

#### Configuring an External PostgreSQL Database

Add the following settings to your `config.toml`:

```toml
[global.v1.external.postgresql]
enable = true
nodes = ["<pghostname1>:<port1>", "<pghostname2>:<port2>", "..."]

# To use postgres with SSL, uncomment and fill out the following:
# [global.v1.external.postgresql.ssl]
# enable = true
# root_cert = """$(cat </path/to/root/cert.pem>)"""

[global.v1.external.postgresql.auth]
scheme = "password"

# Create these postgres users before starting the Automate deployment;
# Automate assumes they already exist.
[global.v1.external.postgresql.auth.password.superuser]
username = "<admin username>"
password = "<admin password>"
[global.v1.external.postgresql.auth.password.dbuser]
username = "<dbuser username>"
password = "<dbuser password>"

[global.v1.external.postgresql.backup]
enable = true
```
