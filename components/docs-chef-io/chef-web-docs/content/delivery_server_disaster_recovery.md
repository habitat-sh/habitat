+++
title = "Chef Automate Disaster Recovery"
draft = false
robots = "noindex"


aliases = ["/delivery_server_disaster_recovery.html"]


[menu]
  [menu.legacy]
    title = "Delivery Disaster Recovery"
    identifier = "legacy/workflow/managing_workflow/delivery_server_disaster_recovery.md Delivery Disaster Recovery"
    parent = "legacy/workflow/managing_workflow"
    weight = 160
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_server_disaster_recovery.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Use a standby Chef Automate server to protect against the loss of the
primary Chef Automate server. A standby Chef Automate server is
configured to replicate data from the primary Chef Automate server. In
the event of loss of the primary Chef Automate server, the standby is
then reconfigured to become the primary.

{{< note >}}

Disaster Recovery for Chef Automate pertains to the workflow
capabilities only. Also, these instructions assume that the primary and
standby servers are in the same data center. If they are in different
geographical locations additional considerations are necessary, as well
as tuning the configuration to account for latency between data centers.

{{< /note >}}

## Requirements

A disaster recovery configuration for Chef Automate has the following
requirements:

-   Two identically-configured Chef Automate servers, one to act as the
    primary server and the other to act as a standby

    {{< note spaces=4 >}}

    You cannot log in to the Chef Automate web UI on the standby server.

    {{< /note >}}

-   SSH access between both Chef Automate servers via port 22

-   PostgreSQL replication allowed between both Chef Automate servers
    via port 5432

-   The latest version of ChefDK is installed on the provisioning node

-   A Chef Automate license

## Install a Standby Chef Automate Server

The following steps describe how to manually install a Chef Automate
server for use as a standby.

{{< note >}}

Look for items delimited with `<BRACKETS>`. Replace the bracketed words
(and the brackets) with the correct values for your configuration. All
files require default permissions, unless noted. All commands must be
run as the root user or by using `sudo`.

{{< /note >}}

1.  Provision a standby server that is exactly the same as the existing
    Chef Automate server.

2.  Download the Chef Automate package to the standby server:
    <https://downloads.chef.io/automate/>.

3.  As a root user, install the Chef Automate package on the server,
    using the name of the package provided by Chef.

    For Debian:

    ``` bash
    dpkg -i $PATH_TO_AUTOMATE_SERVER_PACKAGE
    ```

    For Red Hat or Centos:

    ``` bash
    rpm -Uvh $PATH_TO_AUTOMATE_SERVER_PACKAGE
    ```

    After a few minutes, Chef Automate will be installed.

4.  Create the license directory:

    ``` bash
    sudo mkdir -p /var/opt/delivery/license
    ```

    and then copy the `delivery.license` file that exists in the
    `/var/opt/delivery/license` directory on the primary Chef Automate
    server into the license directory.

5.  Create the configuration directory:

    ``` bash
    sudo mkdir -p /etc/delivery
    ```

6.  Edit the `/etc/delivery/delivery.rb` file:

    ``` bash
    sudo vi /etc/delivery/delivery.rb ## you may use any editor you wish
    ```

    and add the following settings:

    ``` ruby
    delivery_fqdn "<AUTOMATE_URL>"

    delivery['chef_username']    = "delivery"
    delivery['chef_private_key'] = "/etc/delivery/delivery.pem"
    delivery['chef_server']      = "https://<CHEF_SERVER_URL>/organizations/delivery"

    delivery['default_search']   = "((recipes:delivery_build OR recipes:delivery_build\\\\:\\\\:default) AND chef_environment:_default)"

    delivery['primary'] = false
    delivery['primary_ip'] = '<PRIMARY_IP_ADDRESS>'
    postgresql['listen_address'] = 'localhost,<STANDBY_IP_ADDRESS>'
    ```

    where `PRIMARY_IP_ADDRESS`, `STANDBY_IP_ADDRESS`, and
    `AUTOMATE_URL`, `CHEF_SERVER_URL` should be replaced with the actual
    values for the Chef Automate configuration. The `PRIMARY_IP_ADDRESS`
    and `STANDBY_IP_ADDRESS` values should be from a private network
    between the two machines.

7.  Create a directory for the SSH key--if one is not already
    present--on the primary Chef Automate server:

    ``` bash
    sudo mkdir -p /opt/delivery/embedded/.ssh
    ```

