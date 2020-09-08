+++
title = "IAM Users Guide"

draft = false

[menu]
  [menu.automate]
    title = "IAM Users Guide"
    parent = "automate/authorization"
    identifier = "automate/authorization/iam_v2_guide.md IAM Users Guide"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/iam_v2_guide.md)

{{< note >}}
This documentation covers Chef Automate's IAM feature in release 20200326170928 and later.
{{< /note >}}

This guide shows you how to perform important administrative operations.
You will add members to Chef-managed v2 policies, delete a legacy policy, and write a Team Admin v2 policy that lets a Team Admin manage their users and teams.

## View Policies

After you have signed in to Chef Automate, select the **Settings** tab in the top navigation bar, and then select and locate the `Policies` section in the left navigation.

In this section, you can view all of your v2 policies.

This policy display includes the following:

* New default, *Chef-managed* policies: Administrator, Ingest, Editors, and Viewers.
* Imported v1 default policies--now called *legacy policies*--in the new v2 policy format and marked with the `[Legacy]` prefix.
* Imported v1 custom policies that you created, which are marked with the `[Legacy]` prefix and a `(custom)` suffix.

![](/images/automate/admin-policies-migrated.png)

## Policy Conversion

If you started on IAM v1 and migrated your IAM v1 policies to IAM v2, then you should move any members of your IAM v1 legacy policies to the appropriate IAM v2 policies, and afterwards, delete the IAM v1 legacy policies.

To delete a legacy policy, open the menu on any custom policy, which is located at the end of the policy row, and select **Delete Policy** from the menu.

A warning appears if members are still attached to the policy because deleting that policy disrupts access for all of its members.
However, you will still be able to delete the policy.

The next few sections explain how to use Chef-managed policies and how to create custom policies.

### Using Chef-Managed Policies

In this example conversion, you will create two local users and add them to default teams that are automatically included in default policies.
Note that you could also add users directly to policies without the intermediate teams, but using teams can make managing your system more flexible.

#### Create Users

Follow the instructions on [Creating Users]({{< relref "users.md#creating-local-users" >}}) to:

* Create a local user with the username `test_viewer`.
* Create a local user with the username `test_editor`.

#### Add Users to Teams

Select `Teams` from the left navigation of the **Settings** tab.
Three teams are provided by default: `admins`, `viewers`, and `editors`.

Follow the instructions for [Adding Users to a Team]({{< relref "teams.md#adding-local-users-to-teams" >}}) to:

* Add the user `test_viewer` to the Chef-managed `viewers` team.
* Add the user `test_editor` to the Chef-managed `editors` team.

After upgrading, those teams will be part of the `Viewers` and `Editors` policies, respectively.
The default `Viewers` policy is provided, so you can quickly grant read-only access to everything in Chef Automate, except for admin resources.
Similarly, the default `Editors` policy is provided, so you can quickly grant complete access to everything in Chef Automate, except for admin resources.
Once this step is done, the `test_viewer` and `test_editor` users may log in with appropriate system access.

### Creating Custom Policies

The Chef-managed policies give you a starting point for permissions.
You may want to write more fine-grained policies, tailored to the demands of your organization.
Defining your own policies must be done from the command line.

As an example, let us say that you, the admin, want to delegate a portion of your tasks to a colleague, but without granting them full admin access.
In this case, you could create a policy called `Team Devops Managers`, which grants its members some--but not all--administrative privileges.
Create a JSON file in the editor of your choice.
Following JSON syntax, begin with the name property.

See the end of this section for a [complete JSON policy example]({{< relref "iam_v2_guide.md#complete-json-policy-example" >}})

```json
  "name": "Team Devops Managers",
```

The `name` field is for human consumption. When you want to refer to the policy in commands, you will need to know the policy's ID.
Let us give this policy the ID value: `team-managers-devops`.

```json
  "id": "team-managers-devops",
```

Additionally, we can permission actions on this policy just like any other IAM resource by assigning it to one or more projects.
If we leave the projects array empty, then we indicate the policy is _unassigned_.
For example, anyone with permission to view _unassigned_ policies can view this policy.

```json
  "projects": [],
```

Let us further assume you want user `Bob` as well as anyone on team `gamma` to be authorized by this policy. This grouping comprises the `members` array, as seen below.

```json
  "members": [
    "user:local:bob",
    "team:local:gamma",
  ],
```

