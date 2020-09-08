+++
title = "Integrate Chef Automate with SAML for Authentication"
draft = false
robots = "noindex"


aliases = ["/integrate_chef_automate_saml.html"]

[menu]
  [menu.legacy]
    title = "Authentication w/SAML"
    identifier = "legacy/workflow/managing_workflow/integrate_chef_automate_saml.md Authentication w/SAML"
    parent = "legacy/workflow/managing_workflow"
    weight = 130
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/integrate_chef_automate_saml.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Security Assertion Markup Language (SAML) is an XML-based, open-standard
data format for exchanging authentication and authorization data between
parties, in particular, between an identity provider and a service
provider. Chef Automate supports SAML-backed Single Sign On (SSO) as a
service provider, integrating with your chosen identity provider.

## Configuring SAML for your Chef Automate enterprise

As an enterprise admin, you can configure a SAML Service to enable
single sign on. To do this from the Chef Automate UI, click on the
`Admin` menu item. From the `Admin` screen, navigate to the SAML Setup
tab. Once you are on the SAML Setup tab, you can configure the details
necessary to integrate Chef Automate and SAML.

This can either be done by supplying Chef Automate with your Identity
Provider's metadata endpoint, or by manually entering the required
fields. Please note that both options require you to set a NameID Policy
(explained below).

A Default Role for new users must be set in order to successfully set up
a SAML service. Any combination of roles may be selected in the SCM
Setup tab; all auto-provisioned users will be assigned these permissions
when they first log in to Chef Automate.

{{< note >}}

Metadata-driven SAML configuration enables Chef Automate to periodically
update its SAML certificates from this metadata, enabling certificate
rolling for signed SAML assertions.

{{< /note >}}

### Automatic SAML configuration through Identity Provider metadata

To make Chef Automate configure SAML automatically from the metadata
published by your Identity Provider, check the <span
class="title-ref">Import Metadata</span> box and enter its URL in the
text field. For example, the metadata endpoint for Azure AD deployments
is of the form
`https://login.microsoftonline.com/SOMEHASH/federationmetadata/2007-06/federationmetadata.xml`,
and Okta's metadata is similar to
`https://CORP.okta.com/app/SOMEHASH/sso/saml/metadata`. You should be
able to look at the XML document served there, and find that it starts
with the following:

``` xml
<EntityDescriptor xmlns="urn:oasis:names:tc:SAML:2.0:metadata" ID="_f4168057-a418-4b84-a250-29b25e927b73" entityID="https://sts.windows.net/1b218ca8-3694-4fcb-ac12-d2112c657830/">
```

Since it is uncommon to use CA-signed certificates for this, and the set
of certificates retrieved from that endpoint is trusted in the
verification of SAML logins, it is crucial for establishing trust to use
HTTPS for retrieving the metadata file. Chef Automate will default to
verifying the HTTPS endpoints certificate using your operating system's
trusted certificate bundle. See the Trust SSL Certificate section of
[Integrate Chef Automate with
BitBucket](/integrate_delivery_bitbucket/) for more information.

The periodic refresh can be controlled through `delivery.rb`. The
following are the default settings:

``` ruby
auth['saml_metadata_refresh_interval'] = '1d'
auth['saml_metadata_retry_interval'] = '1m'
```

With these settings, the Identity Provider's metadata will be refreshed
every day (<span class="title-ref">1d</span>), and if this request
fails, Chef Automate will wait one minute (<span
class="title-ref">1m</span>) before trying again. On failure, a retry
will be attempted five times total. If the retries don't succeed, the
next attempt to fetch the metadata will be at the next refresh interval.

### Manual SAML configuration

Fill out the following fields to configure SAML SSO. These details can
often be found through your Identity Provider's metadata file.

1.  The Identity Provider's Id, which is a URL that uniquely identifies
    your SAML identity provider. This is found as an attribute entityId
    under the EntityDescriptor element. Copy this value and put it in
    the Identity Provider URL text box. SAML assertions sent to Chef
    Automate must match this value exactly in the \<saml:Issuer\>
    attribute of SAML assertions.

    Metadata XML example:

    ``` xml
    <EntityDescriptor xmlns="urn:oasis:names:tc:SAML:2.0:metadata" ID="_0579740c-32a1-46a0-a8d0-fb583f0566e7" entityID="https://sts.windows.net/1b218ca8-3694-4fcb-ac12-d2112c657830/">
    ```

