+++
title = "Chef Habitat Builder on-prem + Chef Automate"
description = "Using Chef Automate for Chef Habitat Builder on-prem authentication "

[menu]
  [menu.habitat]
    title = "Builder on-prem + Automate"
    identifier = "habitat/builder-on-prem/builder-automate"
    parent = "habitat"

+++

+++
{{< note >}}

Chef Automate has the `./chef-automate deploy --product builder --product automate` command, which offers the fastest and easiest Chef Habitat Builder on-prem installation and authentication path. See the [Install Chef Habitat on-prem with Chef Automate Guide](https://docs.chef.io/automate/on_prem_builder/) for more information.

{{< /note >}

Chef Habitat Builder on-prem has two alternative authentication paths:

* Authenticate with Cached Custom Certificates


## Authenticate with Cached Custom Certificates

The Chef Habitat 0.85.0 release simplifies custom certificate management, making authentication with Chef Automate easier. Install Builder on-prem and authenticate with Automate in five steps. This is possible because Habitat now looks for custom certificates in its `~/.hab/cache/ssl` directory (or `/hab/cache/ssl` when running as root). Copying self-signed and custom certificates to the cache directory automatically makes them available to the Habitat client.

Configuring Chef Habitat on-prem to use Chef Automate's Authentication takes five steps:

1. Patch the Chef Automate `automate-credentials.toml` to recognize Chef Habitat
1. Set up the Chef Habitat Builder on-prem `bldr.env` to use Chef Automate's authentication
1. Copy the any custom certificate `.crt` and `.key` files to the same location as the `./install.sh` script.
1. Install Chef Habitat Builder on-prem
1. Copy Automate's certificate to the `/hab/cache/ssl` directory

For existing Chef Automate installations, copy the TLS certificate from the Chef Automate `config.toml` under the `load_balancer.v1.sys.frontend_tls` entry and save it as `automate-cert.pem` in the `/hab/cache/ssl` directory on Chef Habitat Builder on-prem.

### Step One: Patch Chef Automate's Configuration

To authenticate with Chef Automate, create a patch with the Chef Automate command line:

1. From the command line, access Chef Automate, for example:

    ```bash
    ssh <automate hostname>
    #or
    ssh <ipaddress>
    ```

1. Create the file `patch-automate.toml`:

    ```bash
    touch patch-automate.toml
    ```

1. Edit the `patch-automate.toml`:

    ```toml
    [session.v1.sys.service]
    bldr_signin_url = "https://chef-builder.test/"
    # OAUTH_CLIENT_ID
    bldr_client_id = "0123456789abcdef0123"
    # OAUTH_CLIENT_SECRET
    bldr_client_secret = "0123456789abcdef0123456789abcdef01234567"
    ```

1. Apply the `patch-automate.toml` to the Chef Automate configuration from the command line:

    ```bash
    sudo chef-automate config patch patch-automate.toml
    ```

    A successful patch displays the output:

    ```output
    Updating deployment configuration
    Applying deployment configuration
      Started session-service
    Success: Configuration patched
    ```

1. For existing Chef Automate installations, copy the TLS certificate from the Chef Automate `config.toml` under the `load_balancer.v1.sys.frontend_tls` entry and save it to your workstation as `automate-cert.pem`

1. Exit Chef Automate

### Step Two: Set up `bldr.env`

1. From the command line, access the location where you will install Chef Habitat Builder on-prem:

    ```bash
    ssh <builder hostname>
    #or
    ssh <ipaddress>
    ```

1. From Builder host command line, install Chef Habitat Builder on-prem package:

    ```bash
    git clone https://github.com/habitat-sh/on-prem-builder.git
    ```

1. Change to the `on-prem-builder` directory:

    ```bash
    cd on-prem-builder
    ```

1. Create a `bldr.env` file:

    ```bash
    touch bldr.env
    ```

    Or, if you need more explanations about the contents of the `bldr.env` file, copy the existing sample file:

    ```bash
    cp bldr.env.sample bldr.env
    ```

1. Edit `bldr.env`:
      * Match the `SSL` with the `URL`. The `APP_SSL_ENABLED` configuration coordinates  with the type of hypertext transfer protocol named in `APP_URL`.
        * To disable SSL, use `APP_SSL_ENABLED=false` and a `APP_URL` beginning with `http`.
        * To enable SSL, use `APP_SSL_ENABLED=true` and a `APP_URL` beginning with `https`.
      * Always be closing. Close the Builder addresses provided in `APP_URL` and `OAUTH_REDIRECT_URL` with a forward slash, `/`.
        * `https://chef-builder.test` will NOT work.
        * `https://chef-builder.test/` will work.
  This `bldr.env` example shows an on-prem SSL-enabled Habitat Builder authenticating using Chef Automate's OAuth.
  `APP_SSL_ENABLED=true` and the `APP_URL` starts with `https`.

### Step Three: Put the Certs with the Install Script

If necessary, rename the custom certificates cert file as `ssl-certificate.crt` and the key file as `ssl-certificate.key`. Habitat recognizes only these names and will not recognize any other names. Copy the `ssl-certificate.crt` and `ssl-certificate.key` files to the same directory as the `./install.sh` script.

1. Locate the SSL certificate and key pair.
1. Copy the key pair to the same directory as the install script, which is `/on-prem-builder`, if the repository was not renamed.
1. Make the keys accessible to Habitat during the installation.
1. If you're testing this workflow, make your own key pair and copy them to `/on-prem-builder`.

  ```bash
  sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/ssl-certificate.key -out /etc/ssl/certs/ssl-certificate.crt %>
  sudo cp /etc/ssl/private/ssl-certificate.key .
  sudo cp /etc/ssl/certs/ssl-certificate.crt .
  sudo chown vagrant:vagrant ssl-certificate.*
  ```

1. You can confirm that the keys were copied:

    ```bash
    cat ./ssl-certificate.key
    cat ./ssl-certificate.crt
    ```

1. For existing Chef Automate installations, move the `cert.pem` that you saved /hab/cache/ssl

### Step Four: Install Builder

1. Run the install script. This installs both Chef Habitat Builder on-prem and the Chef Habitat datastore:

    ```bash
    bash ./install.sh
    ```

1. Accept both licenses.
1. All services should report back as `up`. It make take a few minutes to come up.

    ```bash
    sudo hab svc status
    ```

    Should return something similar to:

    ```output
    package                                        type        desired  state  elapsed (s)  pid    group
    habitat/builder-api/8473/20190830141422        standalone  up       up     595          28302  builder-api.default
    habitat/builder-api-proxy/8467/20190829194024  standalone  up       up     597          28233  builder-api-proxy.default
    habitat/builder-memcached/7728/20180929144821  standalone  up       up     597          28244  builder-memcached.default
    habitat/builder-datastore/7809/20181019215440  standalone  up       up     597          28262  builder-datastore.default
    habitat/builder-minio/7764/20181006010221      standalone  up       up     597          28277  builder-minio.default
    ```

### Step Five: Copy Automate's Certificate to Builder

1. View and copy the Chef Automate certificate. Change the server name to your Chef Automate installation FQDN:

    ```bash
    openssl s_client -showcerts -servername chef-automate.test -connect chef-automate.test:443 < /dev/null | openssl x509
    ```

    Copy the output to an accessible file.

    ```output
    # Copy the contents including the begin and end certificate
    # -----BEGIN CERTIFICATE-----
    # Certificate content here
    #-----END CERTIFICATE-----
    ```

1. Make a file for you cert at `/hab/cache/ssl/`, such as `automate-cert.crt`. For a `.pem` file, `automate-cert.pem`. Overwriting `cert.pem` will cause your Builder installation to fail.
1. Paste the Chef Automate certificate into your file, `/hab/cache/ssl/automate-cert.crt`
1. For existing Chef Automate installations, copy the `automate-cert.pem` file that you saved on your workstation in step one to the `/hab/cache/ssl/` directory.
1. Restart builder

    ```bash
    sudo systemctl restart hab-sup
    ```

### You're Done

1. Login at

    ```bash
    https://chef-builder.test
    ```

## Configuring an OAuth Provider for Chef Habitat Builder on-prem

{{< warning >}}
These instruction are under construction.
{{< /warning >}}

To configure Chef Automate as an OAuth Provider for Habitat Builder, create a TOML file with the partial configuration below.

Run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

`bldr_client_id` and `bldr_client_secret` must match the vales configured in Chef Habitat Builder (see below). However, we strongly recommend those values follow
[best practices](https://www.oauth.com/oauth2-servers/client-registration/client-id-secret/)
for `client_id` and `client_secret` in the Oauth2 standard.

```toml
[session.v1.sys.service]
bldr_signin_url = "<your Builder fqdn>" # for example, "http://builder.test/"
# This needs to match what you configured OAUTH_CLIENT_ID as when you configured Habitat Builder.
bldr_client_id = "<your Habitat Builder Oauth2 Client ID>"
# This needs to match what you configured OAUTH_CLIENT_SECRET as when you configured Habitat Builder.
bldr_client_secret = "<your Habitat Builder Oauth2 Client Secret>"
```

In addition, add Automate's TLS certificate to Builder's list of accepted certificates.
Locate Automate's default self-signed certificate by running `cat /hab/svc/automate-load-balancer/data/{{< example_fqdn "automate" >}}.cert`, copy this default certificate, and then add it to your Builder instance's list of accepted certificates.

```text
-----BEGIN CERTIFICATE-----
MIIDfDCCAmSgAcaSldKaf...
-----END CERTIFICATE-----
```

If you are using a certificate signed by a trusted certificate authority instead of the default certificate, you can provide Builder with the root certificate authority for the signed certificate.

## Related Resources

* [Chef Habitat Builder on-prem in Chef Automate](https://docs.chef.io/automate/on_prem_builder/)

## Next Steps

[Bootstrap Core Origin](./bootstrap-core.md)
