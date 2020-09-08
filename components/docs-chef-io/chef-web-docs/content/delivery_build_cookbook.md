+++
title = "About the build-cookbook"
draft = false
robots = "noindex"


aliases = ["/delivery_build_cookbook.html", "/release/automate/delivery_build_cookbook.html"]

[menu]
  [menu.legacy]
    title = "build-cookbook (cookbook)"
    identifier = "legacy/workflow/managing_workflow/delivery_build_cookbook.md build-cookbook (cookbook)"
    parent = "legacy/workflow/managing_workflow"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_build_cookbook.md)

Chef Automate uses Chef Infra Client to run recipes for each phase in a
build pipeline. The phases are grouped into different stages.

The following illustration shows the phases of each pipeline stage.

<img src="/images/delivery_build_cookbook.svg" class="align-center" width="600" alt="image" />

The recipes for these phases are run from the `build-cookbook`. A
`build-cookbook` varies by project type, because projects may use
different tools for running unit tests, syntax checks, or lint analysis.

## Build Cookbook Structure

A `build-cookbook` is located in the `.delivery` directory in a project
and defines how the Chef Automate pipeline will build, test, and deploy
a project. A `build-cookbook` should be initially configured to use the
`delivery-truck` cookbook as a dependency in all recipes, after which it
may be modified as necessary. The `build-cookbook` is effectively a
wrapper cookbook for the `delivery-truck` cookbook.

A build node is configured via two isolated Chef Infra Client runs:
First, the `default.rb` recipe is run by Chef Infra Client as the root
user, after which the phase-specific recipe is run by Chef Infra Client
as the build user (`dbuild`). For example, during the unit phase the
first run is the `default.rb` file, and then the second is the `unit.rb`
file.

The following recipes should be configured to include the corresponding
`delivery-truck` recipe as a dependency:

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

## Create Build Cookbook

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

## Set Up Projects

Chef Automate uses projects to organize work across multiple teams. You
can create as many projects as you need. A common approach is to have
one project for each major component of the system. Each project has its
own git repository.

Each project has one (or more) pipelines. Each pipeline has a designated
target branch into which it will merge approved changes. Chef Automate
uses a "gated master" model that manages merges to the target branch.
The typical setup is for each project to have a single pipeline that
targets the master branch.

### Use the Delivery CLI

{{% delivery_projects_add_with_delivery_truck %}}

### Use the Web UI

To add a project using the Chef Automate web UI:

1.  Log into the Chef Automate web UI as user with **Admin** role.

2.  Open the **Organizations** page and select your organization.

3.  Click the plus sign (**+**) next to **Add a New Project**.

4.  Enter a project name and select a **Source Code Provider**, either
    **Chef Delivery** (the default), **GitHub**, or **Bitbucket**.

5.  If you choose **Chef Delivery**, simply click **Save and Close** to
    finish adding the project.

6.  If you choose **GitHub**, a text area opens. Enter the following:

    **GitHub Organization Name**

    **GitHub Project Name**

    **Pipeline Branch** The name of the target branch that Chef Automate
    will manage (most projects will have master as the target branch).
    The target branch must exist in the repository.

    **Verify SSL** When selected, have GitHub perform SSL certificate
    verification when it connects to Chef Automate to run its web hooks.

7.  If you choose **Bitbucket**, you must follow the integration steps
    in [Integrate Delivery with
    Bitbucket](/integrate_delivery_bitbucket/) before you can add a
    project. After you have done that you can add a new Chef Automate
    project through this web UI by entering the Bitbucket project key,
    repository, and target branch information.

8.  Click **Save and Close**.

## Custom build-cookbook

`chef generate` can also create a custom build cookbook for use with
Delivery:

``` bash
chef generate build-cookbook NAME [options]
```

The following options are available with `chef generate build-cookbook`:

``` none
-C, --copyright COPYRIGHT        Name of the copyright holder - defaults to 'The Authors'
-m, --email EMAIL                Email address of the author - defaults to 'you@example.com'
-a, --generator-arg KEY=VALUE    Use to set arbitrary attribute KEY to VALUE in the code_generator cookbook
-h, --help                       Show this message
-I, --license LICENSE            all_rights, apachev2, mit, gplv2, gplv3 - defaults to all_rights
-v, --version                    Show chef version
-g GENERATOR_COOKBOOK_PATH,      Use GENERATOR_COOKBOOK_PATH for the code_generator cookbook
       --generator-cookbook
```
