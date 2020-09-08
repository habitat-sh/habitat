An attribute is a specific detail about a node. Attributes are used by
Chef Infra Client to understand:

-   The current state of the node
-   What the state of the node was at the end of the previous Chef Infra
    Client run
-   What the state of the node should be at the end of the current Chef
    Infra Client run

Attributes are defined by:

-   The state of the node itself
-   Attributes passed via JSON on the CLI
-   Cookbooks (in attribute files and/or recipes)
-   Roles
-   Environments
-   Policyfiles

During every Chef Infra Client run, Chef Infra Client builds the
attribute list using:

-   Attributes passed via JSON on the CLI
-   Data about the node collected by [\[Ohai\]](/ohai.html).
-   The node object that was saved to the Chef Infra Server at the end
    of the previous Chef Infra Client run.
-   The rebuilt node object from the current Chef Infra Client run,
    after it is updated for changes to cookbooks (attribute files and/or
    recipes), roles, and/or environments, and updated for any changes to
    the state of the node itself.

After the node object is rebuilt, all of the attributes are compared,
and then the node is updated based on attribute precedence. At the end
of every Chef Infra Client run, the node object that defines the current
state of the node is uploaded to the Chef Infra Server so that it can be
indexed for search.