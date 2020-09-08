+++
title = "Configure a Project through config.json"
draft = false
robots = "noindex"


aliases = ["/config_json_delivery.html", "/release/automate/config_json_delivery.html"]

[menu]
  [menu.legacy]
    title = "Configure a Project"
    identifier = "legacy/workflow/workflow_basics/config_json_delivery.md Configure a Project"
    parent = "legacy/workflow/workflow_basics"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_json_delivery.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

The `config.json` file is located at the root of the `.delivery` folder
that is located within a project managed by Workflow and configures that
project to publish to a pipeline.

## Structure

The structure of the `config.json` file is similar to:

``` json
{
  "version": "2",
  "build_cookbook": {
    "name": "build-cookbook",
    "path": ".delivery/build-cookbook"
  },
  "build_nodes": {
    "default"    : ["name:name_of_builder"]
  },
  "skip_phases": [
    "quality",
    "security"
  ],
  "delivery-truck":{
    "publish": {
      "github": "chef/chef-web-docs"
    }
  }
}
```

### Configuration Settings

{{% delivery_config_json_setting %}}

`build_cookbook`

:   **Required**

    {{< readFile_shortcode file="delivery_config_json_setting_build_cookbook.md" >}}

`build_nodes`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_build_nodes.md" >}}

`delivery-truck`

:   **Optional**

    The `delivery-truck` setting specifies configurations for specific
    phases of the Chef Automate pipeline:

    ``` javascript
    "delivery-truck": {
      "lint": {
        "foodcritic": {
          "only_rules": ["FC002"]
        }
      }
      "publish": {
        "chef_server": "true"
        "github": "chef/chef-web-docs"
        "git": "ssh://git@stash:2222/<project-name>/<repo-name>"
        "supermarket": "https://supermarket.chef.io",
        "supermarket-custom-credentials": "true"
      }
      ...
      <more_phases>
      ...
    }
    ```

`dependencies`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_dependencies.md" >}}

<a id="job-dispatch-config-settings" markdown="1"></a>
<!-- link from runners.md -->

`job_dispatch`

:   **Optional**

    The `job_dispatch` setting is needed to use the [improved SSH job
    dispatch system](/runners/). If you use this setting, you must
    remove any `build_nodes` settings from your configuration file.

    -   `"version"` Set the value to "v2" if you wish to use runners and
        the new job dispatch system:

        ``` javascript
        {
           ...
           "job_dispatch": {
              "version": "v2"
           ...
        }
        ```

        {{< note spaces=8 >}}

        If you omit this setting or set it to "v1", the previous job
        dispatch system using Push Jobs 1.x will be used instead.

        {{< /note >}}

    -   `"filters"` Similar to the former Chef Push Jobs-based dispatch
        system, you can set a variety of filters. Filters control which
        runners can run a job for a Chef Automate project. You can set
        filters for the entire project as well as specific filters per
        phase. You can also specify a matrix of filters to a run phase
        job repeatedly on multiple platform configurations.

        The values you can filter on are `os`, `platform`,
        `platform_family`, and `platform_version`. If you omit a value,
        job dispatch will not filter on it.

        **Using a default filter**

        If you wish to use a default filter for the entire project, you
        need to set the "default" tag. For example, if you wanted your
        project phase jobs to be executed on only ubuntu and centos
        platforms, you could write:

        ``` javascript
        {
           ...
           "job_dispatch": {
              "version": "v2",
              "filters" : {
                 "default" : {
                   "os" : ["linux"],
                   "platform" : ["ubuntu", "centos"]
                 }
              }
           ...
        }
        ```

        **Using a phase filter**

        Phase filters are filters that can be set per phase. They
        override a default filter for that phase if a default is set.
        You can specify a phase filter without setting a default. For
        example, to run the project's deploy phase specifically on
        Fedora 6 based systems that overrides a default of Windows, you
        could write:

        ``` javascript
        {
           ...
           "job_dispatch": {
              "version": "v2",
              "filters" : {
                 "default" : {
                   "os" : ["windows"]
                 }
                 "unit" : {
                   "platform_family" : ["fedora"],
                   "platform_version" : ["6"]
                 }
              }
           ...
        }
        ```

        **Using a matrix phase filter**

        You can set up a matrix of sub-jobs to run a phase on multiple
        platform configurations. This is itself a phase filter,
        overriding the default filter but running the phase job
        repeatedly on multiple runners. Matrix filters are only for
        phase filters and not the default filter.

        For example, if you want to unit test your project across
        multiple versions of Ubuntu, you could write something like:

        ``` javascript
        {
           ...
           "job_dispatch": {
              "version": "v2",
              "filters" : {
                 "unit" :
                 [
                    {
                      "platform_family" : ["ubuntu"],
                      "platform_version" : ["12.04"]
                    },
                    {
                      "platform_family" : ["ubuntu"],
                      "platform_version" : ["14.04"]
                    },
                    {
                      "platform_family" : ["ubuntu"],
                      "platform_version" : ["16.04"]
                    }
                 ]
              }
           ...
        }
        ```

