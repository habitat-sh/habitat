+++
title = "Security"
draft = false

aliases = ["/server_security.html"]

runbook_weight = 50

[menu]
  [menu.infra]
    title = "Security"
    identifier = "chef_infra/managing_chef_infra_server/server_security.md Security"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 90
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/runbook/server_security.md)

This guide covers the security features available in Chef Infra Server.

## SSL Certificates

Initial configuration of the Chef Infra Server is done automatically
using a self-signed certificate to create the certificate and private
key files for Nginx. This section details the process for updating a
Chef Infra Server's SSL certificate.

### Automatic Installation (recommended)

The Chef Infra Server can be configured to use SSL certificates by
adding the following settings to the server configuration file:

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
<td><code>nginx['ssl_certificate']</code></td>
<td>The SSL certificate used to verify communication over HTTPS.</td>
</tr>
<tr class="even">
<td><code>nginx['ssl_certificate_key']</code></td>
<td>The certificate key used for SSL communication.</td>
</tr>
</tbody>
</table>

and then setting their values to define the paths to the certificate and
key.

For example:

``` ruby
nginx['ssl_certificate']  = "/etc/pki/tls/certs/your-host.crt"
nginx['ssl_certificate_key']  = "/etc/pki/tls/private/your-host.key"
```

Save the file, and then run the following command:

``` bash
sudo chef-server-ctl reconfigure
```

For more information about the server configuration file, see
[chef-server.rb](/config_rb_server/).

### Manual Installation

SSL certificates can be updated manually by placing the certificate and
private key file obtained from the certifying authority in the correct
files, after the initial configuration of Chef Infra Server.

The locations of the certificate and private key files are:

-   `/var/opt/opscode/nginx/ca/FQDN.crt`
-   `/var/opt/opscode/nginx/ca/FQDN.key`

Because the FQDN has already been configured, do the following:

1.  Replace the contents of `/var/opt/opscode/nginx/ca/FQDN.crt` and
    `/var/opt/opscode/nginx/ca/FQDN.key` with the certifying authority's
    files.

2.  Reconfigure the Chef Infra Server:

    ``` bash
    chef-server-ctl reconfigure
    ```

3.  Restart the Nginx service to load the new key and certificate:

    ``` bash
    chef-server-ctl restart nginx
    ```

{{< warning >}}

The FQDN for the Chef Infra Server should be resolvable, lowercase, and
have fewer than 64 characters including the domain suffix, when using
OpenSSL, as OpenSSL requires the `CN` in a certificate to be no longer
than 64 characters.

{{< /warning >}}

### SSL Protocols

{{% server_tuning_nginx %}}

**Example: Configure SSL Keys for Nginx**

