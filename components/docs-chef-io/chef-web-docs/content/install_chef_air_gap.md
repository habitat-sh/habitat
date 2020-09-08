+++
title = "Install Chef in an air-gapped environment"
draft = false

aliases = ["/install_chef_air_gap.html"]

[menu]
  [menu.infra]
    title = "Air-gapped Installation"
    identifier = "chef_infra/setup/install_chef_air_gap.md Air-gapped Installation"
    parent = "chef_infra/setup"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_chef_air_gap.md)

This guide will show you how to run a fully functional Chef environment
within an
[air-gapped](https://en.wikipedia.org/wiki/Air_gap_(networking))
network.

## Prerequisites

Since a variety of different practices are used to create an air-gapped
network, this guide focuses solely on the implementation of Chef
software - as such, it makes the following assumptions:

-   You have a way to get packages to your air-gapped machines
-   Machines on your air-gapped network are able to resolve each other
    via DNS
-   A server's Fully Qualified Domain Name (FQDN) is the name that will
    be used by other servers to access it
-   You have a private Ruby gem mirror to supply gems as needed
-   You have an artifact store for file downloads. At a minimum, it
    should have the following packages available:
    -   Chef Workstation
    -   Chef Infra Client
    -   Chef Supermarket
    -   An [install
        script](/install_chef_air_gap/#create-an-install-script) for
        Chef Infra Client

### Required cookbooks

This guide will link to the required cookbooks for each piece of
software in that software's respective section, but this is a full list
of the cookbooks required to complete the entire guide:

For Chef Supermarket:

-   [supermarket-omnibus-cookbook](https://supermarket.chef.io/cookbooks/supermarket-omnibus-cookbook)
-   [chef-ingredient](https://supermarket.chef.io/cookbooks/chef-ingredient)
-   [hostsfile](https://supermarket.chef.io/cookbooks/hostsfile)

### Required Gems

The following Ruby gems are required to install private Supermarket via
the supermarket-omnibus-cookbook:

-   mixlib-install
-   mixlib-shellout
-   mixlib-versioning
-   artifactory

These should be accessible from your Gem mirror.

### Create an install script

An install script is used to install Chef Infra Client when
bootstrapping a new node. It simply pulls the Chef Infra Client package
from your artifact store, and then installs it. For example, on
Debian-based Linux systems, it would look similar to this:

``` bash
#!/bin/bash

cd /tmp/
wget http://packages.example.com/chef_13.2.20-1_amd64.deb
dpkg -i chef_13.2.20-1_amd64.deb
```

The install script should be accessible from your artifact store.

## Chef Infra Server

In this section you'll install the Chef Infra Server, and create your
organization and user. Note that in order to configure Supermarket later
in this guide, you will need a user that is a member of the `admins`
group.

1.  Download the package from <https://downloads.chef.io/chef-server/>.

2.  Upload the package to the machine that will run the Chef Infra
    Server, and then record its location on the file system. The rest of
    these steps assume this location is in the `/tmp` directory.

3.  {{< readFile_shortcode file="install_chef_server_install_package.md" >}}

4.  Run the following to start all of the services:

    ``` bash
    sudo chef-server-ctl reconfigure
    ```

    Because the Chef Infra Server is composed of many different services
    that work together to create a functioning system, this step may
    take a few minutes to complete.

5.  {{< readFile_shortcode file="ctl_chef_server_user_create_admin.md">}}

6.  {{< readFile_shortcode file="ctl_chef_server_org_create_summary.md">}}

## Chef Workstation

### Install Chef Workstation

1.  First, install the Chef Workstation [installer
    package](https://downloads.chef.io/chef-workstation). Use the
    appropriate tool to run the installer:

    ``` bash
    dpkg -i chef-workstation_0.14.16-1_amd64.deb
    ```

2.  Use the `chef generate repo` command to generate your Chef repo:

    ``` bash
    chef generate repo chef-repo
    ```

3.  Within your Chef repo, create a `.chef` directory:

    ``` bash
    mkdir /chef-repo/.chef
    ```

4.  Copy the `USER.pem` and `ORGANIZATION.pem` files from the server,
    and move them into the `.chef` directory.

    ``` bash
    scp ssh-user@chef-server.example.com:/path/to/pem/files /chef-repo/.chef/
    ```

### Create a bootstrap template

By default, `knife bootstrap` uses the `chef-full` template to bootstrap
a node. This template contains a number of useful features, but it also
attempts to pull an installation script from `downloads.chef.io`. In
this section, you'll copy the contents of the `chef-full` template to a
custom template, and then modify the package and Ruby gem sources.

1.  Navigate to the `.chef` directory, and create a `bootstrap`
    directory within it:

    ``` bash
    mkdir bootstrap
    ```

2.  Move to the `bootstrap` directory and create a blank template file;
    this example will use `airgap.erb` for the template name:

    ``` bash
    touch airgap.erb
    ```

3.  Still in the `bootstrap` directory, issue the following command to
    copy the `chef-full` configuration to your new template:

    ``` bash
    find /opt/chef-workstation/embedded/lib/ruby -type f -name chef-full.erb -exec cat {} \; > airgap.erb
    ```

    This command searches for the `chef-full` template file under
    `/opt/chef-workstation/embedded/lib/ruby`, and then outputs the
    contents of the file to `airgap.erb`. If you used a different
    template file name, be sure to replace `airgap.erb` with the
    template file you created during the last step.

4.  Update `airgap.erb` to replace `omnitruck.chef.io` with the URL of
    `install.sh` on your artifact store:

    ``` ruby
    install_sh="<%= knife_config[:bootstrap_url] ? knife_config[:bootstrap_url] : "http://packages.example.com/install.sh" %>"
    ```

5.  Still in your text editor, locate the following line near the bottom
    of your `airgap.erb` file:

    ``` ruby
    cat > /etc/chef/client.rb <<'EOP'
    <%= config_content %>
    EOP
    ```

    Beneath it, add the following, replacing `gems.example.com` with the
    URL of your gem mirror:

    ``` ruby
    cat >> /etc/chef/client.rb <<'EOP'
    rubygems_url "http://gems.example.com"
    EOP
    ```

    This appends the appropriate `rubygems_url` setting to the
    `/etc/chef/client.rb` file that is created during bootstrap, which
    ensures that your nodes use your internal gem mirror.

### Configure knife

Within the `.chef` directory, create a `config.rb` file and replace
`USER` and `ORGANIZATION` with the user and organization that you
created on your Chef Infra Server; replace `chef-server.example.com`
with your Chef Infra Server URL:

``` ruby
current_dir = File.dirname(__FILE__)
log_level                :info
log_location             STDOUT
node_name                'USER'
client_key               "#{current_dir}/USER.pem"
validation_client_name   'ORGANIZATION-validator'
validation_key           "#{current_dir}/ORGANIZATION.pem"
chef_server_url          'https://chef-server.example.com/organizations/ORGANIZATION'
cache_type               'BasicFile'
cache_options( :path => "#{ENV['HOME']}/.chef/checksums" )
cookbook_path            ["#{current_dir}/../cookbooks"]
knife[:bootstrap_template] = "#{current_dir}/bootstrap/airgap.erb"
```

The `knife[:bootstrap_template]` option in this example allows you to
specify the template that `knife bootstrap` will use by default when
bootstrapping a node. It should point to your custom template within the
`bootstrap` directory.

Now that `knife` is configured, copy the SSL certificates from your Chef
Infra Server to your trusted certificates:

``` ruby
knife ssl fetch
```

## Private Supermarket

Private Supermarket allows you to host your own internal version of the
[Chef Supermarket](https://supermarket.chef.io) within your air-gapped
network.

### Requirements

In this section, you will use a wrapper around the
[supermarket-omnibus-cookbook](https://supermarket.chef.io/cookbooks/supermarket-omnibus-cookbook)
to install private Supermarket. The Supermarket cookbook depends upon
the following cookbooks:

-   [chef-ingredient](https://supermarket.chef.io/cookbooks/chef-ingredient)
-   [hostsfile](https://supermarket.chef.io/cookbooks/hostsfile)

The following Gems must be accessible via your Gem mirror:

-   mixlib-install
-   mixlib-shellout
-   mixlib-versioning
-   artifactory

Your `cookbooks` directory must have all three of these cookbooks
installed before you will be able to use the Supermarket cookbook
wrapper. In addition the necessary cookbooks, a private Chef Supermarket
has the following requirements:

-   An operational Chef Infra Server (version 12.0 or higher) to act as
    the OAuth 2.0 provider
-   A user account on the Chef Infra Server with `admins` privileges
-   A key for the user account on the Chef server
-   An x86_64 compatible Linux host with at least 1 GB memory
-   System clocks synchronized on the Chef Infra Server and Supermarket
    hosts
-   Sufficient disk space to meet project cookbook storage capacity or
    credentials to store cookbooks in an Amazon Simple Storage Service
    (S3) bucket

### Configure credentials

First, you'll configure Chef Identity credentials for Supermarket. Chef
Identity is an OAuth 2.0 service packaged with the Chef Infra Server,
that allows you to use the same credentials to access both server and
Supermarket.

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

### Create a Wrapper

1.  Generate the cookbook:

    ``` bash
    chef generate cookbook my_supermarket_wrapper
    ```

2.  Change directories into that cookbook:

    ``` bash
    cd my_supermarket_wrapper
    ```

3.  Defines the wrapper cookbook's dependency on the
    `supermarket-omnibus-cookbook` cookbook. Open the `metadata.rb` file
    of the newly-created cookbook, and then add the following line:

    ``` ruby
    depends 'supermarket-omnibus-cookbook'
    ```

4.  Save and close the `metadata.rb` file.

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
approach is to place these attributes in a [data bag](/data_bags/),
and then reference them from the recipe. For example, the data bag could
be named `apps` and then a data bag item within the data bag could be
named `supermarket`. The following attributes are required:

-   `chef_server_url`: the url for your chef server.
-   `chef_oauth2_app_id`: the Chef Identity uid from
    `/etc/opscode/oc-id-applications/supermarket.json`
-   `chef_oauth2_secret`: The Chef Identity secret from
    `/etc/opscode/oc-id-applications/supermarket.json`
-   `package_url`: The location of the Supermarket package on your
    artifact store

To define these attributes, do the following:

1.  Open the `recipes/default.rb` file and add the following, **before**
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
    node.override['supermarket_omnibus']['package_url'] = app['package_url']
    ```

    Note that the `['package_url']` setting points to the location of
    the Supermarket package on your artifact store. When finished, the
    `/recipes/default.rb` file should have code similar to:

    ``` ruby
    app = data_bag_item('apps', 'supermarket')

    node.override['supermarket_omnibus']['chef_server_url'] = app['chef_server_url']
    node.override['supermarket_omnibus']['chef_oauth2_app_id'] = app['chef_oauth2_app_id']
    node.override['supermarket_omnibus']['chef_oauth2_secret'] = app['chef_oauth2_secret']

    include_recipe 'supermarket-omnibus-cookbook'
    ```

    Alternatively, if you chose not to use a data bag to store these
    values, your `default.rb` should look similar to this:

    ``` ruby
    node.override['supermarket_omnibus']['chef_server_url'] = 'https://chef-server.example.com:443'
    node.override['supermarket_omnibus']['chef_oauth2_app_id'] = '0bad0f2eb04e935718e081fb71asdfec3681c81acb9968a8e1e32451d08b'
    node.override['supermarket_omnibus']['chef_oauth2_secret'] = '17cf1141cc971a10ce307611beda7ffadstr4f1bc98d9f9ca76b9b127879'
    node.override['supermarket_omnibus']['package_url'] = 'http://packages.example.com/supermarket_3.1.22-1_amd64.deb'


    include_recipe 'supermarket-omnibus-cookbook'
    ```

3.  Save and close the `recipes/default.rb` file.

4.  Upload all of your cookbooks to the Chef Infra Server:

    ``` ruby
    knife cookbook upload -a
    ```

### Bootstrap Supermarket

Bootstrap the node on which Chef Supermarket is to be installed. For
example, to bootstrap a node running Ubuntu on Amazon Web Services
(AWS), the command is similar to:

``` bash
knife bootstrap ip_address -N supermarket-node -x ubuntu --sudo
```

where:

-   `-N` defines the name of the Chef Supermarket node:
    `supermarket-node`
-   `-x` defines the (ssh) user name: `ubuntu`
-   `--sudo` ensures that sudo is used while running commands on the
    node during the bootstrap operation

When the bootstrap operation is finished, do the following:

1.  Add the wrapper cookbook's `/recipes/default.rb` recipe to the
    run-list:

    ``` bash
    knife node run_list set supermarket-node recipe[my_supermarket_wrapper::default]
    ```

    where `supermarket-node` is the name of the node that was just
    bootstrapped.

2.  Start Chef Infra Client on the newly-bootstrapped Chef Supermarket
    node. For example, using SSH:

    ``` bash
    ssh ubuntu@your-supermarket-node-public-dns
    ```

3.  After accessing the Chef Supermarket node, run Chef Infra Client:

    ``` bash
    sudo chef-client
    ```

### Connect to Supermarket

To reach the newly spun up private Chef Supermarket, the hostname must
be resolvable from a workstation. For production use, the hostname
should have a DNS entry in an appropriate domain that is trusted by each
user's workstation.

1.  Visit the Chef Supermarket hostname in the browser. A private Chef
    Supermarket will generate and use a self-signed certificate, if a
    certificate is not supplied as part of the installation process (via
    the wrapper cookbook).
2.  If an SSL notice is shown due to your self-signed certificate while
    connecting to Chef Supermarket via a web browser, accept the SSL
    certificate. A trusted SSL certificate should be used for private
    Chef Supermarket that is used in production.
3.  After opening Chef Supermarket in a web browser, click the **Create
    Account** link. A prompt to log in to the Chef Infra Server is
    shown. Authorize the Chef Supermarket to use the Chef Infra Server
    account for authentication.

{{< note >}}

The redirect URL specified for Chef Identity **MUST** match the FQDN
hostname of the Chef Supermarket server. The URI must also be correct:
`/auth/chef_oauth2/callback`. Otherwise, an error message similar to
`The redirect uri included is not valid.` will be shown.

{{< /note >}}

### Configuration updates

#### Knife

Update the `config.rb` file on your workstation to use your private
Supermarket:

``` ruby
knife[:supermarket_site] = 'https://supermarket.example.com'
```

#### Berkshelf

If you're using Berkshelf, update your `Berksfile` to replace
`https://supermarket.chef.io` with the URL of your private Supermarket:

``` ruby
source 'https://supermarket.example.com'
```

### Upload cookbooks to Supermarket

To upload new cookbooks to your private Supermarket, use the
`knife supermarket share` command on your workstation:

``` ruby
knife supermarket share chef-ingredient
```