`skip_phases`

:   **Optional**

    {{< readFile_shortcode file="delivery_config_json_setting_skip_phases.md" >}}


`version`

:   **Required**

    {{< readFile_shortcode file="delivery_config_json_setting_version.md" >}}

{{< note >}}

{{% delivery_cookbook_delivery_truck %}}

{{< /note >}}

### Phase Settings

The individual phases of Chef Automate may be configured, grouped under
the `delivery-truck` configuration setting by phase.

#### publish

The `publish` phase configuration settings specify the location(s) to
which cookbooks are published.

**Chef Infra Server**

{{% delivery_config_json_setting_delivery_truck_publish_chef_server %}}

**git**

{{% delivery_config_json_setting_delivery_truck_publish_git %}}

**GitHub**

{{% delivery_config_json_setting_delivery_truck_publish_github %}}

**Supermarket**

{{% delivery_config_json_setting_delivery_truck_publish_supermarket %}}

{{% delivery_config_json_setting_delivery_truck_publish_supermarket_private %}}

{{% delivery_config_json_setting_delivery_truck_publish_supermarket_credentials %}}

{{< note >}}

To enable Chef Automate to upload cookbooks to a private Supermarket,
you have to manually log into the Supermarket server with the `delivery`
user, and when it prompts you to enable the user for Supermarket, enter
`yes`. Also, you must copy the Supermarket certificate file to
`/etc/delivery/supermarket.crt` on the Chef Automate server.

{{< /note >}}

**Multiple Locations**

If the `config.json` file may specify some or all of the publish options
together as a single block:

``` javascript
"delivery-truck":{
  "publish": {
    "chef_server": "true"
    "github": "chef/chef-web-docs"
    "git": "ssh://git@stash:2222/<project-name>/<repo-name>"
    "supermarket": "https://supermarket.chef.io",
    "supermarket-custom-credentials": "true"
  }
}
```

or:

``` javascript
"delivery-truck":{
  "publish": {
    "chef_server": "true"
    "supermarket": "https://supermarket.chef.io"
  }
}
```

## Examples

The following examples show different ways to specify settings and
pipeline behaviors in the `config.json` file.

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

### Stages and Platforms

The `"build_nodes"` section may also specify build nodes by stages
and/or platform:

``` javascript
{
  ...
    "build_nodes": {
      "default"    : ["name:builder"],
      "unit"       : ["name:builder AND platform_family:platform"],
      "..."        : ["name:builder AND platform_family:platform"]
    }
  ...
}
```

For example:

``` javascript
{
  ...
    "build_nodes": {
      "default"    : ["name:builder*.foo.com"],
      "unit"       : ["name:builder*.foo.com AND platform_family:debian"],
      "syntax"     : ["name:builder*.foo.com AND platform_family:rhel"],
      "publish"    : ["name:builder*.foo.com AND platform_family:debian", "name:builder*.foo.com AND platform_family:rhel"]
    }
  ...
}
```

### Test Patterns

{{% delivery_config_example_test_patterns %}}

#### Foodcritic, excludes

{{% delivery_config_json_setting_delivery_truck_lint_foodcritic_excludes %}}

#### Foodcritic, ignore_rules

{{% delivery_config_json_setting_delivery_truck_lint_foodcritic_ignore_rules %}}

#### Foodcritic, only_rules

{{% delivery_config_json_setting_delivery_truck_lint_foodcritic_only_rules %}}
