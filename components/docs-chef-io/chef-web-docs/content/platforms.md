+++
title = "Platforms"
draft = false

aliases = ["/platforms.html", "/supported_platforms.html"]

[menu]
  [menu.overview]
    title = "Platforms"
    identifier = "overview/packages_&_platforms/platforms.md Platforms"
    parent = "overview/packages_&_platforms"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/platforms.md)

Chef software is supported on the various operating systems (platforms)
listed below. To see which versions of our software we currently
support, see the [Supported Versions](/versions/) page.

## Platforms

The sections below list the platforms that Chef Software Inc. supports.
Support is divided into two levels:

-   **Commercial Support** consists of the platforms that are supported
    as part of a paid commercial support contract with Chef Software
    Inc.
-   **Community Support** is made up of platforms for which support is
    only available through the Chef community

Any platforms or versions not explicitly listed here are unsupported,
both commercially and by the community.

Commercial support generally follows Chef community support policies,
which track the lifecycle policies of the underlying operating system
vendor.

In all cases (beyond community support), a maintenance contract with
Chef Software Inc. is required in order to open support tickets and get
SLA-based assistance from our support desk.

### Chef Infra Client

#### Commercial Support

{{< important >}}

**Chef Infra Client 16 currently cannot build for Solaris**

Due to the impact of COVID-19, Chef's employees cannot access our physical data center, which is a requirement for Solaris support. Until we can physically access the data center, Solaris builds on Chef Infra Client 16 will not be supported. They are available on Chef Infra Client 15, and we will begin building for Chef Infra Client 16 as soon as we responsibly can.

{{< /important >}}

The following table lists the commercially-supported platforms and
versions for Chef Infra Client:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>AIX</td>
<td><code>powerpc</code></td>
<td><code>7.1</code> (TL5 SP2 or higher, recommended), <code>7.2</code></td>
</tr>
<tr class="even">
<td>Amazon Linux</td>
<td><code>x86_64</code>, <code>aarch64</code> (2.x only)</td>
<td>2013+ and 2.0</td>
</tr>
<tr class="odd">
<td>CentOS</td>
<td><code>x86_64</code>, <code>ppc64le</code> (7.x only), <code>ppc64</code> (7.x only), <code>aarch64</code> (7.x / 8.x only)</td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="even">
<td>Debian</td>
<td><code>x86_64</code><code>aarch64</code> (10.x only)</td>
<td><code>9</code>, <code>10</code></td>
</tr>
<tr class="odd">
<td>FreeBSD</td>
<td><code>amd64</code></td>
<td><code>11.x</code>, <code>12.x</code></td>
</tr>
<tr class="even">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.13</code>, <code>10.14</code>, <code>10.15</code></td>
</tr>
<tr class="odd">
<td>Oracle Enterprise Linux</td>
<td><code>x86_64</code><code>aarch64</code> (7.x / 8.x only)</td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="even">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code>, <code>ppc64le</code> (7.x only), <code>ppc64</code> (7.x only), <code>aarch64</code> (7.x / 8.x only)</td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="odd">
<td>Solaris</td>
<td><code>sparc</code>, <code>i86pc</code></td>
<td><code>11.2</code>, <code>11.3</code>, <code>11.4</code></td>
</tr>
<tr class="even">
<td>SUSE Enterprise Linux Server</td>
<td><code>x86_64</code>, <code>aarch64</code> (15.x only)</td>
<td><code>12</code>, <code>15</code></td>
</tr>
<tr class="odd">
<td>Ubuntu (LTS releases)</td>
<td><code>x86_64</code>,<code>aarch64</code> (18.04/20.04 only)</td>
<td><code>16.04</code>, <code>18.04</code>, <code>20.04</code></td>
</tr>
<tr class="even">
<td>Microsoft Windows</td>
<td><code>x86</code>, <code>x64</code></td>
<td><code>8.1</code>, <code>2012</code>, <code>2012 R2</code>, <code>2016</code>, <code>10 (all channels except "insider" builds)</code>, <code>2019 (Long-term servicing channel (LTSC), both Desktop Experience and Server Core)</code></td>
</tr>
</tbody>
</table>

