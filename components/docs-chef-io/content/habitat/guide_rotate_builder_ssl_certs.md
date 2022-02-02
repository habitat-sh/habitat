+++
title = "Rotate Builder SSL Certs"
description = "How to rotate SSL Certs On The On-Prem Builder"
draft = false
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Rotate Builder SSL Certs"
    identifier = "habitat/guides/rotate-builder-certs"
    parent = "habitat/guides"
    weight = 50
+++

A short guide to updating the SSL certs used by Builder's web interface.

## Understanding where the Builder certs live

Chef Habitat Builder's web front-end is hosted via NGINX running via the `habitat/builder-api-proxy` service.  The NGINX config file for the api-proxy service tells NGINX to load SSL certificate and key from files located at `/hab/svc/builder-api-proxy/files`.  The certificate and key names **_need_** to be named `ssl-certificate.crt` and `ssl-certificate.key`.  The `files` directory is managed via the `hab file upload` functionality.  So in order to change these certificates permanently, you need to upload the files through hab, and then restart the proxy.

## Rotating the SSL certificate and key

There's really a few simple commands to run in order to rotate your key.

First, rename your cert-chain and key file to the names required by the builder-api-proxy service.

```shell
cp <CERTIFICATE_CHAIN_FILENAME> ssl-certificate.crt
cp <CERTIFICATE_KEY_FILENAME> ssl-certificate.key
```

Then upload the certificate and key files to the builder service.

```shell
hab file upload "builder-api-proxy.default" "$(date +%s)" ./ssl-certificate.crt
hab file upload "builder-api-proxy.default" "$(date +%s)" ./ssl-certificate.key
```

Finally, restarting the builder-api-proxy service will put the updated files into the appropriate path and restart NGINX so that it's using your new certificate and key.

```shell
hab svc stop habitat/builder-api-proxy && hab svc start habitat/builder-api-proxy
```

You should now be able to verify through your browser or via an `openssl s_client -connect` command that your builder server has an updated certificate.
