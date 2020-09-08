As a root user, install the Chef Infra Server package on the server,
using the name of the package provided by Chef. For Red Hat Enterprise
Linux and CentOS:

``` bash
sudo rpm -Uvh /tmp/chef-server-core-<version>.rpm
```

For Ubuntu:

``` bash
sudo dpkg -i /tmp/chef-server-core-<version>.deb
```

After a few minutes, the Chef Infra Server will be installed.