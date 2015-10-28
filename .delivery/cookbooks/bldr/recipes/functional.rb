env = { 'DOCKER_HOST' => "tcp://#{node['ipaddress']}:2376" }
workspace = node['delivery']['workspace']['repo']

execute 'make volume-clean all' do
  cwd workspace
  environment env
end

execute "docker ps -a -f 'name=bldr-*'" do
  environment env
end

execute 'make functional' do
  cwd workspace
  environment env
end
