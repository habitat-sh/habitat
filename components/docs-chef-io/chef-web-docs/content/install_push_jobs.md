+++
title = "Install Chef Push Jobs"
draft = false

aliases = ["/install_push_jobs.html"]

[menu]
  [menu.infra]
    title = "Push Jobs"
    identifier = "chef_infra/setup/install_push_jobs.md Push Jobs"
    parent = "chef_infra/setup"
    weight = 90
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_push_jobs.md)

Chef Push Jobs is installed to the following locations:

-   The same machine as the Chef Infra Server (Chef Server version 12.6
    or higher)
-   One (or more) nodes on which Chef Infra Client (Chef Client version
    12 or higher) is installed
-   At least one management workstation

## Install the Server

Use the [chef-server-ctl install](/ctl_chef_server/#install) command
to add Chef Push Jobs.

## Install the Client

To set up the Chef Push Jobs client:

1.  Add the **push-jobs** cookbook to the run-list for each of the nodes
    on which Chef Push Jobs is to be configured.

2.  Add the following default attributes on all nodes that are managed
    by Chef Push Jobs:

    ``` javascript
    "push_jobs": {
      "package_url": "<package_url>",
      "package_checksum": "<checksum>"
    }
    ```

3.  Run Chef Infra Client to configure Chef Push Jobs

4.  Verify that the Chef Push Jobs client is running as a daemon or as a
    service:

    ``` bash
    knife node status node_name
    ```

    for a specific node and:

    ``` bash
    knife node status
    ```

    for all nodes.

## Install the Workstation

knife-push ships in Chef Workstation. Install the latest version of Chef
Workstation from [Chef Downloads](https://downloads.chef.io/chef-workstation)

The following subcommands will be available from Chef Workstation:
`knife job list`, `knife job start`, `knife job output`,
`knife job status`, and `knife node status`.

### **push-jobs** Cookbook

The **push-jobs** cookbook at
<https://github.com/chef-cookbooks/push-jobs> is used by Chef Infra
Client to configure Chef Push Jobs as a client on a target node. This
cookbook is also used to define the whitelist, which is a list of
commands that Chef Push Jobs may execute when it runs. A command that is
not in the whitelist will not be executed by Chef Push Jobs. The
**push-jobs** cookbook should be managed like any other cookbook, i.e.
"downloaded from GitHub, managed using version source control, and
uploaded to the Chef server". To manage nodes using Chef Push Jobs, add
the **push-jobs** cookbook to the run-list for each node that will be
managed using Chef Push Jobs.

The whitelist is defined using the `node['push_jobs']['whitelist']`
attribute located in the default attributes file:

``` ruby
default['push_jobs']['whitelist']   = {
     "job_name" => "command",
     "job_name" => "command",
     "chef-client" => "chef-client" }
```

where `job_name` represents each of the jobs that are defined in the
whitelist and `command` is the command line that will be run on the
target node. The `chef-client` job is the only job in the whitelist
after the initial installation of Chef Push Jobs.

After the whitelist is defined, add the jobs to the client.rb file on
each node that will be managed by Chef Push Jobs:

``` ruby
whitelist({ "job_name" => "command",
            "job_name" => "command",
            "chef-client" => "chef-client"
          })
```

For example:

``` ruby
{
  "chef-client": "sudo chef-client",
  "chef_client_with_special_run_list": "sudo chef-client -o recipe[special_recipe]",
  "sv restart apache"
}
```

By default, any attempt to run a Chef Push Jobs command other than
`chef-client` will be rejected with `nack`. For example:

``` bash
knife job start some_command my_node
```

will return something similar to:

``` bash
Started.  Job ID: 67079444838d123456f0c1ea614c1fcaa0f
Failed.
command:     some_command
created_at:  Tue, 29 Oct 2013 21:22:59 GMT
id:          67079444425fdcdd0c1ea614c1fcaa0f
nodes:
  nacked: my_node
run_timeout: 3600
status:      nacked
updated_at:  Tue, 29 Oct 2013 21:23:04 GMT
```

To add commands, simply append them to the whitelist for roles,
environments, and nodes. For example, to set all of the nodes in the
`dev` environment to accept a Chef Push Jobs command to restart Apache,
run the following command:

``` bash
knife edit environments/dev.json
```

and then update the default attributes to include something like:

``` javascript
{
  "name": "dev",
  "description": "The development environment",
  "default_attributes": {
    "push_jobs": {
      "whitelist": {
        "chef-client": "chef-client",
        "chef_client_with_special_run_list": "sudo chef-client -o recipe[special_recipe]",
        "restart_apache": "sv restart apache"
      }
    }
  }
}
```

after which the following command can be run against nodes in the `dev`
environment to restart Apache:

``` bash
knife job start restart_apache NODE1 NODE2 ...
```

where `NODE1 NODE2 ...` defines a list of individual nodes against which
that command is run.

## Chef Push Jobs Groups

{{% push_jobs_summary %}}

{{< note >}}

The Chef Infra Server uses role-based access control to define the
[organizations, groups, and users](/server_orgs/), including those
needed by Chef Push Jobs.

{{< /note >}}

{{% server_rbac_groups_push_jobs %}}
