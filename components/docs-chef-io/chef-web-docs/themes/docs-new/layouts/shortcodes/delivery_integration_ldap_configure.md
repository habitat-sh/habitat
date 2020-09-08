To configure LDAP for Workflow:

1.  Open `/etc/delivery/delivery.rb` and enter the LDAP attributes you
    want Workflow to use. If you do not specify an LDAP port, the
    default port of `3269` is used.

    ``` ruby
    delivery['ldap_hosts'] = ["ldap.tld"]
    delivery['ldap_port'] = 3269
    delivery['ldap_timeout'] = 5000
    delivery['ldap_base_dn'] = "OU=Employees,OU=Domain users,DC=opscodecorp,DC=com"
    delivery['ldap_bind_dn'] = "ldapbind"
    delivery['ldap_bind_dn_password'] = "secret123"
    delivery['ldap_encryption'] = "start_tls"
    delivery['ldap_attr_login'] = 'sAMAccountName'
    delivery['ldap_attr_mail'] = 'mail'
    delivery['ldap_attr_full_name'] = 'fullName'
    ```

2.  Run the following command to complete the configuration process:

    ``` bash
    sudo automate-ctl reconfigure
    ```

Once Workflow is set up, you will have a usable **LDAP** option in the
Workflow **Users** page that allows you to find users through your LDAP
database.