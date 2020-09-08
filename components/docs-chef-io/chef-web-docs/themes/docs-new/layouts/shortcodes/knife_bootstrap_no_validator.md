The ORGANIZATION-validator.pem is typically added to the .chef directory
on the workstation. When a node is bootstrapped from that workstation,
the ORGANIZATION-validator.pem is used to authenticate the newly-created
node to the Chef Infra Server during the initial Chef Infra Client run.
It is possible to bootstrap a node using the USER.pem file instead of
the ORGANIZATION-validator.pem file. This is known as a "validatorless
bootstrap".

To create a node via the USER.pem file, simply delete the
ORGANIZATION-validator.pem file on the workstation. For example:

``` bash
rm -f /home/lamont/.chef/myorg-validator.pem
```

and then make the following changes in the config.rb file:

-   Remove the `validation_client_name` setting
-   Edit the `validation_key` setting to be something that isn't a path
    to an existent ORGANIZATION-validator.pem file. For example:
    `/nonexist`.

As long as a USER.pem is also present on the workstation from which the
validatorless bootstrap operation will be initiated, the bootstrap
operation will run and will use the USER.pem file instead of the
ORGANIZATION-validator.pem file.

When running a validatorless `knife bootstrap` operation, the output is
similar to:

``` bash
desktop% knife bootstrap 10.1.1.1 -N foo01.acme.org \
  -E dev -r 'role[base]' -j '{ "foo": "bar" }' \
  --ssh-user vagrant --sudo
Node foo01.acme.org exists, overwrite it? (Y/N)
Client foo01.acme.org exists, overwrite it? (Y/N)
Creating new client for foo01.acme.org
Creating new node for foo01.acme.org
Connecting to 10.1.1.1
10.1.1.1 Starting first Chef Client run...
[....etc...]
```