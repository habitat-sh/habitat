+++
title = "Install Private Supermarket"
draft = false

aliases = ["/install_supermarket.html"]

[menu]
  [menu.infra]
    title = "Install Private Supermarket"
    identifier = "chef_infra/setup/supermarket/install_supermarket.md Install Private Supermarket"
    parent = "chef_infra/setup/supermarket"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_supermarket.md)

{{% supermarket_summary %}}

{{% supermarket_private %}}

{{< note >}}

{{% supermarket_private_source_code %}}

{{< /note >}}

## Requirements

A private Chef Supermarket has the following requirements:

-   An operational Chef Infra Server (Chef Server version 12.0 or
    higher) to act as the OAuth 2.0 provider
-   A user account on the Chef Infra Server with `admins` privileges
-   A key for the user account on the Chef server
-   An x86_64 compatible Linux host with at least 1 GB memory
-   System clocks synchronized on the Chef Infra Server and Supermarket
    hosts
-   Sufficient disk space on host to meet project cookbook storage
    capacity **or** credentials to store cookbooks in an Amazon Simple
    Storage Service (S3) bucket

**Considerations with regard to storage capacity:**

-   PostgreSQL database size will grow linearly based on the number of
    cookbooks and the number of cookbook versions published
-   Redis database size is negligible as it is used only for background
    job queuing, and to cache a small number of API responses
-   Cookbook storage growth is entirely dependent on the size of the
    cookbooks published. Cookbooks that include binaries or other large
    files will consume more space than code-only cookbooks
-   Opting to run a private Supermarket with off-host PostgreSQL, Redis,
    and cookbook store is less a decision about storage sizing; it is
    about data service uptime, backup, and restore procedure for your
    organization
-   As a point of reference: as of September 2017 after three years of
    operation, the public Supermarket has approx 70,000 users, 3,300
    cookbooks with a total of 20,000 versions published. The PostgreSQL
    database weighs in at 310 MB (50 MB when exported with `pg_dump`),
    and the S3 bucket containing all of the published community
    cookbooks weighs in at 2.7 GB

## Chef Identity

Chef Identity (also referred to as **oc-id**) is an OAuth 2.0
authentication and authorization service packaged with the Chef Infra
Server. Chef Identity must be configured to run with a private Chef
Supermarket, after which users may use the same credentials to access
the Chef Supermarket as they do to access the Chef Infra Server.

{{< note >}}

The Chef Supermarket server must be able to reach (via HTTPS) the
specified `chef_server_url` during OAuth 2.0 negotiation. This type of
issue is typically with name resolution and firewall rules.

{{< /note >}}

### Configure

To configure Chef Supermarket to use Chef Identity, do the following:

1.  Log on to the Chef Infra Server via SSH and elevate to an
    admin-level user. If running a multi-node Chef Infra Server cluster,
    log on to the node acting as the primary node in the cluster.

2.  Update the `/etc/opscode/chef-server.rb` configuration file.

    {{< readFile_shortcode file="config_ocid_application_hash_supermarket.md" >}}

3.  Reconfigure the Chef Infra Server.

    ``` bash
    sudo chef-server-ctl reconfigure
    ```

