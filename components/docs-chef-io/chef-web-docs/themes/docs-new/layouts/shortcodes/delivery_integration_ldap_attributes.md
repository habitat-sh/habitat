The following table describes the LDAP attributes that may be used with
Workflow:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>ldap_attr_fullname</code></td>
<td>The full user name for an LDAP user. Default value: <code>nil</code>.</td>
</tr>
<tr class="even">
<td><code>ldap_attr_login</code></td>
<td>The login user name for an LDAP user. Default value: <code>sAMAccountName</code>.</td>
</tr>
<tr class="odd">
<td><code>ldap_attr_mail</code></td>
<td>The email for an LDAP user. Default value: <code>mail</code>.</td>
</tr>
<tr class="even">
<td><code>ldap_base_dn</code></td>
<td>Base dn to use when searching for users in LDAP, typically <code>OU=Users</code> and then the domain. Default value: <code>OU=Employees,OU=Domain users,DC=examplecorp,DC=com</code>.</td>
</tr>
<tr class="odd">
<td><code>ldap_bind_dn</code></td>
<td>The user Workflow will use to perform LDAP searches. This is often the administrator or manager user. This user needs to have read access to all LDAP users that require authentication. The Workflow server must do an LDAP search before any user can log in. Many LDAP systems do not allow an anonymous bind. If anonymous bind is allowed, leave the <code>bind_dn</code> and <code>bind_dn_password</code> settings blank. If anonymous bind is not allowed, a user with <code>READ</code> access to the directory is required. This user must be specified as an LDAP distinguished name (<code>dn</code>). Default value: <code>nil</code>.</td>
</tr>
<tr class="even">
<td><code>ldap_bind_dn_password</code></td>
<td>The password for the user specified by <code>ldap['bind_dn']</code>. Leave this value and <code>ldap['bind_dn']</code> unset if anonymous bind is sufficient. Default value: <code>secret123</code>. We do not recommend using a backslash (<code>\</code>) in the password, but if the password needs to have a backslash, please contact support.</td>
</tr>
<tr class="odd">
<td><code>ldap_encryption</code></td>
<td>The type of encryption used to communicate with Workflow. Default value: <code>start_tls</code>. If tls is not in use, set to <code>no_tls</code>.</td>
</tr>
<tr class="even">
<td><code>ldap_hosts</code></td>
<td>An array of hostname(s) of the LDAP server. Be sure Workflow is able to resolve any host names. Default value: <code>[]</code>.</td>
</tr>
<tr class="odd">
<td><code>ldap_port</code></td>
<td>The default value is an appropriate value for most configurations. Default value: <code>3269</code>.</td>
</tr>
<tr class="even">
<td><code>ldap_timeout</code></td>
<td>Timeout when Workflow connects to LDAP. Default value: <code>5000</code>.</td>
</tr>
</tbody>
</table>