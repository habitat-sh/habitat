+++
title = "Manage Users and Roles"
draft = false
robots = "noindex"


aliases = ["/delivery_users_and_roles.html", "/release/automate/delivery_users_and_roles.html"]

[menu]
  [menu.legacy]
    title = "Users and Roles"
    identifier = "legacy/workflow/managing_workflow/delivery_users_and_roles.md Users and Roles"
    parent = "legacy/workflow/managing_workflow"
    weight = 110
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_users_and_roles.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

This topic describes the roles and permissions that may be assigned to
users of Chef Automate, how to integrate an LDAP system with Chef
Automate, how to add and edit users, and how to add user SSH keys.

## Roles and Permissions

Chef Automate has a standard users and roles permissions scheme. Roles
are sets of permissions defined by Chef Automate. Users can be assigned
multiple roles.

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Role</th>
<th>Permissions</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><strong>Admin</strong></td>
<td>All of the permissions plus all administrative permissions, including user management. Can create and delete users and restore passwords. Also, can create, delete, or modify organizations, projects, and pipelines. Can modify, delete, or add recipes in a phase (at the source code level); and can read and write comments.</td>
</tr>
<tr class="even">
<td><strong>Committer</strong></td>
<td>Can commit a change to Chef Automate. Also, can modify, delete, or add recipes in a phase (at the source code level); and can read and write comments.</td>
</tr>
<tr class="odd">
<td><strong>Observer</strong></td>
<td>Can observe the actions and results of the pipeline as the change progresses through the stages. Cannot take any action.</td>
</tr>
<tr class="even">
<td><strong>Reviewer</strong></td>
<td>Can approve the successful completion of the <strong>Verify</strong> stage. Once approved, the change automatically moves to the <strong>Build</strong> stage. Also, can read and write comments.</td>
</tr>
<tr class="odd">
<td><strong>Shipper</strong></td>
<td>Can ship a change from the <strong>Acceptance</strong> stage into the shared pipeline stages, <strong>Union</strong> <strong>Rehearsal</strong> and <strong>Delivered</strong>.</td>
</tr>
</tbody>
</table>

## Integrate LDAP

{{% delivery_integration_ldap %}}

### LDAP Attributes

{{% delivery_integration_ldap_attributes %}}

### Configure LDAP

{{% delivery_integration_ldap_configure %}}

### Manage Users

{{% delivery_integration_ldap_users %}}

#### Add

{{% delivery_integration_ldap_users_add %}}

#### Edit

{{% delivery_integration_ldap_users_edit %}}

## Onboard Users

Onboarding users to a project is different depending on whether you have
integrated with GitHub or not.

### Chef Automate with GitHub

Once a project is created, you will want to add users to that project so
that they can submit changes and collaborate via the Chef Automate
shared workflow using GitHub.

