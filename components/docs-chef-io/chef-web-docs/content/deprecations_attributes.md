+++
title = "Deprecation: Some Attribute Methods (CHEF-4)"
draft = false
robots = "noindex"


aliases = "/deprecations_attributes.html"


+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_attributes.md)



We are continuously improving and streamlining the way attributes work
in Chef, to make it easier for users to reason about and safely
configure their servers.

This page documents many deprecations over the course of many Chef
releases.

## Method Access

Setting and accessing node attributes has been standardized on "bracket"
syntax. The older "method" syntax is deprecated and will be removed in
Chef Client 13.

Removal: Chef Client 13

### Example

Both lines in the example will cause separate deprecation warnings.

``` ruby
node.chef.server = "https://my.chef.server"
chef_server = node.chef.server
```

### Remediation

Convert method syntax to bracket syntax by using brackets to denote
attribute names. The code below is identical in function to the example
above:

``` ruby
node['chef']['server'] = "https://my.chef.server"
chef_server = node['chef']['server']
```

## Set and Set_Unless

Setting node attributes with `set` or `set_unless` has been deprecated
in favor of explicitly setting the precedence level. These methods will
be removed in Chef Client 14.

Removal: Chef Client 14

### Example

``` ruby
node.set['chef']['server'] =  "https://my.chef.server"
node.set_unless['chef']['server'] =  "https://my.chef.server"
```

### Remediation

Choose the appropriate [precedence
level](/attributes/#attribute-precedence), then replace `set` with
that precedence level.

``` ruby
node.default['chef']['server'] =  "https://my.chef.server"
node.default_unless['chef']['server'] =  "https://my.chef.server"
```
