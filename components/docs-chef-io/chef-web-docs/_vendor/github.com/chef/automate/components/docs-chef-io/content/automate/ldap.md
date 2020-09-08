+++
title = "LDAP"

date = 2018-05-11T09:27:09+00:00
draft = false
[menu]
  [menu.automate]
    title = "LDAP"
    parent = "automate/configuring_automate"
    identifier = "automate/configuring_automate/ldap.md LDAP"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/ldap.md)

## Authentication via Existing Identity Management Systems

Chef Automate can integrate with existing LDAP services to authenticate users in Chef Automate, and thus use their existing group memberships to determine their Chef Automate permissions.

Chef Automate supports using both local users and externally managed users from an external identity provider (IdP).
Both _one_ LDAP service (or MSAD for simplified configuration of Active Directory setups) and _one_ SAML IdP can be used.
You do not need to configure an external IdP if you simply want to create users and teams local to Chef Automate.
See the [Users]({{< relref "users.md" >}}) documentation for additional information.

Chef Automate uses [Dex](https://github.com/dexidp/dex) to support LDAP integrations.
To configure authentication for your Chef Automate installation, create a TOML file that contains the partial LDAP configuration.
Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

{{% warning %}}
You may only integrate one IdP using SAML and one IdP using LDAP at a time.
Chef Automate does not support using _two_ SAML IdPs or _two_ LDAP services simultaneously.
{{% /warning %}}

Switching between a Microsoft AD configuration and generic LDAP configuration will
not affect your policies, as they are both LDAP configurations.
However, switching between either of those configurations and a SAML configuration will
require you to adjust the [IAM]({{< relref "iam_v2_overview.md" >}}) policy membership.

{{< note >}}
Local, MSAD, and LDAP users will have their Chef Automate sessions refreshed while their Chef Automate browser window remains open or until they sign out directly.
{{< /note >}}

## Supported Identity Management Systems

- Azure Active Directory
- Microsoft Active Directory (MSAD)

## Overview

This is documentation for configuring Chef Automate's Lightweight Directory Application Protocol (LDAP) and Microsoft Active Directory (MSAD) integrations. LDAP is an established and open standard protocol for interacting with directory servers. A directory server stores information--in this case information for authenticating and authorizing users--in a tree of entries. (It is not a relational database.)

## Microsoft Active Directory

Microsoft Active Directory (MSAD) is a type of directory server that supports LDAP. Chef Automate comes with a default LDAP configuration for MSAD.
The Chef Automate default MSAD configuration is a minimal configuration for standard MSAD systems, which you can extend by overriding default values and using additional configuration options.
Chef Automate's default configuration for Microsoft AD is specific to LDAP.
To configure Microsoft AD using SAML, see the [SAML documentation]({{< relref "saml.md" >}}).

## Changing Chef Automate Configuration

If you need to change your configured external identity provider settings, replace your existing configuration by following these steps:

1. Run `chef-automate config show config.toml`.
2. Edit `config.toml` to replace the `dex.sys.connectors` section with the configuration values for your new identity provider.
3. Run `chef-automate config set config.toml` to set your updated configuration.

### Minimal MSAD Configuration

base_user_search_dn
: "your base user search DN"

base_group_search_dn
: "your base group search DN"

bind_dn
: "your bind_dn"

bind_password
: "your bind_password"

ca_contents
: Your certificate authority (CA) certificate contents. You can provide multiple PEM-encoded CA certs. Optional.

  ```toml
  # Example ca_contents setting:
  ca_contents = """-----BEGIN CERTIFICATE-----
  MIICsDCCAhmgAwIBAgIJAJxMopMJbhPkMA0GCSqGSIb
  ...
  X0uRzUPlpTtd5tYFs43nKqxJT6s=
  -----END CERTIFICATE-----"""
  ```

host
: The domain name of your directory server, for example `"ldap.corp.com"`. Default port: `636`. Override the port by appending it to the host setting, `"ldap.corp.com:10636"`

#### Minimal MSAD config.toml

```toml
[dex.v1.sys.connectors.msad_ldap]
host = "<your host>"
bind_dn = "<your bind_dn>"
bind_password = "<your bind_password>"
base_user_search_dn = "<your base user search DN>"
base_group_search_dn = "<your base group search DN>"
ca_contents = "<your ca contents>" # optional, but recommended
```

### Full MSAD Configuration

The MSAD configuration is an LDAP configuration with more provided default values that are commonly a good fit for Active Directory. Override any single default value by uncommenting it in the configuration and setting its value:

email_attr
: "mail"

filter_groups_by_user_attr
: "member"

filter_groups_by_user_value
: "DN"

group_query_filter
: "(objectClass=group)"

group_display_name_attr
: "displayName"

insecure_no_ssl
: false
{{% warning %}}
Connecting to an LDAP service without TLS is not recommended.
{{% /warning %}}

user_display_name_attr
: "displayName"

user_id_attr
: "sAMAccountName"

user_query_filter
: "(objectClass=person)"

username_attr
: "sAMAccountName"

#### Example Full MSAD config.toml

```toml
[dex.v1.sys.connectors.msad_ldap]
host = "<your host>"
bind_dn = "<your bind_dn>"
bind_password = "<your bind_password>"
base_user_search_dn = "<your base user search DN>"
base_group_search_dn = "<your base group search DN>"
ca_contents = "<your ca contents>" # optional

# MSAD default values (uncomment to override a specific one)
# insecure_no_ssl = false
# user_query_filter = "(objectClass=person)"
# user_id_attr = "sAMAccountName"
# username_attr = "sAMAccountName"
# email_attr = "mail"
# user_display_name_attr = "displayName"
# group_query_filter = "(objectClass=group)"
# filter_groups_by_user_value = "DN"
# filter_groups_by_user_attr = "member"
# group_display_name_attr = "displayName"
```

## Extended LDAP Settings

For those who do not use Microsoft AD or require greater control over their configuration,
Chef Automate has the following customizable LDAP configuration settings:

base_group_search_dn
: "your base group search DN"

base_user_search_dn
: "your base user search DN"

bind_dn
: "your bind_dn"

bind_password
: "your bind_password"

ca_contents
: "your ca contents"

email_attr
: "your email attribute"

filter_groups_by_user_attr
: "groups to filter by user attribute"

filter_groups_by_user_value
: "groups to filter by user value"

group_display_name_attr
: "group display name attribute"

group_query_filter
: "your group query filter"

host
: "your host"

insecure_no_ssl
:true or false

user_query_filter
: "your user query filter"

username_attr
: "your username attribute"

user_id_attr
: "your userid attribute"

user_display_name_attr
: "your user display name attribute"

### Example Extended LDAP config.toml

```toml
[dex.v1.sys.connectors.ldap]
# authentication options
ca_contents = "<your ca contents>"
host = "<your host>"
bind_dn = "<your bind_dn>"
bind_password = "<your bind_password>"
insecure_no_ssl = true or false

# ldapsearch options
base_user_search_dn = "<your base user search DN>"
user_query_filter = "<your user query filter>"
username_attr = "<your username attribute>"
user_id_attr = "<your userid attribute>"
email_attr = "<your email attribute>"
user_display_name_attr = "<your user display name attribute>"
base_group_search_dn = "<your base group search DN>"
group_query_filter = "<your group query filter>"
filter_groups_by_user_attr = "<groups to filter by user attribute>"
filter_groups_by_user_value = "<groups to filter by user value>"
group_display_name_attr = "<group display name attribute>"
```

See the [LDAP]({{< relref "ldap.md" >}}) for more information on configuration fields.
You have the full extent of TOML is at your disposal for declaring configuration fields.

{{% warning %}}
Connecting to an LDAP service without TLS is not recommended.
{{% /warning %}}

However, if you wish to integrate with an LDAP server with TLS disabled:

```toml
insecure_no_ssl = true
```

### Sign In with LDAP

Once the user has provided a username and password at the sign in screen, Chef Automate goes through a sequence of operations to complete the sign in:

1. [Connect]({{< relref "#connect" >}})
1. [Bind]({{< relref "#bind" >}})
1. [User Search]({{< relref "#user-search" >}})
1. [Sign in Bind]({{< relref "#signin-bind" >}})
1. [Group Search]({{< relref "#group-search" >}})

#### Authorization with LDAP

Chef Automate supports defining permissions for LDAP users and their groups. See [IAM members and policies]({{< ref "iam_v2_overview.md#members-and-policies" >}}).

#### Connect

Chef Automate first needs to establish a TCP connection to your LDAP service, secured by TLS.
It will connect to the host configured in your TOML configuration, for example:

```toml
host = "ldap.corp.com"
```

Automate uses port `636` by default. To override the port, append it to the host setting, e.g.

```toml
host = "ldap.corp.com:10636"
```

Whether the validity of the server's TLS certificate will be enforced depends on the TLS setup: if you provide a certificate authority's (CA) certificate(s), Chef Automate will only communicate with the LDAP service if the certificate provided by the host can be validated using the CA certificate(s).



{{% warning %}}
Connecting to an LDAP service without TLS is not recommended.
{{% /warning %}}

However, if you wish to integrate with an LDAP server with TLS disabled:

```toml
insecure_no_ssl = true
```

See [Troubleshoot your Connection]({{< relref "#troubleshoot-your-connection" >}})
for common issues related to _Connect_.

#### Bind

Chef Automate then authenticates with (or "binds to") the LDAP service using _bind credentials_.
In your configuration TOML file, these would be (for example):

```toml
bind_dn = "cn=service_account,dc=corp,dc=com"
bind_password = "i<3ldap"
```

If your LDAP server supports _anonymous bind_, and you want to use that, unset
both bind DN and password:

```toml
bind_dn = ""
bind_password = ""
```

Wrap special characters in a bind_password in triple single quotes.

```toml
bind_password = '''$p3c"i'@l ! %#'''
```

See [Troubleshoot Bind]({{< relref "#troubleshoot-bind" >}})
for common issues related to _Bind_.

#### User Search

After binding successfully, Chef Automate will try to obtain the directory name of the user that is trying to sign in.

To do so, it will search, using the configured _base_ `base_user_search_dn`,
for an entry such that `username_attr` equals the username that attempted to
sign in.

If configured, it will retrieve additional attributes, using the configured
names (`user_id_attr`, `email_attr`, and `user_display_name_attr`). See
[Configuration: LDAP]({{< relref "configuration.md#ldap" >}}) for an overview.

{{< note >}}
The `ldapsearch` command line corresponding to _User Search_ is

```shell
ldapsearch -h $host -D $bind_dn -w $bind_password \
  -s sub \
  -b $base_user_search_dn \
  "($username_attr=$username)" \
  $user_id_attr $user_display_name_attr $email_attr
```

where `username` is what was typed into the **username input box** in the Sign in
form.
{{< /note >}}

{{% warning %}}
If the LDAP search fails to retrieve the configured attributes, the sign in process will fail.
{{% /warning %}}

See [Troubleshoot User Search]({{< relref "#troubleshoot-user-search" >}})
for common issues related to _User Search_.

##### Filtering Which Users Can Sign In

You can further restrict the user search by providing a valid LDAP filter to `user_query_filter`.
For example,

```toml
user_query_filter = "(objectClass=person)"
```

which will be concatenated with the search filter constructed from the provided
username in the sign in screen. The contents of `user_query_filter` gets expanded
to `(&<user_query_filter_value>)` so you can pass in multiple filters.

For example, if you wanted to only allow people that were members of a specific Active Directory
group to sign in to Chef Automate, you could define a `user_query_filter` with multiple filters like:

```toml
user_query_filter = "(objectClass=person)(memberof=CN=YourGroupToFilterOn,OU=Users,DC=YourDomain,DC=com)"
```

This filter says "only allow people who are members of YourGroupToFilterOn to sign in to Chef Automate".
When a user tries to sign in, they would only be authorized if they were found after the filter is applied:

```LDIF
(&(objectClass=person)(memberof=CN=YourGroupToFilterOn,OU=Users,DC=YourDomain,DC=com))
```

{{< note >}}
The `ldapsearch` command line corresponding to _User Search_ with restricted
groups is

```shell
ldapsearch -h $host -D $bind_dn -w $bind_password \
  -s sub \
  -b $base_user_search_dn \
  "(&$user_query_filter($username_attr=$username))"
```

where `username` is what was typed into the username input box in the Sign in
form.
{{< /note >}}

See [`ldapsearch` Example Queries]({{< relref "#ldapsearch-example-queries" >}})
for an example on using `ldapsearch`, and different directory layouts.

#### Sign In Bind

When the search for a user directory entry has succeeded, the LDAP connector
will attempt to bind as the user entry, using the supplied password.

For example, if the sign in using `jane:janespassword` has resulted in a
successful user search, returning `cn=jane,ou=People,dc=corp,dc=com`, the next
step will be to _bind again_ using that DN, and the password `janespassword`.

{{< note >}}
The `ldapsearch` command line corresponding to _User Search_ is

```shell
ldapsearch -h $host -D $user_dn -w $password
```

where `user_dn` is the DN of the user that was returned in [User Search]({{< relref "#user-search" >}}),
and `password` is what was typed into the **password input box** in the Sign in form.

Note that `result: 32 No such object` is the successful response here, a failed sign in bind using `ldapsearch` returns:

```shell
ldap_bind: Invalid credentials (49)
        additional info: INVALID_CREDENTIALS: Bind failed: Cannot authenticate user uid=test2,ou=users,ou=system
```

{{< /note >}}

See [Troubleshoot Sign In Bind]({{< relref "#troubleshoot-sign-in-bind" >}})
for common issues related to _Sign\_In\_Bind_.

#### Group Search

Finally, after the user has been authenticated, their internal record is
enriched with LDAP-provided groups. This happens by executing another search
using the same bind DN and password that was used for user search.

Similar to user search, a base DN has to be provided; and the result can be
restricted by providing an additional filter:

 ```toml
base_group_search_dn = "ou=Groups,dc=corp,dc=com"
group_query_filter = "(objectClass=group)"
```

The correct configuration settings again depend on your directory server's schema;
see the example configs below.

{{% warning %}}
The `base_group_search_dn` setting is optional. However, if it is not provided,
users authenticating via LDAP (or MSAD) will not be members of any teams.
{{% /warning %}}

{{< note >}}
The `ldapsearch` command line corresponding to _Group Search_ is

```shell
ldapsearch -h $host -D $bind_dn -w $bind_password \
  -s sub \
  -b $base_group_search_dn \
  "($filter_groups_by_user_attr=$user_attr)" \
  $group_display_name_attr
```

where `user_attr` is the `$filter_groups_by_user_value` of the user that was returned in [User Search]({{< relref "#user-search" >}}).
{{< /note >}}

See [Troubleshoot Group Search]({{< relref "#troubleshoot-group-search" >}})
for common issues related to _Group Search_.

#### Configuration Overview

See below for the full configuration and additional details about all LDAP configuration options.

```toml
[dex.v1.sys.connectors.ldap]
  ###
   # Configuration for querying your LDAP server
   ###
   ca_contents = "<your ca contents>"
   host = "<your host>"

   # The DN and password you wish to bind to your LDAP server to search for
   # users to authenticate for Chef Automate (and also to search for their group membership).
   # Example: "uid=seviceaccount,cn=users,dc=example,dc=com"
   bind_dn = "<your bind_dn>"
   bind_password = "<your bind_password>"

   ###
   # User Query (search for LDAP users to authenticate for Chef Automate)
   ###
   # The base DN to start the user query.
   # Chef Automate will use this as the base DN on which to search for users to authenticate against your LDAP server.
   # Example: "cn=users,dc=example,dc=com"
   base_user_search_dn = "<your base user search DN>"

   # The LDAP field used to filter the query for users to authenticate for Chef Automate.
   # Example: Setting this to "uid" would result in a filter of "(uid=<username_for_user_trying_to_authenticate>)".
   username_attr = "<your username attribute>"

   # Optional: LDAP query filter to apply when searching for users to authenticate.
   # This will be combined with username_attr filter above.
   # Example: Setting this to "(objectClass=person)" will filter on human actors only.
   user_query_filter = "<your user query filter>"

   ###
   # Populating the Chef Automate User via LDAP
   ###
   # Determines which LDAP field populates the username in a user's Chef Automate session on successful authentication.
   user_id_attr = "<your userid attribute>"

   # Optional: determines which LDAP field populates the email in a user's Chef Automate session on successful authentication.
   # Defaults to "user_id_attr" if not specified.
   email_attr = "<your email attribute>"

   # Optional: determines which LDAP field populates the display name in a user's Chef Automate session on successful authentication.
   # Defaults to "name" if not specified.
   user_display_name_attr = "<your user display name attribute>"

   ###
   # Group Query (search for LDAP group membership for an authenticated user)
   ###
   # The base DN to start the group membership query.
   # Chef Automate will use this as the base DN on which to search for LDAP group membership for a specific LDAP user.
   # Example: "cn=groups,dc=freeipa,dc=example,dc=com"
   base_group_search_dn = "<your base group search DN>"

   # The following two fields are used to match a user to a group.
   # If the defaults are used, then you end up with a group membership
   # filter of "(&(objectClass=group)(member=<user's DN>))".
   # Optional: The LDAP field by which you wish to filter group membership.
   # Defaults to "member".
   filter_groups_by_user_attr = "<groups to filter by user attribute>"
   # Optional: The LDAP field from the authenticated user you wish to use as input to the above filter.
   # Defaults to "DN".
   filter_groups_by_user_value = "<groups to filter by user value>"

   # Optional: Additional LDAP filter you can define to further filter group membership results.
   group_query_filter = "<your group query filter>"

   # The LDAP field on the group you wish to use as the Chef Automate Team name for the group.
   # Defaults to "name".
   group_display_name_attr = "<group display name attribute>"
```

##### Example Configs

Depending on your directory's schema, different Group Search settings are
required:

If your directory looks like this

```LDIF
dn: dc=corp,dc=com
objectClass: dcObject
objectClass: organization
o: Example Company
dc: corp

dn: ou=People,dc=corp,dc=com
objectClass: organizationalUnit
ou: People

dn: cn=jane,ou=People,dc=corp,dc=com
objectClass: person
objectClass: inetOrgPerson
sn: doe
cn: jane

dn: cn=john,ou=People,dc=corp,dc=com
objectClass: person
objectClass: inetOrgPerson
sn: doe
cn: john

# Groups
dn: ou=Groups,dc=corp,dc=com
objectClass: organizationalUnit
ou: Groups

dn: cn=admins,ou=Groups,dc=corp,dc=com
objectClass: groupOfNames
cn: admins
member: cn=john,ou=People,dc=corp,dc=com
member: cn=jane,ou=People,dc=corp,dc=com

dn: cn=developers,ou=Groups,dc=corp,dc=com
objectClass: groupOfNames
cn: developers
member: cn=jane,ou=People,dc=corp,dc=com
```

then the following would be required:

```toml
base_user_search = "ou=People,dc=corp,dc=com"
username_attr = "cn"
user_id_attr = "cn"
user_display_name_attr = "cn"

base_group_search = "ou=Groups,dc=corp,dc=com"
filter_groups_by_user_value = "DN"
filter_groups_by_user_attr = "member" # default
group_display_name_attr = "cn"
```

However, if your schema looks like this -- with no list of members in your group entries:

```LDIF
dn: dc=corp,dc=com
objectClass: dcObject
objectClass: organization
o: Example Company
dc: corp

dn: ou=People,dc=corp,dc=com
objectClass: organizationalUnit
ou: People

dn: cn=jane,ou=People,dc=corp,dc=com
objectClass: person
objectClass: inetOrgPerson
sn: doe
cn: jane
departmentNumber: 1000
departmentNumber: 1001

dn: cn=john,ou=People,dc=corp,dc=com
objectClass: person
objectClass: inetOrgPerson
sn: doe
cn: john
departmentNumber: 1000
departmentNumber: 1002

dn: ou=Groups,dc=corp,dc=com
objectClass: organizationalUnit
ou: Groups

dn: cn=admins,ou=Groups,dc=corp,dc=com
objectClass: posixGroup
cn: admins
gidNumber: 1000

dn: cn=developers,ou=Groups,dc=corp,dc=com
objectClass: posixGroup
cn: developers
gidNumber: 1001

dn: cn=designers,ou=Groups,dc=corp,dc=com
objectClass: posixGroup
cn: designers
gidNumber: 1002
```

You will need different settings to tie users and groups together:

```toml
base_user_search = "ou=People,dc=corp,dc=com"
username_attr = "cn"
user_id_attr = "cn"
user_display_name_attr = "cn"

base_group_search = "ou=Groups,dc=corp,dc=com"
filter_groups_by_user_value = "departmentNumber"
filter_groups_by_user_attr = "gidNumber"
group_display_name_attr = "cn"
```

### Troubleshooting

The following section will lay down some indicators to determine which step of
the sign in process has failed.

#### Troubleshoot your Connection

If the host or port was wrong, or Chef Automate was not able to reach the LDAP
service, the sign in screen will display

> Internal Server Error
>
> Login error.

In the logs (`journalctl -u chef-automate`), you will find a line from
`automate-dex.default` like this -- note that for readability, the timestamp and service name has been removed from this example log):

