``` ruby
yum_key node['yum']['elrepo']['key'] do
  url  node['yum']['elrepo']['key_url']
  action :add
end

yum_repository 'elrepo' do
  description 'ELRepo.org Community Enterprise Linux Extras Repository'
  key node['yum']['elrepo']['key']
  mirrorlist node['yum']['elrepo']['url']
  includepkgs node['yum']['elrepo']['includepkgs']
  exclude node['yum']['elrepo']['exclude']
  action :create
end
```