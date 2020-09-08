+++
title = "Quick Start Demo"

weight = 10
draft = false

[menu]
  [menu.automate]
    title = "Quick Start Demo"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/_index.md Quick Start Demo"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/_index.md)

Hello and welcome to Chef Automate! This Quickstart guides you through the initial installation and trial activation.

The easiest way to start exploring Chef Automate is to install it on a local virtual machine (VM).
We created this [Vagrant](https://www.vagrantup.com/) file for you to use in creating your own test environment.

## Prerequisites

For this demonstration, you need:

* Administrator (root) access to your system
* A virtualization product such as [VirtualBox](https://www.virtualbox.org/) installed on your system
* [Vagrant](https://www.vagrantup.com/intro/getting-started/) installed on your system
* 4GB RAM, or more, available for the VM

## Quickstart Overview

Use the command line script below to create a demonstration instance of Chef Automate.
The script is mostly automated, but it does require a few manual steps.
By installing Chef Automate with the Vagrantfile provided below you're automatically consenting to Chef Software, Inc.'s [Terms of Service](https://www.chef.io/terms-of-service) and [Master License and Services Agreement](https://www.chef.io/online-master-agreement)

## Create a Vagrantfile

Copy and paste this script into your command line to create the Chef Automate `Vagrantfile`:

```ruby
cat > Vagrantfile <<'EOH'
CFG_BOX      = 'bento/ubuntu-18.04'
CFG_IP       = '192.168.33.199'
CFG_HOSTNAME = 'chef-automate.test'

$deployscript = <<-SCRIPT
export DEBIAN_FRONTEND='noninteractive'
apt-get -qq update
apt-get install -y --no-install-recommends curl unzip
apt-get clean
sysctl -w vm.max_map_count=262144
sysctl -w vm.dirty_expire_centisecs=20000
echo "${CFG_IP} ${CFG_HOSTNAME}" | tee -a /etc/hosts
curl -fsSL https://packages.chef.io/files/current/automate/latest/chef-automate_linux_amd64.zip -o /tmp/chef-automate_linux_amd64.zip
unzip -qod /usr/local/bin /tmp/chef-automate_linux_amd64.zip
chmod +x /usr/local/bin/chef-automate
chef-automate deploy --accept-terms-and-mlsa
echo "Server is up and running. Please log in at https://${CFG_HOSTNAME}/"
echo 'You may log in using credentials provided below:'
cat /home/vagrant/automate-credentials.toml
SCRIPT

Vagrant.configure(2) do |config|
  class AcceptLicense
    def to_s
      return 'true' if ENV['ACCEPT_CHEF_TERMS_AND_MLSA'] == 'true'
      puts <<TERMS
To continue, you'll need to accept our terms of service:
Terms of Service
https://www.chef.io/terms-of-service

Master License and Services Agreement
https://www.chef.io/online-master-agreement

I agree to the Terms of Service and the Master License and Services Agreement (y/n)
TERMS
      if STDIN.gets.chomp == 'y'
        'true'
      else
        puts 'Chef Software Terms of Service and Master License and Services Agreement were not accepted'
        exit 1
      end
    end
  end
  config.vm.box      = CFG_BOX
  config.vm.hostname = CFG_HOSTNAME

  config.vm.provider "virtualbox" do |v|
    v.name       = 'chef-automate'
    v.memory     = 4096
    v.cpus       = 2
    v.customize ['modifyvm', :id, '--audio', 'none']
  end

  config.vm.synced_folder '.', '/opt/a2-testing', create: true
  config.vm.network       'private_network', ip: CFG_IP
  config.vm.provision     'shell', env: {'CFG_IP' => CFG_IP,
                                         'CFG_HOSTNAME' => CFG_HOSTNAME,
                                         'ACCEPT_CHEF_TERMS_AND_MLSA' => AcceptLicense.new}, inline: $deployscript
end
EOH
```

## Add an entry to /etc/hosts

The example Vagrantfile sets the VM hostname to `chef-automate.test`. The installer uses this hostname as the FQDN that Chef Automate listens on.
For security reasons, you must access Chef Automate's web UI using the configured FQDN.
To avoid configuring DNS for your test instance, we recommend adding an entry to your workstation's `/etc/hosts` file.
Use the following command to append the required entry:

```bash
echo 192.168.33.199 chef-automate.test | sudo tee -a /etc/hosts
```

## Start the VM

Now that you have the `Vagrantfile` in your current directory, provision a new VM by running:

```bash
vagrant up
```

If this is your first time provisioning a VM, this operation may take a few minutes to download the base OS image.

When the command completes, you can access the web UI by browsing to `https://chef-automate.test/`.

You can find login credentials by opening the `automate-credentials.toml`, which the installer has written in your current working directory, or alternatively in an output log.

## Start a free 60-day Trial

Retrieving a trial license through Chef Automate requires the Vagrant instance to have internet connectivity (at the time of trial license creation only).

1. Sign into the web UI at `https://chef-automate.test/`. Your credentials are in the automate-credentials.toml file.
1. Follow the instructions in the web UI to obtain a 60-day trial license by entering your name and email address.

## Installing without Vagrant

You can install Chef Automate on any x86_64 Linux instance running CentOS 7.5,
RHEL 7.5, or Ubuntu 16.04 with the following minimum system resources:

* 4GB RAM
* 5 GB free disk space
* 2 CPUs

Follow the [Installation Guide]({{< relref "install.md" >}}) to install Chef Automate on your own resources.