#### Community Support

The following platforms are supported only via the community:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Arch Linux</td>
<td><code>x86_64</code></td>
<td>current version</td>
</tr>
<tr class="even">
<td>Fedora</td>
<td><code>x86_64</code></td>
<td>current non-EOL releases</td>
</tr>
<tr class="odd">
<td>Gentoo</td>
<td><code>x86_64</code></td>
<td>current version</td>
</tr>
<tr class="even">
<td>openSUSE</td>
<td><code>x86_64</code></td>
<td><code>15.x</code></td>
</tr>
<tr class="odd">
<td>Scientific Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="even">
<td>Ubuntu</td>
<td><code>x86_64</code></td>
<td>Current non-LTS releases</td>
</tr>
<tr class="odd">
<td>Windows</td>
<td><code>x64</code></td>
<td><code>Windows Server, Semi-annual channel (SAC) (Server Core only)</code></td>
</tr>
</tbody>
</table>

### Chef Workstation

#### Commercial Support

The following table lists the commercially-supported platforms and
versions for the Chef Workstation:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.13</code>, <code>10.14</code>, <code>10.15</code></td>
</tr>
<tr class="even">
<td>Debian</td>
<td><code>x86_64</code></td>
<td><code>9</code>, <code>10</code></td>
</tr>
<tr class="odd">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="even">
<td>Ubuntu</td>
<td><code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code>, <code>20.04</code></td>
</tr>
<tr class="odd">
<td>Microsoft Windows</td>
<td><code>x64</code></td>
<td><code>8.1</code>, <code>2012</code>, <code>2012 R2</code>, <code>2016</code>, <code>10 (all channels except "insider" builds)</code>, <code>2019 (Long-term servicing channel (LTSC), Desktop Experience only)</code></td>
</tr>
</tbody>
</table>

### Chef InSpec

#### Commercial Support

The following table lists the commercially-supported platforms and
versions for Chef InSpec:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Amazon Linux</td>
<td><code>x86_64</code>, <code>aarch64</code></td>
<td><code>2013+ and 2.0</code></td>
</tr>
<tr class="even">
<td>Debian</td>
<td><code>x86_64</code><code>aarch64</code> (10.x only)</td>
<td><code>9.x</code>, <code>10.x</code></td>
</tr>
<tr class="odd">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.13</code>, <code>10.14</code>, <code>10.15</code></td>
</tr>
<tr class="even">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code>, <code>aarch64</code> (7.x and 8.x only)</td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="odd">
<td>SUSE Enterprise Linux Server</td>
<td><code>x86_64</code></td>
<td><code>12</code>, <code>15</code></td>
</tr>
<tr class="even">
<td>Ubuntu</td>
<td><code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code>, <code>20.04</code></td>
</tr>
<tr class="odd">
<td>Microsoft Windows</td>
<td><code>x86_64</code></td>
<td><code>8.1</code>, <code>2012</code>, <code>2012 R2</code>, <code>2016</code>, <code>10 (all channels except "insider" builds)</code>, <code>2019</code></td>
</tr>
</tbody>
</table>

Chef InSpec Target Mode (`inspec --target`) may be functional on
additional platforms, versions, and architectures but are not validated
by Chef Software, Inc.

### ChefDK

#### Commercial Support

The following table lists the commercially-supported platforms and
versions for ChefDK:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Debian</td>
<td><code>x86_64</code></td>
<td><code>9.x</code>, <code>10.x</code></td>
</tr>
<tr class="even">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.13</code>, <code>10.14</code>, <code>10.15</code></td>
</tr>
<tr class="odd">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="even">
<td>SUSE Enterprise Linux Server</td>
<td><code>x86_64</code></td>
<td><code>12</code>, <code>15</code></td>
</tr>
<tr class="odd">
<td>Ubuntu</td>
<td><code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code>, <code>20.04</code></td>
</tr>
<tr class="even">
<td>Microsoft Windows</td>
<td><code>x86</code>, <code>x64</code></td>
<td><code>2012</code>, <code>2012 R2</code>, <code>2016</code>, <code>10 (all channels except "insider" builds)</code>, <code>2019 (Long-term servicing channel (LTSC), Desktop Experience only)</code></td>
</tr>
</tbody>
</table>

