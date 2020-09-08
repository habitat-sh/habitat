An attribute is a specific detail about a node, such as an IP address, a
host name, a list of loaded kernel modules, the version(s) of available
programming languages that are available, and so on. An attribute may be
unique to a specific node or it can be identical across every node in
the organization. Attributes are most commonly set from a cookbook, by
using knife, or are retrieved by Ohai from each node prior to every Chef
Infra Client run. All attributes are indexed for search on the Chef
Infra Server. Good candidates for attributes include:

-   any cross-platform abstraction for an application, such as the path
    to a configuration file
-   default values for tunable settings, such as the amount of memory
    assigned to a process or the number of workers to spawn
-   anything that may need to be persisted in node data between Chef
    Infra Client runs

In general, attribute precedence is set to enable cookbooks and roles to
define attribute defaults, for normal attributes to define the values
that should be specific for a node, and for override attributes to force
a certain value, even when a node already has that value specified.

One approach is to set attributes at the same precedence level by
setting attributes in a cookbook's attribute files, and then also
setting the same default attributes (but with different values) using a
role. The attributes set in the role will be deep merged on top of the
attributes from the attribute file, and the attributes set by the role
will take precedence over the attributes specified in the cookbook's
attribute files.