+++
title = "Chef Infra Client Security"
draft = false

aliases = ["/chef_client_security.html"]

[menu]
  [menu.infra]
    title = "Security"
    identifier = "chef_infra/setup/nodes/chef_client_security.md Security"
    parent = "chef_infra/setup/nodes"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_client_security.md)

{{% chef_auth %}}

## Authentication

{{% chef_auth_authentication %}}

### chef-validator

{{% security_chef_validator %}}

{{% security_chef_validator_context %}}

### During a Chef Infra Client Run

As part of [every Chef Infra Client
run](/chef_client/#the-chef-client-run), Chef Infra Client
authenticates to the Chef Infra Server using an RSA private key and the
Chef Infra Server API.

### authentication_protocol_version

The `authentication_protocol_version` option in the `client.rb` file is
used to determine the authentication protocol that communicates with
Chef Infra Server. For example, specify protocol version 1.3 to enable
support for SHA-256 algorithms:

``` ruby
knife[:authentication_protocol_version] = '1.3'
```

Note that authentication protocol 1.3 is only supported on Chef Server
versions 12.4.0 and above.

## SSL Certificates

{{< warning >}}

The following information does not apply to hosted Chef Server 12, only
to on-premises Chef Server 12.

{{< /warning >}}

{{% server_security_ssl_cert_client %}}

### `/.chef/trusted_certs`

The `/.chef/trusted_certs` directory stores trusted SSL certificates
used to access the Chef Infra Server:

-   On each workstation, this directory is the location into which SSL
    certificates are placed after they are downloaded from the Chef
    Infra Server using the `knife ssl fetch` subcommand
-   On every node, this directory is the location into which SSL
    certificates are placed when a node has been bootstrapped with Chef
    Infra Client from a workstation

### SSL_CERT_FILE

Use the `SSL_CERT_FILE` environment variable to specify the location for
the SSL certificate authority (CA) bundle that is used by Chef Infra
Client.

A value for `SSL_CERT_FILE` is not set by default. Unless updated, the
locations in which Chef Infra will look for SSL certificates are:

-   Chef Infra Client: `/opt/chef/embedded/ssl/certs/cacert.pem`
-   ChefDK: `/opt/chefdk/embedded/ssl/certs/cacert.pem`
-   Chef Workstation:
    `/opt/chef-workstation/embedded/ssl/certs/cacert.pem`

Keeping the default behavior is recommended. To use a custom CA bundle,
update the environment variable to specify the path to the custom CA
bundle. If (for some reason) SSL certificate verification stops working,
ensure the correct value is specified for `SSL_CERT_FILE`.

### client.rb Settings

Use following client.rb settings to manage SSL certificate preferences:

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
<td><code>local_key_generation</code></td>
<td>Whether the Chef Infra Server or Chef Infra Client generates the private/public key pair. When <code>true</code>, Chef Infra Client generates the key pair, and then sends the public key to the Chef Infra Server. Default value: <code>true</code>.</td>
</tr>
<tr class="even">
<td><code>ssl_ca_file</code></td>
<td>The file in which the OpenSSL key is saved. Chef Infra Client generates this setting automatically and most users do not need to modify it.</td>
</tr>
<tr class="odd">
<td><code>ssl_ca_path</code></td>
<td>The path to where the OpenSSL key is located. Chef Infra Client generates this setting automatically and most users do not need to modify it.</td>
</tr>
<tr class="even">
<td><code>ssl_client_cert</code></td>
<td>The OpenSSL X.509 certificate used for mutual certificate validation. This setting is only necessary when mutual certificate validation is configured on the Chef Infra Server. Default value: <code>nil</code>.</td>
</tr>
<tr class="odd">
<td><code>ssl_client_key</code></td>
<td>The OpenSSL X.509 key used for mutual certificate validation. This setting is only necessary when mutual certificate validation is configured on the Chef Infra Server. Default value: <code>nil</code>.</td>
</tr>
<tr class="even">
<td><p><code>ssl_verify_mode</code></p></td>
<td><p>Set the verify mode for HTTPS requests.</p>
<ul>
<li>Use <code>:verify_none</code> to do no validation of SSL certificates.</li>
<li>Use <code>:verify_peer</code> to do validation of all SSL certificates, including the Chef Infra Server connections, S3 connections, and any HTTPS <strong>remote_file</strong> resource URLs used in a Chef Infra Client run. This is the recommended setting.</li>
</ul>
<p>Depending on how OpenSSL is configured, the <code>ssl_ca_path</code> may need to be specified. Default value: <code>:verify_peer</code>.</p></td>
</tr>
<tr class="odd">
<td><code>verify_api_cert</code></td>
<td>Verify the SSL certificate on the Chef Infra Server. When <code>true</code>, Chef Infra Client always verifies the SSL certificate. When <code>false</code>, Chef Infra Client uses the value of <code>ssl_verify_mode</code> to determine if the SSL certificate requires verification. Default value: <code>false</code>.</td>
</tr>
</tbody>
</table>

### Knife Subcommands

The Chef Infra Client includes two knife commands for managing SSL
certificates:

-   Use [knife ssl check](/workstation/knife_ssl_check/) to troubleshoot SSL
    certificate issues
-   Use [knife ssl fetch](/workstation/knife_ssl_fetch/) to pull down a
    certificate from the Chef Infra Server to the `/.chef/trusted_certs`
    directory on the workstation.

After the workstation has the correct SSL certificate, bootstrap
operations from that workstation will use the certificate in the
`/.chef/trusted_certs` directory during the bootstrap operation.

#### knife ssl check

Run the `knife ssl check` subcommand to verify the state of the SSL
certificate, and then use the response to help troubleshoot issues that
may be present.

**Verified**

{{% knife_ssl_check_verify_server_config %}}

**Unverified**

{{% knife_ssl_check_bad_ssl_certificate %}}

#### knife ssl fetch

Run the `knife ssl fetch` to download the self-signed certificate from
the Chef Infra Server to the `/.chef/trusted_certs` directory on a
workstation.

**Verify Checksums**

{{% knife_ssl_fetch_verify_certificate %}}
