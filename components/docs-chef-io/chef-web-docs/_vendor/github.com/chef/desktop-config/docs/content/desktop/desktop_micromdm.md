+++
title = "Zero Touch Deployment with MicroMDM for macOS"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Zero Touch Deployment with MicroMDM for macOS"
    identifier = "desktop/desktop_micromdm.md Zero Touch Deployment with MicroMDM for macOS"
    parent = "desktop"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop_micromdm.md)

Info:
The application management documentation for Chef Desktop is under active development.
Check back for upcoming enhancements and improvements.

## Introduction

The Chef Desktop management pattern allows you to manage all your macOS devices
using a MicroMDM server for a fully automated experience.

If you have not done so, read [The Chef Desktop Development Pattern](/desktop/desktop_pattern/)
to familiarize yourself with some of the basic steps for getting started. We will
repeat a number of those steps here.

## Overview

This document describes how to set up the following things:

- Install Chef Infra Server and Chef Automate
- Build your local repo and test Chef Desktop
- Build the InstallApplications package.
- Build the MDM server
- Build and deploy Munki to deploy apps to the laptops

## Setting up the MDM

We need a Mobile Device Management (MDM) service to capture macOS machines as they
boot, and to securely connect and push applications and configuration settings to them.
In this document we use MicroMDM, but there are others on the market like VMware
Airwatch, SimpleMDM, and others.

Your configuration and setup may begin like this:

1. Stand up a new Linux instance in Azure or AWS and ensure that you use SSH keys
   for authentication rather than passwords.

1. Once the instance is running, upgrade the installed packages:

   ```powershell
   sudo apt-get update && sudo apt-get upgrade -y
   sudo reboot
   ```

1. Create a micromdm directory on the drive.

1. Clone MicroMDM from Github:

   ```powershell
   curl -L https://github.com/micromdm/micromdm/releases/download/v1.6.0/micromdm_v1.6.0.zip
   ```

1. Open a terminal window on the Linux box.

   ```powershell
   sudo apt-get install unzip
   unzip micromdm_v1.6.0.zip
   cd /micromdm/build/linux
   ./mdmctl config set -name <some name> -api-token <password> -server-url https://somefqdn
   ```

   Where:

   `-name`
   : is the label you choose to identify your MicroMDM server

   `-api-token`
   : is a password you will use to connect to your server to configure it

   `-server-url`
   : is the publicly accessible IP or FQDN of your created server

1. Start the MDM and note the password you use. You will need this password with `mdmctl` to configure the server.

   ```powershell
   sudo ./micromdm serve -server-url https://somefqdn/ -api-key <password>
   ```

1. There are three important certificates on your MDM server:

   - A TLS capable certificate that allows you to connect to your MDM server via port 443.
   - An APNS certificate, which is a Push certificate, that allows your MDM server to talk to your macOS clients.
   - A DEP certificate that allows your MDM server to talk to deploy.apple.com
     and accept incoming boot requests from Apple's servers.

### Securing the MDM

