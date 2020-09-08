+++
title = "Organizations and Groups"
draft = false

aliases = ["/server_orgs.html", "/auth_authorization.html"]

[menu]
  [menu.infra]
    title = "Organizations & Groups"
    identifier = "chef_infra/managing_chef_infra_server/server_orgs.md Organizations & Groups"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 80
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_orgs.md)

{{% server_rbac %}}

The Chef Infra Server uses organizations, groups, and users to define
role-based access control:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><img src="/images/icon_server_organization.svg" class="align-center" width="100" alt="image" /></td>
<td>An organization is the top-level entity for role-based access control in the Chef Infra Server. Each organization contains the default groups (<code>admins</code>, <code>clients</code>, and <code>users</code>, plus <code>billing_admins</code> for the hosted Chef Infra Server), at least one user and at least one node (on which the Chef Infra Client is installed). The Chef Infra Server supports multiple organizations. The Chef Infra Server includes a single default organization that is defined during setup. Additional organizations can be created after the initial setup and configuration of the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><p><img src="/images/icon_server_groups.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>A group is used to define access to object types and objects in the Chef Infra Server and also to assign permissions that determine what types of tasks are available to members of that group who are authorized to perform them. Groups are configured per-organization.</p>
<p>Individual users who are members of a group will inherit the permissions assigned to the group. The Chef Infra Server includes the following default groups: <code>admins</code>, <code>clients</code>, and <code>users</code>. For users of the hosted Chef Infra Server, an additional default group is provided: <code>billing_admins</code>.</p></td>
</tr>
<tr class="odd">
<td><img src="/images/icon_server_users.svg" class="align-center" width="100" alt="image" /></td>
<td>A user is any non-administrator human being who will manage data that is uploaded to the Chef Infra Server from a workstation or who will log on to the Chef management console web user interface. The Chef Infra Server includes a single default user that is defined during setup and is automatically assigned to the <code>admins</code> group.</td>
</tr>
<tr class="even">
<td><img src="/images/icon_chef_client.svg" class="align-center" width="100" alt="image" /></td>
<td>A client is an actor that has permission to access the Chef Infra Server. A client is most often a node (on which the Chef Infra Client runs), but is also a workstation (on which knife runs), or some other machine that is configured to use the Chef Infra Server API. Each request to the Chef Infra Server that is made by a client uses a private key for authentication that must be authorized by the public key on the Chef Infra Server.</td>
</tr>
</tbody>
</table>

When a user makes a request to the Chef Infra Server using the Chef
Infra Server API, permission to perform that action is determined by the
following process:

1.  Check if the user has permission to the object type
2.  If no, recursively check if the user is a member of a security group
    that has permission to that object
3.  If yes, allow the user to perform the action

Permissions are managed using the Chef management console add-on in the
Chef Infra Server web user interface.

## Organizations

A single instance of the Chef Infra Server can support many
organizations. Each organization has a unique set of groups and users.
Each organization manages a unique set of nodes, on which a Chef Infra
Client is installed and configured so that it may interact with a single
organization on the Chef Infra Server.

![image](/images/server_rbac_orgs_groups_and_users.png)

A user may belong to multiple organizations under the following
conditions:

-   Role-based access control is configured per-organization
-   For a single user to interact with the Chef Infra Server using knife
    from the same chef-repo, that user may need to edit their config.rb
    file prior to that interaction

Using multiple organizations within the Chef Infra Server ensures that
the same toolset, coding patterns and practices, physical hardware, and
product support effort is being applied across the entire company, even
when:

-   Multiple product groups must be supported---each product group can
    have its own security requirements, schedule, and goals
-   Updates occur on different schedules---the nodes in one organization
    are managed completely independently from the nodes in another
-   Individual teams have competing needs for object and object
    types---data bags, environments, roles, and cookbooks are unique to
    each organization, even if they share the same name

### Permissions

{{% server_rbac_permissions %}}

#### Object Permissions

{{% server_rbac_permissions_object %}}

#### Global Permissions

The Chef Infra Server includes the following global permissions:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Permission</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><strong>Create</strong></td>
<td>Use the <strong>Create</strong> global permission to define which users and groups may create the following server object types: cookbooks, data bags, environments, nodes, roles, and tags. This permission is required for any user who uses the <code>knife [object] create</code> argument to interact with objects on the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><strong>List</strong></td>
<td>Use the <strong>List</strong> global permission to define which users and groups may view the following server object types: cookbooks, data bags, environments, nodes, roles, and tags. This permission is required for any user who uses the <code>knife [object] list</code> argument to interact with objects on the Chef Infra Server.</td>
</tr>
</tbody>
</table>

