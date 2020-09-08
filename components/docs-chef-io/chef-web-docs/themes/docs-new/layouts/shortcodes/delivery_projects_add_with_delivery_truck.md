<div class="admonition-note"><p class="admonition-note-title">Note</p><div class="admonition-note-text">

These instructions assume that you will use Chef Automate as your source
code source of truth and that Chef Automate is not integrated with
GitHub Enterprise or GitHub.com.

</div></div>

This topic describes the recommended setup for a Chef cookbook project
using Chef Automate.

The following example shows how to create a cookbook, with project and
pipeline, configure it to be built with Chef Automate, and then imported
it into Chef Automate itself. From your workstation as user with admin
privileges on the Chef Automate server, do the following:

1.  Make a working directory (`workspace` in the example):

    ``` bash
    mkdir ~/workspace && cd ~/workspace
    ```

2.  Setup the Delivery CLI to, by default, contact the Chef Automate
    server at SERVER, with a default ENTERPRISE and ORGANIZATION:

    ``` bash
    delivery setup --server=SERVER --ent=ENTERPRISE --org=ORGANIZATION --user=USERNAME
    ```

    <div class="admonition-note">

    <p class="admonition-note-title">Note</p>

    <div class="admonition-note-text">

    The server, enterprise, organization, and user must already exist.

    

    </div>

    </div>

3.  Create a cookbook:

    ``` bash
    chef generate cookbook NEW-COOKBOOK-NAME
    ```

    ``` bash
    cd NEW-COOKBOOK-NAME
    ```

    This uses Chef Workstation to generate a new cookbook, including a
    default recipe and default ChefSpec tests.

4.  Create an initial commit (use `git status` to verify the change) on
    the "master" branch:

    ``` bash
    git add .
    ```

    ``` bash
    git commit -m 'Initial Commit'
    ```

    Running `chef generate` initialized a git repository automatically
    for this cookbook. If you created the build cookbook manually,
    initialize the git repository with the `git init` command.

5.  Initialize the cookbook for Chef Automate:

    ``` bash
    delivery init
    ```

    This creates a new project in Chef Automate, pushes the master
    branch, creates a feature branch, generates a default Chef Automate
    project configuration file, pushes the first change for review, and
    then opens a browser window that shows the change.

6.  Now that you have initialized your project, it is recommended that
    you integrate the delivery-truck cookbook with your project.
    Delivery Truck can ensure good build cookbook behavior as well as
    provide you with recipes already set up to test your project
    cookbooks and applications.