4.  Retrieve Supermarket's OAuth 2.0 client credentials:

    Depending on your Chef Infra Server version and configuration (see
    [chef-server.rb](/config_rb_server_optional_settings/#config-rb-server-insecure-addon-compat)),
    this can be retrieved via [chef-server-ctl oc-id-show-app
    supermarket](/ctl_chef_server/#ctl-chef-server-oc-id-show-app)
    or is located in `/etc/opscode/oc-id-applications/supermarket.json`:

    ``` javascript
    {
      "name": "supermarket",
      "uid": "0bad0f2eb04e935718e081fb71asdfec3681c81acb9968a8e1e32451d08b",
      "secret": "17cf1141cc971a10ce307611beda7ffadstr4f1bc98d9f9ca76b9b127879",
      "redirect_uri": "https://supermarket.mycompany.com/auth/chef_oauth2/callback"
    }
    ```

    The `uid` and `secret` values will be needed later on during the
    setup process for Chef Supermarket.

{{< note >}}

Add as many Chef Identity applications to the
`/etc/opscode/chef-server.rb` configuration file as necessary. A JSON
file is generated for each application added, which contains the
authentication tokens for that application. The secrets are added to the
Chef Identity database and are available to all nodes in the Chef Infra
Server front end group. The generated JSON files do not need to be
copied anywhere.

{{< /note >}}

{{< note >}}

The redirect URL specified **MUST** match the FQDN of the Chef
Supermarket server. The URI must also be correct:
`/auth/chef_oauth2/callback`. Otherwise, an error message similar to
`The redirect uri included is not valid.` will be shown.

{{< /note >}}

## Install Supermarket

To install a private Chef Supermarket use the
`supermarket-omnibus-cookbook`. This cookbook is [available from the
public](https://supermarket.chef.io/cookbooks/supermarket-omnibus-cookbook)
Chef Supermarket.

-   The `supermarket-omnibus-cookbook` cookbook is attribute-driven; use
    a custom cookbook to specify your organization's unique
    `node[supermarket_omnibus]` attribute values.
-   The custom cookbook is a wrapper around
    `supermarket-omnibus-cookbook`, which performs the actual
    installation of the Chef Supermarket packages, and then writes the
    custom `node[supermarket_omnibus]` values to
    `/etc/supermarket/supermarket.json`.
-   The Chef Supermarket package itself contains an internal cookbook
    which configures the already-installed package using the attributes
    defined in `/etc/supermarket/supermarket.json`.

{{< note >}}

In general, for production environments Chef recommends to start running
Chef Supermarket with small virtual machines, and then increase the size
of the virtual machine as necessary. Put the `/var/opt/supermarket`
directory on a separate disk, and then use LVM so that may be expanded.

{{< /note >}}

### Create a Wrapper

A wrapper cookbook is used to define project- and/or
organization-specific requirements around a community cookbook.

<img src="/images/supermarket_wrapper_cookbook.svg" class="align-left" width="400" alt="image" />

In the case of installing a private Chef Supermarket, Chef recommends
the use of a wrapper cookbook to specify certain attributes that are
unique to your organization, while enabling the use of the generic
installer cookbook which, in turn, installs the Chef Supermarket package
behind your firewall.

All of the keys under `node['supermarket_omnibus']` are written out as
`/etc/supermarket/supermarket.json`. Add other keys as needed to
override the default attributes specified in the Chef Supermarket
[omnibus
package](https://github.com/chef/supermarket/blob/master/omnibus/cookbooks/omnibus-supermarket/attributes/default.rb).
For example:

``` ruby
default['supermarket_omnibus']['chef_server_url'] = 'https://chefserver.mycompany.com:443'
default['supermarket_omnibus']['chef_oauth2_app_id'] = '14dfcf186221781cff51eedd5ac1616'
default['supermarket_omnibus']['chef_oauth2_secret'] = 'a49402219627cfa6318d58b13e90aca'
default['supermarket_omnibus']['chef_oauth2_verify_ssl'] = false
default['supermarket_omnibus']['fqdn'] = 'supermarket.mycompany.com'
```

On your workstation, generate a new cookbook using the `chef` command
line interface:

1.  Generate the cookbook:

    ``` bash
    chef generate cookbook my_supermarket_wrapper
    ```

2.  Change directories into that cookbook:

    ``` bash
    cd my_supermarket_wrapper
    ```

3.  Defines the wrapper cookbook's dependency on the
    `supermarket-omnibus-cookbook` cookbook. Open the metadata.rb file
    of the newly-created cookbook, and then add the following line:

    ``` ruby
    depends 'supermarket-omnibus-cookbook'
    ```

4.  Save and close the metadata.rb file.

5.  Open the `/recipes/default.rb` recipe located within the
    newly-generated cookbook and add the following content:

    ``` ruby
    include_recipe 'supermarket-omnibus-cookbook'
    ```

    This ensures that the `default.rb` file in the
    `supermarket-omnibus-cookbook` is run.

### Define Attributes

Define the attributes for the Chef Supermarket installation and how it
connects to the Chef Infra Server. One approach would be to hard-code
attributes in the wrapper cookbook's `default.rb` recipe. A better
approach is to place these attributes in a data bag, and then reference
them from the recipe. For example, the data bag could be named `apps`
and then a data bag item within the data bag could be named
`supermarket`.

The following attribute values must be defined:

-   `chef_server_url`
-   `chef_oauth2_app_id`
-   `chef_oauth2_secret`

Once configured, you can get the `chef_oauth2_app_id` and
`chef_oauth2_secret` values from your Chef Infra Server within
`/etc/opscode/oc-id-applications/supermarket.json`:

For `chef_server_url`, enter in the url for your chef server. For
`chef_oauth2_app_id`, enter in the uid from
`/etc/opscode/oc-id-applications/supermarket.json` For
`chef_oauth2_secret`, enter in the secret from
`/etc/opscode/oc-id-applications/supermarket.json`

To define these attributes, do the following:

1.  Open the `/recipes/default.rb` file and add the following, BEFORE
    the `include_recipe` line that was added in the previous step. This
    example uses a data bag named `apps` and a data bag item named
    `supermarket`:

    ``` ruby
    app = data_bag_item('apps', 'supermarket')
    ```

2.  Set the attributes from the data bag:

    ``` ruby
    node.override['supermarket_omnibus']['chef_server_url'] = app['chef_server_url']
    node.override['supermarket_omnibus']['chef_oauth2_app_id'] = app['chef_oauth2_app_id']
    node.override['supermarket_omnibus']['chef_oauth2_secret'] = app['chef_oauth2_secret']
    ```

    When finished, the `/recipes/default.rb` file should have code
    similar to:

    ``` ruby
    app = data_bag_item('apps', 'supermarket')

    node.override['supermarket_omnibus']['chef_server_url'] = app['chef_server_url']
    node.override['supermarket_omnibus']['chef_oauth2_app_id'] = app['chef_oauth2_app_id']
    node.override['supermarket_omnibus']['chef_oauth2_secret'] = app['chef_oauth2_secret']

    include_recipe 'supermarket-omnibus-cookbook'
    ```

3.  Save and close the `/recipes/default.rb` file.

{{< note >}}

If you are running your private Supermarket in AWS, you may need to set
an additional attribute for the node's public IP address:

``` ruby
node.override['supermarket_omnibus']['config']['fqdn'] = your_node_public_ip
```

{{< /note >}}

### Upload the Wrapper

The wrapper cookbook around the `supermarket-omnibus-cookbook` cookbook
must be uploaded to the Chef Infra Server, along with any cookbooks
against which the `supermarket-omnibus-cookbook` cookbook has
dependencies.

To upload the cookbooks necessary to install Chef Supermarket, do the
following:

1.  Install Berkshelf:

    ``` bash
    berks install
    ```

2.  Change directories into `~/.berkshelf/cookbooks`:

    ``` bash
    cd ~/.berkshelf/cookbooks
    ```

3.  Upload all cookbooks to the Chef Infra Server:

    ``` bash
    knife cookbook upload -a
    ```

4.  Change directories into the location in which the wrapper cookbook
    was created:

    ``` bash
    cd path/to/wrapper/cookbook/
    ```

5.  Upload the wrapper cookbook to the Chef Infra Server:

    ``` bash
    knife cookbook upload -a
    ```

### Bootstrap Supermarket

Bootstrap the node on which Chef Supermarket is to be installed. For
example, to bootstrap a node running Ubuntu on Amazon Web Services
(AWS), the command is similar to:

``` bash
knife bootstrap ip_address -N supermarket-node -x ubuntu --sudo
```

where

-   `-N` defines the name of the Chef Supermarket node:
    `supermarket-node`
-   `-x` defines the (ssh) user name: `ubuntu`
-   `--sudo` ensures that sudo is used while running commands on the
    node during the bootstrap operation

When the bootstrap operation is finished, do the following:

1.  Edit the node to add the wrapper cookbook's `/recipes/default.rb`
    recipe to the run-list:

    ``` bash
    knife node edit supermarket-node
    ```

    where `supermarket-node` is the name of the node that was just
    bootstrapped.

2.  Add the recipe to the run-list:

    ``` ruby
    "run_list": [
      "recipe[my_supermarket_wrapper::default]"
    ]
    ```

3.  Start Chef Infra Client on the newly-bootstrapped Chef Supermarket
    node. For example, using SSH:

    ``` bash
    ssh ubuntu@your-supermarket-node-public-dns
    ```

4.  After accessing the Chef Supermarket node, run Chef Infra Client:

    ``` bash
    sudo chef-client
    ```

## Install Supermarket Directly (without a cookbook)

While there are many benefits to using the cookbook method to install
Supermarket, there are also cases where it's simpler to set up the
Supermarket installation manually. These steps will walk you through the
process of manually configuring your private Supermarket server.

Before following these steps, be sure to complete the OAuth setup
process detailed in the [Chef
Identity](/install_supermarket/#chef-identity) section of this
guide.

1.  [Download](https://downloads.chef.io/supermarket/) the correct
    package for your operating system from `downloads.chef.io`.

2.  Install Supermarket using the appropriate package manager for your
    distribution:

    -   For Ubuntu:

        ``` bash
        dpkg -i /path/to/package/supermarket*.deb
        ```

    -   For RHEL / CentOS:

        ``` bash
        rpm -Uvh /path/to/package/supermarket*.rpm
        ```

3.  Run the `reconfigure` command to complete the initial installation:

    ``` none
    sudo supermarket-ctl reconfigure
    ```

4.  Create an `/etc/supermarket/supermarket.json` file and add the
    following information, substituting the values for each
    configuration option with the OAuth 2.0 client credentials that were
    created in the [previous
    section](/install_supermarket/#chef-identity):

    ``` ruby
    {
        "chef_server_url": "https://chefserver.mycompany.com",
        "chef_oauth2_app_id": "0bad0f2eb04e935718e081fb71asdfec3681c81acb9968a8e1e32451d08b",
        "chef_oauth2_secret": "17cf1141cc971a10ce307611beda7ffadstr4f1bc98d9f9ca76b9b127879",
        "fqdn": "supermarket.mycompany.com",
        "chef_oauth2_verify_ssl": false
    }
    ```

    Where:

    -   `"chef_server_url"` should contain the FQDN of your Chef Infra
        Server. Note that if you're using a non-standard SSL port, this
        much be appended to the URL. For example:
        `https://chefserver.mycompany.com:65400`
    -   `"chef_oauth2_app_id"` should contain the `"uid"` value from
        your OAuth credentials
    -   `"chef_oauth2_secret"` should contain the `"secret"` value from
        your OAuth credentials
    -   `chef_oauth2_verify_ssl` is set to false, which is necessary
        when using a self-signed certificate without a properly
        configured certificate authority
    -   `fqdn` should contain the desired URL that will be used to
        access your private Supermarket. If not specified, this default
        to the FQDN of the machine

5.  Issue another `reconfigure` command to apply your changes:

    ``` none
    sudo supermarket-ctl reconfigure
    ```

## Connect to Supermarket

To reach the newly spun up private Chef Supermarket, the hostname must
be resolvable from a workstation. For production use, the hostname
should have a DNS entry in an appropriate domain that is trusted by each
user's workstation.

1.  Visit the Chef Supermarket hostname in the browser. A private Chef
    Supermarket will generate and use a self-signed certificate, if a
    certificate is not supplied as part of the installation process (via
    the wrapper cookbook).
2.  If an SSL notice is shown while connecting to Chef Supermarket via a
    web browser, accept the SSL certificate. A trusted SSL certificate
    should be used for private Chef Supermarket that is used in
    production.
3.  After opening Chef Supermarket in a web browser, click the **Create
    Account** link. A prompt to log in to the Chef Infra Server is
    shown, but only if the user is not already logged in. Authorize the
    Chef Supermarket to use the Chef Infra Server account for
    authentication.

{{< note >}}

The redirect URL specified for Chef Identity **MUST** match the fqdn
hostname of the Chef Supermarket server. The URI must also be correct:
`/auth/chef_oauth2/callback`. Otherwise, an error message similar to
`The redirect uri included is not valid.` will be shown.

{{< /note >}}

## Customize Supermarket

Chef Supermarket is a Ruby on Rails application with a PostgreSQL
backend. The private Chef Supermarket configuration may be scaled-out,
such as using an external database, using an external cache, and using
an external cookbook storage location.

### External Database

A Chef Supermarket installation can use an external database running
PostgreSQL (9.3 or higher) and with the `pgpsql` and `pg_trgm` installed
and loaded. The public Chef Supermarket uses Amazon Relational Database
Service (RDS). To use an external database, configure the following
attributes in the `/recipes/default.rb` recipe of the wrapper cookbook:

``` ruby
node.override['supermarket_omnibus']['config']['postgresql']['enable'] = false
node.override['supermarket_omnibus']['config']['database']['user'] = 'supermarket'
node.override['supermarket_omnibus']['config']['database']['name'] = 'supermarket'
node.override['supermarket_omnibus']['config']['database']['host'] = 'yourcompany...rds.amazon.com'
node.override['supermarket_omnibus']['config']['database']['port'] = '5432'
node.override['supermarket_omnibus']['config']['database']['pool'] = '25'
node.override['supermarket_omnibus']['config']['database']['password'] = 'topsecretneverguessit'
```

### External Cache

Chef Supermarket installations can also use an external cache store. The
public Chef Supermarket uses Redis on Amazon ElastiCache. One Redis
instance per private Chef Supermarket application server may be run
safely. Use Redis 2.8 (or higher) for a high availability pair. To use
an external cache, configure the following attributes in the
`/recipes/default.rb` recipe of the wrapper cookbook:

``` ruby
node.override['supermarket_omnibus']['config']['redis']['enable'] = false
node.override['supermarket_omnibus']['config']['redis_url'] = 'redis://your-redis-instance:6379'
```

### External Cookbook Storage

Cookbook artifacts---tar.gz artifacts that are uploaded to Chef
Supermarket when sharing a cookbook---can be stored either on the local
filesystem of the Chef Supermarket node (`/var/opt/supermarket/data` by
default) or in an Amazon Simple Storage Service (S3) bucket. To use an
S3 bucket, configure the following attributes in the
`/recipes/default.rb` recipe of the wrapper cookbook:

``` ruby
node.override['supermarket_omnibus']['config']['s3_access_key_id'] = 'yourkeyid'
node.override['supermarket_omnibus']['config']['s3_bucket'] = 'all-our-awesome-cookbooks'
node.override['supermarket_omnibus']['config']['s3_region'] = 'some-place-3'
node.override['supermarket_omnibus']['config']['s3_secret_access_key'] = 'yoursecretaccesskey'
```

{{< note >}}

Encrypted S3 buckets are currently not supported.

{{< /note >}}

## Upgrade a Private Supermarket

1. Shut down the server running Private Supermarket.
1. Backup the `/var/opt/supermarket` directory.

1. Download the [Chef Supermarket](https://downloads.chef.io/) package.
1. Upgrade your system with the new package using the appropriate package manager for your distribution:

    -   For Ubuntu:

        ``` bash
        dpkg -i /path/to/package/supermarket*.deb
        ```

    -   For RHEL / CentOS:

        ``` bash
        rpm -Uvh /path/to/package/supermarket*.rpm
        ```

1. [Reconfigure](/ctl_supermarket/#reconfigure) the server that Chef Supermarket is installed on:

    ``` bash
    sudo supermarket-ctl reconfigure
    ```

Private Supermarket is updated on your server now. We recommend restarting the services that run Chef Supermarket to ensure that the old installation of Chef Supermarket doesn't persist in the server memory.

1. Get the name of the active unit:

    ``` bash
    systemctl list-units | grep runsvdir
    ```

1. Restart the unit:

    ``` bash
    systemctl restart UNIT_NAME
    ```

    This will restart the `runsvdir`, `runsv`, and `svlogd` service processes that run Chef Supermarket.
