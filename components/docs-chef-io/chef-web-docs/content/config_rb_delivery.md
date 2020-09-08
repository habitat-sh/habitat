+++
title = "delivery.rb Settings"
draft = false
robots = "noindex"


aliases = ["/config_rb_delivery.html", "/release/automate/config_rb_delivery.html"]

[menu]
  [menu.legacy]
    title = "delivery.rb"
    identifier = "legacy/workflow/reference/config_rb_delivery.md delivery.rb"
    parent = "legacy/workflow/reference"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_delivery.md)



{{% chef_automate_mark %}}

{{% EOL_a1 %}}

The `delivery.rb` file, located at `/etc/delivery/delivery.rb`, contains
all of the non-default configuration settings used by the Chef Automate.
(The default settings are built-in to the Chef Automate configuration
and should only be added to the `delivery.rb` file to apply non-default
values.) These configuration settings are processed when the
`delivery-server-ctl reconfigure` command is run, such as immediately
after setting up Chef Automate or after making a change to the
underlying configuration settings after the server has been deployed.
The `delivery.rb` file is a Ruby file, which means that conditional
statements can be used in the configuration file.

## Recommended Settings

{{% delivery_server_tuning_general %}}

### SSL Protocols

The following settings are often modified from the default as part of
the tuning effort for the **nginx** service and to configure the Chef
Automate server to use SSL certificates:

`delivery['ssl_certificates']`

:   A hash of SSL certificate files to use for FQDNs. Uses `remote_file`
    to download the key and certificate specified to the standard
    location of `/var/opt/delivery/nginx/ca` and replaces the contents
    of any file found there using the name `delivery.example.com.crt` or
    `delivery.example.com.key`. If the content in the new custom
    certificate and key files and the original files match, these files
    are not reconfigured. To use a pre-generated SSL certificate for the
    main FQDN (`delivery_fqdn`), follow this example:

    ``` ruby
    delivery['ssl_certificates'] = {
      'delivery.example.com' => {
        'key' => 'file:///etc/ssl_certificates/delivery.example.com.key',
        'crt' => 'file:///etc/ssl_certificates/delivery.example.com.crt'
      }
    }
    ```

`nginx['ssl_ciphers']`

:   The list of supported cipher suites that are used to establish a
    secure connection. To favor AES256 with ECDHE forward security, drop
    the `RC4-SHA:RC4-MD5:RC4:RSA` prefix. See [this
    link](https://www.openssl.org/docs/man1.0.2/man1/ciphers.html) for more
    information. Default value:

    ``` ruby
    "RC4-SHA:RC4-MD5:RC4:RSA:HIGH:MEDIUM:!LOW:!kEDH:!aNULL:!ADH:!eNULL:!EXP:!SSLv2:!SEED:!CAMELLIA:!PSK"
    ```

`nginx['ssl_protocols']`

:   The SSL protocol versions that are enabled. For the highest possible
    security, disable SSL 3.0 and allow only TLS:

    ``` ruby
    nginx['ssl_protocols'] = 'TLSv1 TLSv1.1 TLSv1.2'
    ```

    Default value: Default value: `"SSLv3 TLSv1"`.

{{< note >}}

See <https://www.openssl.org/docs/man1.0.2/man1/ciphers.html> for more
information about the values used with the `nginx['ssl_ciphers']` and
`nginx['ssl_protocols']` settings.

{{< /note >}}

For example, after copying the SSL certificate files to the Chef
Automate server, update the `delivery['ssl_certificates']` hash settings
to specify the paths to those files, and then (optionally) update the
`nginx['ssl_ciphers']` and `nginx['ssl_protocols']` settings to reflect
the desired level of hardness for the Chef Automate server:

``` ruby
delivery['ssl_certificates'] = {
   'delivery.example.com' => {
      'key' => 'file:///etc/ssl_certificates/delivery.example.com.key',
      'crt' => 'file:///etc/ssl_certificates/delivery.example.com.crt'
   }
}
nginx['ssl_ciphers'] = "HIGH:MEDIUM:!LOW:!kEDH:!aNULL:!ADH:!eNULL:!EXP:!SSLv2:!SEED:!CAMELLIA:!PSK"
nginx['ssl_protocols'] = "TLSv1 TLSv1.1 TLSv1.2"
```

## Proxy Settings

If you wish to operate your Chef Automate server from behind a proxy,
you may specify you proxy host name and configuration using these
options.

`delivery['proxy']['host']`

:   The hostname to your proxy server such as `foo.bar.com` or
    `192.0.2.00`.

`delivery['proxy']['port']`

:   The port to connect on. This will be used for all connections (http
    and https).

`delivery['proxy']['user']`

:   Optional authentication user name when contacting the proxy server.

`delivery['proxy']['password']`

:   Optional authentication password when contacting the proxy server.

`delivery['proxy']['no_proxy']`

:   A list of hostnames that are blacklisted from using the proxy. Chef
    Automate will attempt to connect directly to these hosts. By
    default, this is set to `["localhost", "127.0.0.1"]`.

## Optional Settings

Additional settings are available for performance tuning of the Chef
Automate server.

{{< note >}}

When changes are made to the `delivery.rb` file the Chef Automate server
must be reconfigured by running the following command:

``` bash
delivery-server-ctl reconfigure
```

{{< /note >}}

{{< note >}}

Review the full list of [optional
settings](/config_rb_delivery_optional_settings/) that can be added
to the `delivery.rb` file. Many of these optional settings should not be
added without first consulting with Chef support.

{{< /note >}}
