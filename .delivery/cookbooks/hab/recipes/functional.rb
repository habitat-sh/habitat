#
# Cookbook Name:: hab
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

machine_dir = HabDockerMachine.dbuild_machine_dir
docker_machine_config = HabDockerMachine.load_config
ssh_key = data_bag_item('delivery-secrets', 'chef-bldr-acceptance')['github']

env = {
  'IN_DOCKER' => 'true',
  'GITHUB_DEPLOY_KEY' => ssh_key,
  'DELIVERY_GIT_SHASUM' => node['delivery']['change']['sha'],
  'DOCKER_TLS_VERIFY' => '1',
  'DOCKER_CERT_PATH' => machine_dir,
  'DOCKER_HOST' => "tcp://#{HabDockerMachine.machine_ip}:2376",
  'DOCKER_MACHINE_NAME' => 'bldr-docker-machine'
}

execute 'make distclean' do
  cwd node['delivery']['workspace']['repo']
  environment(env)
  not_if { HabDocker.fresh_image?(HabDocker.devshell_name) }
end

log "TESTS DISABLED UNTIL DEVSHELL CAN BE BUILT CORRECTLY"
return

execute "make functional refresh=true" do
  cwd node['delivery']['workspace']['repo']
  environment(env)
end
