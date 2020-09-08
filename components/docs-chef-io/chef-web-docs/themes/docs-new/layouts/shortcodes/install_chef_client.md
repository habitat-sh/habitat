The `knife bootstrap` command is a common way to install Chef Infra
Client on a node. The default for this approach assumes that a node can
access the Chef website so that it may download the Chef Infra Client
package from that location.

The Chef Infra Client installer will detect the version of the operating
system, and then install the appropriate Chef Infra Client version using
a single command to install Chef Infra Client and all of its dependencies,
including an embedded version of Ruby, OpenSSL, parsers, libraries,
and command line utilities.

The Chef Infra Client installer puts everything into a unique directory
(`/opt/chef/`) so that Chef Infra Client will not interfere with other
applications that may be running on the target machine. Once installed,
Chef Infra Client requires a few more configuration steps before it can
perform its first Chef Infra Client run on a node.