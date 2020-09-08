+++
title = "The Chef Desktop Development Pattern"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "The Chef Desktop Development Pattern"
    identifier = "desktop/desktop_pattern.md Get Started with Chef Desktop"
    parent = "desktop"
    weight = 50
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/content/desktop/desktop_pattern.md)

## Install the Chef Desktop Cookbook

Copy the Chef Desktop cookbook you received from Chef and unzip that file into your cookbooks directory. Now you have two cookbooks.

Update the `metadata.rb` file for the Chef Desktop cookbook to add your contact details.

## Select Chef Desktop Settings

Chef Desktop comes with a large number of options for configuring your Windows and Mac desktops. Look through the `mac.rb` and `windows.rb` files to explore what settings you want to turn on for your testing and evaluation. For those resources you do not want to explore yet, set their action to ':nothing'. See the [Chef Desktop cookbook documentation]({{< relref "desktop-cookbook.md" >}}) for more information about settings.

## Test the Chef Desktop Cookbook

### Run the Virtual Devices

You downloaded the two virtual devices, also called testing images. Now issue the following command to get them started:

```powershell
kitchen create
```

### Apply the Desktop Cookbook to VMs

Next, run the following command to 'converge' the cookbooks with the base OS testing image:

```powershell
kitchen converge
```

### Verify the settings

Confirm that the converged code is the code that you meant to apply. In your VSCode Chef Desktop directory, navigate to the `test\integration\default` directory and examine the generated integration tests. Carefully go through these and adjust them to match the settings you chose in your `mac.rb` and `windows.rb` files. Next, run:

```powershell
kitchen verify
```

If any of the tests fail, check the output and compare your settings in the `mac.rb` or `windows.rb` files against the matching tests.

### Cleanup Test Kitchen

When you finish with your testing, you can run the following command to delete the running test images:

```powershell
kitchen destroy
```

### Combined Kitchen Command

In the future when you get more comfortable, you can run `kitchen test` to perform all of the above steps at once, including the cleanup step.

```powershell
kitchen test
```

## Upload the Chef Desktop Cookbook

Upload the Chef Desktop Cookbook to your Chef Infra Server. From the development environment command line, navigate to the `/cookbooks` directory and run:

```powershell
knife cookbook upload <your cookbook name>
```

Knife and other Chef tools use the cookbook name specified inside of either the `metadata.rb` file or the `policyfile.rb` file, which is case sensitive. Naming cookbooks in all lower-case is the easiest way to avoid conflicts with your `knife` or `chef` commands.

Now your Chef Infra Server has the cookbook and settings it needs to apply to the nodes.

## Configure the Policyfile for Chef Infra Server

Check the Policyfile and apply it to our test nodes. Policies are a convenient strategy for managing nodes. Read [more about policies](https://blog.jerryaldrichiii.com/chef_infra/2019/05/28/using-policyfile-cookbooks.html).

### Check the Policyfile

Your Chef Desktop `Policyfile.rb` should look similar to:

```ruby
name 'desktop-config'

# default_source :supermarket, 'https://supermarket.chef.io' do |s|
#   s.preferred_for 'chef-client'
# end

# run_list: chef-client will run these recipes in the order specified.
# cookbook::recipe
run_list 'desktop-config::default'

# Specify a custom source for a single cookbook:
cookbook 'desktop-config', path: '.'
```

### Upload the Policyfile

Upload the Policyfile to the Chef Infra Server. Call `chef update` first to do some needed housekeeping around your policyfile.

If this is the first time that you are using a Policyfile, use the `chef install` command to generate a lock file:

```powershell
chef install Policyfile.rb
```

Run `chef update` and `chef push` every time you update the version of your cookbook:

```powershell
chef update
chef push 'my_Policy_Group' 'Policyfile.rb'
```

## Deploy Desktop Cookbook to a Node

Now you are ready to try out your Chef Desktop cookbook on your first test node. Before you begin, you need to:

### Bootstrap the First Test Node

1. Create a [`client.rb` file](https://docs.chef.io/config_rb_client/#example) with the basic information needed to connect to the Chef Infra Server instance
1. Identify a 'test node' - a virtual machine or laptop/desktop that you can test your working cookbook against
1. Get the serial number of your 'test node'

From your workstation, configure the server and the `client.rb` file for your node. `S90T7HK2` is an example node serial number.

### Create a client.rb

Create a local `client.rb` file with settings similar to:

```ruby
log_level              :info
log_location           STDOUT
validation_client_name 'my_org-validator'
validation_key         File.expand_path('c:\chef\my_org-validator.pem')
chef_server_url        "https://my.fqdn.com/my_chef_server_instance"
ssl_verify_mode        :verify_peer
local_key_generation   true
rest_timeout           30
http_retry_count       3
chef_license           'accept'
node_name              'S90T7HK2'
```

### Identify a Test Node

```powershell
C:\> knife node create S90T7HK2
Created node [S90T7HK2]
```

### Apply the Chef Desktop Policy

Use `knife node policy set` to apply the policy to a node.

Use the name of the policy specified in the Policyfile.rb that was uploaded to the Chef Infra Server.

```powershell
knife node policy set S90T7HK2 'Windows_Node_Policy_Group' 'desktop-config'
```

### Install the Chef Infra Client

Go to your test node and install the Chef Infra Client from an elevated PowerShell window, or use `sudo` if you are installing it from MacOS. For additional information, see the [Chef Install Script](https://docs.chef.io/chef_install_script/) documentation.

#### On Windows

```powershell
. { iwr -useb https://omnitruck.chef.io/install.ps1 } | iex; install -project chef
```

#### On macOS

```bash
curl -L https://omnitruck.chef.io/install.sh | sudo bash
```

### Load the client.rb

After Chef Infra Client finishes installing, copy the `client.rb` file with the correct data for your node and Chef Infra Server, and place that in `c:\chef`.

### Load the Key

Then copy the `validator.pem` file you downloaded from your Chef Infra Server at the beginning of the guide and put it in the same folder.

### Deploy the Chef Desktop Cookbook to Your Test Node

Run the client:

```powershell
chef-client
```

- Next: [Zero-touch Deployment with Azure]({{< relref desktop_azure.md>}})
- Last: [Chef Desktop Cookbook Development Environment]({{< relref desktop_setup_cookbook.md >}})
