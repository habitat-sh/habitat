+++
title = "About the delivery-truck Cookbook"
draft = false
robots = "noindex"


aliases = ["/delivery_truck.html", "/release/automate/delivery_truck.html"]

[menu]
  [menu.legacy]
    title = "delivery-truck (cookbook)"
    identifier = "legacy/workflow/managing_workflow/delivery_truck.md delivery-truck (cookbook)"
    parent = "legacy/workflow/managing_workflow"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_truck.md)

{{% delivery_cookbook_delivery_truck %}}

{{< note >}}

The `delivery-truck` cookbook has a dependency on the `delivery-sugar`
cookbook, which contains a set of helper methods and custom resources
that may be used in build cookbook recipes. Using these helper methods
and custom resources in a build cookbook is optional.

{{< /note >}}

## delivery-truck Recipes

The following recipes are available by default in the `delivery-truck`
cookbook:

`default.rb`

:   {{% delivery_cookbook_common_recipe_default %}}

`deploy.rb`

:   {{% delivery_cookbook_common_recipe_deploy %}}

`functional.rb`

:   {{% delivery_cookbook_common_recipe_functional %}}

`lint.rb`

:   {{% delivery_cookbook_common_recipe_lint %}}

`provision.rb`

:   {{% delivery_cookbook_common_recipe_provision %}}

`publish.rb`

:   {{% delivery_cookbook_common_recipe_publish %}}

`quality.rb`

:   {{% delivery_cookbook_common_recipe_quality %}}

`security.rb`

:   {{% delivery_cookbook_common_recipe_security %}}

`smoke.rb`

:   {{% delivery_cookbook_common_recipe_smoke %}}

`syntax.rb`

:   {{% delivery_cookbook_common_recipe_syntax %}}

`unit.rb`

:   {{% delivery_cookbook_common_recipe_unit %}}

## Create a build-cookbook

{{% delivery_cookbook_setup %}}

{{< note >}}

This section assumes that Chef Automate is already configured, a project
exists, a user may access that project and submit changes, and that all
work is being done from that project's root directory.

{{< /note >}}

### Edit the Berksfile

{{% delivery_cookbook_setup_berksfile %}}

### Edit metadata.rb

{{% delivery_cookbook_setup_metadata %}}

### Add delivery-truck to Recipes

{{% delivery_cookbook_setup_recipes %}}

## Read the Tutorial

