# -*- mode: ruby -*-
# vi: set ft=ruby :

$script = <<SCRIPT
cd /vagrant
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_latest.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
SCRIPT

Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-16.04"
  config.vm.provision "shell", inline: $script, privileged: true

  # For builder-api
  config.vm.network "forwarded_port", guest: 9636, host: 9636, auto_correct: true

  # For builder-web
  config.vm.network "forwarded_port", guest: 3000, host: 3000, auto_correct: true

  config.vm.provider "virtualbox" do |v|
    v.memory = 2048
    v.cpus = 2
  end

  config.vm.provider "hyperv" do |hv, override|
    override.vm.box = "ericmann/trusty64"
    hv.ip_address_timeout = 240
    hv.memory = 2048
    hv.cpus = 2
  end
end