#### Community Support

The following platforms are supported only via the community:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>openSUSE</td>
<td><code>x86_64</code></td>
<td><code>15.x</code></td>
</tr>
<tr class="even">
<td>Scientific Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
</tbody>
</table>

### Chef Infra Server

#### Commercial Support

{{% adopted_platforms_server %}}

### Chef Automate Server

#### Commercial Support

Commercial support for the [Chef Automate 2
Server](/automate/system_requirements/) is available
for platforms that use:

- a Linux kernel version of 3.2 or greater
- `systemd` as the init system
- `useradd`
- `curl` or `wget`

### Chef Automate Job Runners

#### Commercial Support

Chef Automate Job Runners are supported on the Commercial Support
platforms for Chef Automate Server listed above as well as on the
following platforms:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.12</code></td>
</tr>
</tbody>
</table>

### Chef Push Jobs Client

#### Commercial Support

The following table lists the commercially-supported platforms for the
Chef Push Jobs client:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>AIX</td>
<td><code>powerpc</code></td>
<td><code>7.1</code> (TL5 SP2 or higher, recommended)</td>
</tr>
<tr class="even">
<td>CentOS</td>
<td><code>i386</code>, <code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="odd">
<td>Debian</td>
<td><code>x86_64</code></td>
<td><code>9</code></td>
</tr>
<tr class="even">
<td>macOS</td>
<td><code>x86_64</code></td>
<td><code>10.11</code>, <code>10.12</code>, <code>10.13</code>, <code>10.14</code></td>
</tr>
<tr class="odd">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="even">
<td>Ubuntu (LTS releases)</td>
<td><code>i386</code>, <code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code></td>
</tr>
<tr class="odd">
<td>Microsoft Windows</td>
<td><code>x86</code>, <code>x64</code></td>
<td><code>2012</code>, <code>2012 R2</code>, <code>10</code></td>
</tr>
</tbody>
</table>

### Chef Push Jobs Server

#### Commercial Support

The following table lists the commercially-supported platforms for the
Chef Push Jobs server:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>CentOS</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="even">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="odd">
<td>Ubuntu (LTS releases)</td>
<td><code>x86_64</code></td>
<td><code>16.04</code></td>
</tr>
</tbody>
</table>

### Chef Backend

#### Commercial Support

The following table lists the commercially-supported platforms for Chef
Backend, the high-availability solution for Chef Infra Server:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>CentOS</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="even">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code></td>
</tr>
<tr class="odd">
<td>Ubuntu (LTS releases)</td>
<td><code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code></td>
</tr>
</tbody>
</table>

### Chef Manage

#### Commercial Support

The following table lists the commercially-supported platforms for Chef
Manage:

<table>
<colgroup>
<col style="width: 30%" />
<col style="width: 35%" />
<col style="width: 35%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>CentOS</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="even">
<td>Red Hat Enterprise Linux</td>
<td><code>x86_64</code></td>
<td><code>6.x</code>, <code>7.x</code>, <code>8.x</code></td>
</tr>
<tr class="odd">
<td>Ubuntu (LTS releases)</td>
<td><code>x86_64</code></td>
<td><code>16.04</code>, <code>18.04</code></td>
</tr>
</tbody>
</table>

## Platform End-of-Life Policy

Chef's products on particular platforms and versions generally reach
end-of-life on the same date as the vendor EOL milestone for that
operating systems. Because different vendors use different terminology,
the following table clarifies when Chef products are end-of-life
according to those vendors' terms:

<table>
<colgroup>
<col style="width: 74%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Vendor End-of-Life</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>AIX</td>
<td>IBM End of Support Date</td>
</tr>
<tr class="even">
<td>Debian</td>
<td>End of maintenance updates</td>
</tr>
<tr class="odd">
<td>Enterprise Linux (covers Red Hat Enterprise Linux, CentOS)</td>
<td>End of Production 3</td>
</tr>
<tr class="even">
<td>FreeBSD</td>
<td>End of Life</td>
</tr>
<tr class="odd">
<td>Microsoft Windows</td>
<td>End of Extended Support</td>
</tr>
<tr class="even">
<td>Oracle Enterprise Linux</td>
<td>Premier Support Ends</td>
</tr>
<tr class="odd">
<td>Oracle Solaris</td>
<td>Premier Support Ends</td>
</tr>
<tr class="even">
<td>SUSE Linux Enterprise Server</td>
<td>General Support Ends</td>
</tr>
<tr class="odd">
<td>Ubuntu Linux</td>
<td>End of maintenance updates</td>
</tr>
</tbody>
</table>

