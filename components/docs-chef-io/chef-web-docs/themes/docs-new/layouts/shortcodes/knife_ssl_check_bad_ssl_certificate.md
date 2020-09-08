If the SSL certificate cannot be verified, the response to

``` bash
knife ssl check
```

is similar to:

``` bash
Connecting to host chef-server.example.com:443
ERROR: The SSL certificate of chef-server.example.com could not be verified
Certificate issuer data:
  /C=US/ST=WA/L=S/O=Corp/OU=Ops/CN=chef-server.example.com/emailAddress=you@example.com

Configuration Info:

OpenSSL Configuration:
* Version: OpenSSL 1.0.2u  20 Dec 2019
* Certificate file: /opt/chef-workstation/embedded/ssl/cert.pem
* Certificate directory: /opt/chef-workstation/embedded/ssl/certs
Chef SSL Configuration:
* ssl_ca_path: nil
* ssl_ca_file: nil
* trusted_certs_dir: "/Users/grantmc/Downloads/chef-repo/.chef/trusted_certs"

TO FIX THIS ERROR:

If the server you are connecting to uses a self-signed certificate,
you must configure chef to trust that certificate.

By default, the certificate is stored in the following location on the
host where your Chef Infra Server runs:

  /var/opt/opscode/nginx/ca/SERVER_HOSTNAME.crt

Copy that file to your trusted_certs_dir (currently:

  /Users/grantmc/Downloads/chef-repo/.chef/trusted_certs)

using SSH/SCP or some other secure method, then re-run this command to
confirm that the certificate is now trusted.
```