Next, you will specify the permissions themselves--which in IAM v2 are the `statements`-- declared as an array.
The statement allows us to specify the `actions` a user is permitted to take upon resources that have been assigned to a `project`.
The `projects` field on a statement is an array that may contain more than one existing project, a wildcard `*` to indicate permission to resources in _any project_, or `(unassigned)` to indicate permission to resources that have not been assigned to a project.

Note that the `projects` property in statements designates permission for the resources within the statement (here, that is `iam:users` and `iam:teams`), _not_ for the policy itself, and _cannot_ be left empty.
For more about projects, see [Projects in the IAM Guide]({{< relref "iam_v2_guide.md#projects" >}}) documentation.

In this case, we only need a single statement providing access to the _get_, _list_, and _update_ actions for _users_ and _teams_ that have been assigned to the project `project-devops`.

```json
    {
      "effect": "ALLOW",
      "actions": [
        "iam:users:update",
        "iam:users:list",
        "iam:users:get",
        "iam:teams:update",
        "iam:teams:list",
        "iam:teams:get"
      ],
      "projects": ["project-devops"],
    },
```

#### Complete JSON Policy Example

```json
{
  "name": "Team Devops Managers",
  "id": "team-managers-devops",
  "projects": [],
  "members": [
    "user:local:bob",
    "team:local:gamma"
  ],
  "statements": [
    {
      "effect": "ALLOW",
      "actions": [
        "iam:users:update",
        "iam:users:list",
        "iam:users:get",
        "iam:teams:update",
        "iam:teams:list",
        "iam:teams:get"
      ],
      "projects": ["project-devops"]
    }
  ]
}
```