```text
level=error msg="Failed to login user: failed to connect: LDAP Result Code 200 \"\": dial tcp 192.168.33.223:10637: getsockopt: connection refused"
```

Note that the log contains the _IP address_ even when the LDAP server was
configured via hostname. Double-checking that can be helpful to exclude issues
in domain-name resolution.

Issues in TLS verification manifest in the same way, but the log indicates that:

```text
level=error msg="Failed to login user: failed to connect: LDAP Result Code 200 \"\": x509: certificate is valid for localhost, not dex-dev.test"
```

#### Troubleshoot Bind

Issues in bind manifest in the same way ("Internal Server Error") as Connect issues.
However, they differ in what gets logged:

```text
level=error msg="Failed to login user: ldap: initial bind for user \"cn=service_account,dc=corp,dc=com\" failed: LDAP Result Code 49 \"Invalid Credentials\": "
```

#### Troubleshoot User Search

There's two main ways the user search could fail, and they lead to different
sign in failures: One is queries that cannot be executed at all, leading to

> Internal Server Error
>
> Login error.

in the browser and a line like

```text
level=info msg="performing ldap search ou=Peoples,dc=example,dc=org sub (cn=jane)"
level=error msg="Failed to login user: ldap: search with filter \"(cn=jane)\" failed: LDAP Result Code 32 \"No Such Object\": "
```

