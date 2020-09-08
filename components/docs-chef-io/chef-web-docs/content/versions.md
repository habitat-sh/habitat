+++
title = "Supported Versions"
draft = false

aliases = ["/versions.html"]

[menu]
  [menu.overview]
    title = "Supported Versions"
    identifier = "overview/packages_&_platforms/versions.md Supported Versions"
    parent = "overview/packages_&_platforms"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/versions.md)

This section lists the free and commercial Chef products and versions we
currently support. The lifecycle stage defines the involvement by Chef
Software in updating and maintaining each product.

## Lifecycle Definitions

### Generally Available (GA)

This stage indicates that the product or version is in active
development and/or maintenance.

-   Chef continues to provide releases to the application or version in
    response to customer needs and security vulnerabilities
-   Chef welcomes customer feature requests for the product roadmap for
    the application

### Deprecated

This stage indicates that an application or version is no longer in
active development and will eventually move to end of life status. Chef
continues to provide support [according to our
SLAs](https://www.chef.io/service-level-agreement/).

-   Chef no longer provides scheduled releases
-   Customers should use the GA alternative to these products; contact
    us for help with product selection and deployment
-   Chef may provide a release for a critical defect or security
    vulnerability

### End of Life (EOL)

This stage indicates that Chef has set a date after which the
application or version will no longer be supported or recommended for
use by customers.

-   As of the end of life date, the application will no longer be
    supported by Chef and will no longer be available for download
-   Documentation for the application will be moved to
    <https://docs-archive.chef.io>

### Versions and Status

{{< important >}}

Unless otherwise stated, versions older than those listed below are EOL.

{{< /important >}}

## Supported Commercial Distributions

Use of these and later versions of these distributions must be in
accordance with the [Chef End User License
Agreement](https://www.chef.io/end-user-license-agreement/) or a
commercial agreement with Chef. Additional information is available in
[this
announcement](https://blog.chef.io/2019/04/02/chef-software-announces-the-enterprise-automation-stack/).

<table>
<colgroup>
<col style="width: 18%" />
<col style="width: 31%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Product</th>
<th>Version</th>
<th>Lifecycle Status</th>
<th>EOL Date</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Automate</td>
<td>2</td>
<td>GA</td>
<td>n/a</td>
</tr>
<tr class="even">
<td>Chef Infra Client</td>
<td>15.x</td>
<td>GA</td>
<td>April 30, 2021</td>
</tr>
<tr class="odd">
<td>Chef Infra Client</td>
<td>16.x</td>
<td>GA</td>
<td>April 30, 2022</td>
</tr>
<tr class="even">
<td>Chef Infra Server</td>
<td>13.x</td>
<td>GA</td>
<td>n/a</td>
</tr>
<tr class="odd">
<td>Chef Habitat</td>
<td>0.81+</td>
<td>GA</td>
<td>n/a</td>
</tr>
<tr class="even">
<td>Chef InSpec</td>
<td>4.x</td>
<td>GA</td>
<td>n/a</td>
</tr>
<tr class="odd">
<td>Chef Workstation</td>
<td>20.6+ (June 2020)</td>
<td>GA</td>
<td>n/a</td>
</tr>
<tr class="odd">
<td>Chef Backend</td>
<td>3.x</td>
<td>Releasing 2020</td>
<td>n/a</td>
</tr>
</tbody>
</table>

{{< note >}}

**Chef Backend** does not directly require acceptance of the Chef
EULA, but it does have functionality that requires its acceptance in other
products.

{{< /note >}}

## Supported Free Distributions

Use of the following distributions is governed by the Apache License,
version 2.0.

<table>
<colgroup>
<col style="width: 18%" />
<col style="width: 31%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Product</th>
<th>Version</th>
<th>Lifecycle Status</th>
<th>EOL Date</th>
</tr>
</thead>
<tbody>
<tr class="even">
<td>Supermarket</td>
<td>3.x</td>
<td>GA</td>
<td>TBD</td>
</tr>
</tbody>
</table>

## Deprecated Products and Versions

The following products are deprecated. Users are advised to move to
newer versions or products.

<table>
<colgroup>
<col style="width: 18%" />
<col style="width: 31%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Product</th>
<th>Version</th>
<th>Lifecycle Status</th>
<th>EOL Date</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Backend</td>
<td>2.x</td>
<td>Deprecated</td>
<td>December 31, 2021</td>
</tr>
<tr class="even">
<td>Chef Infra Server</td>
<td>12.x</td>
<td>Deprecated</td>
<td>December 31, 2020</td>
</tr>
<tr class="odd">
<td>ChefDK</td>
<td>4.x</td>
<td>Deprecated</td>
<td>December 31, 2020</td>
</tr>
<tr class="even">
<td>Chef Manage</td>
<td>2.5.x+</td>
<td>Deprecated</td>
<td>December 31, 2021</td>
</tr>
<tr class="odd">
<td>Chef Workflow</td>
<td>2.x</td>
<td>Deprecated</td>
<td>December 31, 2020</td>
</tr>
<tr class="even">
<td>Push Jobs</td>
<td>2.5.x</td>
<td>Deprecated</td>
<td>December 31, 2020</td>
</tr>
<tr class="odd">
<td>Chef InSpec</td>
<td>3.x</td>
<td>Deprecated</td>
<td>April 30, 2020</td>
</tr>
</tbody>
</table>

## End of Life (EOL) Products

{{< note >}}

Chef Compliance Server, which reached EOL status in 2018, should not be
confused with the modern [Chef Compliance offering](/compliance/).

{{< /note >}}

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Product</th>
<th>Version</th>
<th>Lifecycle Status</th>
<th>EOL Date</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Analytics</td>
<td>All</td>
<td>EOL</td>
<td>December 31, 2018</td>
</tr>
<tr class="even">
<td>Automate</td>
<td>1.x</td>
<td>EOL</td>
<td>December 31, 2019</td>
</tr>
<tr class="odd">
<td>Chef Client</td>
<td>14 and under</td>
<td>EOL</td>
<td>April 30, 2020</td>
</tr>
<tr class="even">
<td>Chef Compliance Server</td>
<td>All</td>
<td>EOL</td>
<td>December 31, 2018</td>
</tr>
<tr class="odd">
<td>ChefDK</td>
<td>3 and under</td>
<td>EOL</td>
<td>April 30, 2020</td>
</tr>
<tr class="even">
<td>Enterprise Chef</td>
<td>All</td>
<td>EOL</td>
<td>December 31, 2018</td>
</tr>
<tr class="odd">
<td>Chef InSpec</td>
<td>2 and under</td>
<td>EOL</td>
<td>December 31, 2019</td>
</tr>
<tr class="even">
<td>Chef Provisioning</td>
<td>All</td>
<td>EOL</td>
<td>August 31, 2019</td>
</tr>
<tr class="odd">
<td>Chef Replication/Sync</td>
<td>All</td>
<td>EOL</td>
<td>August 31, 2019</td>
</tr>
<tr class="even">
<td>Reporting</td>
<td>All</td>
<td>EOL</td>
<td>December 31, 2018</td>
</tr>
<tr class="odd">
<td>Chef Server DRBD HA</td>
<td>All</td>
<td>EOL</td>
<td>March 31, 2019</td>
</tr>
</tbody>
</table>
