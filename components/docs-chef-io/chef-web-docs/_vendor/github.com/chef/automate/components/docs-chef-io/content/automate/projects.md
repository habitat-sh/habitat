+++
title = "Projects"

draft = false
[menu]
  [menu.automate]
    title = "Projects"
    identifier = "automate/settings/projects.md Projects"
    parent = "automate/settings"
    weight = 110
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/projects.md)

## Overview

Identity and Access Management projects allow for filtering and segregation of your data among your user base.

Permission for the `iam:projects` action is required to interact with projects.

## Managing Projects

### Creating Projects

Navigate to _Projects_ in the **Settings** tab. Then use the **Create Project** button, which opens a dialog box for entering the project's _Name_. A project ID automatically generates upon creation. If you would like to change the project ID, use the **Edit ID** button.

When a project is created, three policies that control access to that project are also created. Those policies include: _Project Owner_, _Project Editor_, and _Project Viewer_. Adding members to these policies will grant them access to the project.

![](/images/automate/settings-projects.png)

### Deleting Projects

Only projects with zero ingest rules and no pending edits can be deleted. To delete a project, navigate to _Projects_ in the **Settings** tab. Then open the menu at the end of the table row and select **Delete Role**.

### Changing Project Details

#### Ingest Rules

Ingest rules allow ingested events and nodes to be added to projects. *Node* corresponds to ingested client run and compliance nodes, and *Event* corresponds to ingested events on the _Event Feed_ page. 

Ingest rules can be created or changed by navigating to _Projects_ in the **Settings** tab and then selecting a project. 
When you create or update ingest rules, those changes are staged and **not** directly applied.
Other users may also stage changes.

Each rule describes a list of **conditions**, where each condition describes a single characteristic.

##### Project Ingest Rule Conditions

A condition consists of these properties:

Property               | Description
-----------------------|------------
Event Attribute        | Chef Organization or Chef Infra Server
Node Attribute         | Chef Organization, Chef Infra Server, Environment, Chef Role, Chef Tag, Chef Policy Name, or Chef Policy Group
Operator               | equals of member of
Values                 | list of one or more values to match on the specified attribute

#### Details

To change the name of a project, navigate to _Projects_ in the **Settings** tab and then select the **Details** tab.

### Updating Projects

The _Project List_ page displays the status of project ingest rules (*No rules*, *Edits pending*, or *Applied*).

If a project has pending edits from changes to ingest rules, then all projects must be updated for those pending edits to take effect. Updating projects will apply all pending edits and move ingested resources into the correct projects. This background process can take a few minutes for systems with a limited number of nodes, and **several days** for systems with a large number of nodes. 

All changes will be applied together when you update projects. To update projects navigate to _Projects_ in the **Settings** tab and use the **Update Projects** button.

#### Stopping the Project Update Early

Stopping a project update early will leave your resources in a bad state. Some resources will be in the correct projects and others will not be. To resolve this, make sure to start another project update.

To stop the project update background process before it finishes, navigate to _Projects_ in the **Settings** tab and select the **Stop Updating Projects** button.
