+++
title = "Custom Certificates"
description = "Handling custom (CA) certificates"

[menu]
  [menu.habitat]
    title = "Custom Certificates"
    identifier = "habitat/reference/certs-custom Custom Certs"
    parent = "habitat/reference"

+++

Many enterprise environments use custom certificates (for example, self-signed). For example, an on-premises Chef Habitat Builder Depot might have a self-signed SSL certificate.

Attempting to perform an operation using the Habitat client to communicate with a service that has a custom certificate can produce an error, such as:

```output
✗✗✗
✗✗✗ the handshake failed: The OpenSSL library reported an error: error:14090086:SSL routines:ssl3_get_server_certificate:certificate verify failed:s3_clnt.c:1269:: unable to get local issuer certificate
✗✗✗
```

One option to remediate this error is to define a `SSL_CERT_FILE` environment variable pointing to the custom certificate path before performing the client operation.

The Habitat 0.85.0 release in September 2019 improved the handling of custom certificates.  Now Habitat knows to look for custom certificates in the `~/.hab/cache/ssl` directory, which is `/hab/cache/ssl` when you are running as root. Copying multiple certificates--for example, a self-signed certificate and a custom certificate authority certificate--to the Chef Habitat cache directory makes them automatically available to the Habitat client.

The `/hab/cache/ssl` directory is also available inside a Habitat Studio. As long as the certificates are inside the cache directory before you enter the Studio, you'll also find them inside the Studio. In addition, if you've set the `SSL_CERT_FILE` environment variable, you'll also find both it and the file that it points to inside the Studio`/hab/cache/ssl` directory.

Note: The `cert.pem` file name is reserved for Habitat. Do not use `cert.pem` as a file name when copying certs into the cache directory.
