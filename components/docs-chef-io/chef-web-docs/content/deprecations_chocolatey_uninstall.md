+++
title = "Deprecation: :uninstall Resource for chocolatey_package (CHEF-21)"
draft = false
robots = "noindex"


aliases = "/deprecations_chocolatey_uninstall.html"

+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_chocolatey_uninstall.md)

The Chocolatey cookbook's `chocolatey_package` resource originally
contained an `:uninstall` action. When
[chocolatey_package](/resources/chocolatey_package/) was moved into
core Chef, we made `:uninstall` an alias for `:remove`. In Chef Client
14, `:uninstall` will no longer be a valid action.

The [Cookstyle](/workstation/cookstyle/) cop
[ChefDeprecations/ChocolateyPackageUninstallAction](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationschocolateypackageuninstallaction)
has been introduced to detect and autocorrect this deprecation.

## Remediation

The `:uninstall` action must be replaced with the `:remove` action when
using the `chocolatey_package` resource in recipes that you intend to
use with Chef Client 14. For example, where you might previously have
used the following code to uninstall `nginx`:

``` ruby
chocolatey_package 'nginx' do
  action :uninstall
end
```

You would instead use:

``` ruby
chocolatey_package 'nginx' do
  action :remove
end
```
