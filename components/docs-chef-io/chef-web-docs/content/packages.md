+++
title = "Chef Software Inc Packages"
draft = false

aliases = ["/packages.html"]

[menu]
  [menu.overview]
    title = "Packages"
    identifier = "overview/packages_&_platforms/packages.md Packages"
    parent = "overview/packages_&_platforms"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/packages.md)

Packages for Chef Software Inc. products may be installed using
platform-native package repositories or the Chef Software Install script. Both
installation methods support the following release channels:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Channel</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>stable</code></td>
<td>A build from this channel is an "official" release that has passed full user acceptance testing. Artifacts in this channel are retained indefinitely.</td>
</tr>
<tr class="even">
<td><code>current</code></td>
<td>A build from this channel is an "integration" build that has passed full testing, but has not been officially released. Artifacts in this channel are retained for 30 days and then removed automatically.</td>
</tr>
</tbody>
</table>

Chef recommends using the stable channel when installing any of these
products on production systems.

## Package Repositories

The `stable` and `current` release channels support the following
package repositories:

-   Apt (Debian and Ubuntu platforms)
-   Yum (Enterprise Linux platforms)

Chef Software Inc. GPG public key is can be downloaded
[here](https://packages.chef.io/chef.asc).

### Debian / Ubuntu

To set up an Apt package repository for Debian and Ubuntu platforms:

1.  Enable Apt to fetch packages over HTTPS:

    ``` bash
    sudo apt-get install apt-transport-https
    ```

2.  Install the public key for Chef Software Inc:

    ``` bash
    wget -qO - https://packages.chef.io/chef.asc | sudo apt-key add -
    ```

3.  Create the Apt repository source file:

    ``` bash
    echo "deb https://packages.chef.io/repos/apt/<CHANNEL> <DISTRIBUTION> main" > chef-<CHANNEL>.list
    ```

    Replace `<CHANNEL>` with the release channel: `stable` or `current`.

    Replace `<DISTRIBUTION>` with the appropriate distribution name:

    -   For Debian 8: `jessie`
    -   For Debian 9: `stretch`
    -   For Debian 10: `buster`
    -   For Ubuntu 16.04: `xenial`
    -   For Ubuntu 18.04: `bionic`

4.  Update the package repository list:

    ``` bash
    sudo mv chef-stable.list /etc/apt/sources.list.d/
    ```

5.  Update the cache for the package repository:

    ``` bash
    sudo apt-get update
    ```

### Enterprise Linux

To set up a Yum package repository for Enterprise Linux platforms:

1.  Install the public key for Chef Software Inc:

    ``` bash
    sudo rpm --import https://packages.chef.io/chef.asc
    ```

2.  Create the Yum repository source file:

    ``` bash
    cat >chef-<CHANNEL>.repo <<EOL
    [chef-<CHANNEL>]
    name=chef-<CHANNEL>
    baseurl=https://packages.chef.io/repos/yum/<CHANNEL>/el/<VERSION>/\$basearch/
    gpgcheck=1
    enabled=1
    EOL
    ```

    Replace `<CHANNEL>` with the release channel: `stable` or `current`.

    Replace `<VERSION>` with your Enterprise Linux version; the
    allowable versions are `6`, `7`, or `8`.

3.  Update the package repository list:

    ``` bash
    sudo yum-config-manager --add-repo chef-stable.repo
    ```

    Note that the `yum-config-manager` command requires the `yum-utils`
    package, which is not installed on CentOS by default. You can
    install the package by running `sudo yum install yum-utils`, or you
    can use the `mv` command to add the repository to the
    `/etc/yum.repos.d/` directory:

    ``` bash
    sudo mv chef-stable.repo /etc/yum.repos.d/
    ```

## Chef Software Install Script

{{% packages_install_script %}}

### Run the Chef Software Install Script

{{% packages_install_script_run %}}

#### UNIX and Linux

{{% packages_install_script_run_unix_linux %}}

#### Microsoft Windows

{{% packages_install_script_run_windows %}}

### Chef Software Install Script Options

{{% packages_install_script_options %}}

### Examples

{{% packages_install_script_examples %}}
