+++
title = "ohai (executable)"
draft = false

aliases = ["/ctl_ohai.html"]

[menu]
  [menu.infra]
    title = "ohai (executable)"
    identifier = "chef_infra/features/ohai/ctl_ohai.md ohai (executable)"
    parent = "chef_infra/features/ohai"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/ctl_ohai.md)

{{% ctl_ohai_summary %}}

## Options

{{% ctl_ohai_options %}}

## Examples

The following examples show how to use the Ohai command-line tool:

**Run a plugin independently of a Chef Infra Client run**

An Ohai plugin can be run independently of a Chef Infra Client run.
First, ensure that the plugin is located in the `/plugins` directory and
then use the `-f` option when running Ohai from the command line. For
example, a plugin named `sl_installed` may look like the following:

``` ruby
Ohai.plugin(:Sl) do
  provides "sl"

  collect_data(:default) do
    sl Mash.new

    if ::File.exist?("/usr/games/sl")
      sl[:installed] = true
    else
      sl[:installed] = false
    end

    # sl[:installed] = ::File.exist?("/usr/games/sl")

  end
end
```

To run that plugin from the command line, use the following command:

``` bash
ohai --directory /path/to/directory sl
```

The command will return something similar to:

``` javascript
{
  "sl": {
    "installed": true
  }
}
```