2.  The Identity Provider's SSO Login location. This can be retrieved
    from the metadata file. Look for the SingleSignOnService element and
    the Binding and Location attributes in that element. Ensure that the
    binding is a HTTP-Redirect binding. This is currently the only SSO
    Login Binding type supported in Chef Automate. Copy this location
    and put it in the Single Sign-On Login URL text box.

    Metadata XML example:

    ``` xml
    <SingleSignOnService Binding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect" Location="https://login.microsoftonline.com/1b218ca8-3694-4fcb-ac12-d2112c657830/saml2"/>
    ```

    {{< note spaces=4 >}}

    There can be multiple SingleSignOnService tags, each with a
    different binding.

    {{< /note >}}

3.  Selection of a Name Id Policy option. The Name Id Policy is used to
    request a specific user identification format from your Identity
    Provider (IdP). This can be left at "Default (No Policy)" if a
    specific format is not required, in which case the IdP will identify
    the user with its default configured Name Id Policy.

4.  A certificate from the IdP is required to verify integrity and
    authenticity of SAML assertions. From your metadata file copy only
    the certificate information from the KeyInfo block of XML, leaving
    out the XML tags. Paste this information into the Identity Provider
    Certificate box.

    Metadata XML example:

    ``` xml
    <KeyDescriptor use="signing">
        <KeyInfo>
            <X509Data>
                <X509Certificate>
                    MIIC4jCCAcqgAwIBAgIQQNXrmzh...
                </X509Certificate>
            </X509Data>
        </KeyInfo>
    </KeyDescriptor>
    ```

### Removing SAML configuration

The SAML configuration UI also allows for the removal of SAML
configuration from the system. In order to remove the configuration,
navigate to the SAML Setup tab, and then click on the <span
class="title-ref">Remove Configuration</span> button. After a
confirmation prompt, the SAML configuration will be removed from Chef
Automate. Once the configuration is removed, SAML users will no longer
be able to log into Chef Automate.

{{< note >}}

The SAML type accounts that may have been created will still continue to
exist even after the SAML configuration has been removed.

{{< /note >}}

## Configuring your Identity Provider to accept SAML requests from Chef Automate

To configure your IdP to accept SAML requests, you need the following:

-   The entity identification, or the issuer. If you have not overridden
    this setting in your <span class="title-ref">delivery.rb</span> (see
    below), enter:

    ``` none
    https://<yourChefAutomateDomain>/api/v0/e/<yourEnterprise>/saml/metadata
    ```

-   Assertion Consumer Service / Reply URL. This is where Chef Automate
    receives SAML assertions from the Identity Provider:

    ``` none
    https://<yourChefAutomateDomain>/api/v0/e/<yourEnterprise>/saml/consume
    ```

-   Audience. This will be the metadata URL for Chef Automate:

    ``` none
    https://<yourChefAutomateDomain>/api/v0/e/<yourEnterprise>/saml/metadata
    ```

Chef Automate currently only supports a subset of existing SAML
communication schemes. To ensure this works with your IdP, please ensure
these configuration options are set up:

-   Check that the identity provider endpoints are configured to accept
    `HTTP-Redirect` from the service provider.
-   Check that the identity provider is configured to use `HTTP-POST` to
    connect to the endpoints of the service provider.

## Enabling users to authenticate through SAML

By default, any users that authenticate successfully with the configured
Identity Provider will be logged in: both users with existing user
accounts in Chef Automate that are set up for SAML authentication, and
users hitherto unknown to Chef Automate, which then get a user account
created in Chef Automate automatically. It is also possible to migrate
existing users, or to create SAML users manually.

### Auto-provisioned users

The new user's name will match their NameId value as reported by the
Identity Provider (see below for the possible options). Also note that
changing the NameId Policy settings after users have been created
automatically will lead to new user accounts being created -- since
their NameId no longer matches a user's username in Chef Automate.

These users will be assigned the default role(s) selected as part of the
SAML configuration within the enterprise.

### Migrating existing users and manual user creation

