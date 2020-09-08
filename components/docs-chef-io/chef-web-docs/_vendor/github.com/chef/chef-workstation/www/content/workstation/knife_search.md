+++
title = "knife search"
draft = false

aliases = ["/knife_search.html", "/knife_search/"]

[menu]
  [menu.workstation]
    title = "knife search"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_search.md knife search"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_search.md)

{{% search %}}

{{% knife_search_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife search INDEX SEARCH_QUERY
```

where `INDEX` is one of `client`, `environment`, `node`, `role`, or the
name of a data bag and `SEARCH_QUERY` is the search query syntax for the
query that will be executed.

`INDEX` is implied if omitted, and will default to `node`. For example:

``` bash
knife search '*:*' -i
```

will return something similar to:

``` bash
8 items found

centos-62-dev
opensuse-15
ubuntu-1604-dev
ubuntu-1804-orgtest
ubuntu-1804-ohai-test
ubuntu-1804-ifcfg-test
ohai-test
win2k19-dev
```

and is the same search as:

``` bash
knife search node '*:*' -i
```

If the `SEARCH_QUERY` does not contain a colon character (`:`), then the
default query pattern is
`tags:*#{@query}* OR roles:*#{@query}* OR fqdn:*#{@query}* OR addresses:*#{@query}*`,
which means the following two search queries are effectively the same:

``` bash
knife search ubuntu
```

or:

``` bash
knife search node "tags:*ubuntu* OR roles:*ubuntu* OR fqdn:*ubuntu* (etc.)"
```

### Query Syntax

{{% search_query_syntax %}}

### Keys

{{% search_key %}}

To search for the available fields for a particular object, use the
`show` argument with any of the following knife subcommands:
`knife client`, `knife data bag`, `knife environment`, `knife node`, or
`knife role`. For example: `knife data bag show`.

#### Nested Fields

{{% search_key_nested %}}

#### Examples

{{% search_key_name %}}

{{% search_key_wildcard_question_mark %}}

{{% search_key_wildcard_asterisk %}}

{{% search_key_nested_starting_with %}}

{{% search_key_nested_range %}}

### About Patterns

{{% search_pattern %}}

#### Exact Matching

{{% search_pattern_exact %}}

{{% search_pattern_exact_key_and_item %}}

{{% search_pattern_exact_key_and_item_string %}}

#### Wildcard Matching

{{% search_pattern_wildcard %}}

{{% search_pattern_wildcard_any_node %}}

{{% search_pattern_wildcard_node_contains %}}

#### Range Matching

{{% search_pattern_range %}}

{{% search_pattern_range_in_between %}}

{{% search_pattern_range_exclusive %}}

#### Fuzzy Matching

{{% search_pattern_fuzzy %}}

{{% search_pattern_fuzzy_summary %}}

### About Operators

{{% search_boolean_operators %}}

{{% search_boolean_operators_andnot %}}

#### AND

{{% search_boolean_and %}}

#### NOT

{{% search_boolean_not %}}

#### OR

{{% search_boolean_or %}}

### Special Characters

{{% search_special_characters %}}

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`-a ATTR`, `--attribute ATTR`

:   The attribute (or attributes) to show.

`-b ROW`, `--start ROW`

:   The row at which return results begin.

`-f FILTER`, `--filter-result FILTER`

:   Use to filter the search output based on the pattern that matches
    the specified `FILTER`. Only attributes in the `FILTER` will be
    returned. For example: `\"ServerName=name, Kernel=kernel.version\`.

`-i`, `--id-only`

:   Show only matching object IDs.

`INDEX`

:   The name of the index to be queried: `client`, `environment`,
    `node`, `role`, or `DATA_BAG_NAME`. Default index: `node`.

`-l`, `--long`

:   Display all attributes in the output and show the output as JSON.

`-m`, `--medium`

:   Display normal attributes in the output and to show the output as
    JSON.

`-q SEARCH_QUERY`, `--query SEARCH_QUERY`

:   Protect search queries that start with a hyphen (-). A `-q` query
    may be specified as an argument or an option, but not both.

`-r`, `--run-list`

:   Show only the run-list.

`-R INT`, `--rows INT`

:   The number of rows to be returned.

`SEARCH_QUERY`

:   The search query used to identify a list of items on a Chef Infra
    Server. This option uses the same syntax as the `search` subcommand.

## Examples

The following examples show how to use this knife subcommand:

**Search by platform ID**

{{% knife_search_by_platform_ids %}}

**Search by instance type**

{{% knife_search_by_platform_instance_type %}}

**Search by recipe**

{{% knife_search_by_recipe %}}

**Search by cookbook, then recipe**

{{% knife_search_by_cookbook %}}

**Search by node**

{{% knife_search_by_node %}}

**Search by node and environment**

{{% knife_search_by_node_and_environment %}}

**Search for nested attributes**

{{% knife_search_by_nested_attribute %}}

**Search for multiple attributes**

{{% knife_search_by_query_for_many_attributes %}}

**Search for nested attributes using a search query**

{{% knife_search_by_query_for_nested_attribute %}}

**Use a test query**

{{% knife_search_test_query_for_ssh %}}
