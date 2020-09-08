+++
title = "Bootstrap a Node"
draft = false

aliases = ["/install_bootstrap.html"]

[menu]
  [menu.infra]
    title = "Install via Bootstrap"
    identifier = "chef_infra/setup/nodes/install_bootstrap.md Install via Bootstrap"
    parent = "chef_infra/setup/nodes"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_bootstrap.md)

{{% chef_client_bootstrap_node %}}

{{% chef_client_bootstrap_stages %}}

## knife bootstrap

{{% install_chef_client %}}

**Run the bootstrap command**

The `knife bootstrap` subcommand is used to run a bootstrap operation
that installs Chef Infra Client on the target node. The following steps
describe how to bootstrap a node using knife.

1.  Identify the FQDN or IP address of the target node. The
    `knife bootstrap` command requires the FQDN or the IP address for
    the node in order to complete the bootstrap operation.

2.  Once the workstation machine is configured, it can be used to
    install Chef Infra Client on one (or more) nodes across the
    organization using a knife bootstrap operation. The
    `knife bootstrap` command is used to SSH into the target machine,
    and then do what is needed to allow Chef Infra Client to run on the
    node. It will install the Chef Infra Client executable (if
    necessary), generate keys, and register the node with the Chef Infra
    Server. The bootstrap operation requires the IP address or FQDN of
    the target system, the SSH credentials (username, password or
    identity file) for an account that has root access to the node, and
    (if the operating system is not Ubuntu, which is the default
    distribution used by `knife bootstrap`) the operating system running
    on the target system.

    In a command window, enter the following:

    ``` bash
    knife bootstrap 172.16.1.233 -U USERNAME --sudo
    ```

    where `172.16.1.233` is the IP address or the FQDN for the node,
    and `USERNAME` is the username you want to use to connect, and
    `--sudo` specifies to elevate privileges using the sudo command
    on UNIX-based systems.

    Then while the bootstrap operation is running, the command
    window will show something similar to the following:

    ``` bash
    Enter password for ubuntu@172.16.1.233:

    Connecting to 172.16.1.233
    Performing legacy client registration with the validation key at /Users/USERNAME/.chef/validator.pem...
    Delete your validation key in order to use your user credentials for client registration instead.
    Bootstrapping 172.16.1.233
    [172.16.1.233] -----> Installing Chef Omnibus (stable/16)
    downloading https://omnitruck.chef.io/chef/install.sh
    [172.16.1.233]   to file /tmp/install.sh.1624/install.sh
    [172.16.1.233] trying wget...
    [172.16.1.233] ubuntu 20.04 aarch64
    [172.16.1.233] Getting information for chef stable 16 for ubuntu...
    [172.16.1.233] downloading https://omnitruck.chef.io/stable/chef/metadata?v=16&p=ubuntu&pv=20.04&m=aarch64
      to file /tmp/install.sh.1628/metadata.txt
    [172.16.1.233] trying wget...
    [172.16.1.233] sha1  8d89f8ac2e7f52d170be8ec1c2a028a6449d7e3a
    sha256  85cc73bed06e8d6699fc5c0b26c20d2837bf03831873444febccfc8bfa561f00
    url  https://packages.chef.io/files/stable/chef/16.1.16/ubuntu/20.04/chef_16.1.16-1_arm64.deb
    version  16.1.16
    [172.16.1.233]
    [172.16.1.233] downloaded metadata file looks valid...
    [172.16.1.233] downloading https://packages.chef.io/files/stable/chef/16.1.16/ubuntu/20.04/chef_16.1.16-1_arm64.deb
      to file /tmp/install.sh.1628/chef_16.1.16-1_arm64.deb
    [172.16.1.233] trying wget...
    [172.16.1.233] Comparing checksum with sha256sum...
    [172.16.1.233] Installing chef 16
    installing with dpkg...
    [172.16.1.233] Selecting previously unselected package chef.
    [172.16.1.233] (Reading database ... 99114 files and directories currently installed.)
    [172.16.1.233] Preparing to unpack .../chef_16.1.16-1_arm64.deb ...
    [172.16.1.233] Unpacking chef (16.1.16-1) ...
    [172.16.1.233] Setting up chef (16.1.16-1) ...
    [172.16.1.233] Thank you for installing Chef Infra Client! For help getting started visit https://learn.chef.io
    [172.16.1.233] Starting the first Chef Infra Client Client run...
    [172.16.1.233] +---------------------------------------------+
    ✓ 2 product licenses accepted.
    +---------------------------------------------+
    [172.16.1.233] Starting Chef Infra Client, version 16.1.16
    [172.16.1.233] [2020-06-08T23:49:10+00:00] ERROR: shard_seed: Failed to get dmi property serial_number: is dmidecode installed?
    [172.16.1.233] Creating a new client identity for name_of_node using the validator key.
    [172.16.1.233] resolving cookbooks for run list: []
    [172.16.1.233] Synchronizing Cookbooks:
    [172.16.1.233] Installing Cookbook Gems:
    [172.16.1.233] Compiling Cookbooks...
    [172.16.1.233] [2020-06-08T23:49:17+00:00] WARN: Node name_of_node has an empty run list.
    [172.16.1.233] Converging 0 resources
    [172.16.1.233]
    [172.16.1.233] Running handlers:
    [172.16.1.233] Running handlers complete
    [172.16.1.233] Chef Infra Client finished, 0/0 resources updated in 11 seconds
    ```