To use SAML for existing users, they can be migrated from Chef Automate
or LDAP authentication. This can also be used to create SAML users in
Chef Automate before they have logged with SAML for the first time
(triggering auto-provisioning). For example, this allows you to grant a
user more roles in their enterprise. The username in Chef Automate must
match the NameId, such as email address, of the user in their Identity
Provider. See [Notes on NameId Policy](#notes-on-nameid-policy) for more
information.

To migrate an account:

1.  Click on the <span class="title-ref">Admin</span> menu item.
2.  Click on the user you wish to edit.
3.  The current authentication type will be highlighted. Change it to
    <span class="title-ref">SAML</span>.
4.  Rename username to match the user's full email address associated
    with their SAML account.
5.  Click <span class="title-ref">Save and Close</span>.

Chef Automate makes a SAML request to the Identity Provider with the
NameIdPolicy Format of
`` `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress ``\`. Your
Identity Provider must support NameIds in this format.

It is recommended that an administrator account remain a Chef Automate
authenticated user. This will allow an administrator to access Chef
Automate in the case of a SAML misconfiguration or problem with the SAML
Identity Provider.

{{< note >}}

For Okta users, Okta has to be configured to get a user's first name,
email address, and last name. When you are setting up SAML for Chef
Automate, log into your in Okta account. From the <span
class="title-ref">Admin</span> tab, go to Applications -\> Your
Application -\> General -\> SAML Settings. Click the edit button and
then on step 2, "Configure SAML" in the section "ATTRIBUTE STATEMENTS
(OPTIONAL)" set up the attribute mappings with the following values:

{{< /note >}}

![image](/images/samlattributes.jpg)

## Notes on NameId Policy

The Name Id Policy is important because it identifies the user that the
SAML assertion applies to. In order for Chef Automate to authenticate
the user, the Name Id that the IdP returns must exactly match a Chef
Automate SAML user name. In addition, it must match the user name that
was entered at the Chef Automate login page. Therefore, the the IdP or
the SP must be configured with an appropriate Name Id Policy. In some
cases, you (or your system administrator) may need to either negotiate
or configure the Name Id Policy on the IdP itself. Name Id mismatches
will lead to successful logins (as far as the IdP is concerned), but not
leading to the Chef Automate login of the expected user. Instead, a new
user will be provisioned with the username matching the returned NameId.

The following Name Id policies are not supported by Chef Automate:
Transient, X509Subject.

For illustration purposes, below we discuss two common scenarios:

1.  Configure Name Id Policy on the IdP side: If you are using an IdP
    such as Okta, you can configure the Name Id Policy when your
    application is added to Okta . For more information, see
    <http://developer.okta.com/docs/guides/setting_up_a_saml_application_in_okta>.

    In this case, you can leave the Name Id Policy setting on the Chef
    Automate side to "Default (No Policy)", since the IdP will always
    return what is pre-configured.

2.  Configure Name Id Policy on the Chef Automate side: On the other
    hand, you may be using an IdP (for example Microsoft Azure), that
    does not allow configuration of the Name Id Policy during
    application setup. For more information, see
    <https://azure.microsoft.com/en-us/documentation/articles/active-directory-authentication-scenarios/>.

    In this case, you will need to request a specific Name Id Policy
    through the Chef Automate configuration - for example, 'Email
    Address'.

## Notes on EntityId

By default, Chef Automate's SAML integration will use EntityId
`https://<yourChef AutomateDomain>/api/v0/e/<yourEnterprise>/saml/metadata`.
This can be overridden in `delivery.rb` as follows:

``` ruby
auth['saml_entity_id'] = 'https://delivery.corp.com/saml'
```

## Workflow ('delivery') CLI

The Workflow CLI in Chef Automate (`delivery-cli`) can be used with
SAML-authenticated users:

1.  When SAML is configured, `delivery token` defaults to
    SAML-authenticating the user, and it will prompt the user to use
    their browser to login to Chef Automate:

    ``` bash
    delivery token
    Chef Chef Automate
    Loading configuration from /path/to/project
    Requesting Token
    Press Enter to open a browser window to retrieve a new token. [ENTER]
    Launching browser.
    ```

2.  The Chef Automate CLI will then wait for the user to enter the token
    retrieved from the web interface:

    ``` none
    Enter token:
    ```

3.  The token retrieved will then be verified and saved in the usual
    token store.

    ``` none
    Enter token: [enter oMMoQ9N7XXYHI6X6lV7GaxEjxEP4Yv1TafTx7hFWH1U=]
    token: oMMoQ9N7XXYHI6X6lV7GaxEjxEP4Yv1TafTx7hFWH1U=
    saved API token to: /Users/alice/.delivery/api-tokens
    token: oMMoQ9N7XXYHI6X6lV7GaxEjxEP4Yv1TafTx7hFWH1U=
    Verifying Token: valid
    ```

4.  To log in as an internal user when SAML is configured, use the
    option `--saml=false`

## Enabling SAML proxying for Chef Server

The integration between the management console in Chef Infra Server and
Chef Automate's SAML capabilities is done using OpenID Connect.

### OpenID Connect Signing Key

Chef Automate signs the ID token given to the management console
following successful SAML authentication. To do that, a private signing
key needs to be provided. An alternate location can be configured in
`/etc/delivery/delivery.rb`:

``` ruby
auth['oidc_signing_private_key'] = '/etc/delivery/oidc_signing_private_key.pem' # this is the default
```

If the file does not exist, a 2048-bit RSA key will be generated using
OpenSSL (when running `automate-ctl reconfigure`). You can also provide
that RSA private key in PEM format yourself:

``` none
/etc/delivery# cat > oidc_signing_private_key.pem <<EOF
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDfBg/WS60hE8k/
4R3qvcoiH3noL0mQ0rUEEsfXEEiXgg2Wr0Vt7p9bB7rGH/6BTxEscVQbcpmpHeFu
TNvuPsENy9thT5lNWVH6goO1O9MsasqfXbLoZYprV/lA2V32ol5DpCyN09ozO1u0
LhMhnDqEgOiYpDiGw2HQNR58AuBqTxWvbc7ML5muDJ3/K2bf40uAYkziZA2Nv2Z3
...
-----END PRIVATE KEY-----
EOF
/etc/delivery#
```

You can verify that Chef Automate can read and parse your key by
accessing `https://<yourChef AutomateDomain>/api/v0/oidc/jwks`:

``` bash
curl https://delivery.corp.com/api/v0/oidc/jwks | jq .
 {
   "keys": [
     {
       "alg": "RS256",
       "e": "AQAB",
       "kid": "1",
       "kty": "RSA",
       "n": "3wYP1kutIRPJP-Ed6r3KIh956C9JkNK1BBLH1xBIl4INlq9Fbe6fWwe6xh_-gU8RLHFUG3KZqR3hbkzb7j7BDcvbYU-ZTVlR-oKDtTvTLGrKn12y6GWKa1f5QNld9qJeQ6QsjdPaMztbtC4TIZw6hIDomKQ4hsNh0DUefALgak8Vr23OzC-Zrgyd_ytm3-NL
 gGJM4mQNjb9md2eoUHh5iTpvbxCFQDA3LMBZje7Ls45mNvjC8wAX6b26fq1otoxmGeDiMoovjIFWp3tL3_KphTs0mDOoBQsEUA9FtZJXGBWQIyEibM5v9LBt43s8lJqAVMfVzSNW8uXKhBC9O7h2ZQ",
       "use": "sig"
     }
   ]
 }
```

If no key is configured or the key file can't be read, the keys array
will be empty: `[]`.

### Chef Infra Server as OpenID Connect client

To allow Chef Infra Server to act as an OpenID Connect client to Chef
Automate, it needs to be known to Chef Automate. To achieve this, add
the following to your `/etc/delivery/delivery.rb`

``` ruby
auth['oidc_clients'] = {
   'manage-client-id' => {
     'client_secret' => 'ohai',
     'client_redirect_uri' => 'https://manage.corp.com/oidc/callback'
   }
}
```

In the above snippet, the 'manage-client-id' should be a unique string
for each Chef Infra Server whose management console will authenticate
through SAML. Also, if you have multiple Chef Servers that will
authenticate through SAML, you will need to create additional entries
for the client id, the client secret and the client redirect URI in the
section above for each one.

### Configuration of Chef Infra Server

Note that all of the client-related values need to match the
configuration in the Chef Infra Server management console. See
[Configuring for SAML Authentication](/server_configure_saml/) for
more details.

## Troubleshooting

If you have problems with SAML configuration and integration, see the
SAML section of [Troubleshooting Chef Automate
Deployments](/troubleshooting_chef_automate/) for debugging tips.
