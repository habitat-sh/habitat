#
# Cookbook Name:: hab
# Recipe:: _docker_machine
#
# Copyright 2015 Chef Software, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

if node.attribute?('delivery')
  machine_dir = HabDockerMachine.dbuild_machine_dir
  load_delivery_chef_config
else
  chef_dir = ENV['CHEF_DIR'] || '.chef'

  unless ::File.exist?(chef_dir)
    Chef::Log.error("Could not find chef configuration directory, tried ENV['CHEF_DIR'] and '.chef'")
  end

  Chef::Config.from_file(::File.join(chef_dir, 'config.rb'))
  Chef::Config.trusted_certs_dir(::File.join(chef_dir, 'trusted_certs'))
  Chef::Config[:solo] = false
  machine_dir = "#{ENV['HOME']}/.docker/machine/machines/bldr-docker-machine"
end

begin
  docker_machine_config = data_bag_item('bldr-acceptance', 'bldr-docker-machine')
rescue Net::HTTPServerException
  Chef::Log.warn("Could not load data bag item 'bldr-docker-machine' in '#{cookbook_name}::#{recipe_name}'")
  Chef::Log.warn("Leaving '#{cookbook_name}::#{recipe_name}', acceptance docker machine will be done later")

  return
end

directory machine_dir do
  recursive true
end

%w(ca.pem server.pem server-key.pem cert.pem key.pem).each do |keyfile|
  file ::File.join(machine_dir, keyfile) do
    content docker_machine_config[keyfile.gsub(/[\.-]/, '_')]
    sensitive true
  end
end

file ::File.join(machine_dir, 'id_rsa') do
  content docker_machine_config['id_rsa_private']
  sensitive true
end

file ::File.join(machine_dir, 'id_rsa.pub') do
  content docker_machine_config['id_rsa_public']
  sensitive true
end

file ::File.join(machine_dir, 'config.json') do
  content ::JSON.pretty_generate(docker_machine_config['config'])
  sensitive true
end

ruby_block 'docker-env' do
  block do
    puts <<-EOH.gsub(/^\s+/, '')
      # Connect to the docker machine by exporting the following to your shell:
      export DOCKER_TLS_VERIFY=1
      export DOCKER_CERT_PATH=#{machine_dir}
      export DOCKER_HOST=tcp://#{docker_machine_config['config']['IPAddress']}:2376
      export DOCKER_MACHINE_NAME=bldr-docker-machine
    EOH
  end
end
