+++
title = "Upgrading Chef Infra Client"
draft = false

aliases = ["/upgrade_client.html"]

[menu]
  [menu.infra]
    title = "Upgrades"
    identifier = "chef_infra/setup/nodes/upgrade_client.md Upgrades"
    parent = "chef_infra/setup/nodes"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/upgrade_client.md)

The following sections describe the upgrade process for Chef Client 12.

Please [view the notes](/upgrade_client_notes/) for more background
on the upgrade process.

## Upgrade via Command Line

To upgrade Chef Infra Client on a node via the command line, run the
following command on each node to be upgraded:

``` bash
curl -L https://chef.io/chef/install.sh | sudo bash
```

Using the `knife ssh` subcommand is one way to do this.

## Upgrade via Cookbook

The
[chef_client_updater](https://supermarket.chef.io/cookbooks/chef_client_updater)
cookbook can be used to install or upgrade Chef Infra Client package on
a node.

## Considerations

Preparing your Chef Infra Client system before upgrading to Chef Infra
Server will enhance your Chef experience. During a Chef Infra Server
upgrade, all of the data is extracted, converted to a new format, and
then uploaded. A large amount of data (cookbooks, nodes, etc.,) can
increase the upgrade process significantly and extend your downtime.

Managing your data prior to upgrading will improve experience upgrading
and using Chef Infra Server and client. Following this list of
client-based tasks prior to upgrading will expedite the upgrade process
and mitigate many common issues.

Install the version of chef client you plan on using after the upgrade on a small number of test nodes and verify:

:   -   All nodes can authenticate and converge successfully.
    -   Custom Ohai plugins still work as expected.
    -   Custom Handlers still work as expected.

Ensure that all of the cookbooks used by your organization are correctly located and identified:

:   -   Do all cookbooks used by your organization exist in source
        control? Upload any missing cookbooks and dependencies.
    -   Do all cookbooks have a `metadata.rb` or `metadata.json` file?
    -   Delete unused cookbook versions. First, run
        `knife cookbook list` to view a list of cookbooks. Next, for
        each cookbook, run `knife cookbook show COOKBOOK_NAME` to view
        its versions. Then, delete each unused version with
        `knife cookbook delete -v VERSION_NAME`.

Download all cookbooks and validate the following against each cookbook:

:   -   Run `egrep -L ^name */metadata.rb`. Does each have a
        `metadata.rb` file?
    -   Does the cookbook name in the `metadata.rb` file match the name
        in the run-list? (Some older versions of Chef Infra Client used
        the cookbook name for the run-list based on the directory name
        of the cookbook and not the cookbook_name in the `metadata.rb`
        file.)

Cook as lean as possible:

:   -   Verify cookbook size and mitigate the size of large cookbooks
        where possible. Most cookbooks are quite small, under \~200 KB.
        For any cookbook over 200 KB, consider why they are that large.
        Are there binary files?
    -   Clean up `git` history for any cookbook found to be excessively
        large.

Verify nodes and clients that are in use:

:   -   Are all nodes and/or clients in use? Clean up any extra nodes
        and clients. Use the `knife node list`, `knife client list`, and
        [knife status](/workstation/knife_status/) commands to verify nodes and
        clients in use.
    -   Use the
        `` knife client delete` command to remove unused clients. Use the ``knife
        node delete\`\` command to remove unused nodes.

Run the test nodes against the Chef Infra Server. If the server is also
being upgraded, **first** complete the Chef Infra Server upgrade
process, and **then** verify the test nodes against the upgraded Chef
Infra Server.
