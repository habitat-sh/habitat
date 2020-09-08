+++
title = "Install Chef Components"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Install Chef Components"
    identifier = "desktop/desktop_setup_chef.md Required Chef Setup"
    parent = "desktop"
    weight = 30
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop_setup_chef.md)

*Estimated time: 2 hours*

In this step, you will install and set up all of the Chef Software components that you need for the Chef Desktop cookbook.

1. Install Chef Automate and Chef Infra Server
   - Set Up Chef Infra Server
1. Install Chef Workstation
   - Set up the `.chef` directory for storing Chef Infra keys and configurations

Chef Infra
: Chef Infra is a powerful automation platform that transforms infrastructure into code. Chef Infra automates how infrastructure is configured, deployed, and managed across your network, no matter its size.

Chef Workstation
: Chef Workstation gives you everything you need to get started with Chef. Start scanning and configuring your environments today with Chef InSpec and chef-run. Chef Workstation runs on the computer you use everyday, whether it's Linux, macOS, or Windows.
Chef Workstation ships with Chef Infra Client, Chef InSpec, Chef CLI, Test Kitchen, Cookstyle, and several other useful Chef tools. With this collection of programs and tools, you can make sure your Chef Infra code does what you intended before you deploy it to environments used by others.

Chef InSpec
: Chef InSpec is a testing framework with a human- and machine-readable language for specifying compliance, security and policy requirements. When compliance is expressed as code, you can integrate it into your deployment pipeline and automatically test for adherence to security policies.

## Install Chef Automate and Chef Infra Server

We recommend starting with a clean installation of Chef Automate, Chef Infra Server, and Chef Desktop. Combining existing Chef Automate nodes with new Chef Desktop leads to significant confusion, because the servers do not distinguish between the types of nodes that they mange.

For the purposes of this demonstration, we recommend installing Chef Automate and Chef Infra Server together on the same host. Talk to your account representative to decide if you should follow a different deployment pattern in production.

### Prepare the System

In your clean Linux environment, update and install the system packages:

```bash
apt-get -qq update
apt-get install -y --no-install-recommends
apt-get clean
```

The Chef Automate installation requires the following settings:

```bash
sysctl -w vm.max_map_count=262144
sysctl -w vm.dirty_expire_centisecs=20000
```

### Chef Product Download and Install

Download the Chef Automate CLI:

```bash
curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64zip | gunzip - > chef-automate && chmod +x chef-automate
```

Deploy the packages with the following command:

```bash
sudo chef-automate deploy --product automate --product chef-server --product desktop
```

*You will need the host names and user information later in this guide. They are located them in the `automate-credentials.toml`.*

### Chef Infra Server Setup

1. SSH into Chef Automate using the user name from your local workstation `ssh <workstation_user_name>@<chef-automate.test>`

1. [Set Up Chef Infra Server](https://automate.chef.io/docs/infra-server/#use-knife-with-chef-infra-server).

   After you have created your Chef Infra Server with Chef Automate, connect to the Chef Infra Server instance and run the following commands to create your first and user and organization. You need run these commands as an administrator or use `sudo` before each command.

   *You will need information from both of these commands. Copy them from your command line to a file. Store the file securely.*

    ```powershell
    chef-server-ctl user-create USER_NAME FIRST_NAME LAST_NAME EMAIL 'PASSWORD' --filename USER_NAME.pem
    ```

   Create an organization:

   ```powershell
   chef-server-ctl org-create SHORT_NAME 'FULL_ORGANIZATION_NAME' --association_user USER_NAME --filename ORGANIZATION-validator.pem
   ```

## Install Chef Workstation

[Download Chef Workstation](https://downloads.chef.io/chef-workstation) and run the installer on the developer machine. Accept the defaults.

### Workstation Environment Setup

Create a `.chef` folder for storing your configuration and keys. Creating the `.chef` directory it in your "home" or "root" folder makes those settings globally available. Navigate to the root directory and create a `.chef` directory:

  For Powershell:

  ```powershell
  Set-Location -Path C:\Users\<user_name>
  New-Item -Path . -Name ".chef" -ItemType "directory"
  ```

  For macOS:

  ```bash
  cd ~
  mkdir .chef
  ```

## Key Management

"Key management" is a software term that means "Safely and securely getting the right credentials from remote and local computers into the right directories--usually, but not always, on your local computer--in order to use software to run commands between computers".

We recommend using a secure copy protocol (SCP) to move the public key and configuration file from the Chef Infra Server to the `.chef` directory on your workstation.

- macOS workstations should have the `scp` command,
- Windows workstations will need to install [WinSCP](https://winscp.net/eng/index.php) or another similar tool.

  - The public key is `ORGANIZATION-validator.pem`
  - The configuration file is `config.toml`

*This step is the reason that you copied output of the above commands to files.* To manage your keys and credentials, you need to know:

- The host name (also called a FQDN) or ip of the Chef Infra Server
- The user name on the Chef Infra Server
- The password on the Chef Infra Server

### Transfer Keys on Windows Workstations

1. Install WinSCP
1. Open the program using the icon on your workstation desktop.
1. Select **SCP** as the file protocol
1. Set port **22**
1. Fill in the host name, the user name, and the password that you created on your Chef Infra Server.

### Transfer Keys on macOS Workstations

macOS systems come with `scp` installed. Download the key and configuration files:

```bash
scp user_name@chef-automate.test:/remote/ORGANIZATION-validator.pem ~/.chef
scp user_name@chef-automate.test:/remote/config.toml ~/.chef
```

### More Information on SCP

[SCP for macOS](https://linuxize.com/post/how-to-use-scp-command-to-securely-transfer-files/)
: SCP is used to move files between your workstation to a remote computer. It uses ssh for data transfer. `scp` asks for passwords or passphrases if needed for authentication.

[WinSCP for Windows](https://winscp.net/index.php)
: WinSCP is an open source application for Windows used to move files between your workstation and a remote computer. WinSCP offers scripting and basic file manager functionality. The download page has many junk buttons, make sure you select **Download WinSPC**.

[SCP on Azure](https://docs.microsoft.com/azure/virtual-machines/linux/copy-files-to-linux-vm-using-scp)
: SCP on Azure is used to move files from your workstation up to an Azure Linux VM, or from an Azure Linux VM down to your workstation.

- Next: [Chef Desktop Cookbook Development Environment]({{< relref "desktop_setup_cookbook.md" >}})
- Last: [Chef Desktop Requirements]({{< relref "desktop_requirements.md" >}})
