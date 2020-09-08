The Chef Infra Server acts as a hub for configuration data. The Chef
Infra Server stores cookbooks, the policies that are applied to nodes,
and metadata that describes each registered node that is being managed
by Chef Infra Client. Nodes use Chef Infra Client to ask the Chef Infra
Server for configuration details, such as recipes, templates, and file
distributions. Chef Infra Client then does as much of the configuration
work as possible on the nodes themselves (and not on the Chef Infra
Server). This scalable approach distributes the configuration effort
throughout the organization.