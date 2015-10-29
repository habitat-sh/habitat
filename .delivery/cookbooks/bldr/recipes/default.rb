#
# Cookbook Name:: bldr
# Recipe:: default
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

docker_kernel = node['kernel']['name']
docker_arch = node['kernel']['machine']
compose_version = '1.4.2'
compose_checksum = 'd5fca08d54f59649b93b66a781b22998955f2bd701244fcfd650c00daa9e948c'
compose_url = "https://github.com/docker/compose/releases/download/#{compose_version}/docker-compose-#{docker_kernel}-#{docker_arch}"

log "debugging" do
  message "I want to know all about node['delivery'], so:\n\n>>>>>>>>>>>>>>>>>>>>>>\n#{node['delivery'].inspect}\n<<<<<<<<<<<<<<<<<<<<<<<\n"
  level :warn
end

# to give us `make` and friends
include_recipe 'build-essential'

docker_service 'default' do
  host 'tcp://0.0.0.0:2376'
  action [:create, :start]
end

execute 'docker info' do
  environment({
    'DOCKER_HOST' => "tcp://#{node['ipaddress']}:2376"
  })
end

remote_file '/usr/bin/docker-compose' do
  source compose_url
  checksum compose_checksum
  owner 'root'
  mode '0755'
end

execute 'docker-compose version'
