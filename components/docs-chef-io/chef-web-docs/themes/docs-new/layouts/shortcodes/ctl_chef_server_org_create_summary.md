Run the following command to create an organization:

``` bash
sudo chef-server-ctl org-create short_name 'full_organization_name' --association_user user_name --filename ORGANIZATION-validator.pem
```

For example:

``` bash
sudo chef-server-ctl org-create 4thcoffee 'Fourth Coffee, Inc.' --association_user janedoe --filename /path/to/4thcoffee-validator.pem
```

The name must begin with a lower-case letter or digit, may only contain
lower-case letters, digits, hyphens, and underscores, and must be
between 1 and 255 characters. For example: `4thcoffee`.

The full name must begin with a non-white space character and must be
between 1 and 1023 characters. For example: `'Fourth Coffee, Inc.'`.

The `--association_user` option will associate the `user_name` with the
`admins` security group on the Chef Infra Server.

An RSA private key is generated automatically. This is the
chef-validator key and should be saved to a safe location. The
`--filename` option will save the RSA private key to the specified
absolute path.