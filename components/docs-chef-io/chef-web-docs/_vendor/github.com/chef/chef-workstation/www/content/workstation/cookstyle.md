+++
title = "Cookstyle"
draft = false

aliases = ["/cookstyle.html", "/rubocop.html", "/cookstyle/"]

[menu]
  [menu.workstation]
    title = "Cookstyle"
    identifier = "chef_workstation/chef_workstation_tools/cookstyle.md Cookstyle"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 90
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/cookstyle.md)

Cookstyle is a code linting tool that helps you write better Chef Infra
cookbooks by detecting and automatically correcting style, syntax, and
logic mistakes in your code.

Cookstyle is powered by the RuboCop linting engine. RuboCop ships with
over three-hundred rules, or cops, designed to detect common Ruby coding
mistakes and enforce a common coding style. We've customized Cookstyle
with a subset of those cops that we believe are perfectly tailored for
cookbook development. We also ship Chef-specific cops that catch common
cookbook coding mistakes, cleanup portions of code that are no longer
necessary, and detect deprecations that prevent cookbooks from running
on the latest releases of Chef Infra Client.

Cookstyle increases code quality by:

-   Enforcing style conventions and best practices.
-   Helping every member of a team author similarly structured code.
-   Maintaining uniformity in the source code.
-   Setting expectations for fellow (and future) project contributors.
-   Detecting deprecated code that creates errors after upgrading to a
    newer Chef Infra Client release.
-   Detecting common Chef Infra mistakes that cause code to fail or
    behave incorrectly.

## Cookstyle vs. Rubocop

Cookstyle is more stable than Rubocop and is customized for Chef
Cookbook code. This means that linting Cookbooks with Cookstyle will be
more consistent and less likely to produce CI test failures.

### Tailored cops

Cookbook development differs from traditional Ruby software development,
so we maintain a tailored set of built-in cops from Rubocop. Cops that
are not useful for cookbook development are disabled and occasionally we
change the configuration of a rule to enforce a different behavior.
We've also extended the base RuboCop package with a set of our own Chef
Infra-specific cops. These cops are only found in Cookstyle and will
help you to write more reliable and future-proof cookbooks.

### New cops

New cops are continuously added to Rubocop. New cops can make existing
codebases fail CI tests and force authors to constantly update their
code.

With Cookstyle, we update the RuboCop engine for bug and performance
fixes, but we only change the set of cops that will fail tests once a
year during Chef Infra's major release in April. All new cops are
introduced at RuboCop's "refactor" alert level, meaning they will alert
to the screen as you run Cookstyle, but they won't fail a build. This
stability means you are free to upgrade releases of Cookstyle without
being forced to update your infrastructure code.

## Run Cookstyle

Cookstyle is run from the command line, typically against a single
cookbook and all of the Ruby files contained within it:

``` bash
cookstyle /path/to/cookbook
```

Cookstyle may also be run from the root of an individual cookbook
directory:

``` bash
cookstyle .
```

Cookstyle returns a list, via standard output, that shows the results of
the evaluation:

``` bash
Inspecting 8 files
CWCWCCCC

Offences:

cookbooks/apache/attributes/default.rb:1:1: C: Missing utf-8 encoding comment.
default["apache"]["indexfile"] = "index1.html"
^
cookbooks/apache/attributes/default.rb:1:9: C: Prefer single-quoted strings when you don't
need string interpolation or special symbols.
default["apache"]["indexfile"] = "index1.html"
        ^^^^^^^^
cookbooks/apache/attributes/default.rb:1:19: C: Prefer single-quoted strings when you
don't need string interpolation or special symbols.
default["apache"]["indexfile"] = "index1.html"
                  ^^^^^^^^^^^
```

### Output

Cookstyle output:

-   States the number of files found and examined. For example:
    `Inspecting 8 files`
-   Lists the results of those files as a series of symbols. For
    example: `CWCWCCCC`
-   For each symbol, states the file name, line number, character
    number, type of issue or error, describes the issue or error, and
    specifies the location in the source code at which the issue or
    error is located

