+++
title = "Chef Infra Server Prerequisites"
draft = false

aliases = ["/install_server_pre.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Server Prerequisites"
    identifier = "chef_infra/setup/chef_infra_server/install_server_pre.md Chef Infra Server Prerequisites"
    parent = "chef_infra/setup/chef_infra_server"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_server_pre.md)

The following is a detailed discussion of the prerequisites for every
installation of the Chef Infra Server. See <span
class="title-ref">Install Chef Infra Server
\</install_server.html\></span> for installation instructions.

## Platforms

{{% adopted_platforms_server %}}

### Untested Platforms

The following platforms are not tested by Chef Software:

-   Any Linux or UNIX distribution that is not listed as a Foundational
    platform.
-   Microsoft Windows
-   32-bit architectures

## Capacity Planning

Read the [guidance around capacity
planning](/server_overview/#capacity-planning) for information about
how to choose the right topology for the Chef Infra Server.

## Hardware Requirements

{{% system_requirements_server_hardware %}}

## Software Requirements

{{% system_requirements_server_software %}}

### UIDs and GIDs

The installation process for the Chef Infra Server requires the use of
at least two user and group identifiers (UIDs and GIDs). These are used
to create the `opscode` and `opscode-pgsql` users and their default
groups.

{{< note >}}

The creation of required user and group identifiers is done
**automatically** during the installation process for the Chef Infra
Server; however, the following user and group accounts **may** be
created in advance of installing the Chef Infra Server if specific UIDs
and GIDs are preferred. The user **and** group must be created as a pair
to satisfy reconfiguration requirements.

-   A local user account named `opscode` under which services will run
-   A local user account named `opscode-pgsql` that is used by
    PostgreSQL
-   A group account for each user account, one named `opscode` and the
    other named `opscode-pgsql` under which services will run

{{< /note >}}

{{< warning >}}

If the UID and GID of `opscode` and `opscode-pgsql` do not match on both
backend Chef Infra Server machines, a high availability configuration
will not run correctly.

The embedded Chef Infra Server cookbooks can handle two cases:

-   Both `opscode` and `opscode-pgsql` user and group not found on the
    new server
-   Both `opscode` and `opscode-pgsql` user and group found on the new
    server

Having only the group and not the corresponding users present during a
chef-server-ctl reconfigure is unsupported and may lead to an error in
the reconfiguration run.

To determine the current range of IDs, run the following command:

``` bash
grep -E '(UID|GID)' /etc/login.defs
```

The defaults for CentOS and Red Hat Enterprise Linux systems look like
this:

``` bash
UID_MIN             500
UID_MAX           60000
GID_MIN             500
GID_MAX           60000
```

If the defaults have been changed for any reason, and if that change
would result in less than 2 UID/GIDs being available to the `useradd`
program, edit `/etc/login.defs` with changes to make at least 2 more
UIDs and GIDs available for association. The currently used ID ranges
for UIDs and GIDs can be found in `/etc/passwd` and `/etc/group`,
respectively.

If the `opscode` and `opscode-pgsql` user and group identifiers exist
prior to installing the Chef Infra Server, the Chef Infra Server
installation process will use the existing identifiers instead of
creating them.

{{< /warning >}}

### Firewalls

#### iptables

To allow access to your Chef Infra Server on ports 80 and 443 via the
iptables firewall, issue the following command with root privileges:

``` bash
iptables -A INPUT -p tcp -m multiport --destination-ports 80,443 -j ACCEPT
```

Note that you will need to make use of a tool such as
[iptables-persistent](https://packages.ubuntu.com/xenial/admin/iptables-persistent)
to restore your iptables rules upon reboot.

#### FirewallD

On RHEL and CentOS versions 7 and above, the FirewallD firewall is
enabled by default. Issue the following command with root privileges to
open ports 80 and 443:

``` bash
firewall-cmd --permanent --zone public --add-service http && firewall-cmd --permanent --zone public --add-service https && firewall-cmd --reload
```

#### UFW

While UFW is installed on Ubuntu, it is not enabled by default. However,
if you wish to use a UFW-based firewall on your Chef Infra Server, issue
the following command with root privileges to open ports 80 and 443:

``` bash
ufw allow proto tcp from any to any port 80,443
```

### Security Modules

#### SELinux

On CentOS and Red Hat Enterprise Linux systems, SELinux is enabled in
enforcing mode by default. The Chef Infra Server does not have a profile
available to run under SELinux. In order for the Chef Infra Server to
run, SELinux must be disabled or set to `Permissive` mode.

To determine if SELinux is installed, run the following command:

``` bash
getenforce
```

If a response other than `"Disabled"` or `"Permissive"` is returned,
SELinux must be disabled.

To set SELinux to `Permissive` mode, run:

``` bash
setenforce Permissive
```

and then check the status:

``` bash
getenforce
```

#### AppArmor

On Ubuntu systems, AppArmor is enabled in enforcing mode by default.
Chef products do not have a profile available to run under AppArmor. In
order for the Chef products to run, AppArmor must set to `Complaining`
mode or disabled.

To determine if AppArmor is installed, run the following command:

``` bash
sudo apparmor_status
```

To install AppArmor, run the following command:

``` bash
sudo apt-get install apparmor-utils -yes
```

If a response other than `"0 processes are in enforce mode"` or
`"0 profiles are in enforce mode."` is returned, AppArmor must be set to
`Complaining` mode or disabled.

To set AppArmor to `Complaining` mode, run:

``` bash
sudo aa-complain /etc/apparmor.d/*
```

Or to disable AppArmor entirely, run:

``` bash
sudo invoke-rc.d apparmor kill
sudo update-rc.d -f apparmor remove
```

and then check the status:

``` bash
sudo apparmor_status
```

### Apache Qpid

On CentOS and Red Hat Enterprise Linux systems, the Apache Qpid daemon
is installed by default. The Chef Infra Server uses RabbitMQ for
messaging. Because both Apache Qpid and RabbitMQ share the same
protocol, Apache Qpid must be disabled.

To determine if Apache Qpid is installed, run the following command:

``` bash
rpm -qa | grep qpid
```

If Apache Qpid is installed, a response similar to the following is
displayed:

``` bash
qpid-cpp-server-0.12-6.el6.x86_64
```

To disable Apache Qpid run:

``` bash
service qpidd stop
```

and then:

``` bash
chkconfig --del qpidd
```

### cron

Periodic maintenance tasks are performed on the Chef Infra Server
servers via cron and the `/etc/cron.d` directory. With certain CentOS 6
configurations, an additional step is required to install crontab:

``` bash
yum install crontabs
```

### Enterprise Linux Updates

The Chef Infra Server requires an x86_64 compatible systems
architecture. When the Chef Infra Server is installed on Red Hat
Enterprise Linux or CentOS, run `yum update` prior to installing the
Chef Infra Server. This will ensure those platforms are fully compatible
with this requirement.

### IP Addresses

Unless you intend to operate the Chef Infra Server in IPv6 mode, you
should disable ipv6 in the system's `/etc/hosts` file by commenting out
or removing all references to IPv6 addresses like "::1" or
"fe80:db8:85a3:8d3:1319:8a2e:370:7348".

Without these changes, a Chef Infra Server install intended to run in
ipv4 mode will mistakenly only start the postgres service on the ipv6
loopback address of "::1" rather than the ipv4 loopback address of
127.0.0.1. This will make further progress through an initial
reconfiguration impossible.

### Hostnames

The hostname for the Chef Infra Server may be specified using a FQDN or
an IP address. This hostname must be resolvable, be 64 characters or
less, and be lowercase. For example, a Chef Infra Server running in a
production environment with a resolvable FQDN hostname can be added the
DNS system. But when deploying Chef Infra Server into a testing
environment, adding the hostname to the `/etc/hosts` file is enough to
ensure that hostname is resolvable.

-   **FQDN Hostnames** When the hostname for the Chef Infra Server is a
    FQDN be sure to include the domain suffix. For example, something
    like `mychefserver.example.com` (and not something like
    `mychefserver`).

-   **IP Address Hostnames** When the Chef Infra Server is run in IPv6
    mode, a hostname specified using an IP address must also be
    bracketed (`[ ]`) or the Chef Infra Server will not be able to
    recognize it as an IPv6 address. For example:

    ``` ruby
    bookshelf['url'] "https://[2001:db8:85a3:8d3:1319:8a2e:370:7348]"
    ```

The `api_fqdn` setting can be added to the private-chef.rb file (it is
not there by default). When added, its value should be equal to the FQDN
or IP address for the service URI used by the Chef Infra Server. Then
configure the same value for the `bookshelf['vip']` setting prior to
installing the Chef Infra Server. For example:
`api_fqdn "chef.example.com"` or `api_fqdn 123.45.67.890`.

#### Configure Hostnames

Use the following sections to verify the hostnames that is used by the
Chef Infra Server.

**To verify if a hostname is a FQDN**

To verify if a hostname is a FQDN, run the following command:

``` bash
hostname
```

If the hostname is a FQDN, it will return something like:

``` bash
mychefserver.example.com
```

If the hostname is not a FQDN, it must be configured so that it is one.

**To verify the FQDN is all lowercase**

To verify if the alphabetic parts of a FQDN are all lowercase, run the
following command:

``` bash
hostname -f | grep -E '^([[:digit:]]|[[:lower:]]|\.|-|_)+$' && echo yes
```

If the hostname is all lowercase, it will return something like:

``` bash
mychefserver.example.com
yes
```

If the hostname's alphabetic parts are not all lowercase, it must be
configured so that they are.

**To verify a hostname is resolvable**

To verify is a hostname is resolvable, run the following command:

``` bash
hostname -f
```

If the hostname is resolvable, it will return something like:

``` bash
mychefserver.example.com
```

**To change a hostname**

In some cases, the hostname for the Chef Infra Server needs to be
updated. The process for updating a hostname varies, depending on the
platform on which the Chef Infra Server will run. Refer to the manual
for the platform or contact a local systems administrator for specific
guidance for a specific platform. The following example shows how a
hostname can be changed when running Red Hat or CentOS:

``` bash
sudo hostname 'mychefserver.example.com'
```

and then:

``` bash
echo "mychefserver.example.com" | sudo tee /etc/hostname
```

**To add a hostname to /etc/hosts**

If a hostname is not resolvable, refer to a local systems administrator
for specific guidance on how to add the hostname to the DNS system. If
the Chef Infra Server is being into a testing environment, just add the
hostname to `/etc/hosts`. The following example shows how a hostname can
be added to `/etc/hosts` when running Red Hat or CentOS:

``` bash
echo -e "127.0.0.2 `hostname` `hostname -s`" | sudo tee -a /etc/hosts
```

{{< warning >}}

The FQDN for the Chef Infra Server should be resolvable, lowercase, and
should not exceed 64 characters when using OpenSSL, as OpenSSL requires
the `CN` in a certificate to be no longer than 64 characters.

{{< /warning >}}

### Mail Relay

The Chef Infra Server server uses email to send notifications for
various events:

-   Password resets
-   User invitations
-   Failover notifications
-   Failed job notifications

Configure a local mail transfer agent on the Chef Infra Server using the
steps appropriate for the platform on which the Chef Infra Server is
running.

### NTP

The Chef Infra Server requires that the systems on which it is running
be connected to Network Time Protocol (NTP), as the Chef Infra Server is
particularly sensitive to clock drift. For Red Hat and CentOS 6:

``` bash
yum install ntp
```

or:

``` bash
chkconfig ntpd on
```

or:

``` bash
service ntpd start
```

For Ubuntu:

``` bash
apt-get install ntp
```

#### Chef Infra Client

The Chef Infra Server server requires that every node that is under
management by Chef also have an accurate clock that is synchronized very
closely with the clock on the Chef Infra Server. If the clocks are not
synchronized closely, the authentication process may fail when the
clocks are out-of-sync by more than 15 minutes. A failure will trigger a
`401 Unauthorized` response similar to:

``` bash
[Tue, 01 Nov 2011 16:55:23 -0700] INFO: *** Chef 11.X.X ***
[Tue, 01 Nov 2011 16:55:23 -0700] INFO: Client key /etc/chef/client.pem is not present - registering
[Tue, 01 Nov 2011 16:55:24 -0700] INFO: HTTP Request Returned 401 Unauthorized:
    Failed to authenticate as ORGANIZATION-validator. Synchronize the clock on your host.
[Tue, 01 Nov 2011 16:55:24 -0700] FATAL: Stacktrace dumped to /var/chef/cache/chef-stacktrace.out
[Tue, 01 Nov 2011 16:55:24 -0700] FATAL: Net::HTTPClientException: 401 "Unauthorized"
```

In this situation, re-synchronize the system clocks with the Network
Time Protocol (NTP) server, and then re-run Chef Infra Client.

### Required Accounts

By default, accounts required by the Chef Infra Server are created
during setup. If your environment has restrictions on the creation of
local user and group accounts that will prevent these accounts from
being created automatically during setup, you will need to create these
accounts.

{{< note >}}

The Chef Push Jobs feature of the Chef Infra Server use the same user
and group accounts as the Chef Infra Server.

{{< /note >}}

#### Group Accounts

The following group accounts are required:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Group Account</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>opscode</code></td>
<td>The group name under which services will run.</td>
</tr>
</tbody>
</table>

#### User Accounts

The following user accounts are required:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>User Account</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>opscode</code></td>
<td>The user name under which services will run.</td>
</tr>
<tr class="even">
<td><code>opscode-pgsql</code></td>
<td>The user name for PostgreSQL. (This is only required on the back end servers in a high availability setup.)</td>
</tr>
</tbody>
</table>