in the logs.

One possible cause (whose logs you see here) is a misconfigured
`base_user_search_dn`.

When the user search is executed successfully, but fails to return a useful user
record, the browser will show the sign in prompt with an error banner saying

> Username or password is incorrect.

In the logs, you will find more information. There is a line informing you about
the actual _search_ query,

```text
level=info msg="performing ldap search ou=People,dc=corp,dc=com sub (cnn=jane)"
```

together with an entry saying that nothing was returned by the attempted query:

```text
level=error msg="ldap: no results returned for filter: \"(cnn=jane)\""
```

In this example output, the `username_attr` was set to `cnn` (not `cn`).

Since there is no way for the LDAP integration to determine whether a
configuration was _wrong_ or the provided user does not exist, the sign in UI can
only assume that the credentials were invalid.

Note that invalid entries for `user_query_filter` will lead to queries that
return no entries, too. Setting

```toml
user_query_filter = "(objectClass=person)"
```

will lead to the following logs:

```text
level=info msg="performing ldap search ou=People,dc=example,dc=org sub (&(objectClass=person(cn=jane))" connector=LDAP
level=error msg="ldap: no results returned for filter: \"(&(objectClass=person(cn=jane))\"" connector=LDAP
```

{{% warning %}}
User search also fails if more than one user is returned.
{{% /warning %}}

