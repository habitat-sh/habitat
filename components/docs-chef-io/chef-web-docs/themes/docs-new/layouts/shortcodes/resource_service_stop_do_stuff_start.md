The following example shows how to use the **execute**, **service**, and
**mount** resources together to ensure that a node running on Amazon EC2
is running MySQL. This example does the following:

-   Checks to see if the Amazon EC2 node has MySQL
-   If the node has MySQL, stops MySQL
-   Installs MySQL
-   Mounts the node
-   Restarts MySQL

<!-- -->

``` ruby
# the following code sample comes from the ``server_ec2``
# recipe in the following cookbook:
# https://github.com/chef-cookbooks/mysql

if (node.attribute?('ec2') && ! FileTest.directory?(node['mysql']['ec2_path']))

  service 'mysql' do
    action :stop
  end

  execute 'install-mysql' do
    command "mv #{node['mysql']['data_dir']} #{node['mysql']['ec2_path']}"
    not_if do FileTest.directory?(node['mysql']['ec2_path']) end
  end

  [node['mysql']['ec2_path'], node['mysql']['data_dir']].each do |dir|
    directory dir do
      owner 'mysql'
      group 'mysql'
    end
  end

  mount node['mysql']['data_dir'] do
    device node['mysql']['ec2_path']
    fstype 'none'
    options 'bind,rw'
    action [:mount, :enable]
  end

  service 'mysql' do
    action :start
  end

end
```

where

-   the two **service** resources are used to stop, and then restart the
    MySQL service
-   the **execute** resource is used to install MySQL
-   the **mount** resource is used to mount the node and enable MySQL