3.  After the bootstrap operation has finished, verify that the node is
    recognized by the Chef Infra Server. To show only the node that was
    just bootstrapped, run the following command:

    ``` bash
    knife client show NAME_OF_NODE
    ```

    where `NODE_NAME` is the name of the node that was just
    bootstrapped. The Chef Infra Server will return something similar
    to:

    ``` bash
    admin:     false
    chef_type: client
    name:      NODE_NAME
    validator: false
    ```

    and to show the full list of nodes (and workstations) that are
    registered with the Chef Infra Server, run the following command:

    ``` bash
    knife client list
    ```

    The Chef Infra Server will return something similar to:

    ``` bash
    workstation1
    workstation2
    ...
    client1
    client2
    ```

## Validatorless and Legacy Validator Bootstraps

We recommended using "validatorless bootstrapping" to authenticate new nodes with the Chef Infra Server.

The legacy Chef Infra validator-based node bootstrapping process depended on using a shared "validatory" key throughout an organization for authenticating new nodes with the Chef Infra Server.

Shortcomings of the legacy validator process are:

* All users share the same key for bootstrapping new systems
* Key sharing makes key rotation difficult, if it is compromised or if an employee leaves the organization.

The "validatorless bootstrap" generates a key for each node, which is then transferred to the new node and used to authenticate with the Chef Infra Server instead of relying on a shared "validator" key.

The Chef Infra bootstrap process is validatorless by default.
If you receive a warning during a bootstrap that a validator key is in use, remove the configuration for this legacy bootstrap mode.
Edit your [config.rb (knife.rb)](/workstation/config_rb/) file and remove any `validation_key` or `validation_client_name` entries.

## Bootstrapping with chef-vault

Use the following options with a validatorless bootstrap to specify
items that are stored in chef-vault:

`--bootstrap-vault-file VAULT_FILE`

:   The path to a JSON file that contains a list of vaults and items to
    be updated.

`--bootstrap-vault-item VAULT_ITEM`

:   A single vault and item to update as `vault:item`.

`--bootstrap-vault-json VAULT_JSON`

:   A JSON string that contains a list of vaults and items to be
    updated. --bootstrap-vault-json '{ "vault1": \["item1", "item2"\],
    "vault2": "item2" }'

## Examples

The `--bootstrap-vault-*` options add the client identify of the
bootstrapping node to the permissions list of the specified vault item.
This enables the newly-bootstrapped Chef Infra Client to be able to read
items from the vault. Only a single client is authorized at a time for
access to the vault. (The `-S` search query option with the
`knife vault create` subcommand does the same.)

### Recreate a data bag item

The following example shows how to recreate a data bag item:

``` bash
knife vault delete sea power
Do you really want to delete sea/power? (Y/N) Y
Deleted chef_vault_item[sea/power]

echo "{\"some\":\"content for them\"}" > sea-power-content.json

cat sea-power-content.json
{"some":"content for them"}

knife vault create sea power -M client -A sean_horn,angle -J sea-power-content.json
```

No clients, because the `-S` option was not specified while creating the
vault.

At this time, only the users `sean_horn` and `angle` are authorized to
read and manage the vault.

``` bash
knife vault show sea power  --mode client -p all
admins:
  sean_horn
  angle
clients:
id:           power
search_query:
some:         content for them
```

It is definitely an encrypted databag, see?

``` bash
knife data_bag show sea power
WARNING: Encrypted data bag detected, but no secret provided for decoding.  Displaying encrypted data.
id:   power
some:
cipher:         aes-256-cbc
encrypted_data: c7Axnyg+1KDxBPOZdYN9QuIYx6dmSmK70unAQbn12Lygvsv2g9DPJJbueXVh
+yxL
iv:             ONoVR7OjPZiAzaqOZ30bjg==
version:        1
```

### Use --bootstrap-vault-file

Use the `sea:power` recreation step above first, to follow the
difference in the vault permissions.