Ensure that a search for `username_attr` with the given search base can only
return one user. Something like this could happen (simplified for
demonstration):

```LDIF
dn: cn=jane,ou=Denver,ou=People,dc=corp,dc=com
sn: doe
cn: jane
username: jdoe

dn: cn=john,ou=Boston,ou=People,dc=corp,dc=com
sn: doe
cn: john
username: jdoe
```

with

```toml
base_user_search_dn = "ou=People,dc=corp,dc=com"
username_attr = "username"
```

neither Jane Doe nor her brother could sign in to Chef Automate. There would be a
log indicating that multiple users have been returned.

This situation would be averted by setting `username_attr = "cn"`; or by
restricting `base_user_search_dn`, if you only want to allow people from one of
either cities to use Chef Automate.

{{% warning %}}
Attributes that have been configured, but are not found in the results, lead to
user search failures, too. Note that this also affects default values.
{{% /warning %}}

Finally, a successful user search logs a line like the following:

```text
level=info msg="username \"jane\" mapped to entry cn=jane,ou=People,dc=corp,dc=com"
```

#### Troubleshoot Sign In Bind

Failures in sign in bind that are not caused by invalid credentials will lead to

> Internal Server Error
>
> Login error.

accompanied by a log line with more details, starting with `Failed to sign in user`.

