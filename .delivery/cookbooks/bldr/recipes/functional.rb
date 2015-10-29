workspace = node['delivery']['workspace']['repo']

execute 'make volume-clean all' do
  cwd workspace
end

execute "docker ps -a -f 'name=bldr-*'"

execute 'make functional' do
  cwd workspace
end
