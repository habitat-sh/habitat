+++
title = "Install the Chef Infra Server"
draft = false

aliases = ["/install_server.html"]

[menu]
  [menu.infra]
    title = "Install Chef Infra Server"
    identifier = "chef_infra/setup/chef_infra_server/install_server.md Install Chef Infra Server"
    parent = "chef_infra/setup/chef_infra_server"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_server.md)

There are three configuration scenarios for the Chef Infra Server:

-   [Standalone](#standalone) (everything on a single
    machine)
-   [High availability](#high-availability) (machines
    configured for front-end and back-end, allowing for failover on the
    back-end and load-balancing on the front-end, as required)
-   [Tiered](#tiered-single-backend) (machines
    configured for front-end and back-end, with a single back-end and
    load-balancing on the front-end, as required)

## Supported Platforms

{{% adopted_platforms_server %}}

## Prerequisites

The Chef Infra Server has the following prerequisites:

-   An x86_64 compatible system architecture; Red Hat Enterprise Linux
    and CentOS may require updates prior to installation
-   A resolvable hostname that is specified using a FQDN or an IP
    address
-   A connection to Network Time Protocol (NTP) to prevent clock drift
-   If host-based firewalls (iptables, ufw, etc.) are being used, ensure
    that ports 80 and 443 are open. See the
    [firewalls](/install_server_pre/#firewalls) section of the
    install prerequisites for additional details.
-   A local mail transfer agent that allows the Chef Infra Server to
    send email notifications
-   Using cron and the `/etc/cron.d` directory for periodic maintenance
    tasks
-   Disabling the Apache Qpid daemon on CentOS and Red Hat systems. See
    the [Apache Qpid](/install_server_pre/#apache-qpid) of the
    prerequisite documentation for detailed steps.
-   Optional. A local user account under which services will run, a
    local user account for PostgreSQL, and a group account under which
    services will run. See [UIDs and
    GIDs](/install_server_pre/#uids-and-gids) for more information.

{{< note >}}

See the [expanded list of prerequisites](/install_server_pre/) for a
detailed list of software and hardware requirements.

{{< /note >}}

## Standalone

The standalone installation of Chef Infra Server creates a working
installation on a single server. This installation is also useful when
you are installing Chef Infra Server in a virtual machine, for
proof-of-concept deployments, or as a part of a development or testing
loop.

To install Chef Server:

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

5.  {{< readFile_shortcode file="ctl_chef_server_user_create_admin.md" >}}

6.  {{< readFile_shortcode file="ctl_chef_server_org_create_summary.md" >}}

## Update Configuration for Purchased Nodes

To use more than 25 nodes, you'll need to change Chef Infra Server
configuration for the server to recognize your purchased licenses. Edit
your `/etc/opscode/chef-server.rb` file by following the process below:

1.  On your Chef Infra Server, if the `chef-server.rb` file does not
    exist, create it.

    ``` bash
    sudo mkdir /etc/opscode && sudo touch /etc/opscode/chef-server.rb
    ```

2.  Open up the newly created `chef-server.rb` file in your favorite
    text editor, for example:

    ``` bash
    sudo vi /etc/opscode/chef-server.rb
    ```

3.  Paste or add the following text. Please note the placement of the
    single quotation (') marks. If you're using the vi text editor,
    you'll need to use the <span class="title-ref">i</span> key to
    insert the text.

    ``` bash
    license['nodes'] = N where N is the number of licensed nodes you have purchased
    ```

4.  Save the file. If you're using vi, from the example above, use the
    <span class="title-ref">esc</span> key and then:

    ``` bash
    :wq
    ```

5.  Run `chef-server-ctl reconfigure` for the changes to be picked up by
    your Chef Infra Server.

    ``` bash
    sudo chef-server-ctl reconfigure
    ```

For more information on configuring your Chef Infra Server, see
[chef-server.rb Settings](/config_rb_server/) and [chef-server.rb
Optional Settings](/config_rb_server_optional_settings/).

## High Availability

The following links describe how to configure the Chef Infra Server for
high availability. The **Backend Cluster** setup is strongly recommended
for new installations:

-   [High Availability using Backend Cluster](/install_server_ha/)

## Tiered (Single Backend)

The following link describes how to configure the Chef Infra Server with
a single backend machine and multiple frontend machines. Note that this
process has been deprecated in favor of a [Backend
Cluster](/install_server_ha/) setup:

-   [Tiered Installation](/install_server_tiered/)
