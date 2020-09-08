+++
title = "About Chef Automate Pipelines"
draft = false
robots = "noindex"


aliases = ["/delivery_pipeline.html", "/release/automate/delivery_pipeline.html"]

[menu]
  [menu.legacy]
    title = "Configure a Pipeline"
    identifier = "legacy/workflow/workflow_basics/delivery_pipeline.md Configure a Pipeline"
    parent = "legacy/workflow/workflow_basics"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_pipeline.md)



{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Each project contains a configuration file in its source repository,
located at `.delivery/config.json`, that specifies the build cookbook to
use for the project, and in turn, the build cookbook contains recipes
that control what happens in the pipeline phases. The config file also
allows customization of the behavior of Chef Automate and the build
cookbook. You can create a config file (as well as a build cookbook)
using the Chef Automate CLI tool and the init subcommand:
`delivery init` or `delivery init --local`.

When Chef Automate executes a phase, it selects a build node or runner
to run the job. On the build node/runner, the project's source is
fetched and synchronized to the revision matching the head of the
feature branch for the change. The build node/runner reads the project's
`config.json` file and uses this information to fetch the appropriate
build cookbook. Finally, the build node/runner runs a local chef-zero
run to execute the appropriate phase.

If you are using Chef Automate to manage changes in Chef cookbooks, you
can wrap, or use directly, `delivery-truck`, a build cookbook for
building and testing cookbooks. The `delivery-truck` and
`delivery-sugar` cookbooks contain helpers that can be used for
non-cookbook workflows as well. You can wrap or modify the
`delivery-truck` cookbook to suit your own needs.

Here is an example of a build cookbook recipe that runs JUnit tests with
Maven. For example:

``` ruby
log "Running unit"

repo = node['delivery_builder']['repo']

execute "run my JUnit tests" do
  command "mvn test"
  cwd repo
end
```

This code logs that the unit tests are running and runs JUnit tests
against the current repo.

{{% delivery_config_example_test_patterns %}}

Because build cookbooks read the configuration file, use the
configuration file to customize the build cookbook to suit the needs of
a particular project. In this way, you can share some "standard" version
of a build cookbook with others and then use extra data in the config
file to tailor the cookbook as needed.

## Configuration Settings

{{% delivery_config_json_setting %}}

`version`

:   **Required**

    {{< readFile_shortcode file="delivery_config_json_setting_version.md" >}}

`build-cookbook`

:   **Required**

    {{< readFile_shortcode file="delivery_config_json_setting_build_cookbook.md" >}}

`build_nodes`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_build_nodes.md" >}}

`skip_phases`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_skip_phases.md" >}}

`dependencies`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_dependencies.md" >}}

{{< note >}}

{{% delivery_cookbook_delivery_truck %}}

{{< /note >}}

### build-cookbook Locations

The following examples show how to specify the location of the
`build-cookbook`.

**A local directory**

{{% delivery_config_example_build_cookbook_local %}}

**A git source**

{{% delivery_config_example_build_cookbook_git %}}

**A public Supermarket (https://supermarket.chef.io)**

{{% delivery_config_example_build_cookbook_supermarket_public %}}

**A private Supermarket**

{{% delivery_config_example_build_cookbook_supermarket_private %}}

**A Chef server**

{{% delivery_config_example_build_cookbook_server %}}

**A Chef Automate server**

{{% delivery_config_example_build_cookbook_automate_server %}}

### Build Nodes and Phases

{{% delivery_config_example_build_nodes_by_phase %}}

### Run-time Dependencies

{{% delivery_config_example_dependencies_on_master %}}

### Multiple Pipelines

To set up a second pipeline, there is an assumption that the branch that
will become the second pipeline exists in your local project.

1.  Push that branch to the Chef Automate Server
    `git push delivery $BRANCH_NAME`
2.  Navigate to the project's page
    (`e/$ENT_NAME/#/organizations/$ORG_NAME/projects/$PROJECT_NAME`) in
    the Chef Automate web UI and click on the `Pipelines` tab.
3.  Click on `Add A New Pipeline` on the top of the page.
4.  Give the new pipeline a descriptive name and input the base branch.

To make a pipeline other than `master` the default for a single project,
create a `cli.toml` file in the `/.delivery` directory in the root of
the project that includes `pipeline = "$BRANCH_NAME"`. Now, all
delivery-cli commands that target a pipeline will target \$BRANCH_NAME.

If you wish to target a pipeline that is NOT the defined default, add
the `--pipeline=$BRANCH_NAME` flag to the delivery-cli command.

Example:

`delivery review --pipeline=$BRANCH_NAME`

The commands that take this flag are:

-   `delivery init`
-   `delivery review`
-   `delivery diff`
-   `delivery job`
-   `delivery setup`
