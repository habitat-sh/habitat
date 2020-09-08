Run the following command to create an administrator:

``` bash
sudo chef-server-ctl user-create USER_NAME FIRST_NAME LAST_NAME EMAIL 'PASSWORD' --filename FILE_NAME
```

An RSA private key is generated automatically. This is the user's
private key and should be saved to a safe location. The `--filename`
option will save the RSA private key to the specified absolute path.

For example:

``` bash
sudo chef-server-ctl user-create janedoe Jane Doe janed@example.com 'abc123' --filename /path/to/janedoe.pem
```