#### Troubleshoot Group Search

Failures in retrieving a user's groups will inhibit their sign in with

> Internal Server Error
>
> Login error.

and logs like

```text
level=info msg="performing ldap search ou=Groups,dc=example,dc=org sub (member=cn=jane,ou=People,dc=example,dc=org)"
level=error msg="Failed to login user: ldap: failed to query groups: ldap: search failed: LDAP Result Code 32 \"No Such Object\": "
```

This, for example, is what you see when the `base_group_search_dn` does not
exist (`"ou=Groups,dc=..."`).

However, contrary to how _User Search_ works, an empty result from _Group
Search_ will not inhibit sign in, it will merely not populate the user's
internal record with any groups.

A successful sign in causes log entries like the following:

```text
level=info msg="performing ldap search ou=People,dc=corp,dc=com sub (cn=jane)"
level=info msg="username \"jane\" mapped to entry cn=jane,ou=People,dc=corp,dc=com"
level=info msg="performing ldap search ou=Groups,dc=corp,dc=com sub (member=cn=jane,ou=People,dc=corp,dc=com)"
level=info msg="login successful: connector \"ldap\", username=\"jane\", email=\"janedoe@example.com\", groups=[\"admins\" \"developers\"]"
```

and subsequent API authorization request logs containing the user's _subjects_:

