workspace = node['delivery']['workspace']['repo']

execute 'make clean all force=true' do
  cwd workspace
end

execute "docker ps -a -f 'name=bldr-*'"

execute 'make functional' do
  cwd workspace
end
