#
# Cookbook Name:: bldr
# Recipe:: default
#
# Copyright 2015-2016 Chef Software, Inc.
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

docker_kernel = node['kernel']['name']
docker_arch = node['kernel']['machine']
compose_version = '1.5.0'
compose_checksum = 'f920ae9e3907b5007d3d833f1d369f908eeeabf31f292130636102b0c9b6ddf1'
compose_url = "https://github.com/docker/compose/releases/download/#{compose_version}/docker-compose-#{docker_kernel}-#{docker_arch}"

# to give us `make` and friends
include_recipe 'build-essential'

docker_service 'default' do
  host 'unix:///var/run/docker.sock'
  action [:create, :start]
end

execute 'docker info'

remote_file '/usr/bin/docker-compose' do
  source compose_url
  checksum compose_checksum
  owner 'root'
  mode '0755'
end

execute 'docker-compose version'

# add build user to the docker group to access the domain socket
group 'docker' do
  append true
  members Array(node['delivery_builder']['build_user'])
end

###############################################
### Build a docker machine worthy of Mordor ###
###         (or, acceptance...)             ###
###############################################
load_delivery_chef_config

creds = data_bag_item(
  'delivery-secrets',
  'chef-bldr-acceptance'
)

ENV['AWS_ACCESS_KEY_ID']     = creds['aws_access_key_id']
ENV['AWS_SECRET_ACCESS_KEY'] = creds['aws_secret_access_key']

remote_file '/usr/local/bin/docker-machine' do
  source 'https://github.com/docker/machine/releases/download/v0.5.3/docker-machine_linux-amd64'
  mode 00755
end

execute 'bldr-docker-machine' do
  command <<-EOH.gsub(/^\s+/, '')
    docker-machine create -d amazonec2 \
      --amazonec2-vpc-id vpc-2229ff47 \
      --amazonec2-region us-west-2 \
      --amazonec2-instance-type m3.medium \
      --amazonec2-private-address-only \
      bldr-docker-machine
  EOH
  sensitive true
  environment('AWS_ACCESS_KEY_ID'     => creds['aws_access_key_id'],
              'AWS_SECRET_ACCESS_KEY' => creds['aws_secret_access_key'])
  not_if { BldrDockerMachine.available? }
end

# don't look too closely at this code.
ruby_block 'configure-docker-machine-if-exists' do
  block do
    require 'etc'

    machine_home = ::File.join(Etc.getpwnam('dbuild').dir,
                               '.docker/machine/machines',
                               'bldr-docker-machine')

    db = data_bag_item('bldr-acceptance', 'bldr-docker-machine')

    docker_dir = Chef::Resource::Directory.new(machine_home, run_context)
    docker_dir.recursive(true)
    docker_dir.run_action(:create)

    %w(ca.pem server.pem server-key.pem).each do |keyfile|
      f = Chef::Resource::File.new(::File.join(machine_home, keyfile), run_context)
      f.content(db[keyfile.gsub(/[\.-]/, '_')])
      f.run_action(:create)
    end

    private_key = Chef::Resource::File.new(::File.join(machine_home, 'id_rsa'), run_context)
    private_key.content(db['id_rsa_private'])
    private_key.sensitive(true)
    private_key.run_action(:create)

    public_key = Chef::Resource::File.new(::File.join(machine_home, 'id_rsa.pub'), run_context)
    public_key.content(db['id_rsa_public'])
    public_key.sensitive(true)
    public_key.run_action(:create)

    client_cert = Chef::Resource::File.new(::File.join(machine_home, 'cert.pem'), run_context)
    client_cert.content(db['cert_pem'])
    client_cert.sensitive(true)
    client_cert.run_action(:create)

    client_key = Chef::Resource::File.new(::File.join(machine_home, 'key.pem'), run_context)
    client_key.content(db['key_pem'])
    client_key.sensitive(true)
    client_key.run_action(:create)

    puts <<-EOH.gsub(/^\s+/, '')
      # Connect to the docker machine by exporting the following:
      export DOCKER_TLS_VERIFY=1
      export DOCKER_HOST=tcp://#{db['config']['IPAddress']}:2376
      export DOCKER_CERT_PATH=#{ENV['HOME']}/.docker/machine/machines/bldr-docker-machine
      export DOCKER_MACHINE_NAME=bldr-docker-machine
    EOH
  end

  only_if do
    begin
      data_bag_item('bldr-acceptance', 'bldr-docker-machine')
    rescue Net::HTTPServerException
      false
    end
  end
end

chef_gem 'aws-sdk-v1'

ruby_block 'save-docker-machine-state' do
  block do
    # We're up to hijinks because we don't have a "real" resource for
    # managing a docker-machine instance.
    #
    # TODO: (jtimberman) Make a Chef resource for managing a
    # docker-machine instance?
    machine_home   = BldrDockerMachine.machine_home
    config         = BldrDockerMachine.load_config
    ca_pem         = IO.read(File.join(machine_home, 'ca.pem'))
    server_pem     = IO.read(File.join(machine_home, 'server.pem'))
    server_key_pem = IO.read(File.join(machine_home, 'server-key.pem'))
    cert_pem       = IO.read(File.join(machine_home, 'cert.pem'))
    key_pem        = IO.read(File.join(machine_home, 'key.pem'))
    id_rsa_public  = IO.read(File.join(machine_home, 'id_rsa.pub'))
    id_rsa_private = IO.read(File.join(machine_home, 'id_rsa'))

    dm_bag = Chef::DataBagItem.new
    dm_bag.data_bag('bldr-acceptance')
    dm_bag.raw_data = {
      'id'             => 'bldr-docker-machine',
      'ca_pem'         => ca_pem,
      'server_pem'     => server_pem,
      'server_key_pem' => server_key_pem,
      'cert_pem'       => cert_pem,
      'key_pem'        => key_pem,
      'id_rsa_public'  => id_rsa_public,
      'id_rsa_private' => id_rsa_private,
      'config'         => {
        'InstanceId'   => config['Driver']['InstanceId'],
        'IPAddress'    => config['Driver']['IPAddress'],
        'MachineName'  => config['Driver']['MachineName'],
        'SSHUser'      => config['Driver']['SSHUser'],
        'DriverName'   => config['DriverName'],
        'ConfigVersion' => config['ConfigVersion']
      }
    }

    dm_bag.save
  end

  action :nothing
  subscribes :create, 'execute[bldr-docker-machine]'
end

ruby_block 'tag-docker-machine' do
  block do
    config = BldrDockerMachine.load_config
    instance_id = config['Driver']['InstanceId']

    require 'aws-sdk-v1'

    ec2 = AWS::EC2.new(region: config['Driver']['Region'])
    instance = ec2.instances[instance_id]
    instance.tags['X-Project'] = 'bldr'
  end

  action :nothing
  subscribes :create, 'execute[bldr-docker-machine]'
end