```text
level=info msg="Authorization Query" action=search resource="compliance:profiles" result=true subject="[team:ldap:admins team:ldap:developers user:ldap:jane]"
```

#### `ldapsearch` Example Queries

For debugging purposes it can be useful to execute LDAP queries manually using
the `ldapsearch` utility. On Ubuntu, it is provided via `ldap-utils` (i.e.,
`sudo apt-get install ldap-utils`).
In what follows, we will outline an example directory layout, and the `ldapsearch`
queries corresponding to the different phases.

The _User Search_ query looks like this, with comments referencing the
configurables for LDAP integration:

```shell
ldapsearch -H ldap://ldap-server:636/ \  # host
  -D cn=service_account,dc=corp,dc=com \ # bind_dn
  -w admin \                             # bind_password
  -b ou=People,dc=corp,dc=com \          # base_user_search_dn
  -s sub \
  '(cn=jane)'                            # (username_attr=what-was-provided-via-sign-in-form)
```

When using anonymous bind:

```shell
ldapsearch -H ldap://ldap-server:636/ \ # host
  -b ou=People,dc=corp,dc=com \         # base_user_search_dn
  -s sub \
  '(cn=jane)'                           # (username_attr=what-was-provided-via-sign-in-form)
```

If you have configured a `user_query_filter`, it is wrapped into the filter
argument:

```shell
  '(&(objectClass=person)(cn=jane))'    # (&user_query_filter(username_attr=what-was-provided-via-sign-in-form))
```

Once a user directory entry has been retrieved, the password can be verified,
and the group query can be constructed from it:

Let us assume we have gotten the entry for user `jane`:

```LDIF
# jane, People, corp.com
dn: cn=jane,ou=People,dc=corp,dc=com
objectClass: person
objectClass: inetOrgPerson
sn: doe
cn: jane
```

then the password verification can be simulated by

```shell
ldapsearch -H ldap://ldap-server:636/ \ # host
  -b cn=jane,ou=People,dc=corp,dc=com \ # always the entry's DN
  -w janespassword                      # as provided via sign in from
```

where any non-failure result (such as `32 No such object`) would indicate valid
credentials.

Finally, the group search query for that user entry looks like

```shell
ldapsearch -H ldap://ldapserver:636/ \        # host
  -D cn=service_account,dc=corp,dc=com \      # bind_dn
  -w admin \                                  # bind_password
  -b ou=Groups,dc=corp,dc=com \               # base_group_search_dn
  -s sub \
  '(member=cn=jane,ou=People,dc=corp,dc=com)' # (filter_groups_by_user_attr=[that attr of user entry])
```

With an additional `group_query_filter`, the final filter is

