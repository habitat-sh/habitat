+++
title = "Zero Touch Deployment for macOS and Windows on Azure"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Zero Touch Deployment for macOS and Windows on Azure"
    identifier = "desktop/desktop_azure.md Chef Desktop Demo on Azure"
    parent = "desktop"
    weight = 60
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop_azure.md)

{{< note >}}
The application management documentation for Chef Desktop is under active development.
Check back for upcoming enhancements and improvements.
{{< /note >}}

Chef Software's Desktop pattern and tools expands familiar device management systems with extensive application and configuration capabilities that are designed to scale with automation.

Chef Desktop provides a cookbook with YAML settings that covers the most common (and some less common) device configuration needs. Of course, we continue to support and improve our classic Ruby cookbook customizations for when you need to go beyond the ordinary. The Chef Desktop pattern sets you up to manage applications with a CDN, so even when one thousand devices in your fleet means managing ten thousand applications, your Windows and macOS users can still access them through a unified catalog. Chef Desktop uses Chef Software's leading tools, providing observability, compliance reporting, and an audit trail as part of the process.

This guide covers automating the Chef Desktop pattern to manage your macOS and Windows devices.

## Configuration Overview

A work triangle is in play in Chef Desktop. The first leg is the Chef Infra Server with Chef Automate that holds and applies configurations to your nodes. The second leg is a Developer node running Chef Workstation from which you create and define the policies and settings that the Chef Infra Server metes out. The third leg of the triangle is the list of nodes to which the various polices and settings are applied.

The automation process follows these steps:

1. A node, such as a laptop or desktop, boots up the very first time and checks in with either Apple Business Manager (ABM) or Azure.
1. ABM or Azure reports the device as managed and redirects it to Microsoft Intune for management.
1. Microsoft Intune captures and provisions the device in the time between a user selecting **next** on the sign in screen and the desktop opening.

This guide helps you:

- Build out your Azure Instance
- Build a InstallApplications package to bootstrap macOS
- Build and deploy Munki to deploy apps to the macOS devices
- Write the PowerShell scripts needed to bootstrap Windows
- Build and deploy Gorilla to deploy apps to Windows devices

## Prerequisites

This guide assumes you have completed the previous steps:

1. [Install Chef Components](/desktop/desktop_setup_chef/)
1. [Chef Desktop Cookbook Development Environment](/desktop/desktop_setup_cookbook/)
1. [The Chef Desktop Development Pattern](/desktop/desktop_pattern/)

### Required Software

Integrating Chef Desktop with Microsoft Intune and Windows Autopilot software and subscriptions as well as specific software for macOS and Windows systems.

The solution in this guide requires:

