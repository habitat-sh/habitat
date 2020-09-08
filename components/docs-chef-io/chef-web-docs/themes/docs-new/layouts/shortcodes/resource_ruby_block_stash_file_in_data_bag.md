The following example shows how to use the **ruby_block** resource to
stash a BitTorrent file in a data bag so that it can be distributed to
nodes in the organization.

``` ruby
# the following code sample comes from the ``seed`` recipe
# in the following cookbook: https://github.com/mattray/bittorrent-cookbook

ruby_block 'share the torrent file' do
  block do
    f = File.open(node['bittorrent']['torrent'],'rb')
    #read the .torrent file and base64 encode it
    enc = Base64.encode64(f.read)
    data = {
      'id'=>bittorrent_item_id(node['bittorrent']['file']),
      'seed'=>node.ipaddress,
      'torrent'=>enc
    }
    item = Chef::DataBagItem.new
    item.data_bag('bittorrent')
    item.raw_data = data
    item.save
  end
  action :nothing
  subscribes :create, "bittorrent_torrent[#{node['bittorrent']['torrent']}]", :immediately
end
```