Save your JSON file and refer to the [IAM Policies API reference](/automate/api/#tag/policies) to send that policy data to Chef Automate.

### Policy Membership

Users, teams, and API tokens can all be policy members. Both users and teams can be either locally or externally managed with LDAP or SAML.

#### Local Users and Teams

Local users and teams are managed directly by Chef Automate.

To add or delete members, navigate to the Policies list in the **Settings** tab, and then select a policy in the list to open its details.
Select **Members** to view the current membership.
Use the **Add Members** button to open a list of candidate members.
This lists all those local members (both users and teams) that are *not* members of this policy.
If all of the local members are already included in the policy, then this list will be empty.
Select any members you wish to add to the policy.
Use the **Add Members** button to complete the operation.
This takes you back to the policy details, showing the revised membership list.

#### Member Expressions

Member expressions are required for externally managed users, and teams, as well as API tokens.

LDAP and SAML users' information is saved outside of Chef Automate.
You will need to manually enter the provider-qualified user or team.
To do this, open any policy from the _Policies_ list, then select **Members**.
Select **Add Members** to open the list of candidate local users and teams.
Near the bottom of the page, select the **Add Member Expression** button.

{{< note >}}
The member expression dialog box appears after selecting **Add Member Expression** and guides you through creating a member expression. This dialog box ensures correct syntax use in a user-friendly way.
The next few paragraphs explain the syntax if you want to add members via the API.
{{< /note >}}

Enter a member expression using the format `team:<type>:<name>` or `user:<type>:<name>`. Note that these expressions are case-sensitive.

* The `<type>` expression is either `ldap` or `saml`.
* The `<name>` expression is the name of the user or team that the external identity provider knows. For example, this is a valid member expression `team:ldap:editors_team_1`, assuming the `editors_team_1` team is known by your identity provider.

Alternately, you may add *all* teams to a policy using a wildcard as the last term in the member expression: `team:ldap:*` or `team:saml:*`.

The member expression dialog also supports tokens.
You enter a token using the expression `token:<id>`.
In order to find a token's ID, visit the *API Tokens* page.

### Projects

Projects are used to group and permission Chef Automate resources as well as ingested data, specifically Compliance reports, Chef Infra Server events, and Infrastructure nodes.

Projects can be managed via the Projects list under the **Settings** tab and consist of an ID, a name, and a collection of ingest rules. Project ingest rules are lists of conditions used only when
[assigning ingested resources to projects]({{< relref "iam_v2_guide.md#assigning-ingested-resources-to-projects" >}}),
so they are not relevant when assigning IAM resources such as teams or roles.

#### Configuring Project Limit

By default, Chef Automate limits you to 300 projects. You can increase the project limit using the command line.

First, write the file with your new project limit:

```
cat << EOF > authz.toml
[auth_z.v1.sys.service]
project_limit = <desired-max-projects>
EOF
```

Then, update the existing Chef Automate configuration:

`chef-automate config patch authz.toml`

Note: As a consequence of increasing the project limit, you may see slower loading times in the UI.

#### Creating a Project

To create a project, navigate to the Projects list under the **Settings** tab and select **Create Project**. You will need to provide a name and can optionally edit the ID. You must create a project before you can assign any resources to it.

When you initiate the project creation, the system creates the project, but also three supplemental policies for your convenience:

Policy Name                      | Policy ID                        | Associated Role
---------------------------------|----------------------------------|----------------
`<project-name>` Project Owners  | `<project-name>`-project-owners  | Project Owner
`<project-name>` Project Editors | `<project-name>`-project-editors | Editor
`<project-name>` Project Viewers | `<project-name>`-project-viewers | Viewer

These policies are discussed in more detail in [Project Policies]({{< relref "iam_v2_guide.md#project-policies" >}}).

#### Assigning Teams and Tokens to Projects

Projects can be assigned to Automate-created teams or tokens on creation or update.

To assign a team to projects, select a team from the _Teams_ list, then select **Details**.
Likewise, to assign a token to projects, select a token from the API tokens list, then select **Details**.
In either case, you can select projects from the projects dropdown to assign.

You may also assign teams and tokens to projects on creation. In the creation modal, select any projects to which the new resource should belong.

If you would like to delegate ownership of a project to another user so that they may assign resources, you will want to make that user a [Project Owner]({{< relref "iam_v2_guide.md#project-owners" >}}) of that project.

#### Assigning Ingested Resources to Projects

While Automate's local teams and tokens can be directly assigned to a project, ingested resources must be assigned to projects using ingest rules.

Project ingest rules are used to associate ingested resources with projects within Automate. An ingest rule contains conditions that determine if an ingested resource should be moved into the rule's project.
Each condition contains an attribute, operator, and value. See [IAM Project Rules API reference](/automate/api/#tag/rules) for details on how to manage project rules.

In this example, after [creating a project]({{< relref "iam_v2_guide.md#creating-a-project" >}}) with the ID `project-devops`, you will add an ingest rule to this new project.
You will update projects to apply this new project rule, causing all matching ingested resources to be associated with `project-devops`.
You will then use the global project filter to filter your ingested data by `project-devops`.

First, determine which ingested resources should belong to the project. In this example, we want to add the following ingested resources to `project-devops`:

* Compliance reports with Chef Organization `devops`
* Infrastructure nodes with Environment `dev` and Chef Tag `devops-123`
* Actions on Chef Infra Servers `devops.pizza` or `devops.dog`

You may want to verify that those filters work as expected on the _Event Feed_, _Client Runs_, and
_Reports_ pages.

Navigate to the project details page of `project-devops`, by selecting the project name on the project list page.

Select the `Create Rule` button to create a new project rule. Choose the resource type `Node`, then fill in the first condition's fields.
Feel free to create fake ingest data that corresponds to the example json below, or come up with a condition that matches your existing data set.

{{% warning %}}
Compliance reports must be using **audit cookbook 7.5+** in order to make use of all of the available project rule attributes. Older reports will only have **Environment** and **Chef Role** available as attributes.
{{% /warning %}}

```json
{
  "id": "devops-rule",
  "name": "devops rule",
  "type": "NODE",
  "project_id": "project-devops",
  "conditions": [
    {
      "operator": "EQUALS",
      "attribute": "CHEF_ORGANIZATION",
      "values": [
        "devops"
      ]
    }
  ]
}
```

![](/images/automate/create-project-rule.png)

Save the rule. If you later need to change the name or the conditions, select the project rule name on the project details page.

When edits are pending, a banner will be shown at the bottom of every page. Selecting the `Update Projects` button on that banner will apply those changes.

![](/images/automate/admin-projects.png)

Updating a project begins an operation that applies all pending rule edits and then moves ingested resources into the correct projects according to those latest changes. An ingested resource is moved into a project if it matches at least one of the project's rules.
In this example, upon successful update, all ingested resources whose Chef Organization matches `devops` will be considered a part of the project `project-devops`.
Only these resources will appear in Automate's dashboards when the `project-devops` project has been selected in the global project filter.

A percentage count appears in the bottom banner while the operation takes place.
You may cancel the update at any time by selecting the `Stop Project Update` button in the banner and confirming the cancel in the modal that pops up.

{{% warning %}}
Avoid stopping an update unless absolutely necessary. It will leave your system in an unknown state where only some resources have been moved into their projects while others still remain in old projects. Only another successful update will restore the system to an up-to-date state.
{{% /warning %}}

Once rules have been successfully applied, the banner will be dismissed until the next time there are *pending edits* to any project.

To verify that the ingested resources have been moved into the correct projects, select `project-devops` in the global projects filter, which is on the top navigation. The data in Automate filters by the selected `project-devops` project.
In this example, the effect is revealed by navigating to the Compliance Reports' Nodes tab, which only features nodes that belong to the `devops` Chef Organization.

![](/images/automate/global-projects-filter.png)

Now that we have the first set of our ingested data associated with our new project, let us add another condition and a new rule to add more data to `project-devops`.

{{< note >}}
Compliance and Infrastructure ingested resources are not the exact same nodes, so their properties may not be the same.
Separate conditions governing said resources *may* need to be used if their properties do not match exactly.
{{< /note >}}

{{< note >}}
Ingested events require conditions of `Event` type to be associated with the correct project. A condition of type `Node` will not match an event, even if the condition's operator, attribute, and value all match exactly (and vice versa with `Event` project rules and nodes).
{{< /note >}}

Return again to the project detail page and create two new ingest rules by selecting `Create Rule`. Creating new rules will expand the data set under `project-devops`,
because an ingested resource need only match one rule to be placed in the project.

The first rule should contain two conditions.
Fill in the first condition with attribute `Environment`, operator `equals`, and value `dev`, or any value matching your data set.
Select `Add Condition` to add another condition with attribute `Chef Tag`, operator `equals`, and `devops-123`.
Save the rule.

{{< note >}}
Adding conditions further restricts the ingested data because every condition must be true for an ingested resource to be placed in the project.
{{< /note >}}

For the second rule, choose resource type `Event`. Fill in the first condition with attribute `Chef Server`, operator `member of`, and value `devops.pizza, devops.dog`, or any values matching your data set.

Setting the project rule `Resource Type` determines what condition attributes are available to select. `Event` rule conditions can only have the attributes `Chef Organization` or `Chef Server`.

Rules of type `Node` can have conditions with attributes `Chef Organization`, `Chef Server`, `Environment`, `Chef Role`, `Chef Tag`, `Chef Policy Name`, `Chef Policy Group`.

Select the `Update Projects` button from the bottom banner.
Upon completion of the update, you should be able to filter by `project-devops` across Automate's dashboards and see only the ingested data that you expect.

#### Effortless Infra Project

To create a project that contains all Effortless Infra nodes, create a ingest rule with resource type `Node` and a condition that uses attribute `Chef Server`, operator `equals`, and value `localhost`.

![](/images/automate/effortless-project-rule.png)

The above rule matches on a node's Chef Infra Server field, which is set to `localhost`. This rule works because all Effortless Infra nodes list the `Chef Infra Server` attribute as `localhost`. 

If desired, create subgroups of Effortless Infra nodes by adding a second condition that matches a specific `Chef Policy Name`.

#### Project Policies

When you create a project, Chef Automate automatically creates three supplemental policies.
This next table further describes the roles associated with these policies.

Policy Name                      | Role          | Description of role
---------------------------------|---------------|--------------------
`<project-name>` Project Viewers | Viewer        | **View** everything in the project *except* IAM
`<project-name>` Project Editors | Editor        | **Do** everything in the project *except* IAM
`<project-name>` Project Owners  | Project Owner | Editor + **view** and **assign** projects

Consider the **Project Viewers** policy, which uses the **Viewer** role. The same **Viewer** role is also used in the default **Viewer** policy, which lets a user view everything in the system except IAM. Using the **Viewer** role in **Project Viewers** policy restricts the scope to your new project, and users attached to your **Project Viewers** policy will be able to view objects only associated with your project.

Assume you named your project `Devops`, and have two users: Terry and Kelly. If Terry needs to *view* `Devops`-scoped resources, then add Terry as a member to the `Devops` Project Viewers policy. If Kelly needs the ability to *edit* `Devops`-scoped resources, then add Kelly to the `Devops` Project Editor policy.

When Terry is a member of the `Devops Project Viewers` policy and not a member of any other policy, they will only be able to see resources assigned to `Devops`. They will not be able to update or delete them. Kelly, however, will be able to update and delete.

See [Policy Membership]({{< relref "iam_v2_guide.md#policy-membership" >}}) for more information on policy membership.

## Restoring Admin Access

While we have safeguards to prevent it, it is possible to lock yourself out of Chef Automate.
If you have root access to the node where Chef Automate is installed, use the following commands to restore admin access:

This command resets the local `admin` user's password and ensures that the user is a member of the local `admins` team, which is a permanent member of the Chef-managed `Administrator` policy.

```bash
  chef-automate iam admin-access restore <your new password here>
```
