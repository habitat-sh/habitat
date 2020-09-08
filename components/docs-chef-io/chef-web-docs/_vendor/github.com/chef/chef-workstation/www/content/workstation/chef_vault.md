+++
title = "`chef-vault`"
draft = false

aliases = ["/chef_vault.html", "/chef_vault/"]

[menu]
  [menu.workstation]
    title = "chef-vault (executable)"
    identifier = "chef_workstation/chef_workstation_tools/chef_vault.md chef-vault (executable)"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/chef_vault.md)

`chef-vault` is a Ruby Gem that is included in Chef Workstation and Chef
Infra Client. `chef-vault` allows the encryption of a data bag item by
using the public keys of a list of nodes, allowing only those nodes to
decrypt the encrypted values. `chef-vault` uses the `knife vault`
subcommand.

{{< note >}}

`chef-vault` does not currently support alternate keying mechanisms like
GPG and Amazon KMS.

{{< /note >}}

-   For more information about using the `chef-vault` cookbook, its
    helper methods and resources, see
    <https://github.com/chef-cookbooks/chef-vault>

The `chef-vault cookbook` is maintained by Chef Software. Use it along
with `chef-vault` itself. This cookbook adds the `chef_vault_item`
helper method to the Recipe DSL and the `chef_vault_secret` resource.
Use them both in recipes to work with data bag secrets.

{{< warning >}}

Chef vault requires the use of Chef Infra Client configured to use
public/private key pairs. Chef vault is incompatible with the practice
of using Chef Infra Client with a private key as `client.pem` and a
certificate set as its public identity in the Chef Infra Server
database. To update existing nodes to use `chef-vault`, first
re-register your Chef Infra Client nodes with the Chef Infra Server,
which will generate public/private key pairs, and then install Chef
vault on each node. Chef vault will generate the following error if used
with a Chef Infra Client with a private key as `client.pem` and a
certificate set as its public identity in the Chef Infra Server
database:

``` none
## OpenSSL::PKey::RSAError
Neither PUB key nor PRIV key:: nested asn1 error
```

{{< /warning >}}

## Installation

The Chef Workstation ships with the latest release of chef_vault.

### Configuring config.rb for `chef_vault`

To set 'client' as the default mode, add the following line to the
config.rb file.

``` shell
knife[:vault_mode] = 'client'
```

To set the default list of admins for creating and updating vaults, add
the following line to the config.rb file.

``` shell
knife[:vault_admins] = [ 'example-alice', 'example-bob', 'example-carol' ]
```

(These values can be overridden on the command line by using `-A`)

### Syntax

``` shell
knife vault SUBCOMMAND VAULT ITEM VALUES
```

where:

-   `vault` names the location for storing the encrypted item.
-   `item` names the item stored in the vault.
-   `values` contains the data that will be encrypted and stored in the
    vault.

### Vault Commands

``` shell
knife vault create VAULT ITEM VALUES (options)
knife vault delete VAULT ITEM (options)
knife vault download VAULT ITEM PATH (options)
knife vault edit VAULT ITEM (options)
knife vault isvault VAULT ITEM (options)
knife vault itemtype VAULT ITEM (options)
knife vault list (options)
knife vault refresh VAULT ITEM
knife vault remove VAULT ITEM VALUES (options)
knife vault rotate all keys
knife vault rotate keys VAULT ITEM (options)
knife vault show VAULT [ITEM] [VALUES] (options)
knife vault update VAULT ITEM VALUES (options)
```

### Vault Common Options

`-A`, `--admins ADMINS`

:   Chef users to be added as admins

`-s`, `--server-url URL`

:   Chef Infra Server URL

`--chef-zero-host HOST`

:   Host to start chef-zero on

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges like
    1000,1010 or 8889-9999 will try all given ports until one works.

`-k`, `--key KEY`

:   API Client Key

`-C`, `--clients CLIENTS`

:   Chef clients to be added as clients

`--[no-]color`

:   Use colored output, defaults to enabled

`-c`, `--config CONFIG`

:   The configuration file to use

`--config-option OPTION=VALUE`

:   Override a single configuration option

`--defaults`

:   Accept default values for all questions

`-d`, --disable-editing

:   Do not open EDITOR, just accept the data as is

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment (except for in searches, where this will be
    flagrantly ignored)

`--file FILE`

:   File to be added to vault item as file-content

`--[no-]fips`

:   Enable or disable fips mode

`-F`, `--format FORMAT`

:   Which format to use for output

`-J`, `--json FILE`

:   File containing JSON data to encrypt

`-K`, `--keys-mode KEYS_MODE`

:   Mode in which to save vault keys

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port

`-z`, `--local-mode`

:   Point knife commands at local repository instead of server

`-u`, `--user USER`

:   API Client Username

`--print-after`

