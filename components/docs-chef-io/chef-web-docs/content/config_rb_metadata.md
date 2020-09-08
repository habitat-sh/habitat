+++
title = "metadata.rb"
draft = false

aliases = ["/config_rb_metadata.html"]

[menu]
  [menu.infra]
    title = "metadata.rb"
    identifier = "chef_infra/cookbook_reference/config_rb_metadata.md metadata.rb"
    parent = "chef_infra/cookbook_reference"
    weight = 110
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_metadata.md)

{{% cookbooks_metadata %}}

A metadata.rb file is:

-   Located at the top level of a cookbook's directory structure.
-   Compiled whenever a cookbook is uploaded to the Chef Infra Server or
    when the `knife cookbook metadata` subcommand is run, and then
    stored as JSON data.
-   Created automatically by knife whenever the `knife cookbook create`
    subcommand is run.
-   Edited using a text editor, and then re-uploaded to the Chef Infra
    Server as part of a cookbook upload.

## Error Messages

The Chef Infra Server will only try to distribute the cookbooks that are
needed to configure an individual node. This is determined by
identifying the roles and recipes that are assigned directly to that
system, and then to expand the list of dependencies, and then to deliver
that entire set to the node. In some cases, if the dependency is not
specified in the cookbook's metadata, the Chef Infra Server may not
treat that dependency as a requirement, which will result in an error
message. If an error message is received from the Chef Infra Server
about cookbook distribution, verify the `depends` entries in the
metadata.rb file, and then try again.

{{< note >}}

A metadata.json file can be edited directly, should temporary changes be
required. Any subsequent upload or action that generates metadata will
cause the existing metadata.json file to be overwritten with the newly
generated metadata. Therefore, any permanent changes to cookbook
metadata should be done in the metadata.rb file, and then re-uploaded to
the Chef Infra Server.

{{< /note >}}

## Version Constraints

Many fields in a cookbook's metadata allow the user to constrain
versions. There are a set of operators common to all fields:

<table>
<colgroup>
<col style="width: 87%" />
<col style="width: 12%" />
</colgroup>
<thead>
<tr class="header">
<th>Specification</th>
<th>Operator</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Pessimistic (see note below)</td>
<td><code>~&gt;</code></td>
</tr>
<tr class="even">
<td>Equal to</td>
<td><code>=</code></td>
</tr>
<tr class="odd">
<td>Greater than or equal to</td>
<td><code>&gt;=</code></td>
</tr>
<tr class="even">
<td>Greater than</td>
<td><code>&gt;</code></td>
</tr>
<tr class="odd">
<td>Less than</td>
<td><code>&lt;</code></td>
</tr>
<tr class="even">
<td>Less than or equal to</td>
<td><code>&lt;=</code></td>
</tr>
</tbody>
</table>

{{< note >}}