At Chef's option, additional support may be provided to customers beyond
the vendor end-of-life in the above table. As such, the following table
indicates upcoming product end-of-life dates for particular platforms.
On the Chef end-of-life date, Chef discontinues building software for
that platform and version.

<table>
<colgroup>
<col style="width: 54%" />
<col style="width: 22%" />
<col style="width: 22%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform and Version</th>
<th>Vendor End-of-Life Date</th>
<th>Chef End-of-Life Date</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>AIX 6.1</td>
<td>April 30, 2017</td>
<td>December 31, 2017</td>
</tr>
<tr class="even">
<td>Debian 7 (Wheezy)</td>
<td>May 31st, 2018</td>
<td>May 31st, 2018</td>
</tr>
<tr class="odd">
<td>Debian 8 (Jessie)</td>
<td>June 6th, 2020</td>
<td>June 6th, 2020</td>
</tr>
<tr class="even">
<td>Enterprise Linux 5 (covers Red Hat Enterprise Linux, CentOS)</td>
<td>April 30, 2017</td>
<td>December 31, 2017</td>
</tr>
<tr class="odd">
<td>Enterprise Linux 6 (covers Red Hat Enterprise Linux, CentOS)</td>
<td>November 30, 2020</td>
<td>November 30, 2020</td>
</tr>
<tr class="even">
<td>FreeBSD 10-STABLE</td>
<td>October 31, 2018</td>
<td>October 31, 2018</td>
</tr>
<tr class="odd">
<td>FreeBSD 11-STABLE</td>
<td>September 30, 2021</td>
<td>September 30, 2021</td>
</tr>
<tr class="even">
<td>Microsoft Windows Server 2008 (SP2)/R2 (SP1)</td>
<td>January 13, 2015</td>
<td>January 14, 2020</td>
</tr>
<tr class="odd">
<td>Microsoft Windows Server 2012/2012 R2</td>
<td>October 10, 2023</td>
<td>October 10, 2023</td>
</tr>
<tr class="even">
<td>Microsoft Windows Server 2016</td>
<td>November 11, 2027</td>
<td>November 11, 2027</td>
</tr>
<tr class="odd">
<td>Microsoft Windows Server 2019</td>
<td>October 10, 2028</td>
<td>October 10, 2028</td>
</tr>
<tr class="even">
<td>Oracle Enterprise Linux 5</td>
<td>June 30, 2017</td>
<td>December 31, 2017</td>
</tr>
<tr class="odd">
<td>Oracle Enterprise Linux 6</td>
<td>March 31, 2021</td>
<td>March 31, 2021</td>
</tr>
<tr class="even">
<td>Oracle Solaris 10</td>
<td>January 30, 2018</td>
<td>January 30, 2018</td>
</tr>
<tr class="odd">
<td>SUSE Linux Enterprise Server 11</td>
<td>March 31, 2019</td>
<td>March 31, 2019</td>
</tr>
<tr class="even">
<td>SUSE Linux Enterprise Server 12</td>
<td>October 31, 2024</td>
<td>October 31, 2024</td>
</tr>
<tr class="odd">
<td>Ubuntu Linux 12.04 LTS</td>
<td>April 30, 2017</td>
<td>April 30, 2017</td>
</tr>
<tr class="even">
<td>Ubuntu Linux 14.04 LTS</td>
<td>April 30, 2019</td>
<td>April 30, 2019</td>
</tr>
<tr class="odd">
<td>Ubuntu Linux 16.04 LTS</td>
<td>April 30, 2021</td>
<td>April 30, 2021</td>
</tr>
</tbody>
</table>
