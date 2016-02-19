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

docker_env = {
  'GITHUB_DEPLOY_KEY' => ssh_key,
  'DELIVERY_GIT_SHASUM' => node['delivery']['change']['sha'],
  'DOCKER_TLS_VERIFY' => '1',
  'DOCKER_CERT_PATH' => machine_dir,
  'DOCKER_HOST' => "tcp://#{BldrDockerMachine.machine_ip}:2376",
  'DOCKER_MACHINE_NAME' => 'bldr-docker-machine'
}

# There's a bug in mixlib/shellout or the execute resource where
# stdout/stderr stops getting displayed in Delivery's chef-client run,
# so we send it to a file to inspect later if necessary
log "`make` will log output to #{makelog}"

bash 'make clean package functional' do
  code "make clean package functional force=true 2>&1>#{makelog}"
  cwd node['delivery']['workspace']['repo']
  # set a two hour time out because this compiles :allthethings:
  timeout 7200
  environment(docker_env)
end

log "Docker Machine IP is: #{BldrDockerMachine.machine_ip}"

docker_container 'chef-bldr-web-functional' do
  repo 'chef/bldr-web'
  command 'start chef/bldr-web'
  port '80:80'
  tag 'latest'
  host "tcp://#{BldrDockerMachine.machine_ip}:2376"
  tls_verify true
  tls_ca_cert "#{machine_dir}/ca.pem"
  tls_client_cert "#{machine_dir}/cert.pem"
  tls_client_key "#{machine_dir}/key.pem"
  action :run
  subscribes :redeploy, 'execute[make clean package functional]', :immediately
end