These permissions set the default permissions for the following Chef
Infra Server object types: clients, cookbooks, data bags, environments,
groups, nodes, roles, and sandboxes.

#### Client Key Permissions

{{< note >}}

This is only necessary after migrating a client from one Chef Infra
Server to another. Permissions must be reset for client keys after the
migration.

{{< /note >}}

Keys should have `DELETE`, `GRANT`, `READ` and `UPDATE` permissions.

Use the following code to set the correct permissions:

``` ruby
#!/usr/bin/env ruby
require 'chef/knife'

#previously knife.rb
Chef::Config.from_file(File.join(Chef::Knife.chef_config_dir, 'knife.rb'))

rest = Chef::ServerAPI.new(Chef::Config[:chef_server_url])

Chef::Node.list.each do |node|
  %w{read update delete grant}.each do |perm|
    ace = rest.get("nodes/#{node[0]}/_acl")[perm]
    ace['actors'] << node[0] unless ace['actors'].include?(node[0])
    rest.put("nodes/#{node[0]}/_acl/#{perm}", perm => ace)
    puts "Client \"#{node[0]}\" granted \"#{perm}\" access on node \"#{node[0]}\""
  end
end
```

Save it as a Ruby script---`chef_server_permissions.rb`, for
example---in the `.chef/scripts` directory located in the chef-repo, and
then run a knife command similar to:

``` bash
knife exec chef_server_permissions.rb
```

#### Knife ACL

