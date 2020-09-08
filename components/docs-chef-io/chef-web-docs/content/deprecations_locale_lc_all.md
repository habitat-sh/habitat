+++
title = "Deprecation: Deprecation of lc_all from locale resource (CHEF-27)"
draft = false
robots = "noindex"

aliases = "/deprecations_locale_lc_all.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_locale_lc_all.md)

Setting the `LC_ALL` variable is NOT recommended. As a system-wide
setting, `LANG` should provide the desired behavior. `LC_ALL` is
intended to be used for temporarily troubleshooting issues rather than
an everyday system setting. Changing `LC_ALL` can break Chef's parsing
of command output in unexpected ways. Use one of the more specific `LC_`
properties as needed. This deprecation warning was added in Chef Infra
Client 15.0. Support for property `lc_all` will be removed for Chef
Infra Client 16.0.

The [Cookstyle](/workstation/cookstyle/) cop
[ChefDeprecations/LocaleDeprecatedLcAllProperty](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationslocaledeprecatedlcallproperty)
has been introduced to detect and autocorrect this deprecation.

## Remediation

Set `LC_ALL` in current shell as:

``` bash
export LC_ALL="<locale_name>"
```

To check the `locale` value, run:

``` bash
locale -v
```

You can also use **file** Resource and add this variable in any other
file of your choice and then can source that file to reflect changes.

``` ruby
file "<path_to_file>" do
  content "LC_ALL=<locale_name>"
end
```

Where `path_to_file` could be any one of:

1.  /etc/default/locale
2.  /etc/sysconfig/i18n
3.  /etc/environment

Setting **LC_** variables varies by platform, but these are the common
locations to configure **LC_** variables.

{{< warning >}}

Using the **file** Resource or other manual management method of LC
configuration may overwrite settings from this resource and break your
system.

{{< /warning >}}