To learn more about how to set up a project pipeline for a single
cookbook and basic web application, follow the steps outlined in the
[Deploy infrastructure changes with Chef
Automate](https://learn.chef.io/modules/deploy-infrastructure#/) module
on Learn Chef.

## Project Cookbooks

A project cookbook is a cookbook that is located within a project and is
used to deploy that project's software onto one (or more) nodes in the
Chef Automate pipeline. These cookbooks are located in the `/cookbooks`
directory, which should exist at the root of the project (similar to the
`.delivery` directory).

The `default.rb` recipe in a project cookbook is executed by Chef Infra
Client on infrastructure nodes as the project moves through the Chef
Automate pipeline. The `provision.rb` recipe discovers all `metadata.rb`
and/or `metadata.json` files in the project, including those under the
`/cookbooks` directory.

### Single Cookbook

A project may use a single cookbook to tell Chef Infra Client how to
configure nodes in the Chef Automate pipeline.

#### Add Project Cookbook

Create a project cookbook. From the project's root directory, do the
following:

1.  Create a branch:

    ``` bash
    delivery checkout BRANCH_NAME
    ```

2.  Generate a cookbook under `/cookbooks` in the project directory:

    ``` bash
    chef generate cookbook PROJECT_NAME
    ```

3.  Review the `metadata.rb` file. It should be similar to:

    ``` ruby
    name 'my_project'
    maintainer 'The Authors'
    maintainer_email 'you@example.com'
    license 'all_rights'
    description 'Installs/Configures my_project'
    long_description 'Installs/Configures my_project'
    version '0.1.0'
    ```

    where `version '0.1.0'` must be changed if files within the cookbook
    change. The version number is what gets promoted through the stages
    in the Chef Automate pipeline.

#### Configure default.rb

In the `default.rb` recipe, define how this project is to be deployed.
This is a normal Chef recipe that is executed by Chef Infra Client, so
do the same in this recipe as you would do in any other.

#### Promote the Project

When a change to a project is submitted to Chef Automate, the
`provision.rb` does the work of promoting the project to the various
nodes in the Chef Automate pipeline.

To submit changes to Chef Automate, use commands similar to:

1.  Update to match the working tree:

    ``` bash
    git add -A
    ```

2.  Commit the project:

    ``` bash
    git commit -m "Let us deploy our app."
    ```

3.  Review the changes in Chef Automate:

    ``` bash
    delivery review
    ```

    This command will open the Chef Automate web UI, and then run unit,
    lint, and syntax tests. After the tests pass, the change may be
    approved. Once approved, the `provision.rb` recipe will deploy the
    project onto the acceptance stage's infrastructure nodes.

4.  After the change has built successfully through the Acceptance
    stage, approve the changes by clicking the **Deliver** button in the
    Chef Automate web UI. This sends the project to the Union,
    Rehearsal, and Delivered stages.

#### Update the Project

Update a file in the project, and then update the version number in the
`metadata.rb` file. This ensures this cookbook is promoted, overwriting
the old project cookbook, and then updating the project across each
stage of the Chef Automate pipeline:

1.  Check out the project from Chef Automate:

    ``` bash
    delivery checkout master
    ```

2.  Create a branch:

    ``` bash
    delivery checkout BRANCH_NAME
    ```

3.  Edit the `version` in the `metadata.rb` file:

    ``` ruby
    version '0.2.0'
    ```

    and then make the desired changes.

4.  Update to match the working tree:

    ``` bash
    git add -A
    ```

5.  Add a commit message:

    ``` bash
    git commit -m "Updated our project's code to version 0.2.0."
    ```

6.  Review the changes in Chef Automate:

    ``` bash
    delivery review
    ```

### Multiple Cookbooks

Some projects need more than one project cookbook. Put as many cookbooks
as necessary under the `/cookbooks` directory, which is located at the
root of a project.

Each cookbook under the `/cookbooks` directory must have a valid
cookbook structure. If the cookbook does not have a `metadata.rb` or
`metadata.json` file it will not be discovered by the `provision.rb`
recipe; consequently, that cookbook will not be used to configure nodes
in the Chef Automate pipeline.

The `default.rb` recipes in all project cookbooks are executed by Chef
Infra Client on infrastructure nodes as the project moves through the
Chef Automate pipeline. The `default.rb` recipe in the `build-cookbook`
is run first, and then each `default.rb` recipe in each cookbook under
`/cookbooks` is run (in alphabetical order, by cookbook name).

## Project Applications

A project may be a binary, a package, or some other set of arbitrary
information. The Chef Automate pipeline supports promoting projects
through the pipeline using versioned attributes. This is known as a
project application. A project application is a useful way to promote
projects by using a set of attributes that are pinned to a specific
version, and then using those same versioned attributes when deploying
software to various stages in the Chef Automate pipeline.

### Configure Project Application

Project applications are defined in the `publish.rb` recipe in a
`build-cookbook` using the `define_project_application` helper method,
and then in the `deploy.rb` recipe using the `get_project_application`
method. The publish phase happens at the end of the build stage. It is
at this point where the project application version is pinned, uploaded
to the Chef Infra Server as a data bag item, and then used through the
remaining stages.

{{< note >}}

The `define_project_application` helper method is available from the
`delivery-sugar` cookbook, which is a dependency of the `delivery-truck`
cookbook. This helper is available when the `publish.rb` recipe has
`include_recipe 'delivery-truck::publish'` defined.

{{< /note >}}

To define a project application, do the following:

1.  Open the `publish.rb` recipe in the `build-cookbook` and edit it to
    contain:

    ``` ruby
    define_project_application(
      <app_name>,
      <app_version>,
      [ 'attribute',
        'attribute',
        ... ]
    )
    ```

    where

    -   `<app_name>` is the name of the project application
    -   `<app_version>` is version number to which the project
        application is pinned
    -   `'attribute'` is Hash of attributes associated with this
        version; each attribute is defined as a key-value pair:
        `'key = value'`

2.  Set up the `build-cookbook` to know about this application. Add the
    following to `.delivery/build-cookbook/attributes/default.rb`:

    ``` ruby
    default['delivery']['project_apps'] = ['<app_name>', '<app_name>', ...]
    ```

    where `<app_name>` is a list of one (or more) applications this
    `build-cookbook` should be aware of.

    {{< note spaces=4 >}}

    If the `/attributes/default.rb` directory and/or file does not
    exist, create it.

    {{< /note >}}

3.  Open the `default.rb` recipe in the `build-cookbook` and edit it to
    contain:

    ``` ruby
    { 'hash_of_attributes' } = get_project_application(<app_name>)
    ```

    where `'hash_of_attributes'` is a list of one (or more) attributes
    defined in the `define_project_application` block.

    {{< note spaces=4 >}}

    Do not pass `'id'`, `'version'`, or `'name'` as part of the
    `'hash_of_attributes'` as these are already defined in the
    `define_project_application` block, are pulled in automatically by
    the `get_project_application` helper method, and will overwrite any
    value specified in the Hash.

    {{< /note >}}

### Example Project Application

This example shows how to use project applications to deploy a package
into a `.deb` file during the deploy phase. (This example assumes a Chef
Automate project exists with a properly configured `build-cookbook`.)

1.  Open the `publish.rb` recipe in the `build-cookbook` and edit to
    look like the following:

    ``` ruby
    include_recipe 'delivery-truck::publish'

    # Generate your artifact and document it's location on a download server.
    artifact_location = <generated_artifact_location>

    # It's recommended to generate a checksum from your package too.
    artifact_checksum = <package_checksum>

    # Version the artifact based on the current date.
    artifact_version = Time.now.strftime('%F_%H%M')

    # Name your application.
    name = "<app_name>"

    project_app_attributes = {
      'artifact_location' => artifact_location,
      'artifact_checksum' => artifact_checksum
    }

    define_project_application(
      name,
      artifact_version,
      project_app_attributes
    )
    ```

2.  In the `publish.rb` recipe, update `<generated_artifact_location>`
    and `<package_checksum>` to be correct for this project.

3.  Set up the `build-cookbook` to know about this application. Add the
    following to `.delivery/build-cookbook/attributes/default.rb`:

    ``` ruby
    default['delivery']['project_apps'] = ["<app_name>"]
    ```

    where `<app_name>` is the same value as the name of the application
    in the `publish.rb` file.

    When the publish phase is run, an application is created, versioned
    by timestamp, and including all of the information needed to install
    that version of the application. The provisioning code in
    `delivery-truck` will automatically pin based on this version.

4.  Configure the `build-cookbook` to know how to install the
    application. Add the following to
    `.delivery/build-cookbook/deploy.rb`:

    ``` ruby
    app_attributes = get_project_application("<APPLICATION_NAME>")

    # Download your package.
    remote_file "/tmp/latest_package.deb" do
      source   app_attributes['artifact_location']
      checksum app_attributes['artifact_checksum']
      action :create
    end

    # Install it onto your build infrastructure.
    package app_attributes['name'] do
      source "/tmp/latest_package.deb"
      action :install
    end
    ```

## Validate the Installation

The surest way to validate a Chef Automate installation is to create a
cookbook, and then submit it to Chef Automate to kick off a new build in
the pipeline.

If a project is a cookbook, we recommend starting with `delivery-truck`,
an open source build cookbook created for driving cookbook pipelines in
Chef Automate. You can customize some aspects of `delivery-truck`
through your project's `.delivery/config.json`. To have more control or
to opt-out of some of the behavior of `delivery-truck`, create a wrapper
build cookbook.

{{% delivery_projects_add_with_delivery_truck %}}

## Using `delivery-truck` in air-gapped environment

Chef Automate can be set up to deploy cookbooks and applications in an
air-gapped environment and this section describes how to set up a basic
cookbook to be delivered through Chef Automate using the [delivery-truck
cookbook](https://github.com/chef-cookbooks/delivery-truck) in that
environment.

{{< note >}}

By default, the delivery-truck cookbook is configured for use with Chef
Automate-backed cookbook projects.

{{< /note >}}

### Prerequisites

-   Ensure you have a private Supermarket installed, setup, and running.
    See [Install Private Supermarket](/install_supermarket/) for
    more information.
-   Ensure you have a Chef Infra Server with the Chef Identity
    authentication/authorization service configured, a Chef Automate
    server setup that references your private Supermarket, and at least
    one Chef Automate build node/runner installed, setup, and running.
    See [Install Chef Automate](/install_chef_automate/) and [Chef
    Identity](/install_supermarket/#chef-identity.html) for more
    information.
-   Ensure you have created a project in Chef Automate. Follow these
    instructions to [Set Up
    Projects](/delivery_build_cookbook/#set-up-projects).
-   Ensure you have [Chef
    Workstation](https://downloads.chef.io/chef-workstation/) installed
    on your [workstation](/workstation/).

### Share cookbooks with your private Supermarket

To use `delivery-truck` and its dependency, `delivery-sugar`, you must
first share them with a private Supermarket that is authenticated with
your Chef Infra Server.

1.  From a workstation, create a cookbooks directory, `$COOKBOOKS_DIR`:

    ``` bash
    mkdir -p $COOKBOOKS_DIR
    ```

2.  Clone the `delivery-truck` cookbook and its dependency
    `deliver-sugar` from GitHub:

    ``` bash
    cd $COOKBOOKS_DIR
    git clone https://github.com/chef-cookbooks/delivery-sugar.git
    git clone https://github.com/chef-cookbooks/delivery-truck.git
    ```

3.  To ensure your private Supermarket does not try to connect to
    third-party services, log into it and set the `AIR_GAPPED`
    environment variable to `'true'` in the
    `/etc/supermarket/supermarket.rb` file.

    ``` ruby
    default['supermarket']['air_gapped'] = 'true'
    ```

4.  Save your changes and close the file.

5.  Reconfigure your private Supermarket.

    ``` bash
    supermarket-ctl reconfigure
    ```

6.  Share the `delivery-truck` and `delivery-sugar` cookbooks with your
    private Supermarket using the `knife` command-line tool. If you have
    not configured `knife` to share cookbooks with your private
    Supermarket, see [Upload a
    Cookbook](/supermarket/#upload-a-cookbook) before running the
    following `knife` subcommands.

    ``` bash
    knife supermarket share 'delivery-truck'
    knife supermarket share 'delivery-sugar'
    ```

### Generate a cookbook

1.  Use Chef Workstation's [cookbook generator
    command](/ctl_chef/#chef-generate-cookbook) to create a default
    cookbook directory structure called `my_cookbook`.

    ``` bash
    chef generate cookbook my_cookbook
    ```

2.  Run `delivery init` in your `my_cookbook` local directory to create
    a new project in Chef Automate and push your first change for
    review.

    ``` bash
    cd my_cookbook
    delivery init
    ```

3.  Finally, check out the added files and commit your changes.

### Use the `delivery-truck` cookbook in your project

From the root of your project's directory, do the following:

1.  Modify the build cookbook's Berksfile to reference `delivery-truck`
    and `delivery-sugar`. By default, this file is located at
    `.delivery/build-cookbook/Berksfile`.

    ``` ruby
    source 'https://your_private_supermarket_url'

    metadata

    group :delivery do
      cookbook 'delivery-sugar'
      cookbook 'delivery-truck'
    end
    ```

2.  Modify the build cookbook's metadata to include `delivery-truck`. By
    default, this file is located at
    `.delivery/build-cookbook/metadata.rb`.

    ``` ruby
    depends 'delivery-truck'
    ```

3.  Edit your build cookbook's recipes to include the corresponding
    `delivery-truck` recipe.

    ``` none
    # Cookbook Name:: $BUILD_COOKBOOK_NAME
    # Recipe:: $RECIPE
    #
    # Copyright (c) 2016 The Authors, All Rights Reserved.

    include_recipe "delivery-truck::$RECIPE"
    ```

    By default, each build cookbook recipe `$RECIPE` is located at
    `.delivery/build-cookbook/recipes/$RECIPE.rb`.

4.  Increment your build cookbook's version in the cookbook's metadata
    file.

5.  Commit your changes and run `delivery review`. Changes to your
    cookbook project can now be managed by your Chef Automate cluster.