Pessimistic locking is enabled by proper [semantic
versioning](https://semver.org) of cookbooks. If we're on version 2.2.3
of a cookbook, we know that the API will be stable until the 3.0.0
release. Using traditional operators, we'd write this as
`>= 2.2.0, < 3.0`. Instead, we can write this by combining a tilde "\~"
and right angle bracket "\>"--often called a tilde-rocket or
"twiddle-wakka"--followed by the major and minor version numbers. For
example: `~> 2.2`

{{< /note >}}

## Settings

This configuration file has the following settings:

`chef_version`

:   A range of Chef Client versions that are supported by this cookbook.
    All [version constraint
    operators](/config_rb_metadata/#cookbook-version-constraints)
    are applicable to this field.

    For example, to match any 14.x version of the Chef Client, but not
    13.x or 15.x:

    ``` ruby
    chef_version '~> 14.0'
    ```

    A more complex example where you set both a lower and upper bound of
    the Chef Infra Client version:

    ``` ruby
    chef_version ">= 14.2.1", "< 14.5.1"
    ```

`depends`

:   This field requires that a cookbook with a matching name and version
    exists on the Chef Infra Server. When the match exists, the Chef
    Infra Server includes the dependency as part of the set of cookbooks
    that are sent to the node during a Chef Infra Client run. It is very
    important that the `depends` field contain accurate data. If a
    dependency statement is inaccurate, Chef Infra Client may not be
    able to complete the configuration of the system. All [version
    constraint
    operators](#cookbook-version-constraints)
    are applicable to this field.

    For example, to set a dependency a cookbook named `cats`:

    ``` ruby
    depends 'cats'
    ```

    or, to set a dependency on the same cookbook, but only when the
    version is less than 1.0:

    ``` ruby
    depends 'cats', '< 1.0'
    ```

`description`

:   A short description of a cookbook and its functionality.

    For example:

    ``` ruby
    description 'A fancy cookbook that manages a herd of cats!'
    ```

`gem`

:   Specifies a gem dependency for installation in Chef Infra Client
    through bundler. The gem installation occurs after all cookbooks are
    synchronized but before loading any other cookbooks. Use this
    attribute one time for each gem dependency. For example:

    ``` ruby
    gem "poise"
    gem "chef-sugar"
    ```

    {{< warning spaces=4 >}}

    Use the `gem` setting only for making external chef libraries
    shipped as gems accessible in a Chef Infra Client run for libraries
    and attribute files. The `gem` setting in `metadata.rb` allows for
    the early installation of this specific type of gem, with the
    fundamental limitation that it cannot install native gems.

    Do not install native gems with the `gem` setting in `metadata.rb` .
    The `gem` setting is not a general purpose replacement for the
    [chef_gem resource](/resources/chef_gem/), and does not
    internally re-use the `chef_gem` resource. Native gems require C
    compilation and must not be installed with `metadata.rb` because
    `metadata.rb` runs before any recipe code runs. Consequently, Chef
    Infra Client cannot install the C compilers before the gem
    installation occurs. Instead, install native gems with the
    `chef_gem` resource called from the recipe code. You'll also need to
    use the `build_essential` resource in the recipe code to install the
    prerequisite compilers onto the system.

    Pure ruby gems can also be installed with metadata.rb.

    {{< /warning >}}

`issues_url`

:   The URL for the location in which a cookbook's issue tracking is
    maintained. This setting is also used by Chef Supermarket. In Chef
    Supermarket, this value is used to define the destination for the
    "View Issues" link.

    For example:

    ``` ruby
    issues_url 'https://github.com/chef-cookbooks/chef-client/issues'
    ```

`license`

:   The type of license under which a cookbook is distributed:
    `Apache v2.0`, `GPL v2`, `GPL v3`, `MIT`, or
    `license 'Proprietary - All Rights Reserved` (default). Please be
    aware of the licenses for files inside of a cookbook and be sure to
    follow any restrictions they describe.

    For example:

    ``` ruby
    license 'Apache-2.0'
    ```

    or:

    ``` ruby
    license 'GPL-3.0'
    ```

    or:

    ``` ruby
    license 'MIT'
    ```

    or:

    ``` ruby
    license 'Proprietary - All Rights Reserved'
    ```

`maintainer`

:   The name of the person responsible for maintaining a cookbook,
    either an individual or an organization.

    For example:

    ``` ruby
    maintainer 'Adam Jacob'
    ```

`maintainer_email`

:   The email address for the person responsible for maintaining a
    cookbook. Only one email can be listed here, so if this needs to be
    forwarded to multiple people consider using an email address that is
    already setup for mail forwarding.

    For example:

    ``` ruby
    maintainer_email 'adam@example.com'
    ```

`name`

:   Required. The name of the cookbook.

    For example:

    ``` ruby
    name 'cats'
    ```

`ohai_version`

:   A range of Ohai versions that are supported by this cookbook. All
    [version constraint
    operators](#cookbook-version-constraints)
    are applicable to this field.

    For example, to match any 8.x version of Ohai, but not 7.x or 9.x:

    ``` ruby
    ohai_version "~> 8"
    ```

    {{< note spaces=4 >}}

    This setting is not visible in Chef Supermarket.

    {{< /note >}}

`privacy`

:   Specify a cookbook as private.

    For example:

    ``` ruby
    privacy true
    ```

`source_url`

:   The URL for the location in which a cookbook's source code is
    maintained. This setting is also used by Chef Supermarket. In Chef
    Supermarket, this value is used to define the destination for the
    "View Source" link.

    For example:

    ``` ruby
    source_url 'https://github.com/chef-cookbooks/chef-client'
    ```

`supports`

:   Show that a cookbook has a supported platform. Use a version
    constraint to define dependencies for platform versions: `<` (less
    than), `<=` (less than or equal to), `=` (equal to), `>=` (greater
    than or equal to), `~>` (approximately greater than), or `>`
    (greater than). To specify more than one platform, use more than one
    `supports` field, once for each platform.

    For example, to support every version of Ubuntu:

    ``` ruby
    supports 'ubuntu'
    ```

    or, to support versions of Ubuntu greater than or equal to 16.04:

    ``` ruby
    supports 'ubuntu', '>= 16.04'
    ```

    or, to support only Ubuntu 18.04:

    ``` ruby
    supports 'ubuntu', '= 18.04'
    ```

    Here is a list of all of the supported specific operating systems:

    ``` ruby
    %w( aix amazon centos fedora freebsd debian oracle mac_os_x redhat suse opensuseleap ubuntu windows zlinux ).each do |os|
      supports os
    end
    ```

`version`

:   The current version of a cookbook. Version numbers always follow a
    simple three-number version sequence.

    For example:

    ``` ruby
    version '2.0.0'
    ```
