<div class="admonition-warning">
<p class="admonition-warning-title">Warning</p>
<div class="admonition-warning-text">

The following settings **MUST** be in the config file for LDAP
authentication to Active Directory to work:

-   `base_dn`
-   `bind_dn`
-   `group_dn`
-   `host`

If those settings are missing, you will get authentication errors and be
unable to proceed.

</div>
</div>

This configuration file has the following settings for `ldap`:

`ldap['base_dn']`

:   The root LDAP node under which all other nodes exist in the
    directory structure. For Active Directory, this is typically
    `cn=users` and then the domain. For example:

    ``` ruby
    'OU=Employees,OU=Domain users,DC=example,DC=com'
    ```

    Default value: `nil`.

`ldap['bind_dn']`

:   The distinguished name used to bind to the LDAP server. The user the
    Chef Infra Server will use to perform LDAP searches. This is often
    the administrator or manager user. This user needs to have read
    access to all LDAP users that require authentication. The Chef Infra
    Server must do an LDAP search before any user can log in. Many
    Active Directory and LDAP systems do not allow an anonymous bind. If
    anonymous bind is allowed, leave the `bind_dn` and `bind_password`
    settings blank. If anonymous bind is not allowed, a user with `READ`
    access to the directory is required. This user must be specified as
    an LDAP distinguished name similar to:

    ``` ruby
    'CN=user,OU=Employees,OU=Domainuser,DC=example,DC=com'
    ```

    <div class="admonition-note">
    <p class="admonition-note-title">Note</p>
    <div class="admonition-note-text">

    If you need to escape characters in a distinguished name, such as
    when using Active Directory, they must be [escaped with a backslash
    escape
    character](https://social.technet.microsoft.com/wiki/contents/articles/5312.active-directory-characters-to-escape.aspx).

    ``` ruby
    'CN=example\\user,OU=Employees,OU=Domainuser,DC=example,DC=com'
    ```

    </div>
    </div>

    Default value: `nil`.

`ldap['bind_password']`

:   Legacy configuration for the password of the binding user. The
    password for the user specified by `ldap['bind_dn']`. Leave this
    value and `ldap['bind_dn']` unset if anonymous bind is sufficient.
    Default value: `nil`. As of Chef Server 12.14, this is no longer the
    preferred command.

    Please use `chef-server-ctl set-secret ldap bind_password` from the
    [Secrets
    Management](/ctl_chef_server.html#ctl-chef-server-secrets-management)
    commands.

    ``` bash
    chef-server-ctl set-secret ldap bind_password
    Enter ldap bind_password:    (no terminal output)
    Re-enter ldap bind_password: (no terminal output)
    ```

    Remove a set password via

    ``` bash
    chef-server-ctl remove-secret ldap bind_password
    ```

`ldap['group_dn']`

:   The distinguished name for a group. When set to the distinguished
    name of a group, only members of that group can log in. This feature
    filters based on the `memberOf` attribute and only works with LDAP
    servers that provide such an attribute. In OpenLDAP, the `memberOf`
    overlay provides this attribute. For example, if the value of the
    `memberOf` attribute is `CN=abcxyz,OU=users,DC=company,DC=com`, then
    use:

    ``` ruby
    ldap['group_dn'] = 'CN=abcxyz,OU=users,DC=company,DC=com'
    ```

`ldap['host']`

:   The name (or IP address) of the LDAP server. The hostname of the
    LDAP or Active Directory server. Be sure the Chef Infra Server is
    able to resolve any host names. Default value: `ldap-server-host`.

`ldap['login_attribute']`

:   The LDAP attribute that holds the user's login name. Use to specify
    the Chef Infra Server user name for an LDAP user. Default value:
    `sAMAccountName`.

`ldap['port']`

:   An integer that specifies the port on which the LDAP server listens.
    The default value is an appropriate value for most configurations.
    Default value: `389` or `636` when `ldap['encryption']` is set to
    `:simple_tls`.

`ldap['ssl_enabled']`

:   Cause the Chef Infra Server to connect to the LDAP server using SSL.
    Default value: `false`. Must be `false` when `ldap['tls_enabled']`
    is `true`.

    <div class="admonition-note">
    <p class="admonition-note-title">Note</p>
    <div class="admonition-note-text">

    It's recommended that you enable SSL for Active Directory.

    </div>
    </div>

    <div class="admonition-note">
    <p class="admonition-note-title">Note</p>
    <div class="admonition-note-text">

    Previous versions of the Chef Infra Server used the
    `ldap['ssl_enabled']` setting to first enable SSL, and then the
    `ldap['encryption']` setting to specify the encryption type. These
    settings are deprecated.

    </div>
    </div>

`ldap['system_adjective']`

:   A descriptive name for the login system that is displayed to users
    in the Chef Infra Server management console. If a value like
    "corporate" is used, then the Chef management console user interface
    will display strings like "the corporate login server", "corporate
    login", or "corporate password." Default value: `AD/LDAP`.

    <div class="admonition-warning">
    <p class="admonition-warning-title">Warning</p>
    <div class="admonition-warning-text">

    This setting is **not** used by the Chef Infra Server. It is used
    only by the Chef management console.

    </div>
    </div>

`ldap['timeout']`

:   The amount of time (in seconds) to wait before timing out. Default
    value: `60000`.

`ldap['tls_enabled']`

:   Enable TLS. When enabled, communication with the LDAP server is done
    via a secure SSL connection on a dedicated port. When `true`,
    `ldap['port']` is also set to `636`. Default value: `false`. Must be
    `false` when `ldap['ssl_enabled']` is `true`.

    <div class="admonition-note">
    <p class="admonition-note-title">Note</p>
    <div class="admonition-note-text">

    Previous versions of the Chef Infra Server used the
    `ldap['ssl_enabled']` setting to first enable SSL, and then the
    `ldap['encryption']` setting to specify the encryption type. These
    settings are deprecated.

    </div>
    </div>
