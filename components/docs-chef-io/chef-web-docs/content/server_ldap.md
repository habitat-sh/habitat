+++
title = "Active Directory and LDAP"
draft = false

aliases = ["/server_ldap.html", "/install_server_post.html"]

[menu]
  [menu.infra]
    title = "Active Directory & LDAP"
    identifier = "chef_infra/managing_chef_infra_server/server_ldap.md Active Directory & LDAP"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_ldap.md)

The Chef Infra Server supports Active Directory and LDAP authentication,
which enables users to log in to the Chef Infra Server using their
corporate credential and the Manage interface. Without the Manage interface add-on installed,
there is no need to enable the Chef Infra Server LDAP functionality. LDAP is not used with 
Supermarket logins, nor with any Chef Infra Client related authentication.

## Configure LDAP

The Chef Infra Server supports using Active Directory or LDAP for any
user that has an email address in the LDAP directory. This allows those
users to log in to the Chef Infra Server by using their corporate
credentials instead of having a separate username and password.

{{< warning >}}

The following attributes **MUST** be in the user LDAP record:

-   `mail:`
-   `sAMAccountName:` or `uid:`

The following attributes **SHOULD** be in the user LDAP record:

-   `displayname:`
-   `givenname:`
-   `sn:`
-   `c:`
-   `l:`

{{< /warning >}}

To configure the Chef Infra Server to use Active Directory or LDAP do
the following:

1.  Install the Chef management console (if it is not already).

2.  Add the following settings to the `/etc/opscode/chef-server.rb`
    file. These settings must be added to the `chef-server.rb` file on
    each machine in the Chef Infra Server frontend deployment of a High
    Availability installation as well as on Chef servers in a standalone
    installation.

    {{< readFile_shortcode file="config_rb_server_settings_ldap.md" >}}

    {{< note spaces=4 >}}

    If the chef-server.rb file does not exist, create a file called
    `chef-server.rb` and put it in the `/etc/opscode/` directory.

    {{< /note >}}

3.  {{< readFile_shortcode file="install_chef_server_reconfigure.md" >}}

At this point, all users should be able to use their Active Directory or
LDAP usernames and passwords to log in to the Chef Infra Server.

## Test LDAP Connectivity

Use `ldapsearch` to test the ability of the Chef Infra Server to use
Active Directory or LDAP. First, translate the Chef Infra Server LDAP
settings into `ldapsearch` parameters:

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr class="header">
<th>Chef Infra Server Setting</th>
<th><code>ldapsearch</code> Parameter</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>ldap['host']</code> and <code>ldap['port']</code></td>
<td><code>-H [HOST:PORT]</code></td>
</tr>
<tr class="even">
<td><code>ldap['bind_dn']</code></td>
<td><code>-D [BIND_DN]</code></td>
</tr>
<tr class="odd">
<td><code>ldap['bind_password']</code></td>
<td><code>-W</code>; <code>ldapsearch</code> will prompt for this parameter</td>
</tr>
<tr class="even">
<td><code>ldap['base_dn']</code></td>
<td><code>-b [BASE_DN]</code></td>
</tr>
<tr class="odd">
<td><code>ldap['login_attribute']</code></td>
<td>Defaults to <code>SAMAccountName</code></td>
</tr>
</tbody>
</table>

And then from a front end machine (in a high availability or tiered
configuration) or from the Chef Infra Server in a standalone
configuration, run the following command. Be sure to replace the
uppercase placeholders with the values for your organization:

``` bash
ldapsearch -LLL -H ldap://HOST:PORT -b 'BASE_DN' -D 'BIND_DN' -W '(LOGIN_ATTRIBUTE=YOUR_LDAP_ACCOUNT_USERNAME)'
```

For example:

``` bash
ldapsearch -LLL -H ldap://win-ad1.chef.co:389 -b 'OU=Employees,OU=Domain users,DC=opscodecorp,DC=com' -D 'CN=Robert Forster,OU=Employees,OU=Domain users,DC=opscodecorp,DC=com' -W '(sAMAccountName=rforster)'
```

Output similar to the following is returned:

``` bash
ldapsearch -LLL -H ldap://win-ad1.chef.co:389 -b 'OU=Employees,OU=Domain users,DC=opscodecorp,DC=com' -D 'CN=Robert Forster,OU=Employees,OU=Domain users,DC=opscodecorp,DC=com' -W '(sAMAccountName=rforster)'
Enter LDAP Password:

dn: CN=Robert Forster,OU=Employees,OU=Domain users,DC=opscodecorp,DC=com
objectClass: top
objectClass: person
objectClass: organizationalPerson
objectClass: user
cn: Robert Forster
sn: Forster
c: 0
givenName: Robert
distinguishedName: CN=Robert Forster,OU=Employees,OU=Domain users,DC=opscodecorp,DC
 =com
```

{{< note >}}

The `ldapsearch` command may need to be installed on the platform. It is
not included as part of the Chef Infra Server package.

{{< /note >}}