You may integrate Chef Automate and GitHub Enterprise or
[GitHub.com](https://github.com/). If you do this, you will be able to
use GitHub as a **Source Code Provider** when creating a project.

{{< note >}}

Before you can follow this procedure, you must have integrated Chef
Automate and GitHub Enterprise or [GitHub.com](https://github.com/).
This is the same procedure whether you have Amazon Web Services (AWS)
provisioning or SSH provisioning.

{{< /note >}}

#### Add Users

You must associate a GitHub user with a Chef Automate user in order to
successfully create changes from GitHub pull requests.

To onboard a user for an integrated GitHub Enterprise project or one
that is hosted at [GitHub.com](https://github.com/):

1.  Have the user that you want to add clone the repo for the project
    you want them to join. Ensure that they have write permissions to
    the repo if you want to allow them to submit pull requests.

2.  Add or edit any users who are managed by the LDAP integration.

3.  From a local checkout of a Chef Automate project, run the
    appropriate Chef Automate command that associates a GitHub user with
    a Chef Automate user.

    {{< note spaces=4 >}}

    The Delivery CLI commands are for a user to link their own account
    to GitHub, or others if the user has the **Admin** role; `api` is an
    argument to the Delivery CLI command. The `automate-ctl` command can
    only be run by an administrator from the Chef Automate server and
    can affect any user.

    {{< /note >}}

    For GitHub Enterprise:

    ``` bash
    delivery api put users/$AUTOMATE_USERNAME/set-oauth-alias --data='{"app_name":"github-enterprise","alias":"$GITHUB_USERNAME"}'
    ```

    For GitHub:

    ``` bash
    delivery api put users/$AUTOMATE_USERNAME/set-oauth-alias --data='{"app_name":"github","alias":"$GITHUB_USERNAME"}'
    ```

    *Or*, as an administrator, run the command line tool `automate-ctl`.
    The command uses the enterprise name you set when configuring Chef
    Automate. The username can be an LDAP username (if LDAP integration
    has been completed), or an internal username:

    For GitHub Enterprise:

    ``` bash
    automate-ctl link-github-enterprise-user $AUTOMATE_ENTERPRISE_NAME $AUTOMATE_USERNAME $GITHUB_USERNAME
    ```

    For GitHub:

    ``` bash
    automate-ctl link-github-user $AUTOMATE_ENTERPRISE_NAME $AUTOMATE_USERNAME $GITHUB_USERNAME
    ```

The associated user can now checkout the repository, make changes on a
feature branch and submit the changes for review.

Note the following constraints:

-   You may not link two GitHub accounts to a single Chef Automate user.
-   Two users may not share a GitHub account

#### Submit Changes

For an integrated GitHub Enterprise project or a project that is hosted
on [GitHub.com](https://github.com/), users of Chef Automate should
submit changes as follows:

1.  The standard GitHub process should be followed:

    -   Clone the desired repository
    -   Make and test changes locally
    -   Submit the changes and initiate the Chef Automate review process
        by creating a pull request with the `delivery review` command

    The GitHub webui will display a **Delivery Status** box showing what
    part of the pipeline the pull request is at. When the pull request
    has passed the **Verify** stage, GitHub will message you in the
    GitHub webui that approval must be manually entered for the pipeline
    to proceed.

2.  When the "Approval Required" message appears, enter
    `@delivery approve` in the comment box.

    The pull request moves to the next stage of the Chef Automate
    pipeline, **Build** and **Acceptance**.

3.  When the pull request has passed the **Acceptance** stage, GitHub
    will add another message indicating that that the `deliver` command
    must be issued for the pipeline to proceed. When this message
    appears, enter `@delivery deliver` in the comment box.

    The pull request moves to the final three stages, **Union**,
    **Rehearsal**, and **Delivered**. Other changes in the pipeline that
    would conflict with a change in the **Union** stage, are blocked
    from proceeding to the **Acceptance** stage.

    When the final **Delivered** stage is passed, GitHub updates the
    **Delivery Status** at the top of the GitHub webui page.

### Chef Automate with Internal git

Once a project is created, you will want to add users to that project so
that they can submit changes and collaborate via the Chef Automate
shared workflow. These procedures apply to Chef Automate deployments
that are using the internal Chef Automate git capabilities and are not
integrated to GitHub Enterprise or [GitHub.com](https://github.com/).

#### Add Users

To onboard a user that is not using GitHub Enterprise or a project
hosted at [GitHub.com](https://github.com/), but only the default git
that comes with Chef Automate:

1.  Add or edit any users who are managed by the LDAP integration.
2.  Have the user log into the Chef Automate web UI and add their SSH
    public key to their profile.

The associated user can now create a feature branch and submit changes
to Chef Automate for review.

#### Submit Changes

The change submission process is the familiar git process:

1.  You must be onboarded to Chef Automate, a task likely to be done by
    your sysadmin. Once your GitHub username is linked to your Chef
    Automate username and you have properly set up a workstation.
2.  Clone the GitHub repo to which changes are submitted. Be sure you
    have the right permissions.
3.  Workflow for making changes:
    1.  Create feature branch: `git checkout -b <feature_branch_name>`.
    2.  Make changes.
    3.  Build and test the changes locally.
    4.  Check status: `git status`.
    5.  Add changes: `git add .` or `git add <changed file>`.
    6.  Commit changes: `git commit -m <message>`.
    7.  Submit changes to delivery: `delivery review`. The Chef Automate
        web UI will open to show your change in the pipeline. Note, you
        may need to be on a VPN to access Chef Automate.
    8.  When the change has passed **Verify**, approve change, or get
        someone to, by clicking **Approve** in Chef Automate web UI.
        Doing this marks you as the "Signed-off-by" user of the change.
    9.  After change is approved, sync your local branch to master:
        `git checkout master` and then `git pull delivery master`.
    10. Press the **Deliver** button in the Chef Automate web UI when it
        is active. Note that your change may be superseded by another
        change. That is, if another change in the pipeline is approved
        (merged to master) and then your change is approved, when
        **Deliver** is pressed, both changes are moved to the final
        three stages. This goes for all approved changes in the
        pipeline. Also note that changes that would conflict with
        approved changes will not be moved past **Acceptance**.

## Add User SSH Keys

First install the Delivery CLI, and then generate the user's SSH keys.

### Install the CLI

{{% delivery_cli_install %}}

### Configure the CLI

{{% delivery_cli_configure %}}

### Add SSH Keys

To add SSH keys to Chef Automate, do the following:

1.  Check for an SSH key:

    ``` bash
    cat .ssh/id_rsa.pub
    ```

    if it returns:

    ``` none
    No such file or directory
    ```

2.  Create an SSH key (without a passphrase):

    ``` bash
    ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
    ```

    The output is similar to:

    ``` none
    Generating public/private rsa key pair.
    Enter file in which to save the key (/Users/username/.ssh/id_rsa):
    Enter passphrase (empty for no passphrase):
    Enter same passphrase again:
    Your identification has been saved in /Users/path/to/.ssh/id_rsa.
    Your public key has been saved in /Users/path/to/.ssh/id_rsa.pub.
    The key fingerprint is:
    ac:8a:57:90:58:c1:cd:34:32:18:9d:f3:79:60:f3:41 your_email@chef.io
    The key's randomart image is:
    +--[ RSA 4096]----+
    |  .==*o.E        |
    |  . *o*..        |
    |   o + = .       |
    |  . o o.o        |
    |     . ..S       |
    |      ..         |
    |     ..          |
    |   .*o*.         |
    |  ...            |
    +-----------------+
    ```

3.  Run the following:

    ``` bash
    cat .ssh/id_rsa.pub
    ```

    The output is similar to:

    ``` none
    ssh-rsa
    AAAAB3NzaC1yc2EAAAADAQABAAACAQDa8BR/9bj5lVUfQP9Rsqon5qJMkiVm+JAtGi
    wnhxqgyRhkYLIzm6+gcifDgMOMuwZA88Ib5WNRhxjlmTseapower4rH/jAAczdp1h1
    7xLEEbUfQfkcqiy/Drp3k12345678ad234fgvdsasdfasdfR9ddNIeNvQ7OIpOCfLE
    PCyFz3aRRuhpM/5cySFT7bl1O44bNgfiuqRzcXFscZb03WPlhaPwCvL2uxaRzdrAGQ
    mE5jzCo6nORvKoGdVDa2++def33f3xPZCo3oJ08Q9XJ2CnfJlmyNe1hwI2NOQ3yRbc
    nfSMona7ccSyHRWGs5bS//u6P0NK5AqH5jK8pg3XwtHZqLwUVy1wX0WnnJWg9IWXf3
    2g3P4O4NJGVUeX33Czv32GK8YphuEweqFu/Ej7kQp1ppIxkEtrpBfMi3na0QqZlk6w
    wghZLa++DUfWOhGsuuBgnsocAR5rLGy+gkypdie1Ydoe8qjLVZR/jKybQfQjuZOS30
    fZnwJhl2ZaeraPfkEXlVhK02/8PIALGfeXdt9KvQN0p5c6lRoDxqBqslM+1KbKKcGd
    lSGEsAIP9OOWBECRxlOwqlqGHtrgWKOr376dntMIy2+fFD/74tJMjRwbRzm8IGWmj6
    OcF6EvTYYO4RmISD8G+6dm1m4MlxLS53aZQWgYWvRdfNB1DA
    Zo3h9Q== your_email@chef.io
    ```

4.  Copy the SSH key and add it to Chef Automate.

    Log into the Chef Automate web UI as an administrator.

    Select **Users** from the drop-down menu on the upper right.

    On the **Users** list page, select the user name; use the search
    filter in the upper right if needed.

    Under **Security Information**, paste the SSH key.

    Click **Save & Close**.

5.  Setup Chef Automate for that user. Run the following:

    ``` bash
    delivery setup --server SERVER_DNS --user USERNAME --ent ENTERPRISE --org ORGANIZATION
    ```

    The output is similar to:

    ``` none
    Chef Delivery
    Loading configuration from /Users/USERNAME
    Writing configuration to /Users/USERNAME/.delivery/cli.toml
    New configuration
    -----------------
    api_protocol = "https"
    enterprise = "ENTERPRISE"
    git_port = "8989"
    organization = "ORGANIZATION"
    pipeline = "master"
    server = "SERVER_DNS"
    user = "USERNAME"
    ```

6.  Clone a repo from Chef Automate:

    ``` bash
    delivery clone PROJECT_REPO
    ```

    The output is similar to:

    ``` none
    Chef Delivery
    Loading configuration from /Users/USERNAME/Desktop
    Cloning ssh://USERNAME@chef@SERVER_DNS:8989/ENTERPRISE/ORGANIZATION/PROJECT to PROJECT
    The authenticity of host '[SERVER_DNS]:8989 ([10.100.10.50]:8989)' can't be established.
    RSA key fingerprint is 42:8d:92:31:9e:55:b0:06:28:b7:35:a9:4a:87:47:9d.
    Are you sure you want to continue connecting (yes/no)? yes
    adding remote delivery: ssh://USERNAME@ENTERPRISE@SERVER_DNS:8989/ENTERPRISE/ORGANIZATION/PROJECT
    ```

The user can now create a local branch, make and submit changes to Chef
Automate.