A Cookstyle evaluation has the following syntax:

``` none
FILENAME:LINE_NUMBER:CHARACTER_NUMBER: TYPE_OF_ERROR: MESSAGE
SOURCE CODE
^^^^^^^^^^^
```

For example:

``` none
cookbooks/apache/attributes/default.rb:1:9: C: Prefer single-quoted strings when you don't
need string interpolation or special symbols.
default["apache"]["indexfile"] = "index1.html"
        ^^^^^^^^
```

#### Symbols

The following symbols appear in the standard output and are used to
indicate the result of an evaluation:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Symbol</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>.</code></td>
<td>The file does not have any issues.</td>
</tr>
<tr class="even">
<td><code>C</code></td>
<td>The file has an issue with convention.</td>
</tr>
<tr class="odd">
<td><code>E</code></td>
<td>The file contains an error.</td>
</tr>
<tr class="even">
<td><code>F</code></td>
<td>The file contains a fatal error.</td>
</tr>
<tr class="odd">
<td><code>W</code></td>
<td>The file contains a warning.</td>
</tr>
<tr class="even">
<td><code>R</code></td>
<td>The file contains code that should be refactored.</td>
</tr>
</tbody>
</table>

## Autocorrecting Cookstyle Warnings

Many of the Cookstyle Cops include the ability to autocorrect
violations. To autocorrect code run the following from the cookbook
directory:

``` bash
cookstyle -a .
```

Take particular care after running this command to ensure the
autocorrection logic resulted in appropriate cookbook code.

## .rubocop.yml

Use a .rubocop.yml file in a cookbook to override the default settings
in Cookstyle for enabled and disabled rules. Only enabled rules---either
in the `enabled.yml` file in Cookstyle itself or rules that are
specifically enabled in a cookbook's .rubocop.yml file---will be used
during the evaluation. Any rule that becomes unhelpful should be
disabled in the .rubocop.yml file.

Each cookbook has its own .rubocop.yml file, which means that each
cookbook may have its own set of enabled, disabled, and custom rules.
That said, it's more common for all cookbooks to have the same set of
enabled, disabled, and custom rules. When RuboCop is run against a
cookbook, the full set of enabled and disabled rules (as defined the
`enabled.yml` and `disabled.yml` files in Cookstyle itself) are loaded
first, and are then compared against the settings in the cookbook's
.rubocop.yml file.

Custom rules should be specified in the .rubocop.yml file. The state of
rules---enabled or disabled---in a .rubocop.yml file take precedence
over the state of rules defined in the `enabled.yml` and `disabled.yml`
files.

### Syntax

A .rubocop.yml file has the following syntax:

``` yaml
NAME_OF_RULE:
  Description: 'a description of a rule'
  Enabled : (true or false)
  KEY: VALUE
```

where

-   `NAME_OF_RULE` is the name of a rule
-   `Description` is the string that prints as part of the standard
    output that describes the rule if it is triggered during the
    evaluation
-   `Enabled` enables a rule (`true`) or disables a rule (`false`); for
    non-custom rules, this value will override the settings in the
    `enabled.yml` and `disabled.yml` files in Cookstyle
-   `KEY: VALUE` adds additional details for a rule, if necessary. For
    example, `Max: 200` sets the line length to 200 characters for the
    `LineLength` rule

### .rubocop_todo.yml

Use a .rubocop_todo.yml file to capture the current state of all
evaluations, and then write them to a file. This allows evaluations to
reviewed one at a time. Disable any evaluations that are unhelpful, and
then address the ones that are.

To generate the .rubocop_todo.yml file, run the following command:

``` bash
cookstyle --auto-gen-config
```

{{< note >}}

Rename this file to .rubocop.yml to adopt this evaluation state as the
standard. Include this file in the .rubocop.yml file by adding
`inherit_from: .rubocop_todo.yml` to the top of the .rubocop.yml file.

{{< /note >}}
