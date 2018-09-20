# -*- mode: ruby -*-
# vi: set ft=ruby :
Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-18.04"

  config.vm.provider "virtualbox" do |vb|
    vb.memory = "4096"
    vb.cpus = "4"
  end

  config.vm.provider "vmware_fusion" do |v|
    v.vmx["memsize"] = "4096"
    v.vmx["numvcpus"] = "4"
  end

  config.vm.provision "file", source: "components/hab/install.sh", destination: "/tmp/install.sh"
  config.vm.provision "shell", path: "support/linux/install_dev_0_ubuntu_latest.sh"
  config.vm.provision "shell", path: "support/linux/install_dev_9_linux.sh"
end
