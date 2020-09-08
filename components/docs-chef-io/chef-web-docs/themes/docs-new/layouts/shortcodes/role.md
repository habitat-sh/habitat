A role is a way to define certain patterns and processes that exist
across nodes in an organization as belonging to a single job function.
Each role consists of zero (or more) attributes and a run-list. Each
node can have zero (or more) roles assigned to it. When a role is run
against a node, the configuration details of that node are compared
against the attributes of the role, and then the contents of that role's
run-list are applied to the node's configuration details. When a Chef
Infra Client runs, it merges its own attributes and run-lists with those
contained within each assigned role.