:   Show the data after a destructive operation

`-S`, `--search SEARCH`

:   Chef SOLR search for clients

`-M`, `--mode MODE`

:   Chef mode to run in. Default Value: `solo`

`-V`, `--verbose`

:   More verbose output. Use twice for max verbosity

`-v`, `--version`

:   Show chef version

`-y`, `--yes`

:   Say yes to all prompts for confirmation

`-h`, `--help`

:   Show this message

### Example Commands

### `create`

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for clients
role:webserver, client1 & client2 and admins admin1 & admin2

``` bash
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -C "client1,client2" -A "admin1,admin2"
```

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for clients
role:webserver and admins admin1 & admin2

``` shell
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -A "admin1,admin2"
```

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for clients
role:webserver, client1 & client2

``` shell
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -C "client1,client2"
```

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for clients
role:webserver

``` shell
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver"
```

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for clients client1
& client2

``` shell
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -C "client1,client2"
```

Create a vault called passwords and put an item called root in it with
the given values for username and password encrypted for admins admin1 &
admin2

``` shell
knife vault create passwords root '{"username": "root", "password": "mypassword"}' -A "admin1,admin2"
```

Create a vault called passwords and put an item called root in it
encrypted for admins admin1 & admin2. *Leaving the data off the
command-line will open an editor to fill out the data*

``` shell
knife vault create passwords root -A "admin1,admin2"
```

{{< note >}}

A JSON file can be used in place of specifying the values on the command
line, see global options below for details

{{< /note >}}

### `update`

Update the values in username and password in the vault passwords and
item root. Will overwrite existing values if values already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}'
```

Update the values in username and password in the vault passwords and
item root and add role:webserver, client1 & client2 to the encrypted
clients and admin1 & admin2 to the encrypted admins. Will overwrite
existing values if values already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -C "client1,client2" -A "admin1,admin2"
```

Update the values in username and password in the vault passwords and
item root and add role:webserver to the encrypted clients and admin1 &
admin2 to the encrypted admins. Will overwrite existing values if values
already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -A "admin1,admin2"
```

Update the values in username and password in the vault passwords and
item root and add role:webserver to the encrypted clients. Will
overwrite existing values if values already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver"
```

Update the values in username and password in the vault passwords and
item root and add client1 & client2 to the encrypted clients. Will
overwrite existing values if values already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}' -C "client1,client2"
```

Update the values in username and password in the vault passwords and
item root and add admin1 & admin2 to the encrypted admins. Will
overwrite existing values if values already exist!

``` shell
knife vault update passwords root '{"username": "root", "password": "mypassword"}' -A "admin1,admin2"
```

Add role:webserver to encrypted clients for the vault passwords and item
root.

``` shell
knife vault update passwords root -S "role:webserver"
```

Add client1 & client2 to encrypted clients for the vault passwords and
item root.

``` shell
knife vault update passwords root -C "client1,client2"
```

Add admin1 & admin2 to encrypted admins for the vault passwords and item
root.

``` shell
knife vault update passwords root -A "admin1,admin2"
```

Add admin1 & admin2 to encrypted admins and role:webserver, client1 &
client2 to encrypted clients for the vault passwords and item root.

``` shell
knife vault update passwords root -S "role:webserver" -C "client1,client2" -A "admin1,admin2"
```

Add admin1 & admin2 to encrypted admins and role:webserver to encrypted
clients for the vault passwords and item root.

``` shell
knife vault update passwords root -S "role:webserver" -A "admin1,admin2"
```

Add admin1 & admin2 to encrypted admins and client1 & client2 to
encrypted clients for the vault passwords and item root.

``` shell
knife vault update passwords root -C "client1,client2" -A "admin1,admin2"
```

..Note:: A JSON file can be used in place of specifying the values on
the command line, see global options below for details

### `remove`

Remove the values in username and password from the vault passwords and
item root.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}'
```

Remove the values in username and password from the vault passwords and
item root and remove role:webserver, client1 & client2 from the
encrypted clients and admin1 & admin2 from the encrypted admins.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -C "client1,client2" -A "admin1,admin2"
```

Remove the values in username and password from the vault passwords and
item root and remove role:webserver from the encrypted clients and
admin1 & admin2 from the encrypted admins.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver" -A "admin1,admin2"
```

Remove the values in username and password from the vault passwords and
item root and remove client1 & client2 from the encrypted clients and
admin1 & admin2 from the encrypted admins.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -C "client1,client2" -A "admin1,admin2"
```

Remove the values in username and password from the vault passwords and
item root and remove role:webserver from the encrypted clients.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -S "role:webserver"
```