Read this [guide](https://github.com/micromdm/micromdm/blob/master/docs/user-guide/quickstart.md)
which describes the steps necessary to create both the DEP and APNS certificates
that you need for your MicroMDM server.

## Initial App Deployment to a Node

`InstallApplication` is a stage in the Apple setup process that occurs between
the time the display shows up for the user and when the user reaches the
desktop for the first time.

[InstallApplications](https://github.com/macadmins/installapplications)
(plural) is a corresponding application that will install applications or settings
during that stage of configuration.

1. Start by loading these apps on your macOS machines:

   - The current release of Chef Infra Client
   - DepNotify

     [Depnotify](https://gitlab.com/Mactroll/DEPNotify) is an open source tool that
     tells users how much more time their macOS machine needs to install applications
     before they can starting using it.

   - Caffeinate

     [Caffeinate](https://ss64.com/osx/caffeinate.html) keeps the desktop active
     so that the screensaver does not turn on during installation. The screensaver
     will turn off network connections and break the setup if it activates.

   - [Chef Bootstrap](/install_bootstrap/)

     The last thing we pull down is a python script to configure the `chef/client.rb`
     file and run `chef-client` the first time to configure the laptop to its desired
     state and keep it in that state.

1. Install the latest version of [InstallApplications](https://github.com/macadmins/installapplications).

1. Modify the LaunchDaemon plist to look like the [first example](#example-munki-catalog)
   below. Notice that we updated the JSONUrl and a couple of the identity sections.
   Also notice that we enabled some of the commands needed to properly populate DepNotify so
   it displays useful information to the user.

1. Modify the build-info.json file on the identity line to correctly
   reference your developer certificate. Read the
   [InstallApplications documentation](https://github.com/erikng/installapplications/wiki/Packaging)
   for information about the type of accounts that Apple requires to install packages.

1. Use `munkipkg` to create the actual pkg file. See the [munkipkg documentation](https://github.com/munki/munki-pkg)
   for instructions.

1. Upload the compiled package to your MDM server.

1. Issue the following commands on your MDM:

   ```shell
   ~/mdm/build/linux/mdmctl apply app -pkg ~/Desktop/mdmvid/InstallApplications.pkg -sign "Developer ID Installer: groob (myid)" -upload

   [WARNING] package signing only implemented on macOS. An unsigned macOS package will not install with MDM.
   ```

1. In the example above, the developer ID should match the one you specify in the `build-info` file below.

## Signing Your Apps

Sign and/or notarize everything that the MDM server touches.

## Setting up and Application Share

One of the great things about an automated management setup like this is being
able to actively manage the applications that show up on user's desktops. To handle
that on macOS we use Munki. Our initial goal is to push a couple of required
applications out to our users.

Munki will handle managed installations and uninstallations.
Munki also provides a ready-made application that offers users
unmanaged applications that they can download if they choose.

### Create a CDN to hold the content

To get started, follow [these instructions](https://docs.microsoft.com/azure/cdn/cdn-create-a-storage-account-with-cdn)
for setting up an empty Azure CDN or [these instructions](https://github.com/grahamgilbert/terraform-aws-munki-repo)
for setting up an empty AWS CDN. This empty CDN will host all the application content
that will be deployed for both macOS and Windows devices. However, each OS type
requires a slightly different directory and file structure.

### Create containers in your Storage account

Create a container in the storage account to hold the content for users.
For macOS, all the content, either managed or unmanaged, goes here.

1. Go to your storage account
1. Navigate to Blob Storage -> Containers.
1. Create a container labeled Munki
1. Set the access level to 'Container'

Next, create the basic directory structure in each container that
the app clients expect to see. In the Gorilla container, create folders to match
this structure. We're going to build the files that go in the folders just below.
Build the top level folders for both clients, the child folders are indicated to
give you a reference of how the whole thing looks over time as applications are
added.

Build out a folder structure that looks like this:

```yaml
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

### Create a Catalog and Manifest for your Clients

These steps demonstrate installing Firefox and VS Code on clients to give you an
idea of how to deploy an application on each OS type.

#### Munki Setup

With [Munki](https://github.com/munki/munki), the configuration files are in XML and editing them directly
can produce errors so we recommend using command line tools.

1. Install the Munki tools locally

   [Download](https://github.com/munki/munki/releases) the whole Munki package to the macOS machine and install it.

1. Setup a local File Share on the macOS machine

   Setup a local file share on the macOS machine. Use the tools below to populate it
   with the settings and configuration you need. Follow this [document](https://github.com/munki/munki/wiki/Demonstration-Setup) to setup a local
   repo, use Server Explorer to sync it to Azure - go to "Building a "server" repository"

1. Configure Munki

   Run this command to configure Munki. The repo path must match the one you created
   above. Note that the path must have 3 slashes in it, e.g. "file:///".

   ```bash
   munkiimport --configure
   ```

1. Now import the pkg files. When you import the first package the catalog will build automatically.

   ```bash
   munkiimport - firefox
   munkiimport - VSCode
   ```

1. Next, run the following Autopkg commands to get all the Munki tools pulled in
   for the nodes (laptops) to use

   ```bash
   autopkg repo-add recipes
   autopkg run -k MUNKI_REPO=/Users/Shared/Munki_repo Munkitools4.Munki
   ```

1. After that, run `makecatalogs` to pull the Munki updates in:

   ```bash
   makecatalogs
   ```

1. Finally, run `manifestutil` too create the manifest and pull your apps under
   managed installs. If you run into issues with the tool, use the MunkiAdmin GUI.
   It makes it much easier to see what's going on with the configuration files.

   ```bash
   /usr/local/Munki/manifestutil
   Entering interactive mode... (type "help" for commands)
   > new-manifest site_default
   > add-catalog my_catalog --manifest site_default
   Added testing to catalogs of manifest site_default.
   > add-pkg Firefox --manifest site_default
   Added Firefox to section managed_installs of manifest site_default.
   > add-pkg VSCode --manifest site_default
   Added VSCode to section managed_installs of manifest site_default.
   > add-pkg Munkitools_admin --manifest site_default
   Added Munkitools_admin to section managed_installs of manifest site_default.
   ...
   > exit
   ```

Now you can use Storage Explorer to move the entire thing into your Azure Blob Storage

#### Example Munki Catalog

Below is a section of a Munki catalog. You can manually edit the details if you
need to, but we strongly encourage you to use the tools above to reduce the
chances of introducing an error.

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

Notice that the format for the manifest is similar to what Gorilla uses:

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

Now that the catalog and manifest are ready, test this out from a
macOS node by running the following commands from a terminal window.

Run the first command from the macOS client to verify that the correct configuration
got to that node and then run the second command to actually install the managed applications.

```bash
sudo /usr/local/munki/managedsoftwareupdate --show-config

sudo /usr/local/munki/managedsoftwareupdate
```

## Final Checklist

You are almost ready to start the process. Please ensure that you have completed the following steps:

- [ ] You have setup Apple Business Manager
- [ ] You have imported the serial number for at least one of your macOS machine
- [ ] You have configured your MDM server in Apple Business Manager to accept your devices
- [ ] You have setup and configured the MDM to accept devices from Apple Business Manager
  - [ ] You have all your certificates in place
  - [ ] You have built and uploaded the InstallApplications package and imported it into the MDM
- [ ] If you are going to use Munki, is the S3 bucket correctly configured for it?

## Bootstrapping the first Node

If all goes according to plan, the first node should be ready to restart,
pull down all the packages and/or scripts, load Chef Infra Client, and do the
first client run.
