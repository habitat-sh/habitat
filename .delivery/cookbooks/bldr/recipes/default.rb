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

include_recipe 'bldr::_docker_machine'