Remove the values in username and password from the vault passwords and
item root and remove client1 & client2 from the encrypted clients.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -C "client1,client2"
```

Remove the values in username and password from the vault passwords and
item root and remove admin1 & admin2 from the encrypted admins.

``` shell
knife vault remove passwords root '{"username": "root", "password": "mypassword"}' -A "admin1,admin2"
```

Remove admin1 & admin2 from encrypted admins and role:webserver, client1
& client2 from encrypted clients for the vault passwords and item root.

``` shell
knife vault remove passwords root -S "role:webserver" -C "client1,client2" -A "admin1,admin2"
```

Remove admin1 & admin2 from encrypted admins and role:webserver from
encrypted clients for the vault passwords and item root.

``` shell
knife vault remove passwords root -S "role:webserver" -A "admin1,admin2"
```

Remove role:webserver from encrypted clients for the vault passwords and
item root.

``` shell
knife vault remove passwords root -S "role:webserver"
```

Remove client1 & client2 from encrypted clients for the vault passwords
and item root.

``` shell
knife vault remove passwords root -C "client1,client2"
```

Remove admin1 & admin2 from encrypted admins for the vault passwords and
item root.

``` shell
knife vault remove passwords root -A "admin1,admin2"
```

### `delete`

Delete the item root from the vault passwords

``` shell
knife vault delete passwords root
```

### `show`

Show the items in a vault.

``` shell
knife vault show passwords
```

Show the entire root item in the passwords vault and print in JSON
format.

``` shell
knife vault show passwords root -Fjson
```

Show the entire root item in the passwords vault and print in JSON
format, including the search query, clients, and admins.

``` shell
knife vault show passwords root -Fjson -p all
```

Show the username and password for the item root in the vault passwords.

``` shell
knife vault show passwords root "username, password"
```

Show the contents for the item user_pem in the vault certs.

``` shell
knife vault show certs user_pem "contents"
```

### `edit`

Decrypt the entire root item in the passwords vault and open it in json
format in your \$EDITOR. Writing and exiting out the editor will save
and encrypt the vault item.

``` shell
knife vault edit passwords root
```

### `download`

Decrypt and download an encrypted file to the specified path.

``` shell
knife vault download certs user_pem ~/downloaded_user_pem
```

### `rotate keys`

Rotate the shared key for the vault passwords and item root. The shared
key is that which is used for the chef encrypted data bag item.

``` shell
knife vault rotate keys passwords root
```

To remove clients which have been deleted from Chef but not from the
vault, add the `--clean-unknown-clients` switch:

``` shell
knife vault rotate keys passwords root --clean-unknown-clients
```

### `rotate all keys`

Rotate the shared key for all vaults and items. The shared key is that
which is used for the chef encrypted data bag item.

``` shell
knife vault rotate all keys
```

Removes clients which have been deleted from Chef but not from the
vault.

``` shell
knife vault rotate keys passwords root --clean-unknown-clients
```

### `refresh`

This command reads the search_query in the vault item, performs the
search, and reapplies the results.

``` shell
knife vault refresh VAULT ITEM
```

To remove clients which have been deleted from Chef but not from the
vault, add the `--clean-unknown-clients` switch:

``` shell
knife vault refresh passwords root --clean-unknown-clients
```

### `isvault`

This command checks if the given item is a vault or not, and exit with a
status of 0 if it is and 1 if it is not.

``` shell
knife vault isvault VAULT ITEM
```

### `itemtype`

This command outputs the type of the data bag item: normal, encrypted or
vault

``` shell
knife vault itemtype VAULT ITEM
```

### Global Options

| Short Command           | Long Command      | Description                                                                                        | Default | Valid Values                         | Sub-Commands                  |
|-------------------------|-------------------|----------------------------------------------------------------------------------------------------|---------|--------------------------------------|-------------------------------|
| `-M`, `MODE`            | `--mode MODE`     | Chef mode to run in. Can be set in config.rb                                                       | `solo`  | `solo`, `client`                     | all                           |
| `-S` `SEARCH`           | `--search SEARCH` | Chef Infra Server SOLR Search Of Nodes                                                             | none    | none                                 | `create`, `remove` , `update` |
| `-A` `ADMINS`           | `--admins ADMINS` | Chef clients or users to be vault admins, can be comma list                                        | none    | none                                 | `create`, `remove` , `update` |
| `-J` `FILE`             | `--json FILE`     | JSON file to be used for values, will be merged with VALUES if VALUES is passed                    | none    | none                                 | `create`, `update`            |
| `--file` `FILE`         | none              | File that `chef-vault` should encrypt. It adds "file-content" & "file-name" keys to the vault item | none    | none                                 | `create`, `update`            |
| `-p` `DATA`             | `--print DATA`    | Print extra vault data                                                                             | none    | `search`, `clients`, `admins`, `all` | `show`                        |
| `-F` `FORMAT`           | `--format FORMAT` | Format for decrypted output                                                                        | summary | `summary`, `json`, `yaml`, `pp`      | `show`                        |
| --clean-unknown-clients | none              | Remove unknown clients during key rotation                                                         | none    | none                                 | `refresh`, `remove`, `rotate` |

## Options for knife bootstrap

Use the following options with a validatorless bootstrap to specify
items that are stored in `chef-vault`:

`--bootstrap-vault-file VAULT_FILE`

:   The path to a JSON file that contains a list of vaults and items to
    be updated.

`--bootstrap-vault-item VAULT_ITEM`

:   A single vault and item to update as `vault:item`.

`--bootstrap-vault-json VAULT_JSON`

:   A JSON string that contains a list of vaults and items to be
    updated. --bootstrap-vault-json '{ "vault1": \["item1", "item2"\],
    "vault2": "item2" }'

### Using `chef-vault` in recipes

To use this gem in a recipe to decrypt data you must first install the
gem via a chef_gem resource. Once the gem is installed require the gem
and then you can create a new instance of ChefVault.

`chef-vault` 1.0 style decryption is supported, however it has been
deprecated and `chef-vault` 2.0 decryption should be used instead

### Example Code

``` ruby
chef_gem 'chef-vault' do
  compile_time true if respond_to?(:compile_time)
