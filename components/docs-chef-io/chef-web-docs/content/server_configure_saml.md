+++
title = "Configuring for SAML Authentication"
draft = false
robots = "noindex"


aliases = ["/server_configure_saml.html", "/release/automate/server_configure_saml.html"]

[menu]
  [menu.infra]
    title = "Configure SAML"
    identifier = "chef_infra/features/management_console/server_configure_saml.md Configure SAML"
    parent = "chef_infra/features/management_console"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_configure_saml.md)

{{% EOL_manage %}}

Chef Manage can support logging in users via SAML authentication. In
order to do so, there must be a Chef Automate Server that is configured
to act as a SAML Identity Provider (IdP). When the Chef Automate Server
is configured to do so, it will provide an OpenID Connect (OIDC)
protocol end-point that Chef Manage can use to initiate authentication.

## Configuring Chef Manage

To configure the Chef Infra Server management console, first add the
following code to `/etc/chef-manage/manage.rb`:

``` ruby
saml.enabled true
saml.issuer_url '<AUTOMATE OIDC ENDPOINT>'
```

Second, store your OIDC client credentials with using the [Secrets
Management](/ctl_chef_server/#ctl-chef-server-secrets-management)
commands:

``` bash
chef-server-ctl set-secret saml client_id '<CLIENT ID>'
chef-server-ctl set-secret saml client_secret '<CLIENT SECRET>'
```

The `issuer_url` property should be set to something like
`'https://<yourChefAutomateDomain>/api/v0'`. Also, the `client_id` must
be known to the Chef Automate server. For development purposes only, you
may also need to turn off the TLS verification for the OIDC client by
adding

``` ruby
saml.verify_tls false
```

to your Chef Manage configuration.

Finally, run `chef-manage-ctl reconfigure` to apply these settings.

{{< warning >}}

You cannot have both LDAP and SAML authentication enabled at the same
time. If you do, the reconfigure will fail with an appropriate error
message.

{{< /warning >}}
