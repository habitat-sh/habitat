+++
title = "About Foodcritic"
draft = false

aliases = ["/foodcritic.html", "/foodcritic/"]

[menu]
  [menu.workstation]
    title = "Foodcritic"
    identifier = "chef_workstation/chef_workstation_tools/foodcritic.md Foodcritic"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 110
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/foodcritic.md)

{{< warning >}}

Foodcritic is deprecated and should no longer be used for cookbook
linting. Use [Cookstyle](/workstation/cookstyle/) instead which offers additional
rules, violation autocorrection, Ruby linting, and a robust
configuration system.

{{< /warning >}}

Use Foodcritic to check cookbooks for common problems:

-   Style
-   Correctness
-   Syntax
-   Best practices
-   Common mistakes
-   Deprecations

Foodcritic looks for lint-like behavior and reports it!

Foodcritic is a static linting tool that analyzes all of the Ruby code
that is authored in a cookbook against a number of rules, and then
returns a list of violations. Because Foodcritic is a static linting
tool, using it is fast. The code in a cookbook is read, broken down, and
then compared to Foodcritic rules. The code is **not** run (a Chef Infra
Client run does not occur). Foodcritic does not validate the intention
of a recipe, rather it evaluates the structure of the code, and helps
enforce specific behavior, detect portability of recipes, identify
potential run-time failures, and spot common anti-patterns.

When Foodcritic returns a violation, this does not automatically mean
the code needs to be changed. It is important to first understand the
intention of the rule before making the changes it suggests. For
example, rule `FC003` describes a scenario where a recipe uses the
`search` method in the Recipe DSL to retrieve data from the Chef Infra
Server. Rule `FC003` may suggest that a cookbook will raise an error if
that cookbook is run in a situation where a Chef Infra Server is not
present. Adopting this rule is only necessary when chef-solo is part of
the team's workflow (because chef-solo does not use a Chef Infra
Server).

## Run Foodcritic

Foodcritic is run from the command line, typically against a single
cookbook and all of the Ruby files contained within it:

``` bash
foodcritic /path/to/cookbook
```

Foodcritic may also be run from the root of an individual cookbook
directory:

``` bash
foodcritic .
```

Foodcritic returns a list, via standard output, that shows the results
of the evaluation:

``` bash
FC003: Check whether you are running with chef server before using server-specific features: ./recipes/ip-logger.rb:1
FC008: Generated cookbook metadata needs updating: ./metadata.rb:2
FC008: Generated cookbook metadata needs updating: ./metadata.rb:3
```

### Output

Foodcritic output:

-   States a Foodcritic rule. For example: `FC003`
-   Describes the rule, based on the results of the evaluation. For
    example:
    `Check whether you are running with chef server before using server-specific features`
-   Specifies the file path. For example: `./recipes/ip-logger.rb`
-   Specifies the line number. For example: `1`

A Foodcritic evaluation has the following syntax:

``` bash
RULENUMBER: MESSAGE: FILEPATH:LINENUMBER
```

For example:

``` bash
FC008: Generated cookbook metadata needs updating: ./metadata.rb:3
```

## Rules

A complete list of Foodcritic rules are available on the [Foodcritic
website](http://foodcritic.io).

### Custom Rules

The following rules for Foodcritic have been developed by the Chef
community:

-   [/customink-webops/foodcritic-rules](https://github.com/customink-webops/foodcritic-rules)
-   [/etsy/foodcritic-rules](https://github.com/etsy/foodcritic-rules)

### Exclude Rules

Run the following command to exclude a Foodcritic rule:

``` bash
foodcritic . --tags ~RULE
```

For example, to exclude rule `FC003`:

``` bash
foodcritic . --tags ~FC003
```

## Foodcritic CLI

The `foodcritic` command is used to run Foodcritic against one (or more)
cookbooks.

This command has the following syntax:

``` bash
foodcritic COOKBOOK_PATH
```

This command has the following options:

`-t TAGS`, `--tags TAGS`

:   Use to specify tags to include or exclude when running Foodcritic.

`-l`, `--list`

:   List the name and description of all rules.

`-f TAGS`, `--epic-fail TAGS`

:   Use to trigger a build failure if any of the specified tags are
    matched.

`-c VERSION`, `--chef-version VERSION`

:   Use to specify the chef version to evaluate against instead of
    Foodcritic's default.

`-r`, `--rule-file PATH`

:   Specify file with rules to be used or ignored.

`-B PATH`, `--cookbook-path PATH`

:   Use to specify the path to a cookbook to check

`-C`, `--[no-]context`

:   Use to show lines matched against Foodcritic rules, rather than the
    default summary.

`-E`, `--environment-path PATH`

:   Environment path(s) to check.

`-G`, `--search-gems`

:   Search rubygems for rule files with the path
    `foodcritic/rules/**/*.rb`.

`-I PATH`, `--include PATH`

:   Use to specify the path to a file that contains additional
    Foodcritic rules.

`-P`, `--[no-]progress`

:   Show progress of files being checked.

`-R`, `--role-path PATH`

:   Role path(s) to check.

`-S PATH`, `--search-grammar PATH`

:   Use to specify the path to a file that contains additional grammar
    used when validating search syntax.

`-V`, `--version`

:   Use to display the version of Foodcritic.

`-X`, `--exclude PATH`

:   Exclude path(s) from being linted. PATH is relative to the cookbook,
    not an absolute PATH. Default `test/**/*,spec/**/*,features/**/*`.

## For more information ...

For more information about Foodcritic:

-   <http://www.foodcritic.io/>