The following example shows how the Chef Infra Server sets up and
configures SSL certificates for Nginx. The cipher suite used by Nginx
[is configurable](/config_rb_server/#ssl-protocols) using the
`ssl_protocols` and `ssl_ciphers` settings.

``` ruby
ssl_keyfile = File.join(nginx_ca_dir, "#{node['private_chef']['nginx']['server_name']}.key")
ssl_crtfile = File.join(nginx_ca_dir, "#{node['private_chef']['nginx']['server_name']}.crt")
ssl_signing_conf = File.join(nginx_ca_dir, "#{node['private_chef']['nginx']['server_name']}-ssl.conf")

unless File.exist?(ssl_keyfile) && File.exist?(ssl_crtfile) && File.exist?(ssl_signing_conf)
  file ssl_keyfile do
    owner 'root'
    group 'root'
    mode '0755'
    content '/opt/opscode/embedded/bin/openssl genrsa 2048'
    not_if { File.exist?(ssl_keyfile) }
  end

  file ssl_signing_conf do
    owner 'root'
    group 'root'
    mode '0755'
    not_if { File.exist?(ssl_signing_conf) }
    content <<-EOH
  [ req ]
  distinguished_name = req_distinguished_name
  prompt = no
  [ req_distinguished_name ]
  C                      = #{node['private_chef']['nginx']['ssl_country_name']}
  ST                     = #{node['private_chef']['nginx']['ssl_state_name']}
  L                      = #{node['private_chef']['nginx']['ssl_locality_name']}
  O                      = #{node['private_chef']['nginx']['ssl_company_name']}
  OU                     = #{node['private_chef']['nginx']['ssl_organizational_unit_name']}
  CN                     = #{node['private_chef']['nginx']['server_name']}
  emailAddress           = #{node['private_chef']['nginx']['ssl_email_address']}
  EOH
  end

  ruby_block 'create crtfile' do
    block do
      r = Chef::Resource::File.new(ssl_crtfile, run_context)
      r.owner 'root'
      r.group 'root'
      r.mode '0755'
      r.content "/opt/opscode/embedded/bin/openssl req -config '#{ssl_signing_conf}' -new -x509 -nodes -sha1 -days 3650 -key '#{ssl_keyfile}'"
      r.not_if { File.exist?(ssl_crtfile) }
      r.run_action(:create)
    end
  end
end
```

### Knife, Chef Infra Client

{{% server_security_ssl_cert_client %}}

See [Chef Infra Client SSL
Certificates](/chef_client_security/#ssl-certificates) for more
information on how knife and Chef Infra Client use SSL certificates
generated by the Chef Infra Server.

### Private Certificate Authority

If an organization is using an internal certificate authority, then the
root certificate will not appear in any `cacerts.pem` file that ships by
default with operating systems and web browsers. Because of this, no
currently deployed system will be able to verify certificates that are
issued in this manner. To allow other systems to trust certificates from
an internal certificate authority, this root certificate will need to be
configured so that other systems can follow the chain of authority back
to the root certificate. (An intermediate certificate is not enough
because the root certificate is not already globally known.)

To use an internal certificate authority, append the server--optionally,
any intermediate certificate as well--and root certificates into a
single `.crt` file. For example:

``` bash
cat server.crt [intermediate.crt] root.crt >> /var/opt/opscode/nginx/ca/FQDN.crt
```

Check your combined certificate's validity on the Chef Infra Server:

``` bash
openssl verify -verbose -purpose sslserver -CAfile cacert.pem  /var/opt/opscode/nginx/ca/FQDN.crt
```

The cacert.pem should contain only your root CA's certificate file. This
is not the usual treatment, but mimics how Chef Workstation behaves
after a `knife ssl fetch` followed by a `knife ssl verify`.

### Intermediate Certificates

For use with 3rd party certificate providers, for example, Verisign.

To use an intermediate certificate, append both the server and
intermediate certificates into a single `.crt` file. For example:

``` bash
cat server.crt intermediate.crt >> /var/opt/opscode/nginx/ca/FQDN.crt
```

### Verify Certificate Was Signed by Proper Key

It's possible that a certificate/key mismatch can occur during the
CertificateSigningRequest (CSR) process. During a CSR, the original key
for the server in question should always be used. If the output of the
following commands don't match, then it's possible the CSR for a new key
for this host was generated using a random key or a newly generated key.
The symptoms of this issue will look like the following in the nginx log
files:

``` bash
nginx: [emerg] SSL_CTX_use_PrivateKey_file("/var/opt/opscode/nginx/ca/YOUR_HOSTNAME.key") failed (SSL: error:0B080074:x509    certificate routines:X509_check_private_key:key values mismatch)
```

Here's how to tell for sure when the configured certificate doesn't
match the key

``` bash
## openssl x509 -in /var/opt/opscode/nginx/ca/chef-432.lxc.crt -noout -modulus | openssl sha1
(stdin)= 05b4f62e52fe7ce2351ff81d3e1060c0cdf1fa24

## openssl rsa -in /var/opt/opscode/nginx/ca/chef-432.lxc.key -noout -modulus | openssl sha1
(stdin)= 05b4f62e52fe7ce2351ff81d3e1060c0cdf1fa24
```

To fix this, you will need to generate a new CSR using the original key
for the server, the same key that was used to produce the CSR for the
previous certificates. Install that new certificates along with the
original key and the mismatch error should go away.

### Regenerate Certificates

SSL certificates should be regenerated periodically. This is an
important part of protecting the Chef Infra Server from vulnerabilities
and helps to prevent the information stored on the Chef Infra Server
from being compromised.

To regenerate SSL certificates:

1.  Run the following command:

    ``` bash
    chef-server-ctl stop
    ```

2.  The Chef Infra Server can regenerate them. These certificates will
    be located in `/var/opt/opscode/nginx/ca/` and will be named after
    the FQDN for the Chef Infra Server. To determine the FQDN for the
    server, run the following command:

    ``` bash
    hostname -f
    ```

    Please delete the files found in the ca directory with names like
    this `$FQDN.crt` and `$FQDN.key`.

3.  If your organization has provided custom SSL certificates to the
    Chef Infra Server, the locations of that custom certificate and
    private key are defined in `/etc/opscode/chef-server.rb` as values
    for the `nginx['ssl_certificate']` and
    `nginx['ssl_certificate_key']` settings. Delete the files referenced
    in those two settings and regenerate new keys using the same
    authority.

4.  Run the following command, Chef server-generated SSL certificates
    will automatically be created if necessary:

    ``` bash
    chef-server-ctl reconfigure
    ```

5.  Run the following command:

    ``` bash
    chef-server-ctl start
    ```

## Chef Infra Server Credentials Management

**New in Chef Server 12.14:** Chef Infra Server limits where it writes
service passwords and keys to disk. In the default configuration,
credentials are only written to files in `/etc/opscode`.

By default, Chef Infra Server still writes service credentials to
multiple locations inside `/etc/opscode`. This is designed to maintain
compatibility with add-ons. Chef Server 12.14 introduces the
`insecure_addon_compat` configuration option in
`/etc/opscode/chef-server.rb`, which allows you to further restrict
where credentials are written. `insecure_addon_compat` can be used if
you are not using add-ons, or if you are using the latest add-on
versions. Setting `insecure_addon_compat` to `false` writes credentials
to only one location: `/etc/opscode/private-chef-secrets.json`.

User-provided secrets (such as the password for an external PostgreSQL
instance) can still be set in `/etc/opscode/chef-server.rb` or via the
[Secrets
Management](/ctl_chef_server/#ctl-chef-server-secrets-management)
commands. These commands allow you to provide external passwords without
including them in your configuration file.

### Add-on Compatibility

The following table lists which add-on versions support the more
restrictive `insecure_addon_compat false` setting. These version also
now **require** Chef Server 12.14.0 or greater:

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr class="header">
<th>Add-on Name</th>
<th>Minimum Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Backend</td>
<td><em>all</em></td>
</tr>
<tr class="even">
<td>Chef Manage</td>
<td>2.5.0</td>
</tr>
<tr class="odd">
<td>Push Jobs Server</td>
<td>2.2.0</td>
</tr>
</tbody>
</table>

These newer add-ons will also write all of their secrets to
`/etc/opscode/private-chef-secrets.json`. Older versions of the add-ons
will still write their configuration to locations in `/etc` and
`/var/opt`.

### /etc/opscode/private-chef-secrets.json

`/etc/opscode/private-chef-secrets.json`'s default permissions allow
only the root user to read or write the file. This file contains all of
the secrets for access to the Chef server's underlying data stores and
thus access to it should be restricted to trusted users.

While the file does not contain passwords in plaintext, it is not safe
to share with untrusted users. The format of the secrets file allows
Chef Infra Server deployments to conform to regulations that forbid the
appearance of sensitive data in plain text in configuration files;
however, it does not make the file meaningfully more secure.

## SSL Encryption Between Chef Infra Server and External PostgreSQL

**New in Chef Infra Server 13.1.13:** Chef Infra Server 13.1.13
introduces the ability to encrypt traffic between Chef Infra Server and
an external PostgreSQL server over SSL. These instructions are not
all-encompassing and assume some familiarity with PostgreSQL
administration, configuration, and troubleshooting. Consult the
[PostgreSQL
documentation](https://www.postgresql.org/docs/9.6/ssl-tcp.html) for
more information.

The following is a typical scenario for enabling encryption between a
machine running Chef Infra Server and an external machine running
PostgreSQL. Both machines must be networked together and accessible to
the user.

1.  Run the following command on both machines to gain root access:

    ``` bash
    sudo -i
    ```

2.  Ensure that [OpenSSL](https://www.openssl.org) is installed on the
    PostgreSQL machine.

3.  Ensure that SSL support is compiled in on PostgreSQL. This applies
    whether you are compiling your own source or using a pre-compiled
    binary.

4.  Place SSL certificates in the proper directories on the PostgreSQL
    machine and ensure they have correct filenames, ownerships, and
    permissions.

5.  Enable SSL on PostgreSQL by editing the `postgresql.conf` file. Set
    `ssl = on` and specify the paths to the SSL certificates:

    ``` text
    ssl=on

    ssl_cert_file='/path/to/cert/file'
    ssl_key_file='/path/to/key/file'
    ```

6.  To prevent PostgreSQL from accepting non-SSL connections, edit
    `pg_hba.conf` on the PostgreSQL machine and change the relevant Chef
    Infra Server connections to `hostssl`.

    Here is a sample `pg_hba.conf` file with <span
    class="title-ref">hostssl</span> connections for Chef Infra Server
    (the contents of your `pg_hba.conf` will be different):

    ``` text
    # "local" is for Unix domain socket connections only
    local      all             all                                     peer

    # IPv4 local connections:
    hostssl    all             all             127.0.0.1/32            md5

    # IPv6 local connections:
    hostssl    all             all             ::1/128                 md5

    # nonlocal connections
    hostssl    all             all            192.168.33.100/32        md5
    ```

7.  Restart PostgreSQL. This can typically be done with the following
    command on the PostgreSQL machine:

    ``` bash
    /path/to/postgresql/postgresql restart
    ```

8.  Edit `/etc/opscode/chef-server.rb` on the Chef Infra Server and add
    the following line:

    ``` ruby
    postgresql['sslmode']='require'
    ```

9.  Run reconfigure on the Chef Infra Server:

    ``` bash
    chef-server-ctl reconfigure
    ```

10. Verify that SSL is enabled and that SSL connections are up between
    Chef Infra Server and your running PostgreSQL instance. One way to
    do this is to log into the PostgreSQL database from the Chef Infra
    Server by running `chef-server-ctl psql` and then examine the SSL
    state using SQL queries.

    Start a psql session:

    ``` bash
    chef-server-ctl psql opscode_chef
    ```

    From the psql session, enter `postgres=# show ssl;` which will show
    if ssl is enabled:

    ``` sql
    postgres=# show ssl;

     ssl
    -----
     on
    (1 row)
    ```

    Then enter `postgres=# select * from pg_stat_ssl;` which will return
    true (`t`) in rows with SSL connections:

    ``` sql
    postgres=# select * from pg_stat_ssl;

      pid  | ssl | version |           cipher            | bits | compression | clientdn
    -------+-----+---------+-----------------------------+------+-------------+----------
     16083 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16084 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16085 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16086 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16087 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16088 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16089 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16090 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16091 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16092 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16093 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16094 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16095 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16096 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16097 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16098 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16099 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16100 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16101 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16102 | t   | TLSv1.2 | ECDHE-RSA-AES256-GCM-SHA384 |  256 | f           |
     16119 | f   |         |                             |      |             |
    (21 rows)
    ```

## Key Rotation

See the [chef-server-ctl key rotation
commands](/ctl_chef_server/#key-rotation) for more information about
user key management.
