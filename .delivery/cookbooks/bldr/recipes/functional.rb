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

machine_dir = ::File.join(
  Etc.getpwnam('dbuild').dir,
  '.docker/machine/machines',
  'bldr-docker-machine'
)

docker_machine_config = BldrDockerMachine.load_config

execute 'make clean package functional force=true' do
  cwd node['delivery']['workspace']['repo']
  environment(
    'DOCKER_TLS_VERIFY' => '1',
    'DOCKER_CERT_PATH' => machine_dir,
    'DOCKER_HOST' => "tcp://#{BldrDockerMachine.machine_ip}:2376",
    'DOCKER_MACHINE_NAME' => 'bldr-docker-machine'
  )
end