The knife plugin [knife-acl](https://github.com/chef/knife-acl) provides
a fine-grained approach to modifying permissions, by wrapping API calls
to the `_acl` endpoint and makes such permission changes easier to
manage.

{{% EOL_manage %}}

<span class="title-ref">knife-acl</span> and the Chef Manage browser
interface are incompatible. After engaging <span
class="title-ref">knife-acl</span>, you will need to discontinue using
the Chef Manage browser interface from that point forward due to
possible incompatibilities.

## Groups

The Chef Infra Server includes the following default groups:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>admins</code></td>
<td>The <code>admins</code> group defines the list of users who have administrative rights to all objects and object types for a single organization.</td>
</tr>
<tr class="even">
<td><code>billing_admins</code></td>
<td>The <code>billing_admins</code> group defines the list of users who have permission to manage billing information. This permission exists only for the hosted Chef Infra Server.</td>
</tr>
<tr class="odd">
<td><code>clients</code></td>
<td>The <code>clients</code> group defines the list of nodes on which a Chef Infra Client is installed and under management by Chef. In general, think of this permission as "all of the non-human actors---Chef Infra Client, in nearly every case---that get data from, and/or upload data to, the Chef server". Newly-created Chef Infra Client instances are added to this group automatically.</td>
</tr>
<tr class="even">
<td><code>public_key_read_access</code></td>
<td>The <code>public_key_read_access</code> group defines which users and clients have read permissions to key-related endpoints in the Chef Infra Server API.</td>
</tr>
<tr class="odd">
<td><code>users</code></td>
<td>The <code>users</code> group defines the list of users who use knife and the Chef management console to interact with objects and object types. In general, think of this permission as "all of the non-admin human actors who work with data that is uploaded to and/or downloaded from the Chef server".</td>
</tr>
</tbody>
</table>

### Example Default Permissions

The following sections show the default permissions assigned by the Chef
Infra Server to the `admins`, `billing_admins`, `clients`, and `users`
groups.

{{< note >}}

The creator of an object on the Chef Infra Server is assigned `create`,
`delete`, `grant`, `read`, and `update` permission to that object.

{{< /note >}}

#### admins

The `admins` group is assigned the following:

<table style="width:100%;">
<colgroup>
<col style="width: 24%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Create</th>
<th>Delete</th>
<th>Grant</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>admins</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="even">
<td>clients</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>users</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
</tbody>
</table>

#### billing_admins

The `billing_admins` group is assigned the following:



#### billing_admins

The `billing_admins` group is assigned the following:

<table>
<colgroup>
<col style="width: 28%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Create</th>
<th>Delete</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>billing_admins</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>yes</td>
</tr>
</tbody>
</table>

#### clients

The `clients` group is assigned the following:

<table>
<colgroup>
<col style="width: 28%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
</colgroup>
<thead>
<tr class="header">
<th>Object</th>
<th>Create</th>
<th>Delete</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>clients</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
<tr class="even">
<td>cookbooks</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="odd">
<td>cookbook_artifacts</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>data</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="odd">
<td>environments</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>nodes</td>
<td>yes</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="odd">
<td>organization</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>policies</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="odd">
<td>policy_groups</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>roles</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="odd">
<td>sandboxes</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
</tbody>
</table>

#### public_key_read_access

The `public_key_read_access` group controls which users and clients have
[read permissions to the following endpoints](/api_chef_server/):

-   GET /clients/CLIENT/keys
-   GET /clients/CLIENT/keys/KEY
-   GET /users/USER/keys
-   GET /users/USER/keys/

By default, the `public_key_read_access` assigns all members of the
`users` and `clients` group permission to these endpoints:

<table style="width:100%;">
<colgroup>
<col style="width: 24%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Create</th>
<th>Delete</th>
<th>Grant</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>admins</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
<tr class="even">
<td>clients</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>users</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
</tbody>
</table>

#### users

The `users` group is assigned the following:



#### users

The `users` group is assigned the following:

<table>
<colgroup>
<col style="width: 28%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
</colgroup>
<thead>
<tr class="header">
<th>Object</th>
<th>Create</th>
<th>Delete</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>clients</td>
<td>no</td>
<td>yes</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>cookbooks</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>cookbook_artifacts</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="even">
<td>data</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>environments</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="even">
<td>nodes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>organization</td>
<td>no</td>
<td>no</td>
<td>yes</td>
<td>no</td>
</tr>
<tr class="even">
<td>policies</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>policy_groups</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="even">
<td>roles</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>sandboxes</td>
<td>yes</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
</tbody>
</table>

### chef-validator

{{% security_chef_validator %}}

The chef-validator is allowed to do the following at the start of a Chef
Infra Client run. After the Chef Infra Client is registered with Chef
Infra Server, that Chef Infra Client is added to the `clients` group:

<table>
<colgroup>
<col style="width: 28%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
<col style="width: 17%" />
</colgroup>
<thead>
<tr class="header">
<th>Object</th>
<th>Create</th>
<th>Delete</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>clients</td>
<td>yes</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
</tbody>
</table>

### Chef Push Jobs Groups

{{% push_jobs_summary %}}

{{% server_rbac_groups_push_jobs %}}

## Server Admins

{{% server_rbac_server_admins %}}

### Scenario

{{< readFile_shortcode file="server_rbac_server_admins_scenario.md" >}}

#### Superuser Accounts

{{< readFile_shortcode file="server_rbac_server_admins_superusers.md" >}}

### Manage server-admins Group

{{% ctl_chef_server_server_admin %}}

#### Add Members

{{% ctl_chef_server_server_admin_grant_user %}}

#### Remove Members

{{% ctl_chef_server_server_admin_remove_user %}}

#### List Membership

{{% ctl_chef_server_server_admin_list %}}

## Manage Organizations

{{% ctl_chef_server_org %}}

### org-create

{{% ctl_chef_server_org_create %}}

**Syntax**

{{% ctl_chef_server_org_create_syntax %}}

**Options**

{{% ctl_chef_server_org_create_options %}}

### org-delete

{{% ctl_chef_server_org_delete %}}

**Syntax**

{{% ctl_chef_server_org_delete_syntax %}}

### org-list

{{% ctl_chef_server_org_list %}}

**Syntax**

{{% ctl_chef_server_org_list_syntax %}}

**Options**

{{% ctl_chef_server_org_list_options %}}

### org-show

{{% ctl_chef_server_org_show %}}

**Syntax**

{{% ctl_chef_server_org_show_syntax %}}

### org-user-add

{{% ctl_chef_server_org_user_add %}}

**Syntax**

{{% ctl_chef_server_org_user_add_syntax %}}

**Options**

{{% ctl_chef_server_org_user_add_options %}}

### org-user-remove

{{% ctl_chef_server_org_user_remove %}}

**Syntax**

{{% ctl_chef_server_org_user_remove_syntax %}}
