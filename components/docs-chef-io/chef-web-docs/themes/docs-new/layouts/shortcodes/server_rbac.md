The Chef Infra Server uses role-based access control (RBAC) to restrict
access to objects---nodes, environments, roles, data bags, cookbooks,
and so on. This ensures that only authorized user and/or Chef Infra
Client requests to the Chef Infra Server are allowed. Access to objects
on the Chef Infra Server is fine-grained, allowing access to be defined
by object type, object, group, user, and organization. The Chef Infra
Server uses permissions to define how a user may interact with an
object, after they have been authorized to do so.