``` bash
echo "{\"sea\":\"power\"}" > sea-power-bootstrap-vault-file.json

knife bootstrap localhost -p 2200 -N ubuntu-20.04 -r 'role[group1]' --connection-user vagrant --sudo --bootstrap-vault-file sea-power-bootstrap-vault-file.json
Node ubuntu-20.04 exists, overwrite it? (Y/N) Y
Client ubuntu-20.04 exists, overwrite it? (Y/N) Y
Creating new client for ubuntu-20.04
Creating new node for ubuntu-20.04
Connecting to localhost
localhost -----> Existing Chef installation detected
localhost Starting first Chef Client run...
localhost Starting Chef Client, version 12.2.1
localhost resolving cookbooks for run list: ["delay-test-reporting"]
localhost Synchronizing Cookbooks:
localhost   - delay-test-reporting
localhost Compiling Cookbooks...
localhost Converging 1 resources
localhost Recipe: delay-test-reporting::default
localhost   * execute[sleep 30] action run
localhost     - execute sleep 30
localhost
localhost Running handlers:
localhost Running handlers complete
localhost Chef Client finished, 1/1 resources updated in 34.307257232 seconds
```

The client `ubuntu-20.04` was added to the `chef-vault` during the
bootstrap.

``` bash
knife vault show sea power  --mode client -p all
admins:
  sean_horn
  angle
clients:      ubuntu-20.04
id:           power
search_query:
some:         content for them
```

### Use --bootstrap-vault-item

Use the `sea:power` re-creation step above first, to follow the
difference in the vault permissions.

``` bash
knife bootstrap localhost -p 2200 -N ubuntu-20.04 -r 'role[group1]' --connection-user vagrant --sudo --bootstrap-vault-item sea:power
Node ubuntu-20.04 exists, overwrite it? (Y/N) Y
Client ubuntu-20.04 exists, overwrite it? (Y/N) Y
Creating new client for ubuntu-20.04
Creating new node for ubuntu-20.04
Connecting to localhost
localhost -----> Existing Chef installation detected
localhost Starting first Chef Client run...
localhost Starting Chef Client, version 12.2.1
localhost resolving cookbooks for run list: ["delay-test-reporting"]
localhost Synchronizing Cookbooks:
localhost   - delay-test-reporting
localhost Compiling Cookbooks...
localhost Converging 1 resources
localhost Recipe: delay-test-reporting::default
localhost   * execute[sleep 30] action run
localhost     - execute sleep 30
localhost
localhost Running handlers:
localhost Running handlers complete
localhost Chef Client finished, 1/1 resources updated in 34.322229474
seconds
```

During the above run, the `sea:power` vault item was updated with the
`ubuntu-20.04` client during the validatorless bootstrap. Previously, it
only had the two admins authorized to view the content

``` bash
knife vault show sea power -p all
admins:
  sean_horn
  angle
clients:      ubuntu-20.04
id:           power
search_query: role:stuff
some:         secret stuff for them
```

Then, let's check the `ubuntu-20.04` client. The client itself can decrypt and read
the encrypted databag contents as well using the embedded knife CLI in the Chef Infra
Client package.

``` bash
sudo /opt/chef/bin/knife vault show sea power -c /etc/chef/client.rb -M client -p all
admins:
  sean_horn
  angle
clients:      ubuntu-20.04
id:           power
search_query: role:group1
some:         secret stuff for them
```

Success! The client is authorized to view the content of the `sea:power`
databag item

### Use --bootstrap-vault-json

Use the `sea:power` re-creation step above first, to follow the
difference in the vault permissions.

``` bash
knife bootstrap localhost -p 2200 -N ubuntu-20.04 -r 'role[group1]' --connection-user vagrant --sudo --bootstrap-vault-json '{"sea": "power"}'
Node ubuntu-20.04 exists, overwrite it? (Y/N) Y
Client ubuntu-20.04 exists, overwrite it? (Y/N) Y
Creating new client for ubuntu-.04
Creating new node for ubuntu-20.04
Connecting to localhost
localhost -----> Existing Chef installation detected
localhost Starting first Chef Client run...
localhost Starting Chef Client, version 12.2.1
localhost resolving cookbooks for run list: ["delay-test-reporting"]
localhost Synchronizing Cookbooks:
localhost   - delay-test-reporting
localhost Compiling Cookbooks...
localhost Converging 1 resources
localhost Recipe: delay-test-reporting::default

localhost   * execute[sleep 30] action run
localhost     - execute sleep 30
localhost
localhost Running handlers:
localhost Running handlers complete
localhost Chef Client finished, 1/1 resources updated in 33.732784033 seconds
```

