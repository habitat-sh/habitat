+++
title = "Roles"

draft = false
[menu]
  [menu.automate]
    title = "Roles"
    identifier = "automate/settings/roles.md Roles"
    parent = "automate/settings"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/roles.md)

## Overview

Chef Automate Identity and Access Management roles are named groups of actions used to define [policies]({{< relref "policies.md" >}}). Actions describe what is allowed by users in Automate. [IAM Actions]({{< relref "iam_actions.md" >}}) describes the associated action or actions required to access certain pages in the browser.

Users require permission for the `iam:roles` action to interact with roles. Any user that is part of the `admins` team or the `Administrator` policy will have this permission. Otherwise, [IAM custom policies]({{< relref "iam_v2_guide.md#creating-custom-policies" >}}) can be created to assign this permission.

![](/images/automate/settings-roles.png)

### Chef-Managed Roles

Chef-managed roles are roles provided by Chef that cannot be changed.

Role          | Description
--------------|------------
Viewer        | **View** everything in the system *except* IAM
Editor        | **Do** everything in the system *except* IAM and license application
Owner         | **Do** everything in the system *including* IAM
Project Owner | Editor + **view** and **assign** projects
Ingest        | Ingest data into the system

#### Actions for Chef-Managed Roles

Name | ID| Actions
-----------------------|-----|--------
Owner              | owner         | \*
Project Owner      | project-owner | infra:nodes:\*, infra:nodeManagers:\*, compliance:\*, event:\*, ingest:\*, secrets:\*, iam:projects:list, iam:projects:get, iam:projects:assign, iam:policies:list, iam:policies:get, iam:policyMembers:\*, iam:teams:list, iam:teams:get, iam:teamUsers:\*, iam:users:get, iam:users:list, applications:\*
Editor             | editor        | infra:infraServers:list, infra:infraServers:get, infra:nodes:\*, infra:nodeManagers:\*, compliance:\*, event:\*, ingest:\*, secrets:\*, iam:projects:list, iam:projects:get, iam:projects:assign, applications:\*
Viewer             | viewer        | infra:infraServers:list, infra:infraServers:get, secrets:\*:get, secrets:\*:list, infra:nodes:get, infra:nodes:list, infra:nodeManagers:get, infra:nodeManagers:list, compliance:\*:get, compliance:\*:list, event:\*:get, event:\*:list, ingest:\*:get, ingest:\*:list, iam:projects:list, iam:projects:get, applications:\*:get, applications:\*:list
Ingest             | ingest        | infra:ingest:\*, compliance:profiles:get, compliance:profiles:list

### Custom Roles

Custom roles are roles that any user with the permission for `iam:roles:update` can change. 
In addition to the Chef-managed roles above, Chef Automate includes two custom roles by default.

Role              | Description
------------------|------------
Compliance Viewer |Viewer for compliance resources
Compliance Editor |Editor for compliance resources

You can edit these custom roles like other user-created custom roles.

## Managing Roles

### Creating Roles

_Custom_ roles can only be created using the [Roles API]({{< relref "api/#tag/roles" >}}).

#### Example Custom Role

```json
{
  "name": "Advocate",
  "id": "advocate-role",
  "actions": [
    "infra:*",
    "compliance:*",
    "teams:*",
    "users:*"
  ],
  "projects": [
    "east-region",
    "west-region"
  ]
}
```

### Changing Role Details

For _custom_ roles, use the [Roles API]({{< relref "api/#tag/roles" >}}) to change the role name, actions list, and projects.

### Deleting Roles

Navigate to _Roles_ in the **Settings** tab. Then open the menu at the end of the table row and select **Delete Role**.