end
#
require 'chef-vault'
#
item = ChefVault::Item.load("passwords", "root")
item["password"]
```

Note that in this case, the gem needs to be installed at compile time
because the require statement is at the top-level of the recipe. If you
move the require of `chef-vault` and the call to `::load` to library or
provider code, you can install the gem in the converge phase instead.

### Specifying an alternate node name or client key path

Normally, the value of `Chef::Config[:node_name]` is used to find the
per-node encrypted secret in the keys data bag item, and the value of
<span class="title-ref">Chef::Config\[:client_key\]</span> is used to
locate the private key to decrypt this secret.

These can be overridden by passing a hash with the keys `:node_name` or
`:client_key_path` to `ChefVault::Item.load`:

``` ruby
item = ChefVault::Item.load(
  'passwords', 'root',
  node_name: 'service_foo',
  client_key_path: '/secure/place/service_foo.pem'
)
item['password']
```

The above example assumes that you have transferred
`/secure/place/service_foo.pem` to your system via a secure channel.

This usage allows you to decrypt a vault using a key shared among
several nodes, which can be helpful when working in cloud environments
or other configurations where nodes are created dynamically.

### chef_vault_item helper

The [chef-vault
cookbook](https://supermarket.chef.io/cookbooks/chef-vault/) contains a
recipe to install the `chef-vault` gem and a helper method
`chef_vault_item` which makes it easier to test cookbooks that use
chef-vault using Test Kitchen.

### Determining if Item is a Vault

ChefVault provides a helper method to determine if a data bag item is a
vault, which can be helpful if you produce a recipe for community
consumption and want to support both normal data bags and vaults:

``` ruby
if ChefVault::Item.vault?('passwords', 'root')
  item = ChefVault::Item.load('passwords', 'root')
else
  item = Chef::DataBagItem.load('passwords', 'root')
end
```

This functionality is also available from the command line as
`knife vault isvault VAULT ITEM`.

### Determining Data Bag Item Type

ChefVault provides a helper method to determine the type of a data bag
item. It returns one of the symbols :normal, :encrypted or :vault

``` ruby
case ChefVault::Item.data_bag_item_type('passwords', 'root')
when :normal
  ...
when :encrypted
  ...
when :vault
end
```

This functionality is also available from the command line as
`knife vault itemtype VAULT ITEM`.

### Stand Alone Usage

`chef-vault` can be used as a stand alone binary to decrypt values
stored in Chef. It requires that Chef is installed on the system and
that you have a valid config.rb. This is useful if you want to mix
`chef-vault` into non-Chef recipe code, for example some other script
where you want to protect a password.

It does still require that the data bag has been encrypted for the
user's or client's pem and pushed to the Chef Infra Server. It mixes
Chef into the gem and uses it to go grab the data bag.

Use `chef-vault --help` to see all all available options

### Example usage (password)

``` none
chef-vault -v passwords -i root -a password -k /etc/chef/config.rb
```

### Testing

To stub vault items in ChefSpec, use the
[chef-vault-testfixture](https://rubygems.org/gems/chef-vault-testfixtures)
gem.

To fall back to unencrypted JSON files in Test Kitchen, use the
`chef_vault_item` helper in the aforementioned `chef-vault` cookbook.

## For more information ...

For more information about `chef-vault`:

-   [Nell Shamrell-Harringon's blog
    post](https://blog.chef.io/2016/01/21/chef-vault-what-is-it-and-what-can-it-do-for-you/)
-   [Joshua Timberman's blog
    post](https://www.chef.io/blog/2013/09/19/managing-secrets-with-chef-vault/)
