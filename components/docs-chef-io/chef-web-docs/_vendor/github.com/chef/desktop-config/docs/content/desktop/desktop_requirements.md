+++
title = "Chef Desktop Requirements"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Chef Desktop Requirements"
    identifier = "desktop/requirements.md Prerequisites"
    parent = "desktop"
    weight = 20
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop_requirements.md)

*Estimated time: 1 hour*

## System Requirements

Chef Desktop utilizes the following system architecture:

* A Chef Automate Server
* A Chef Infra Server
* A developer environment running Chef Workstation
* One or more "nodes", which are the Windows or macOS computers that you will manage.

### Chef Automate

System requirements:

* 16 GB of RAM
* 80 GB of disk space (available to `/hab`)
* 4 vCPUs

Operating system requirements:

* a Linux kernel of version 3.2 or greater
* systemd as the init system
* useradd
* cURL or GNU Wget
* The shell that starts Chef Automate should have a max open files setting of at least 65535

For more information, see the Chef Automate [system requirements](https://automate.chef.io/docs/system-requirements/).

### Chef Infra Server

Chef recommends a single virtual machine instance:

* 2 CPU cores and 8GB of RAM, which is equivalent to an Amazon EC2 m3.large instance
* 2MB of disk space on the data partition per managed node

For more information, see [Install Chef Infra Server](https://docs.chef.io/install_server/) and [Chef Infra Server Prerequisites](https://docs.chef.io/install_server_pre/).

### Chef Workstation

Recommended system requirements:

* 4GB of RAM
* 8GB of disk space

For more information, see [Install Chef Workstation](https://docs.chef.io/workstation/install_workstation/).

### Developer Environment

#### Visual Studio Code

Chef does not prescribe any specific editor. However, we highly recommend using [Visual Studio Code](https://code.visualstudio.com/) (VSCode) and the [Chef Infra extension for VSCode](https://marketplace.visualstudio.com/items?itemName=chef-software.Chef), which features code generators and helpful features, such as running Cookstyle linting each time you save a recipe.

In addition to VSCode, for this guide you need the following extensions from the Visual Studio Code Marketplace:

* [Chef Infra Extension](https://marketplace.visualstudio.com/items?itemName=chef-software.Chef)
* [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
* [PowerShell](https://marketplace.visualstudio.com/items?itemName=ms-vscode.PowerShell)

#### Virtualization

Test Kitchen, the environment that you will use for testing your Chef Desktop cookbooks before deploying them to your devices, relies on virtualization. Download VirtualBox and Vagrant to use with Test Kitchen:

[VirtualBox](https://www.virtualbox.org/wiki/Downloads)
: The VirtualBox Base Package is licensed under the [GPL V2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html). (Some parts of VirtualBox, especially libraries, may also be released under other licenses as well.)

[VirtualBox Extensions](https://www.virtualbox.org/wiki/Downloads)
: VirtualBox Extensions is licensed under the [VirtualBox Extension Pack Personal Use and Evaluation License (PUEL)](https://www.virtualbox.org/wiki/VirtualBox_PUEL)

[Vagrant](https://www.vagrantup.com/downloads.html)
: Vagrant is governed by a [EULA](https://www.vagrantup.com/vmware/eula.html)

#### Windows Developer Tools

[WinSCP](https://winscp.net/eng/download.php)
: WinSCP lets you transfer files to and from the Linux-based Chef Infra Server to your Windows workstation. You will use this for managing your keys and configurations between machines.

[Microsoft Azure](https://azure.microsoft.com)

[Microsoft Intune](https://www.microsoft.com/microsoft-365/enterprise-mobility-security/microsoft-intune)
: Microsoft Intune is a cloud-based that you will used to capture and push configuration to devices.

[Windows Autopilot](https://docs.microsoft.com/windows/deployment/windows-autopilot/windows-autopilot)
: Windows Autopilot simplifies the IT-side of managing Windows devices.

[Gorilla](https://github.com/autopkg/autopkg/releases)
: Gorilla is an [open source project](https://github.com/1dustindavis/gorilla). It is a set of tools for managing applications on Windows computers.

[Storage Explorer](https://azure.microsoft.com/features/storage-explorer/)
: Storage Explorer is a free tool from Microsoft for managing your Azure cloud storage resources.

#### macOS Developer Tools

[Apple Enterprise Developer Account](https://developer.apple.com/programs/enterprise/)
: The Apple Developer Enterprise Program is a program that supports large organizations in developing and deploying proprietary, internal-use apps to their employees.

[AutoPkg](https://github.com/autopkg/autopkg/releases/tag/v2.1)
: AutoPkg is an automation framework for macOS software packaging and distribution, oriented towards the tasks one would normally perform manually to prepare third-party software for mass deployment to managed clients.

[Munki](https://www.munki.org/munki/)
: Munki is an [open source project](https://github.com/munki/munki) from Walt Disney Animation Studios. It is a set of tools for managing applications on macOS computers.

[MunkiAdmin](https://github.com/hjuutilainen/munkiadmin/releases/)
: MunkiAdmin is a graphical user interface (GUI) for managing munki repositories

[Storage Explorer](https://azure.microsoft.com/features/storage-explorer/)
: Storage Explorer is a free tool from Microsoft for managing your Azure cloud storage resources.

Next: [Install Chef Components]({{< relref "desktop_setup_chef.md" >}})
Last: [About Chef Desktop]({{< relref "_index.md" >}})
