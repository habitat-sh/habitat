+++
title = "Debug Recipes, Chef Infra Client Runs"
draft = false

aliases = ["/debug.html"]

[menu]
  [menu.infra]
    title = "Debug Recipes, Client Runs"
    identifier = "chef_infra/cookbook_reference/recipes/debug.md Debug Recipes, Client Runs"
    parent = "chef_infra/cookbook_reference/recipes"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/debug.md)

Elements of good approaches to building cookbooks and recipes that are
reliable include:

-   A consistent syntax pattern when constructing recipes
-   Using the same patterns in Ruby
-   Using resources included in Chef Infra Client or community cookbooks
    before creating custom ones

Ideally, the best way to debug a recipe is to not have to debug it in
the first place. That said, the following sections discuss various
approaches to debugging recipes and failed Chef Infra Client runs.

## Basic

Some simple ways to quickly identify common issues that can trigger
recipe and/or Chef Infra Client run failures include:

-   Using an empty run-list
-   Using verbose logging with knife
-   Using logging with Chef Infra Client
-   Using the **log** resource in a recipe to define custom logging

### Empty Run-lists

{{% node_run_list_empty %}}

### Knife

Use the verbose logging that is built into knife:

`-V`, `--verbose`

:   Set for more verbose outputs. Use `-VV` for much more verbose
    outputs. Use `-VVV` for maximum verbosity, which may provide more
    information than is actually helpful.

{{< note >}}

Plugins do not always support verbose logging.

{{< /note >}}

### Chef Infra Client

Use the verbose logging that is built into Chef Infra Client:

`-l LEVEL`, `--log_level LEVEL`

:   The level of logging to be stored in a log file. Possible levels:
    `auto` (default), `debug`, `error`, `fatal`, `info`, `trace`, or `warn`.
    Default value: `warn` (when a terminal is available) or `info` (when
    a terminal is not available).

`-L LOGLOCATION`, `--logfile c`

:   The location of the log file. This is recommended when starting any
    executable as a daemon. Default value: `STDOUT`.

### log Resource

{{% resource_log_summary %}}

New in 12.0, `-o RUN_LIST_ITEM`. Changed in 12.0 `-f` no longer allows
unforked intervals, `-i SECONDS` is applied before a Chef Infra Client
run.

#### Syntax

{{% resource_log_syntax %}}

#### Actions

{{% resource_log_actions %}}

#### Properties

{{% resource_log_properties %}}

#### Examples

The following examples demonstrate various approaches for using
resources in recipes:

**Specify a log entry**

{{% resource_log_set_info %}}

**Set debug logging level**

{{% resource_log_set_debug %}}

**Create log entry when the contents of a data bag are used**

{{% resource_log_set_debug %}}

**Add a message to a log file**

{{% resource_log_add_message %}}

## Advanced

Some more complex ways to debug issues with a Chef Infra Client run
include:

-   Using the **chef_handler** cookbook
-   Using the chef-shell and the **breakpoint** resource to add
    breakpoints to recipes, and to then step through the recipes using
    the breakpoints
-   Using the `debug_value` method from chef-shell to identify the
    location(s) from which attribute values are being set
-   Using the `ignore_failure` method in a recipe to force Chef Infra
    Client to move past an error to see what else is going on in the
    recipe, outside of a known failure
-   Using chef-solo to run targeted Chef Infra Client runs for specific
    scenarios

### chef_handler

{{% handler %}}

{{% handler_types %}}

Read more [about exception, report, and start handlers](/handlers/).

### chef-shell

{{% chef_shell_summary %}}

{{% chef_shell_modes %}}

#### Configure

{{% chef_shell_config %}}

#### chef-shell.rb

{{% chef_shell_config_rb %}}

#### Run as a Chef Infra Client

{{% chef_shell_run_as_chef_client %}}

#### Manage

{{% chef_shell_manage %}}

### breakpoint Resource

{{% chef_shell_breakpoints %}}

{{% resource_breakpoint_summary %}}

#### Syntax

{{% resource_breakpoint_syntax %}}

#### Actions

{{% resource_breakpoint_actions %}}

#### Attributes

{{% resource_breakpoint_properties %}}

#### Examples

The following examples demonstrate various approaches for using
resources in recipes:

**A recipe without a breakpoint**

{{% resource_breakpoint_no %}}

**The same recipe with breakpoints**

{{% resource_breakpoint_yes %}}

### Step Through Run-list

{{% chef_shell_step_through_run_list %}}

### Debug Existing Recipe

{{< readFile_shortcode file="chef_shell_debug_existing_recipe.md" >}}

### Advanced Debugging

{{< readFile_shortcode file="chef_shell_advanced_debug.md" >}}

### debug_value

Use the `debug_value` method to discover the location within the
attribute precedence hierarchy from which a particular attribute (or
sub-attribute) is set. This method is available when running chef-shell
in Chef Infra Client mode:

``` bash
chef-shell -z
```

For example, the following attributes exist in a cookbook. Some are
defined in a role file:

``` ruby
default_attributes 'test' => {'source' => 'role default'}
override_attributes 'test' => {'source' => 'role override'}
```

And others are defined in an attributes file:

``` ruby
default[:test][:source]  = 'attributes default'
set[:test][:source]      = 'attributes normal'
override[:test][:source] = 'attributes override'
```

To debug the location in which the value of `node[:test][:source]` is
set, use chef-shell and run a command similar to:

``` none
pp node.debug_value('test', 'source')
```

This will pretty-print return all of the attributes and sub-attributes
as an array of arrays; `:not_present` is returned for any attribute
without a value:

``` bash
[['set_unless_enabled?', false],
 ['default', 'attributes default'],
 ['env_default', :not_present],
 ['role_default', 'role default'],
 ['force_default', :not_present],
 ['normal', 'attributes normal'],
 ['override', 'attributes override'],
 ['role_override', 'role override'],
 ['env_override', :not_present],
 ['force_override', :not_present],
 ['automatic', :not_present]]
```

where

-   `set_unless_enabled` indicates if the attribute collection is in
    `set_unless` mode; this typically returns `false`
-   Each attribute type is listed in order of precedence
-   Each attribute value shown is the value that is set for that
    precedence level
-   `:not_present` is shown for any attribute precedence level that has
    no attributes

A [blog post by Joshua
Timberman](http://jtimberman.housepub.org/blog/2014/09/02/chef-node-dot-debug-value/)
provides another example of using this method.

### ignore_failure Method

All resources share a set of common actions, attributes, and so on. Use
the following attribute in a resource to help identify where an issue
within a recipe may be located:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Attribute</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>ignore_failure</code></td>
<td>Continue running a recipe if a resource fails for any reason. Default value: <code>false</code>.</td>
</tr>
</tbody>
</table>

### chef-solo

See [chef-solo (executable)](/ctl_chef_solo/) for complete CTL
documentation.

{{% chef_solo_summary %}}

See [chef-solo (executable)](/ctl_chef_solo/) for complete CTL
documentation.
