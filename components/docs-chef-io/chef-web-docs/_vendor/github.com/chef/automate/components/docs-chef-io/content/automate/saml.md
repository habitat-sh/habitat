+++
title = "SAML"

date = 2018-05-11T09:27:09+00:00
draft = false
[menu]
  [menu.automate]
    title = "SAML"
    parent = "automate/configuring_automate"
    identifier = "automate/configuring_automate/saml.md SAML"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/saml.md)

## Authentication via Existing Identity Management Systems

Chef Automate can integrate with existing SAML services to authenticate users in Chef Automate, and thus use their existing group memberships to determine their Chef Automate permissions.

Chef Automate supports using both local users and externally managed users from an external identity provider (IdP).
Both _one_ LDAP service (or MSAD for simplified configuration of Active Directory setups) and _one_ SAML IdP can be used.
You do not need to configure an external IdP if you simply want to create users and teams local to Chef Automate.
See the [Users]({{< relref "users.md" >}}) documentation for additional information.

Chef Automate uses [Dex](https://github.com/dexidp/dex) to support SAML integrations.
To configure authentication for your Chef Automate installation, create a TOML file that contains the partial SAML configuration.
Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

{{% warning %}}
You may only integrate one IdP using SAML and one IdP using LDAP at a time.
Chef Automate does not support using _two_ SAML IdPs or _two_ LDAP services simultaneously.
{{% /warning %}}

If you need to change your configured IdP, you will need to replace
your existing configuration by following these steps:

1. Run `chef-automate config show config.toml`.
2. Edit `config.toml` to replace the `dex.sys.connectors` section with the config values for your new identity provider.
3. Run `chef-automate config set config.toml` to set your updated config.

{{< note >}}
Users who sign in via SAML will have a session time of 24 hours before needing to sign in again.
{{< /note >}}

## Supported Identity Management Systems

- [Azure AD]({{< relref "#azure-ad" >}})
- Office365
- OKTA
- OneLogin
- Ping
- Tivoli Federated Identity Manager

### Azure AD

Using Azure AD as an SAML IdP requires specific configuration for both Azure AD and Chef Automate.

{{< note >}}
The signing certificate used for Chef Automate's SAML integration with Azure AD requires manual management.
Signing key rotation is not done automatically.
{{< /note >}}

In Azure AD, add Chef Automate as a _"non-gallery application"_, and then configure its SAML sign-in method.
[The Azure AD documentation](https://docs.microsoft.com/en-us/azure/active-directory/manage-apps/configure-saml-single-sign-on) provides a detailed guide.
Enter `https://{{< example_fqdn "automate" >}}/dex/callback` as the value for both _Identifier (Entity ID)_ and _Reply URL (Assertion Consumer Service URL)_.

You may use the default claims provided by Azure AD.
Remember to edit the Chef Automate configuration in `config.toml` to reflect this claims information.

Download the _Certificate (Base64)_ in Azure AD and take note of the _Login URL_ for use in the Chef Automate configuration.

After configuring Azure AD, edit your Chef Automate `config.toml` configuration file to reflect the values entered in the Azure AD interface.
The minimal configuration snippet in `config.toml` will look similar to:

```toml
[dex.v1.sys.connectors.saml]
ca_contents="""
<<Certificate (Base64)>>
"""
sso_url = "<<Login URL>>"
email_attr = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress"
username_attr = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress"
entity_issuer = "https://{{< example_fqdn "automate" >}}/dex/callback"
```

where:

- `ca_contents` contains the value of the _Certificate (Base64)_, and includes the `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----` markers
- `sso_url` contains the value of _Login URL_
- `entity_issuer` contains the value of _Identifier (Entity ID)_

See the SAML Configuration Settings below for further configuration options.

## SAML Configuration Settings

The SAML configuration settings are:

```toml
[dex.v1.sys.connectors.saml]
ca_contents = "<your ca contents>"          # required
sso_url = "<your SSO URL>"                  # required
email_attr = "<your email attribute>"       # required
username_attr = "<your username attribute>" # required
groups_attr = "<your groups attribute>"     # optional
allowed_groups = ["group1", "group 2"]      # optional
entity_issuer = "<your entity issuer>"      # optional
name_id_policy_format = "<see below>"       # optional
```

`ca_contents` must contain a copy of the certificate used to sign the SAML assertions.
The certificate should be a PEM-encoded string.
For example,

```toml
ca_contents = """-----BEGIN CERTIFICATE-----
MIIE+DCCAuCgAwIBAgIBATANBgkqhkiG9w0BAQsFADAcMRowGAYDVQQDExFDaGVm
[...]
s1V9oZ7+NcK8vtcdXhjB5N65LbPlaT3nbvXGIvsQmoGc+FQ5WI4agoNlofOCogdW
k2WFcoiiKyeIznNScx/K6AeykKR/lPrJedanSA==
-----END CERTIFICATE-----
"""
```

{{% warning %}}
The `groups_attr` setting is optional, but if not provided, users authenticating via SAML will not be members of any teams.
{{% /warning %}}

Setting `allowed_groups` provides SAML sign in to members of the listed groups.
All of the other user groups that are _not_ in the list are discarded, and not available to Chef Automate.
In the configuration example above, only users in either "group1" or "group2" may sign in, and those groups would appear as `team:saml:group1` and `team:saml:group2` respectively.

Chef Automate supports using SAML to authenticate users and apply permissions to SAML groups. See [IAM Overview]({{< relref "iam_v2_overview.md" >}}).

```toml
[dex.v1.sys.connectors.saml]
  ca_contents = "<your ca contents>"
  sso_url = "<your SAML SSO URL>"

  ###
  # SAML Attributes to map to a user's Chef Automate session
  ###
  # Example: "email"
  email_attr = "<your email attribute>"
  # Example: "name"
  username_attr = "<your username attribute>"
  # Example: "groups"
  groups_attr = "<your groups attribute>"

  # Optional: Manually specify Chef Automate's Issuer value.
  #
  # When provided Chef Automate will include this as the Issuer value in the SAML
  # AuthnRequest. It will also override the redirectURI as the required audience
  # when evaluating AudienceRestriction elements in the response.
  # Example: "https://{{< example_fqdn "automate" >}}/dex/callback"
  entity_issuer = "<your entity issuer>"

  # Optional: Specify the NameIDPolicy to use
  #
  # When provided, Chef Automate will request a name ID of the configured format
  # in the SAML AuthnRequest.
  # Defaults to "urn:oasis:names:tc:SAML:2.0:nameid-format:persistent".
  #
  # Note: Even when configured otherwise, the username gathered from the SAML
  # response is _treated_ as persistent. So, if this is set to
  #    "urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress"
  # and a user has changed their email address, they will be a _new_ user to Chef
  # Automate.
  name_id_policy_format = "urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified"
```

In your SAML Identity Provider (IdP), your Chef Automate instance needs to be referenced as a Service Provider (SP).
To do so, use `https://{{< example_fqdn "automate" >}}/dex/callback`.
The concrete configuration items differ between IdP products, but it is often something like "Assertion Consumption URI" or "Single sign on URL".
For "Audience URI" or "SP Entity ID", use the same address.

These values are accepted for `name_id_policy_format`:

 - `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
 - `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`
 - `urn:oasis:names:tc:SAML:1.1:nameid-format:X509SubjectName`
 - `urn:oasis:names:tc:SAML:1.1:nameid-format:WindowsDomainQualifiedName`
 - `urn:oasis:names:tc:SAML:2.0:nameid-format:encrypted`
 - `urn:oasis:names:tc:SAML:2.0:nameid-format:entity`
 - `urn:oasis:names:tc:SAML:2.0:nameid-format:kerberos`
 - `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent`
 - `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`
