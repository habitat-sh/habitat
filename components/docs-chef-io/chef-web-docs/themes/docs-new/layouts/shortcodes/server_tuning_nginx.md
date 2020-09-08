The following settings are often modified from the default as part of
the tuning effort for the **nginx** service and to configure the Chef
Infra Server to use SSL certificates:

`nginx['ssl_certificate']`

:   The SSL certificate used to verify communication over HTTPS. Default
    value: `nil`.

`nginx['ssl_certificate_key']`

:   The certificate key used for SSL communication. Default value:
    `nil`.

`nginx['ssl_ciphers']`

:   The list of supported cipher suites that are used to establish a
    secure connection. To favor AES256 with ECDHE forward security, drop
    the `RC4-SHA:RC4-MD5:RC4:RSA` prefix. For example:

    ``` ruby
    nginx['ssl_ciphers'] =  "HIGH:MEDIUM:!LOW:!kEDH: \
                             !aNULL:!ADH:!eNULL:!EXP: \
                             !SSLv2:!SEED:!CAMELLIA: \
                             !PSK"
    ```

`nginx['ssl_protocols']`

:   The SSL protocol versions that are enabled. SSL 3.0 is supported by
    the Chef Infra Server; however, SSL 3.0 is an obsolete and insecure
    protocol. Transport Layer Security (TLS)---TLS 1.0, TLS 1.1, and TLS
    1.2---has effectively replaced SSL 3.0, which provides for
    authenticated version negotiation between Chef Infra Client and Chef
    Infra Server, which ensures the latest version of the TLS protocol
    is used. For the highest possible security, it is recommended to
    disable SSL 3.0 and allow all versions of the TLS protocol. For
    example:

    ``` ruby
    nginx['ssl_protocols'] = "TLSv1 TLSv1.1 TLSv1.2"
    ```

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

See <https://www.openssl.org/docs/man1.0.2/man1/ciphers.html> for more
information about the values used with the `nginx['ssl_ciphers']` and
`nginx['ssl_protocols']` settings.



</div>

</div>

For example, after copying the SSL certificate files to the Chef Infra
Server, update the `nginx['ssl_certificate']` and
`nginx['ssl_certificate_key']` settings to specify the paths to those
files, and then (optionally) update the `nginx['ssl_ciphers']` and
`nginx['ssl_protocols']` settings to reflect the desired level of
hardness for the Chef Infra Server:

``` ruby
nginx['ssl_certificate'] = "/etc/pki/tls/private/name.of.pem"
nginx['ssl_certificate_key'] = "/etc/pki/tls/private/name.of.key"
nginx['ssl_ciphers'] = "HIGH:MEDIUM:!LOW:!kEDH:!aNULL:!ADH:!eNULL:!EXP:!SSLv2:!SEED:!CAMELLIA:!PSK"
nginx['ssl_protocols'] = "TLSv1 TLSv1.1 TLSv1.2"
```
