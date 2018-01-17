# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-17.10"
  config.vm.provision "shell", path: "./support/linux/provision.sh", privileged: true

  config.vm.synced_folder ".", "/src"
  config.vm.synced_folder "~/.hab/cache/keys", "/hab/cache/keys"
  config.vm.synced_folder "~/.hab/etc", "/hab/etc"

  config.vm.network "forwarded_port", guest: 80, host: 9636
  config.vm.network "forwarded_port", guest: 9631, host: 9631
  config.vm.network "forwarded_port", guest: 9636, host: 9636

  config.vm.provider "virtualbox" do |v|
    v.memory = 4096
    v.cpus = 4
  end

  config.vm.provider "vmware_fusion" do |v|
    v.vmx["memsize"] = "4096"
    v.vmx["numvcpus"] = "4"
  end

  config.vm.provider "hyperv" do |hv, override|
    override.vm.box = "ericmann/trusty64"
    hv.ip_address_timeout = 240
    hv.memory = 4096
    hv.cpus = 4
  end
end