8.  Create a private key on the primary Chef Automate server. This key
    is used for file synchronization between the two servers. It will be
    created in `/opt/delivery/embedded/.ssh` and must not contain a
    passphrase.

    Move into the directory:

    ``` bash
    cd /opt/delivery/embedded/.ssh
    ```

    then generate the key:

    ``` bash
    sudo ssh-keygen -t rsa -b 4096 -C "<EMAIL_ADDRESS>"
    ```

    and then save to a file (don't overwrite anything) and note the
    filename for later.

9.  On the standby server, create the directory
    `/opt/delivery/embedded/.ssh/authorized_keys`:

    ``` bash
    sudo mkdir -p /opt/delivery/embedded/.ssh/authorized_keys
    ```

10. Copy the public key (from the key pair created above) to
    `/opt/delivery/embedded/.ssh/authorized_keys` on the standby server:

11. On the primary Chef Automate server edit the
    `/etc/delivery/delivery.rb` file to add the following:

    ``` ruby
    delivery['primary'] = true
    postgresql['trust_auth_cidr_addresses'] = [ '127.0.0.1/32',
                                                '::1/128',
                                                '<PRIMARY_IP_ADDRESS>/32',
                                                '<STANDBY_IP_ADDRESS>/32'
                                              ]
    postgresql['listen_address'] = 'localhost,<PRIMARY_IP_ADDRESS>'
    delivery['standby_ip'] = '<STANDBY_IP_ADDRESS>'
    lsyncd['ssh_key'] = '/opt/delivery/embedded/.ssh/<PRIVATE_KEY>'
    ```

    where `PRIMARY_IP_ADDRESS`, `STANDBY_IP_ADDRESS`, and `PRIVATE_KEY`
    should be replaced with the actual values for the Chef Automate
    configuration. The `PRIMARY_IP_ADDRESS` and `STANDBY_IP_ADDRESS`
    values should be from a private network between the two machines.

12. Copy the following files from the `/etc/delivery/` directory on the
    primary Chef Automate server to the standby: `delivery.pem`,
    `builder_key`, `builder_key.pub`, and `delivery-secrets.json`. And
    then verify that `builder_key`, `builder_key.pub`, and
    `delivery-secrets.json` have a mode of `600`.

13. On the standby server, create the `/etc/chef/trusted_certs`
    directory:

    ``` bash
    sudo mkdir -p /etc/chef/trusted_certs
    ```

14. Copy all of the files in `/etc/chef/trusted_certs/` from the primary
    Chef Automate server to the same directory on the standby server.

15. Create the `/var/opt/delivery/nginx/ca/` directory on the standby
    server:

    ``` bash
    sudo mkdir -p /var/opt/delivery/nginx/ca/
    ```

16. Copy all contents of `/var/opt/delivery/nginx/ca/` from the primary
    Chef Automate server to the same directory on the standby server.

17. Run the following command on the primary Chef Automate server:

    ``` bash
    sudo automate-ctl reconfigure
    ```

18. Run the following command on the standby Chef Automate server:

    ``` bash
    sudo automate-ctl reconfigure
    ```

## Disaster Recovery

In most scenarios, converting the standby Chef Automate server to a
standalone configuration is the simplest way to get Chef Automate itself
back up and running, after which you can rebuild a standby server,
update the IP address for the standby server, and then reconfigure the
Chef Automate configuration to have a primary and standby server.

### Failover the Chef Automate Server

To promote a standby Chef Automate server to primary, do the following:

1.  Log into the standby Chef Automate server (via SSH, and not the Chef
    Automate web UI) and make a backup of the data:

    ``` bash
    sudo automate-ctl create-backup
    ```

    Move this data to a location that is not on the standby Chef
    Automate server.

2.  If the primary Chef Automate server is still accessible, log into it
    and run the following command as the root user:

    ``` bash
    automate-ctl stop
    ```

3.  Convert the standby server to a standalone Chef Automate server.
    Update the `delivery["primary"]`, `delivery["primary_ip"]`, and
    `postgresql["listen_address"]` settings in the
    `/etc/delivery/delivery.rb` file to be similar to:

    ``` ruby
    delivery["primary"] = false
    delivery["primary_ip"] = '192.0.2.0'
    postgresql["listen_address"] = 'localhost,192.0.2.0'
    ```

4.  On the standby server, run the following command as the root user:

    ``` bash
    automate-ctl reconfigure
    ```

    This will reconfigure the server to become a standalone Chef
    Automate server, after which a new standby server can be installed
    and configured to be the new standby.

5.  Set the DNS/load balancer to redirect traffic to the new primary
    Chef Automate server, as required.

### Recreate the Standby

Recreating the standby Chef Automate server requires the following
steps:

-   Deleting the old primary server
-   Updating configuration if SSH provisioning is being used
-   Installing a Chef Automate server to act as a standby

#### Delete the Primary

To delete the failed primary, do the following:

1.  Log in to the Chef Infra Server and delete the primary Chef Automate
    server node and client.
2.  Delete or destroy the primary Chef Automate machine.

#### Configure SSH

If provisioning uses the SSH driver, do the following:

1.  Remove the disaster recovery block in the Chef Automate cluster.

2.  Set the correct IP address for new primary node.

3.  Run the following command:

    ``` bash
    rm .chef/provisioning/ssh/delivery-server-test.json
    ```

#### Reinstall Standby

To set up a new standby Chef Automate server, follow the same steps for
installing the Chef Automate server (either manually or using the
`delivery-cluster` cookbook), as described earlier in this topic.
