+++
title = "Integrate Workflow with Bitbucket"
draft = false
robots = "noindex"


aliases = ["/integrate_delivery_bitbucket.html", "/release/automate/integrate_delivery_bitbucket.html"]

[menu]
  [menu.legacy]
    title = "Workflow w/Bitbucket"
    identifier = "legacy/workflow/managing_workflow/integrate_delivery_bitbucket.md Workflow w/Bitbucket"
    parent = "legacy/workflow/managing_workflow"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/integrate_delivery_bitbucket.md)



{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Bitbucket integration with Workflow allows Bitbucket to be used as the
canonical git repository for projects while at the same time benefiting
from Workflow workflow and pipeline automation. When Bitbucket
integration is enabled for a project in Bitbucket, you will be able to:

-   Review pull requests and make code comments in the Bitbucket webui
-   Browse code (including in-flight changes in the Workflow pipeline)
    using Bitbucket
-   Have the target branch (usually master) of a Bitbucket project
    repository be managed by Workflow. When a change is approved in
    Workflow, it will perform the merge in Bitbucket

Bitbucket integration is designed for use with Bitbucket 3.x or 4.x and
supports connecting a Workflow enterprise with a single Bitbucket server
URL. Workflow does not support Bitbucket project repositories that are
hosted at <https://bitbucket.org> at this time. Workflow communicates
with Bitbucket using the Bitbucket REST API and HTTP-based git protocol.
Workflow authenticates with Bitbucket with a user account over HTTP
basic authentication. It is recommended that the connection be protected
using SSL.

## Trust SSL Certificate

Run the following steps to set up self-signed certificates for Workflow.

{{< note >}}

This is only required if the Bitbucket uses a self-signed SSL
certificate or an internal certificate authority.

{{< /note >}}

### Debian

For the Debian platform, do the following:

1.  Log into the Workflow server as root.

2.  Run the following command:

    ``` bash
    cd /usr/local/share/ca-certificates
    ```

3.  Run the following command:

    ``` bash
    openssl s_client -showcerts -connect {BITBUCKET_SERVER}:443 </dev/null 2>/dev/null|openssl x509 -outform PEM >{BITBUCKET_SERVER}.crt
    ```

4.  Run the following command:

    ``` bash
    update-ca-certificates
    ```

### RHEL, Centos

For Red Hat Enterprise Linux (6.x and higher) and CentOS (6.x and
higher), do the following:

1.  Log into the Workflow server as root.

2.  Run the following command:

    ``` bash
    yum install ca-certificates
    ```

    {{< note spaces=4 >}}

    For 6.x servers, run this command only once.

    {{< /note >}}

3.  Run the following command:

    ``` bash
    update-ca-trust force-enable
    ```

    {{< note spaces=4 >}}

    For 6.x servers, run this command only once.

    {{< /note >}}

4.  Run the following command:

    ``` bash
    cd /etc/pki/ca-trust/source/anchors/
    ```

5.  Run the following command:

    ``` bash
    openssl s_client -showcerts -connect {BITBUCKET_SERVER}:443 </dev/null 2>/dev/null|openssl x509 -outform PEM >{BITBUCKET_SERVER}.crt
    ```

6.  Run the following command:

    ``` bash
    update-ca-trust extract
    ```

## Set up Integration

Bitbucket integration with Workflow has the following requirements:

1.  Shell access with `sudo` permission to the Workflow server
2.  A Workflow user account with `admin` role in the Workflow enterprise
    used for this integration
3.  The URL for the Bitbucket instance
4.  The username and password of a Bitbucket user to use as the service
    account. This user must have full access (read/write) to the
    projects you wish to add to Workflow

### Add to Workflow

In the Workflow web UI, open the SCM setup page, and then complete the
following fields:

-   **Bitbucket URL** - The URL for your Bitbucket instance.
-   **Bitbucket Username** - The username of the service account that
    Workflow will use to interact with Bitbucket.
-   **Bitbucket Password** - The password for the service account.

Then submit the form to complete this step.

### Remove from Workflow

To remove integration with Bitbucket:

1.  Update all projects that are integrated with Bitbucket to be
    integrated with Workflow.
2.  In the Workflow webui, open the **SCM Setup** page.
3.  Click the **Remove Link** button.

### Update Integration

To update integration with Bitbucket:

1.  In the Workflow web UI, open the **SCM Setup** page, and update
    Bitbucket credentials and make changes to the appropriate
    information.
2.  Click the **Update** button.

## Create a Project

Repeat these steps for each Bitbucket project to be added to Workflow:

1.  A project repository in Bitbucket with at least one commit.
2.  The service account used by Workflow must have full access to this
    repository.
3.  All team members should have read-only access to this repository.
    Workflow will manage creation of pull requests and merging of pull
    requests to Bitbucket.

### Add an Empty Project

Use the following steps to add an empty project from the Workflow web
UI:

1.  Open the page for the organization in the Workflow webui, and then
    click **Add a New Project**.
2.  Select the **Bitbucket** option from the **Source Code Provider**
    bar, and then and enter the Bitbucket project key, repository, and
    target branch.
3.  Click **Save & Close**.

### Import Project

You can repeat these steps for each Bitbucket project to be added to
Workflow:

1.  Create a local clone of the project **from Bitbucket** and `cd` into
    it.

2.  Create a `.delivery/cli.toml` using `delivery setup`:

    ``` bash
    delivery setup --ent=$DELIVERY_ENTERPRISE --org=$DELIVERY_ORG --user=$DELIVERY_USER_NAME --server=$DELIVERY_SERVER
    ```

3.  Run `delivery init` to push the code to the empty project in
    Workflow (as created above). After importing the code, this command
    will generate a `.delivery/config.json` file, create a build
    cookbook, and submit a change to Workflow to initialize a pipeline
    for the project. Changes are opened in the Workflow web UI. At this
    point, a corresponding pull request is shown in Bitbucket.

    {{< readFile_shortcode file="delivery_cli_init_bitbucket_project.md" >}}

### Convert Project to Bitbucket

To convert a project that is integrated with Workflow to one that is
integrated with Bitbucket:

1.  Ensure that a project repository exists in Bitbucket with at least
    one commit.
2.  Ensure that the service account used by Workflow has full access to
    this project repository.
3.  Ensure that team members who will use this project have read-only
    access to this project repository. (Workflow will manage the
    creation and merging of pull requests.)
4.  In the Workflow web user interface, open the **Organizations** page.
5.  Click the pencil button for the project to be updated.
6.  Click the **Bitbucket** tab.
7.  Enter the project key and the project repository name.
8.  Click **Save and Close**.

### Convert Project to Workflow

To convert a project that is integrated with Bitbucket to one that is
integrated with Workflow:

1.  Merge or close all open changes for the project.
2.  In the Workflow webui, open the **Organizations** page.
3.  Click the pencil button for the project to be updated.
4.  Click the **Chef Delivery** tab.
5.  Click **Save and Close**.

### Update Bitbucket Project

To update the information for a project that is integrated with
Bitbucket:

1.  In the Workflow web UI, open the **Organizations** page.
2.  Click the pencil button for the project to be updated.
3.  Click the **Bitbucket** tab.
4.  Edit the project key and the project repository name.
5.  Click **Save and Close**.

## Workflow w/Bitbucket

This section describes the setup and workflow for a team member who will
interact with a Bitbucket project that is integrated with Workflow. It
is assumed that the initial project is created, imported, and that a
Workflow pipeline already exists.

### Delivery CLI

Perform the following steps to install the Delivery CLI and setup your
project:

1.  [Install the Delivery CLI](/delivery_cli/#install-delivery-cli).

2.  In the command shell, create or navigate to the directory in which
    project repositories are located.

3.  Use the `delivery setup` command with the following arguments to
    create the `.delivery/cli.toml` file:

    ``` bash
    delivery setup --ent=$DELIVERY_ENTERPRISE --org=$DELIVERY_ORG --user=$DELIVERY_USER --server=$DELIVERY_SERVER
    ```

4.  Create a local clone of the project repository:

    ``` bash
    delivery clone $PROJECT
    ```

    If the project is cloned from Bitbucket (or if a pre-existing clone
    is used), add it using `delivery remote`. The URL for
    `delivery clone` can be found on the project's page in the Workflow
    web UI.

5.  Create a remote with the following:

    ``` bash
    git remote add delivery $DELIVERY_CLONE_URL
    ```

### Create a Change

Use the following steps to create a change in Workflow:

1.  Create and check out a topic branch for the change, based on the
    current state of your project's pipeline (usually from `master`).
    For example: `git checkout -b great-feature`.
2.  Make and commit changes to the project as normal.
3.  Submit the change to Workflow with the command `delivery review`.
    This command will open a URL at which details and progress of the
    change may be viewed from the Workflow web UI.
4.  Verification of changes will begin automatically and a corresponding
    pull request will be opened in Bitbucket.

### Code Review

You may conduct a code review using either Workflow or Bitbucket.
However, merging a pull request is handled by Workflow and occurs when a
change in Workflow is approved. You cannot merge the pull request from
within Bitbucket.

To perform code review using Workflow:

1.  Use the URL created by `delivery review` to go directly to the
    change in the Workflow web UI, or browse to the change from the
    Workflow dashboard.
2.  Click the **Review** tab.
3.  Browse the changes and make comments.

### Approve a Change

When verification is finished in Workflow and the code has been reviewed
and is ready to be merged, approve the change. The pull request will be
merged and closed in Bitbucket. The feature branch will also be deleted
in Bitbucket.

1.  Use the URL created by `delivery review` to go directly to the
    change in the Workflow web UI, or browse to the change from the
    Workflow dashboard.
2.  Click the **Review** tab.
3.  Click **Approve**.

### Delete a Change

When verification is finished in Workflow, the code has been reviewed,
and it is decided the change should never be approved, delete the change
in Workflow; the pull request will be declined and closed in Bitbucket.
The feature branch will also be deleted in Bitbucket.

Use the URL created by `delivery review` to go directly to the change,
or browse to the change from the dashboard in the Workflow web UI.

1.  Click the **Review** tab.
2.  Click **Delete**.
