+++
title = "Chef Supermarket"
draft = false

aliases = ["/supermarket.html"]

[menu]
  [menu.infra]
    title = "Supermarket"
    identifier = "chef_infra/concepts/supermarket/supermarket.md Supermarket"
    parent = "chef_infra/concepts/supermarket"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/supermarket.md)

{{% supermarket_summary %}}

## Public Supermarket

The public Chef Supermarket hosted by Chef Software is located at [Chef
Supermarket](https://supermarket.chef.io/).

To interact with the public Chef Supermarket, use [knife
supermarket](/workstation/knife_supermarket/) commands.

<img src="/images/public_supermarket.svg" class="align-center" width="700" alt="image" />

## Private Supermarket

{{% supermarket_private %}}

{{< note >}}

{{% supermarket_private_source_code %}}

{{< /note >}}

### Recommended Tools

The following tools are recommended for use with a private Chef
Supermarket:

-   Berkshelf
-   Stove

#### Berkshelf

Berkshelf can include multiple Chef Supermarket instances for dependency
resolution. Cookbook dependency resolution is performed from the top
down. The first source defined in the Berksfile will be searched for the
cookbook before the second source.

The Berksfile first looks for the cookbook on the private Chef
Supermarket and, if not discovered there, then looks on the public Chef
Supermarket.

``` ruby
source 'https://your_private_supermarket_url'
source 'https://supermarket.chef.io'
```

#### Stove

Stove is a utility for packaging and releasing Chef cookbooks:
<https://github.com/chef/stove>.

### Installing and Upgrading Private Supermarket

Install a private Supermarket using these [instructions](/install_supermarket/).

Upgrade a private Supermarket using these [instructions](/install_supermarket/#upgrade-a-private-supermarket).

### Set up Workstation

#### Configure config.rb

The config.rb file on the workstation should be configured for use with
the private Chef Supermarket.

To configure config.rb for the private Chef Supermarket, do the
following:

1.  Open the config.rb file in an editor.

2.  Add the following setting:

    ``` ruby
    knife[:supermarket_site] = 'https://your-private-supermarket'
    ```

3.  Save and close the file.

### Create a Cookbook

The following examples show how to create a simple cookbook by using the
chef command that is built into Chef Workstation.

**Generate a chef-repo**

To generate a chef-repo, run a command similar to:

``` bash
chef generate repo my_chef_repo
```

Access the chef-repo using the `cd` command:

``` bash
cd my_chef_repo
```

**Generate a cookbook**

{{< note >}}

Duplicate cookbook names on Chef Supermarket are not allowed. So first
verify that a cookbook name is available.

{{< /note >}}

To create the `my_apache2_cookbook` cookbook, run the following command:

``` bash
chef generate cookbook cookbooks/my_apache2_cookbook
```

**Generate a template**

To generate a template, run a command similar to:

``` bash
chef generate template cookbooks/my_apache2_cookbook index.html
```

This will create a file named `index.html.etb` in the
`/cookbooks/my_apache2_cookbook` directory. Open the file using a text
editor to add content. For example, some HTML:

``` html
<html>
  <body>
    <h1>Chef Love!</h1>
  </body>
</html>
```

Save and close the file.

**Create a recipe**

The `default.rb` recipe is created when a cookbook is generated. A
recipe is updated using a text editor. For example:

``` ruby
package 'apache2' # Installs the apache2 package

service 'apache2' do
  action [:start, :enable] # Starts and enables the apache2 service on boot
end

template '/var/www/html/index.html' do
  source 'index.html.erb' # Template for /var/www/html/index.html
end
```

### Upload a Cookbook

To upload a cookbook to Chef Supermarket, do the following:

1.  Add a setting for Chef Supermarket to the config.rb file:

    ``` ruby
    knife[:supermarket_site] = 'https://your-private-supermarket'
    ```

2.  Resolve SSL errors by fetching, and then verifying the SSL
    certificate for Chef Supermarket:

    ``` bash
    knife ssl fetch https://your-private-supermarket
    ```

    and then:

    ``` bash
    knife ssl check https://your-private-supermarket
    ```

3.  Upload the cookbook to Chef Supermarket:

    ``` bash
    knife supermarket share mycookbook "Other"
    ```

### Share a Cookbook

``` bash
knife supermarket share 'my_cookbook'
```

#### Troubleshoot SSL Errors

If an SSL error is returned similar to:

``` bash
ERROR: Error uploading cookbook my_cookbook to the Opscode Cookbook Site: SSL_connect returned=1 errno=0 state=SSLv3 read server certificate B: certificate verify failed. Increase log verbosity (-VV) for more information.
```

this is because Chef Server version 12.0 (and higher) enforces SSL by
default when sharing cookbooks. A private Chef Supermarket uses
self-signed certificates by default. Use the `knife ssl fetch` and
`knife ssl check` commands to resolve this error.

First fetch the SSL certificate for the private Chef Supermarket:

``` bash
knife ssl fetch https://your-private-supermarket
```

and then:

``` bash
knife ssl check https://your-private-supermarket
```

Re-share the cookbook. This time the message returned should be similar
to:

``` bash
Generating metadata for my_cookbook from (...)
Making tarball my_cookbook.tgz
Upload complete!
```

### supermarket-ctl (executable)

{{% ctl_supermarket_summary %}}

For more information about the supermarket-ctl command line tool, see
[supermarket-ctl](/ctl_supermarket/).

### supermarket.rb

{{% config_rb_supermarket_summary %}}

For more information about the supermarket.rb file, see
[supermarket.rb](/config_rb_supermarket/).

### Supermarket API

{{% supermarket_api_summary %}}

For more information about the Supermarket API, see [Supermarket
API](/supermarket_api/).

### fieri

Fieri is an optional service what will check cookbook versions for
certain metrics to determine the quality of the cookbook.

If you are using a private Chef Supermarket, you can activate the Fieri
service like this:

1.  Add Fieri to your features attribute.

    ``` ruby
    ['supermarket_omnibus']['config']['features'] = "tools,github,announcement,fieri"
    ```

2.  Add the following Fieri attributes:

    ``` ruby
    ['supermarket_omnibus']['config']['fieri_key'] = "#{random string you generate}"
    ['supermarket_omnibus']['config']['fieri_supermarket_endpoint'] = "#{your_supermarket_url}"
    ```

3.  Reconfigure your Supermarket.

    ``` bash
    (your-supermarket-node) sudo supermarket-ctl reconfigure
    (your-supermarket-node) sudo supermarket-ctl restart
    ```

After doing these steps, you should see a "Quality" tab when viewing a
cookbook through the Supermarket UI. Click on this tab and you will see
the results of the metrics run by Fieri.
