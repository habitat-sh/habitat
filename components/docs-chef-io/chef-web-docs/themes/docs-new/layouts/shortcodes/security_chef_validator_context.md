However, during the first Chef Infra Client run, this private key does
not exist. Instead, Chef Infra Client attempts to use the private key
assigned to the chef-validator, located in `/etc/chef/validation.pem`.
(If, for any reason, the chef-validator is unable to make an
authenticated request to the Chef Infra Server, the initial Chef Infra
Client run will fail.)

During the initial Chef Infra Client run, Chef Infra Client registers
itself with the Chef Infra Server using the private key assigned to the
chef-validator, after which Chef Infra Client will obtain a `client.pem`
private key for all future authentication requests to the Chef Infra
Server.

After the initial Chef Infra Client run has completed successfully, the
chef-validator is no longer required and may be deleted from the node.
Use the `delete_validation` recipe found in the `chef-client` cookbook
(<https://github.com/chef-cookbooks/chef-client>) to remove the
chef-validator.