The SSL certificate that is downloaded to the `/.chef/trusted_certs`
directory should be verified to ensure that it is, in fact, the same
certificate as the one located on the Chef Infra Server. This can be
done by comparing the SHA-256 checksums.

1.  View the checksum on the Chef Infra Server:

    ``` bash
    ssh ubuntu@chef-server.example.com sudo sha256sum /var/opt/opscode/nginx/ca/chef-server.example.com.crt
    ```

    The response is similar to:

    ``` bash
    <ABC123checksum>  /var/opt/opscode/nginx/ca/chef-server.example.com.crt
    ```

2.  View the checksum on the workstation:

    ``` bash
    gsha256sum .chef/trusted_certs/chef-server.example.com.crt
    ```

    The response is similar to:

    ``` bash
    <ABC123checksum>  .chef/trusted_certs/chef-server.example.com.crt
    ```

3.  Verify that the checksum values are identical.