Ohai is a tool that is used to collect system configuration data, which
is provided to Chef Infra Client for use within cookbooks. Ohai is run
by Chef Infra Client at the beginning of every Chef run to determine
system state. Ohai includes many built-in plugins to detect common
configuration details as well as a plugin model for writing custom
plugins.

The types of attributes Ohai collects include but are not limited to:

-   Operating System
-   Network
-   Memory
-   Disk
-   CPU
-   Kernel
-   Host names
-   Fully qualified domain names
-   Virtualization
-   Cloud provider metadata

Attributes that are collected by Ohai are automatic level attributes, in
that these attributes are used by Chef Infra Client to ensure that these
attributes remain unchanged after Chef Infra Client is done configuring
the node.