```shell
  '(&(objectClass=group)(member=cn=jane,ou=People,dc=corp,dc=com))' # (&group_query_filter(filter_groups_by_user_attr=[...])
```

Note: if the user entry contains more than one `filter_groups_by_user_attr`
attribute, multiple queries will be executed, and their results combined.

#### Other Common Issues

If a user, following a sign in through LDAP or SAML, sees a

> 502 Bad Gateway

error page, the group information collected for the user exceeds some
internal limits.

This can have two causes: the user having too many groups, or referencing LDAP
groups by distinguished names (DN). The latter can cause little information
(e.g. the group name _"admins"_) to grow out of proportion (e.g.
_"cn=admins,ou=DeptA,ou=CityB,ou=StateWA,dc=subcorp,dc=corp,dc=com"_). This can
be mitigated by changing the `group_display_name_attr` from `DN` to `cn`
(common name). Note that for authorization purposes, that is also advisable.
LDAP-provided groups are referenced in policies using `team:ldap:<group-name>`.
Thus `team:ldap:admins` is handier than
`team:ldap:cn=admins,ou=DeptA,ou=CityB,ou=StateWA,dc=subcorp,dc=corp,dc=com`.

The other cause, having too many groups for a user, can be addressed by using
the `group_query_filter` to restrict the group results (for all users).
Anything expressible in an LDAP search query and supported by the LDAP service
can be configured there. For example, given a flat list of groups in a
directory service like

```LDIF
cn=group1,ou=Groups,dc=corp,dc=com
cn=group2,ou=Groups,dc=corp,dc=com
cn=group3,ou=Groups,dc=corp,dc=com
cn=group4,ou=Groups,dc=corp,dc=com
cn=group5,ou=Groups,dc=corp,dc=com
```

a `group_query_filter` of `(|(cn=group1)(cn=group2))` would restrict the
group search results to either one of those groups. Note that this has no
implications on which users get authenticated; it only affects the groups
recognized by Chef Automate. For example, given users Jane and Jack, where
Jane is a member of `group1` and `group3`, and Jack of `group3` and `group4`:
Jane's groups would resolve to `group1` only, and Jack would have no groups
-- but still be able to access Chef Automate.
In a similar manner, selected groups could be excluded from the results
explicitly, by using a filter like `(!cn=group2)`

Given a more structured directory service layout, including multiple trees of
groups, further options become possible:
Assuming the layout is like

```LDIF
cn=group1,ou=AGroups,dc=corp,dc=com
cn=group2,ou=AGroups,dc=corp,dc=com
cn=group3,ou=BGroups,dc=corp,dc=com
cn=group4,ou=BGroups,dc=corp,dc=com
cn=group5,ou=CGroups,dc=corp,dc=com
```

you can use your directory server's query capabilities to restrict the
results to a subtree. The concrete details depend on the product in use; for
example in servers supporting _extensible match_, all group entries below
`AGroups` and `BGroups` could be retrieved using a `group_query_filter` of
`(|(ou:dn:=AGroups)(ou:dn:=BGroups))`.

See [LDAP Wiki Extensible Match Search Filter](http://ldapwiki.com/wiki/ExtensibleMatch) for details.