[Microsoft Azure](https://azure.microsoft.com)
: If you do not already have one, sign up for your account at https://azure.microsoft.com

[Microsoft Intune](https://www.microsoft.com/microsoft-365/enterprise-mobility-security/microsoft-intune)
: Microsoft Intune is a cloud-based service that you will use to capture and push configuration to devices.

[Windows Autopilot](https://docs.microsoft.com/windows/deployment/windows-autopilot/windows-autopilot)
: Windows Autopilot simplifies the IT-side of managing Windows devices.

#### macOS

[Apple Enterprise Developer Account](https://developer.apple.com/programs/enterprise/)
: This program allows for macOS devices to be auto-assigned to an MDM service, which then provisions the devices for you. The Apple Developer Enterprise Program is a program that supports large organizations in developing and deploying proprietary, internal-use apps to their employees. You may find mentions of Apple Device Enrollment Program (DEP) elsewhere, which is the predecessor to ABM and used interchangeably with ABM.

[AutoPkg](https://github.com/autopkg/autopkg)
: AutoPkg is an automation framework for macOS software packaging and distribution, oriented towards the tasks one would perform manually to prepare third-party software for mass deployment to managed clients.

[Munki](https://www.munki.org/munki/)
: Munki is an [open source project](https://github.com/munki/munki) from Walt Disney Animation Studios. It is a set of tools for managing applications on macOS computers.

[MunkiAdmin](https://github.com/hjuutilainen/munkiadmin/releases/)
: MunkiAdmin is a graphical user interface (GUI) for managing munki repositories

[Storage Explorer](https://azure.microsoft.com/features/storage-explorer/)
: Storage Explorer is a free tool from Microsoft for managing your Azure cloud storage resources.

#### Windows

[Gorilla](https://github.com/autopkg/autopkg/releases)
: Gorilla is an [open source project](https://github.com/1dustindavis/gorilla). It is a set of tools for managing applications on Windows computers.

[Storage Explorer](https://azure.microsoft.com/features/storage-explorer/)
: Storage Explorer is a free tool from Microsoft for managing your Azure cloud storage resources.

## Setup Azure

### Configure Microsoft Intune and Active Directory

First, read about [setting up Microsoft Intune](https://docs.microsoft.com/mem/intune/fundamentals/setup-steps)

You will want to spend a bit of time going through that document to get your Microsoft Intune and Azure Active Directory instances configured.

### Register Devices in Azure

The fastest and best way to set up a macOS device is by never physically touching it.

In this case, you will sign on to Apple Business Manager and use the article above to set up Azure as the MDM which receives the laptops and other devices you wish to manage.

#### Windows Options

First, read about [getting started with Windows Autopilot](https://docs.microsoft.com/en-us/mem/intune/enrollment/enrollment-autopilot)

For Windows devices, Azure acts as both the enrollment service and the MDM at the same time. Also, current iterations of Windows 10 will bootstrap from Azure when the correct settings from a laptop are captured.

To make this easier for your workflow, look for and implement Dynamic Device Groups and use these to capture both macOS and Windows devices. These can then be auto-assigned profiles and configuration policies.

#### OS X Options

[OSX Auto Enrollment](https://docs.microsoft.com/en-us/mem/intune/enrollment/device-enrollment-program-enroll-ios)

This article walks you through the configuration of Windows Autopilot and Microsoft Intune to ingest macOS devices as they boot the first time.

### Create Device Groups

You will need to create two device groups for macOS devices and two for Windows (a total of four device groups). You will use the first device group to apply the initial enrollment profile. The second device group is for applying, updating, and removing packages, scripts, etc. on the devices.

### Create Enrollment/Deployment Profiles

Now that you have your systems listed in Azure, we need to get them under an enrollment or deployment profile. This initial configuration of the device will bring it under management.

First, read about [getting started with Microsoft Intune](https://docs.microsoft.com/mem/intune/enrollment/enrollment-autopilot)

### Configuration Scripts

For Windows devices, develop your PowerShell configuration scripts to create and populate the `validation.pem` file and the `client.rb` file in the `c:\chef` directory. You will also want to install the Chef Infra Client.

### Registering the Devices with Chef Infra Server

The `knife` command-line tool provides an interface for interacting with a Chef Infra Server from a local workstation. You'll use two `knife` commands from your developer workstation to the Chef Infra Server. The first command creates the node on the Chef Infra Server, and the second assigns a Chef policy to that node.

```powershell
# knife node policy set SERIAL_NUMBER_OR_FQDN 'NODE_GROUP' 'POLICYFILE'

knife node create S90T7HK2
Created node [S90T7HK2]
knife node policy set S90T7HK2 'Windows_Node_Policy_Group' 'ChefDesktop'
Successfully set the policy on node S90T7HK2
```

### Re-Registering the Devices with Chef Infra Server

During testing--and in any other time that you change the contents of a device--you may need to re-register it. Follow these steps to re-register a device:

```powershell
# Chef Infra Server distinguishes between a Node object and a Client object
knife node delete S90T7HK2
Deleted node [S90T7HK2]
knife client delete S90T7HK2
Deleted client [S90T7HK2]
knife node create S90T7HK2
Created node [S90T7HK2]
knife node policy set S90T7HK2 'Windows_Node_Policy_Group' 'desktop-config'
Successfully set the policy on node S90T7HK2
```

## Setup Applications Management

One of the great things about an automated management setup like this is the ability to actively manage the applications that show up on users' desktops. To handle applications on macOS, we use Munki. To handle applications for Windows, we use Gorilla. Our initial goal is to push a couple of required Apps out to our users.

The two apps will handle both managed installations and managed uninstallations. Also, Munki for macOS provides a ready-made application can display to users and offer them unmanaged apps. For Windows users, we will publish unmanaged apps via the private Microsoft Store.

### Create a CDN

To get started, follow this document to set up an Azure Content Delivery Network (CDN). After setting up, you will have an empty CDN. Use this empty CDN to host all the App content we want to deploy for both our Macs and Windows devices. We will need to set up slightly different directory and file structures for each OS type.

[Create an Azure CDN](https://docs.microsoft.com/azure/cdn/cdn-create-a-storage-account-with-cdn)

### Create Containers in your Storage Account

Now, create two containers in your storage account to hold the content for our users. For macOS devices, all content we want them to have - either 'managed' (we push it to them) or 'unmanaged' (users can download if they wish) goes here. For Windows users, only the managed content goes in the corresponding bucket and we will use the App Store to make things we license available to them.

1. Go to your storage account
1. Navigate to Blob Storage > Containers
1. Create 2 containers with one labeled "Munki" and the other labeled "Gorilla"
1. Set their access level to 'Container'

The next step is to create the basic directory structure in each container that the app clients expect to see. In the Gorilla container, create folders to match this structure. We are going to build the files that go in the folders just below. Just build the top level folders for both clients, the child folders are indicated in the example below to give you a reference of how the file structure will look over time as apps are added.

```shell
[Gorilla web root]
├── manifests
│   ├── *.yaml
├── catalogs
│   ├── *.yaml
└── packages
    ├── Firefox
    ├── Chrome
    ├── *.nupkg
    ├── *.msi
    ├── *.exe
    └── *.ps1
```

Now for your Munki container, build out a folder structure that looks like this:

```shell
[Munki web root]
├── catalogs
│   ├── *.yaml
├── manifests
│   ├── *.yaml
├── icons
│   ├── *.ico
├── pkgsinfo
│   ├── *.*
└── pkgs
    ├── *.pkg
```

## macOS (Munki) Catalog and Manifest

Spend some time thinking about and planning what you ultimately want to put into your manifests and catalogs that your users can access. For simplicity, we will demonstrate installing Firefox and VS Code on clients to give you an idea of how to deploy for each OS type.

### Install Munki

We are going to take a different approach with Munki because the configuration files are in XML and editing them directly can be fraught with peril so we are going to rely on using command line tools for the most part.

Get started by reviewing the [Munki documentation](https://github.com/munki/munki).

Next, [download Munki](https://github.com/munki/munki/releases) to your macOS and install it.

#### Set up a local File Share on your Mac

We are going to set up a local file share on your Mac. We will use the tools below to populate it with the settings and configuration we need. Follow this doc to set up a local repo, we will use Server Explorer to sync it to Azure - go to "Building a "server" repository"

For guidance, follow the [Munki setup demonstration](https://github.com/munki/munki/wiki/Demonstration-Setup).

### Configure Munki

Run this command to configure Munki. The repo path must match the one you created just above. Note that the path must have 3 slashes in it `file:///`

```bash
munkiimport --configure
```

Now you can import pkg files. When you import the first package, the catalog gets built for you automatically.

```bash
munkiimport - firefox
munkiimport - VSCode
```

Next, run the following `autopkg` commands to get all the Munki tools pulled in for the nodes (laptops) to use.`

```bash
autopkg repo-add recipes
autopkg run -k MUNKI_REPO=/Users/Shared/munki_repo munkitools4.munki
```

After that, run `makecatalogs` to pull the Munki updates in.

```bash
makecatalogs
```

Finally, run `manifestutil` to create the manifest and pull your apps under managed installs. If you run into issues with the tool, you can flip over to the MunkiAdmin GUI. The MunkiAdmin GUI makes it much easier to see what is going on with the configuration files.

```bash
/usr/local/munki/manifestutil
Entering interactive mode... (type "help" for commands)
> new-manifest site_default
> add-catalog my_catalog --manifest site_default
Added testing to catalogs of manifest site_default.
> add-pkg Firefox --manifest site_default
Added Firefox to section managed_installs of manifest site_default.
> add-pkg VSCode --manifest site_default
Added VSCode to section managed_installs of manifest site_default.
> add-pkg munkitools_admin --manifest site_default
Added munkitools_admin to section managed_installs of manifest site_default.
...
> exit
```

Now you can use Storage Explorer to move the entire thing into your Azure Blob Storage.

#### Example Munki Catalog

Below is a section of a Munki catalog. You CAN edit the details if needed, but using the tools above is strongly encouraged to reduce the probability of introducing an error.

```xml
<plist version="1.0">
    <array>
        <dict>
            <key>autoremove</key>
            <false/>
            <key>catalogs</key>
            <array>
              <string>mycatalog</string>
            </array>
            <key>display_name</key>
            <string>VSCode</string>
            <key>installed_size</key>
            <integer>1285143</integer>
            <key>installer_item_hash</key>
            <string>
              b9a5b90ff2b0bb733a9b719fe2afea5d5dc02875dc96b969a9fcf8b9de9214a6
            </string>
            <key>installer_item_location</key>
            <string>VSCode.pkg</string>
            <key>installer_item_size</key>
            <integer>513821</integer>
            <key>minimum_os_version</key>
            <string>10.5.0</string>
            <key>name</key>
            <string>VSCode</string>
            <key>receipts</key>
            <array>
                <dict>
                    <key>installed_size</key>
                    <integer>1285143</integer>
                    <key>packageid</key>
                    <string>com.microsoft.visual-studio</string>
                    <key>version</key>
                    <string>8.5.2</string>
                </dict>
            </array>
            <key>unattended_install</key>
            <true/>
            <key>uninstall_method</key>
            <string>removepackages</string>
            <key>uninstallable</key>
            <true/>
            <key>version</key>
            <string>1.45.1</string>
            </dict>
```

#### Example Munki Manifest

Notice that the format for the Munki manifest is similar to what Gorilla uses:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
        <key>catalogs</key>
        <array>
                <string>my_catalog</string>
        </array>
        <key>included_manifests</key>
        <array>
        </array>
        <key>managed_installs</key>
        <array>
                <string>munkitools_app</string>
                <string>munkitools_app_usage</string>
                <string>munkitools_core</string>
                <string>munkitools_launchd</string>
                <string>Firefox</string>
                <string>VSCode</string>
        </array>
        <key>managed_uninstalls</key>
        <array>
        </array>
        <key>managed_updates</key>
        <array>
        </array>
        <key>optional_installs</key>
        <array>
        </array>
</dict>
</plist>
```

You can also [use AWS for your CDN](https://github.com/grahamgilbert/terraform-aws-munki-repo).

Once you have your catalog and manifest ready, you can test this out from a macOS node by running the following commands from a terminal window. Run the first command from your macOS client to verify that the correct configuration got to that node and then run the second command to actually install the managed applications

```bash
sudo /usr/local/munki/managedsoftwareupdate --show-config

sudo /usr/local/munki/managedsoftwareupdate
```

## Windows (Gorilla) Catalog and Manifest

Spend some time thinking about and planning what you ultimately want to put into your manifests and catalogs that your users can access. For simplicity, we will demonstrate installing Firefox and VS Code on clients to give you an idea of how to deploy for each OS type.

### How Gorilla Operates

Save the `config.yaml` file to the `/files` directory of your Chef Desktop cookbook. Configuring the app resource in the Windows recipe file deploys the `config.yaml` setting to your Windows 10 desktops.  The next time Gorilla checks in with your CDN, it pull down the manifest, parses the applications and catalog listings for loading, and finally loads the applications by first loading the catalog(s) and then attempting to load the apps from the catalog(s). To see this in action, use `gorilla -d` from the command line to get the debug output.

[Gorilla Documentation](https://github.com/1dustindavis/gorilla)

#### Sample Gorilla catalog.yaml

```yaml
---
Chocolatey:
  display_name: Chocolatey
  check:
    file:
      - path: C:\ProgramData\chocolatey\choco.exe
        version: 0.10.15
  installer:
    hash: 0C1282378641E03564844D04881209AA946D7D2475049BE32B3151BD68F2758F
    location: packages/chocolatey/chocolatey_installer-1.0.ps1
    type: ps1
  version: 1.0

ChocolateyCoreExtension:
  dependencies:
    - Chocolatey
  display_name: Chocolatey Core Extension
  check:
    file:
      - path: C:\ProgramData\chocolatey\extensions\chocolatey-core\chocolatey-core.psm1
        hash: 376E6EDA567DDDD6AA70CFC9EC5380CE0EB1383BE83C2FBDC87F6FC79252E4E8
  installer:
    hash: 5ECEF3B776508CEBC4B52E9AC7F04D213C2045A6765F12E17545A5FBE2F41928
    location: packages/chocolatey/extensions/core/chocolatey-core.extension.1.3.5.1.nupkg
    type: nupkg
  version: 1.3.5.1

FireFox:
  display_name: Firefox
  check:
    file:
      - path: C:\Program Files\Mozilla Firefox\firefox.exe
        version: 75.0
  installer:
    hash: FF029F6E59D9D92D3AC5F8E837C973B641B3400980624D3A830DCFE55D4C71FC
    location: packages/firefox/Firefox Setup 75.0.exe
    arguments:
      - /S
      - /INI=c:\gorilla\cache\install.ini
    type: exe
  version: 75.0

VSCode:
  display_name: VSCode
  check:
    file:
      - path: C:\Program Files\Microsoft VS Code\Code.exe
        version: 1.45.1
  installer:
    location: packages/vscode/VSCodeSetup-x64-1.45.1.exe
    hash: E9E107CF53F8F06688C881E4616BD9A8553D012A657389399827E0EC2155633C
    arguments:
     - /VERYSILENT
     - /MERGETASKS=!runcode
    type: exe
  uninstaller:
    location: packages/vscode/VSCodeSetup-x64-1.45.1.exe
    hash: E9E107CF53F8F06688C881E4616BD9A8553D012A657389399827E0EC2155633C
    type: exe
  version: 1.45.1
```

#### Sample Gorilla Manifest.yaml

```yaml
---
name: my_manifest
managed_installs:
  - Chocolatey
  - ChocolateyCoreExtension
  - VSCode
  - Firefox

managed_uninstalls:

managed_updates:

included_manifests:

catalogs:
 - my_catalog
```

#### Sample Gorilla config.yaml file

```yaml
---
url: https://<your_cdn_name>.blob.core.windows.net/gorilla/
manifest: my_manifest
catalogs:
  - my_catalog
app_data_path: C:/gorilla/cache
```

### Configuring Apps in a Private Store for Windows

We talked about a second way to get apps to your users in Windows. That method involves you licensing apps and then letting your users have them through your Private Microsoft Store. Follow the directions here to make that work for you.

[Set up a Private Store in Azure](https://docs.microsoft.com/en-us/microsoft-store/distribute-apps-from-your-private-store)

## Final Checklist

You are almost ready to start the process. Please ensure that you have completed the following steps:

- [ ] You have set up Apple Business Manager
- [ ] You have imported the serial number for at least one of your macOS devices
- [ ] You have configured your MDM in Apple Business Manager to accept your devices
- [ ] You have set up and configured the MDM to accept devices from Apple Business Manager
  - [ ] You have all your certificates in place
  - [ ] You have built and uploaded the InstallApplications package and imported it into the MDM
- [ ] If you are going to use Munki, confirm if the S3 bucket is correctly configured for it.

If all goes according to plan, then you should be ready to reset/restart your first node, have it pull down all the packages and/or scripts, load Chef Infra Client, and do the first client run.

- Next: [Zero Touch Deployment with MicroMDM for macOS]({{< relref "desktop_micromdm.md" >}})
- Last: [The Chef Desktop Development Pattern]({{< relref "desktop_pattern.md" >}})
