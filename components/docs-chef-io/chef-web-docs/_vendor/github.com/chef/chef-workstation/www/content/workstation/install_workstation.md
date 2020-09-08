+++
title = "Install Chef Workstation"
draft = false

aliases = ["/install_workstation.html", "/install_dk.html", "/workstation_windows.html", "/dk_windows.html", "/install_workstation/"]

[menu]
  [menu.workstation]
    title = "Install Chef Workstation"
    identifier = "chef_workstation/install_workstation.md Install Chef Workstation"
    parent = "chef_workstation"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/install_workstation.md)

Start your infrastructure automation quickly and easily with [Chef
Workstation](https://www.chef.sh/) . Chef Workstation gives you
everything you need to get started with Chef - ad hoc remote execution,
remote scanning, configuration tasks, cookbook creation tools as well as
robust dependency and testing software - all in one easy-to-install
package.

Chef Workstation includes:

-   Chef Infra Client
-   Chef InSpec
-   chef and knife command line tools
-   Testing tools such as Test Kitchen, ChefSpec, and Cookstyle
-   Everything else needed to author cookbooks and upload them to the
    Chef Infra Server

## Supported Platforms

Supported Host Operating Systems:

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr class="header">
<th>Platform</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Apple macOS</td>
<td>10.13, 10.14, 10.15</td>
</tr>
<tr class="even">
<td>Microsoft Windows</td>
<td>10, Server 2012, Server 2012 R2, Server 2016, Server 2019</td>
</tr>
<tr class="odd">
<td>Red Hat Enterprise Linux / CentOS</td>
<td>6.x, 7.x, 8.x</td>
</tr>
<tr class="even">
<td>Ubuntu</td>
<td>16.04, 18.04</td>
</tr>
<tr class="odd">
<td>Debian</td>
<td>8.x, 9.x, 10.x</td>
</tr>
</tbody>
</table>

## System Requirements

Minimum system requirements:

-   RAM: 2GB
-   Disk: 4GB
-   Running minimum settings may limit your ability to take advantage of
    Chef Workstation tools such as Test Kitchen which creates and
    manages virtualized test environments.

Recommended system requirements:

-   RAM: 4GB
-   Disk 8GB

### Chef Workstation App Requirements

-   Windows: No additional requirements
-   Mac: No additional requirements
-   Linux: You must have a graphical window manager running with support
    for system tray icons.
    -   On some distributions you may need to install additional
        libraries. After you install the Chef Workstation package from
        the terminal, the post-install message will tell you which, if
        any, additional libraries are required to run the Chef
        Workstation App.

## Installation

The Chef Workstation installer must run as a privileged user.

Chef Workstation installs to `/opt/chef-workstation/` on macOS / Linux
and `C:\opscode\chef-workstation\` on Windows. These file locations
should help avoid interference between these components and other
applications that may be running on the target machine.

### macOS

1.  Dependency: Xcode is recommended for running Chef Workstation on
    macOS. While Chef Workstation works without Xcode, it is required
    for native Ruby Gem installation. Run `xcode-select --install` from
    the terminal to install Xcode.
2.  Visit the [Chef Workstation downloads
    page](https://downloads.chef.io/chef-workstation#mac_os_x) and
    select the appropriate package for your macOS version. Click on the
    **Download** button.
3.  Follow the steps to accept the license and install Chef Workstation.

Alternately, install Chef Workstation using Homebrew:

`brew cask install chef-workstation`

### Windows

1.  Visit the [Chef Workstation downloads
    page](https://downloads.chef.io/chef-workstation#windows) and select
    the appropriate package for your Windows version. Click on the
    **Download** button.
2.  Follow the steps to accept the license and install Chef Workstation.
    You will have the option to change your install location; by default
    the installer uses the `C:\opscode\chef-workstation\` directory.
3.  **Optional:** Set the default shell. On Microsoft Windows it is
    strongly recommended to use Windows PowerShell instead of `cmd.exe`.

#### Headless Unattended Install


"Headless" systems are configured to operate without a monitor (the "head") keyboard, and mouse.  They are usually controlled over a network connection.

To install Chef Workstation on a headless Windows system,
exclude the Chef Workstation App from auto-starting on login by using the following
command in Windows PowerShell or `cmd.exe`.  Replace `MsiPath` with the path of
the downloaded Chef Workstation installer.

```
msiexec /q /i MsiPath ADDLOCAL=ALL REMOVE=ChefWSApp
```


#### Spaces and Directories

{{% windows_spaces_and_directories %}}

#### Top-level Directory Names

{{% windows_top_level_directory_names %}}

### Linux

1.  Visit the [Chef Workstation downloads
    page](https://downloads.chef.io/chef-workstation) and download the
    appropriate package for your distribution:

    ``` bash
    wget https://packages.chef.io/files/stable/chefworkstation/0.14/ubuntu/18.04/chefworkstation_0.14.16-1_amd64.deb
    ```

2.  Use your distribution's package manager to install Chef Workstation:

    -   Red Hat Enterprise Linux:

        ``` bash
        rpm -Uvh chef-workstation-0.14.16-1.el7.x86_64.rpm
        ```

    -   Debian/Ubuntu:

        ``` bash
        dpkg -i chefworkstation_0.14.16-1_amd64.deb
        ```

## Verify the Installation

To verify the installation, run:

``` shell
chef -v
```

Which returns the versions of all installed Chef tools:

``` shell
Chef Workstation version: 0.16.31
Chef Infra Client version: 15.8.23
Chef InSpec version: 4.18.85
Chef CLI version: 2.0.0
Test Kitchen version: 2.3.4
Cookstyle version: 5.21.9
```

## Upgrading

### From Chef Workstation

For all platforms, follow the steps provided under [Installing]({{< ref "install_workstation.md" >}}).

### From ChefDK

#### Linux

The Chef Workstation package conflicts with an installed ChefDK package to prevent
unintentional upgrades.

Prior to installing Chef Workstation, first uninstall ChefDK:

Ubuntu, Debian, and related:

```bash
sudo dpkg -P chefdk
```

Red Hat, CentOS, and related:

```bash
sudo rpm -e chefdk
```

#### Other

For other platforms, follow the steps provided under [Installing]({{< ref "#installing" >}}).

## Uninstalling

### Mac

Run ```uninstall_chef_workstation``` in your terminal.

### Windows

Use **Add / Remove Programs** to remove Chef Workstation.

### Linux

Ubuntu, Debian, and related:

```bash
sudo dpkg -P chef-workstation
```

Red Hat, CentOS, and related:

```bash
sudo rpm -e chef-workstation
```

## Next Steps

Now that you've installed Chef Workstation, proceed to the
[Getting Started](/workstation/getting_started/) guide to
configure your Chef Workstation installation.
