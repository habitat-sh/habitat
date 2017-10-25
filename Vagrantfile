# -*- mode: ruby -*-
# vi: set ft=ruby :

$script = <<SCRIPT
cd /vagrant
sh support/linux/install_dev_0_ubuntu_latest.sh
echo 'eval "$(direnv hook bash)"' >> /home/vagrant/.bashrc
apt-get install -y direnv
sh components/hab/install.sh
SCRIPT

Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-17.04"
  config.vm.provision "shell", inline: $script, privileged: true

  config.vm.synced_folder "~/.hab", "/home/vagrant/.hab", nfs: true, :linux__nfs_options => ["no_root_squash"], :map_uid => 0, :map_gid => 0

  config.vm.network "forwarded_port", guest: 80, host: 9636
  config.vm.network "forwarded_port", guest: 9631, host: 9631
  config.vm.network "forwarded_port", guest: 9638, host: 9638

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
