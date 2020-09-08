+++
title = "Deprecation: Amazon linux moved to the Amazon platform_family (OHAI-7)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_amazon_linux.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_amazon_linux.md)

In Ohai/Chef releases prior to Chef Client 13, Amazon Linux was
identified as `platform_family 'rhel'`. In Ohai/Chef Client 13 and
later, Amazon Linux will be identified as the
`platform_family 'amazon'`. When Amazon Linux was created it closely
mirrored the structure and package naming of RHEL 5, and with the
release of RHEL 6 Amazon Linux moved to closely resemble RHEL 6. With
the release of RHEL 7 Red Hat switched to the systemd init system,
however Amazon Linux has not yet decided to make that same switch. In
addition to the init system differences, Amazon Linux has added many
critical packages with their own unique naming conventions. This makes
it very hard for users to write cookbooks for RHEL that will work on
Amazon Linux systems out of the box. In order to simplify multi-platform
cookbook code and to make it more clear when cookbooks actually support
Amazon Linux, we've created the '`amazon` platform family and removed
Amazon Linux from the `rhel` platform family.

## Remediation

If you have a cookbook that relies on `platform_family 'rhel'` to
support Red Hat based distributions as well as Amazon Linux, you'll need
to modify your code to specifically check for the `'amazon'` platform
family.

Existing code only checking for the `rhel` platform family:

``` ruby
if platform_family?('rhel')
  service 'foo' do
    action :start
  end
end
```

Updated code to check for both `rhel` and `amazon` platform families:

``` ruby
if platform_family?('rhel', 'amazon')
  service 'foo' do
    action :start
  end
end
```
