#
# Cookbook Name:: bldr
# Recipe:: functional
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

load_delivery_chef_config

machine_dir = BldrDockerMachine.dbuild_machine_dir
docker_machine_config = BldrDockerMachine.load_config

ssh_key = data_bag_item('delivery-secrets', 'chef-bldr-acceptance')['github']

makelog = ::File.join(Chef::Config[:file_cache_path],
                      'make-functional.out')

# warn level because we use doc formatter and this won't be displayed
# otherwise :)
Chef::Log.warn("`make` will log output to #{makelog}")

execute "make distclean functional force=true 2>&1 | tee #{makelog}" do
  cwd node['delivery']['workspace']['repo']
  # set a two hour time out because this compiles :allthethings:
  timeout 7200
  environment(
    'IN_DOCKER' => 'true',
    'GITHUB_DEPLOY_KEY' => ssh_key,
    'DELIVERY_GIT_SHASUM' => node['delivery']['change']['sha'],
    'DOCKER_TLS_VERIFY' => '1',
    'DOCKER_CERT_PATH' => machine_dir,
    'DOCKER_HOST' => "tcp://#{BldrDockerMachine.machine_ip}:2376",
    'DOCKER_MACHINE_NAME' => 'bldr-docker-machine'
  )
end
