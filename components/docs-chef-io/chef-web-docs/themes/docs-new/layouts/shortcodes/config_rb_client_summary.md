The client.rb file specifies how Chef Infra Client is configured on a
node and has the following characteristics:

-   This file is loaded every time the chef-client executable is run.
-   On Microsoft Windows machines, the default location for this file is
    `C:\chef\client.rb`. On all other systems the default location for
    this file is `/etc/chef/client.rb`.
-   Use the `--config` option from the command line to override the
    default location of the configuration file.
-   This file is not created by default