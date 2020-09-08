The `install` subcommand is used to install premium features of the Chef
server: Chef management console(`chef-manage`) and push
jobs(`opscode-push-jobs-server`).

``` bash
sudo chef-server-ctl install PACKAGE_NAME --path /path/to/package/directory
```

For example:

``` bash
sudo chef-server-ctl install chef-manage --path /root/packages
```

The `chef-server-ctl` command will install the first `chef-manage`
package found in the `/root/packages` directory.