``` bash
knife vault show sea power -M client -p all
admins:
  sean_horn
  angle
clients:      ubuntu-20.04
id:           power
search_query:
some:         content for them
```

## Unattended Installs

Chef Infra Client can be installed using an unattended bootstrap. This
allows Chef Infra Client to be installed from itself, without requiring
SSH. For example, machines are often created using environments like AWS
Auto Scaling, AWS CloudFormation, Rackspace Auto Scale, and PXE. In this
scenario, using tooling for attended, single-machine installs like
`knife bootstrap` or `knife CLOUD_PLUGIN create` is not practical
because the machines are created automatically and someone cannot always
be on-hand to initiate the bootstrap process.

When Chef Infra Client is installed using an unattended bootstrap,
remember that Chef Infra Client:

- Must be able to authenticate to the Chef Infra Server
- Must be able to configure a run-list
- May require custom attributes, depending on the cookbooks that are being used
- Must be able to access the chef-validator.pem so that it may create a new identity on the Chef Infra Server
- Must have a unique node name; Chef Infra Client will use the FQDN for the host system by default

When Chef Infra Client is installed using an unattended bootstrap, it
may be built into an image that starts Chef Infra Client on boot, or
installed using User Data or some other kind of post-deployment script.
The type of image or User Data used depends on the platform on which the
unattended bootstrap will take place.

### Bootstrapping with User Data

The method used to inject a user data script into a server will vary
depending on the infrastructure platform being used. For example, on AWS
you can pass this data in as a text file using the command line tool.

The following user data examples demonstrate the process of
bootstrapping Windows and Linux nodes.

#### PowerShell User Data

``` powershell
## Set host file so the instance knows where to find chef-server
$hosts = "1.2.3.4 hello.example.com"
$file = "C:\Windows\System32\drivers\etc\hosts"
$hosts | Add-Content $file

## Download the Chef Client
$clientURL = "https://packages.chef.io/files/stable/chef/12.19.36/windows/2012/chef-client-<version-here>.msi"
$clientDestination = "C:\chef-client.msi"
Invoke-WebRequest $clientURL -OutFile $clientDestination

## Install the Chef Client
Start-Process msiexec.exe -ArgumentList @('/qn', '/lv C:\Windows\Temp\chef-log.txt', '/i C:\chef-client.msi', 'ADDLOCAL="ChefClientFeature,ChefSchTaskFeature,ChefPSModuleFeature"') -Wait

## Create first-boot.json
$firstboot = @{
   "run_list" = @("role[base]")
}
Set-Content -Path c:\chef\first-boot.json -Value ($firstboot | ConvertTo-Json -Depth 10)

## Create client.rb
$nodeName = "lab-win-{0}" -f (-join ((65..90) + (97..122) | Get-Random -Count 4 | % {[char]$_}))

$clientrb = @"
chef_server_url        'https://chef-server/organizations/my-org'
validation_client_name 'validator'
validation_key         'C:\chef\validator.pem'
node_name              '{0}'
"@ -f $nodeName

Set-Content -Path c:\chef\client.rb -Value $clientrb

## Run Chef
C:\opscode\chef\bin\chef-client.bat -j C:\chef\first-boot.json
```

#### Bash User Data

``` bash
#!/bin/bash -xev

# Do some chef pre-work
/bin/mkdir -p /etc/chef
/bin/mkdir -p /var/lib/chef
/bin/mkdir -p /var/log/chef

# Setup hosts file correctly
cat >> "/etc/hosts" << EOF
10.0.0.5    compliance-server compliance-server.automate.com
10.0.0.6    infra-server infra-server.automate.com
10.0.0.7    automate-server automate-server.automate.com
EOF

cd /etc/chef/

# Install chef
curl -L https://omnitruck.chef.io/install.sh | bash || error_exit 'could not install chef'

# Create first-boot.json
cat > "/etc/chef/first-boot.json" << EOF
{
   "run_list" :[
   "role[base]"
   ]
}
EOF

NODE_NAME=node-$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 4 | head -n 1)

# Create client.rb
cat > '/etc/chef/client.rb' << EOF
log_location            STDOUT
chef_server_url         'https://aut-chef-server/organizations/my-org'
validation_client_name  'my-org-validator'
validation_key          '/etc/chef/my_org_validator.pem'
node_name               "${NODE_NAME}"
EOF

chef-client -j /etc/chef/first-boot.json
```

It is important that settings in the [client.rb
file](/config_rb_client/)---`chef_server_url`, `http_proxy`, and so
on are used---to ensure that configuration details are built into the
unattended bootstrap process.

**Setting the initial run-list**

{{% ctl_chef_client_bootstrap_initial_run_list %}}
