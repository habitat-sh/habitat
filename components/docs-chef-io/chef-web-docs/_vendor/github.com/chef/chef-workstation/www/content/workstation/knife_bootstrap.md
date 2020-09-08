+++
title = "knife bootstrap"
draft = false

aliases = ["/knife_bootstrap.html", "/knife_bootstrap/"]

[menu]
  [menu.workstation]
    title = "knife bootstrap"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_bootstrap.md knife bootstrap"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_bootstrap.md)

{{% chef_client_bootstrap_node %}}

{{% knife_bootstrap_summary %}}

**Considerations:**

-   Knife will copy the contents of the `~/.chef/client.d` directory on
    your local workstation to the `client.d` directory on the device
    being bootstrapped with the `knife bootstrap` command. You can also
    set the `client_d_dir` option in the `config.rb` file to point to an
    arbitrary directory instead of `~/.chef/client.d`, and the contents
    of that directory will be copied to the device being bootstrapped.
    All config files inside the `client.d` directory will get copied
    into the `/etc/chef/client.d` directory on the system being
    bootstrapped.
-   SSL certificates from an on-premises Chef Infra Server can be copied
    to the `/trusted_certs_dir` directory on your local workstation
    automatically by running [knife ssl fetch](/workstation/knife_ssl_fetch/).
    These certificates are used during `knife` operations to communicate
    with the Chef Infra Server.
-   By default, `knife bootstrap` will attempt to use `ssh` to connect to 
    the target node. Use the `-o` to specify a different protocol, such as 
    `winrm` for windows nodes.

## Syntax

This subcommand has the following syntax:

``` bash
knife bootstrap FQDN_or_IP_ADDRESS (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

### General Connection Options

`-U USERNAME`, `--connection-user USERNAME`

:   Authenticate to the target host with this user account.

`-P PASSWORD`, `--connection-password PASSWORD`

:   Authenticate to the target host with this password."

`-p PORT`, `--connection-port PORT`

:   The port on the target node to connect to."

`-o PROTOCOL`, `--connection-protocol PROTOCOL`

:   The protocol to use to connect to the target node. 
    Options are `ssh` or `winrm`. `ssh` is default.

`-W SECONDS`, `--max-wait SECONDS`

:   The maximum time to wait for the initial connection to be
    established.

`--session-timeout SECONDS`

:   The number of seconds to wait for each connection operation to be
    acknowledged while running bootstrap.

### WinRM Connection Options

`--winrm-ssl-peer-fingerprint FINGERPRINT`

:   SSL certificate fingerprint expected from the target.

`-f CA_TRUST_PATH`, `--ca-trust-file CA_TRUST_PATH`

:   The Certificate Authority (CA) trust file used for SSL transport

`--winrm-no-verify-cert`

:   Do not verify the SSL certificate of the target node for WinRM.

`--winrm-ssl`

:   Use SSL in the WinRM connection.

`-w AUTH-METHOD`, `--winrm-auth-method AUTH-METHOD`

:   The WinRM authentication method to use.

`--winrm-basic-auth-only`

:   For WinRM basic authentication when using the 'ssl' auth method.

`-R KERBEROS_REALM`, `--kerberos-realm KERBEROS_REALM`

:   The Kerberos realm used for authentication.

`-S KERBEROS_SERVICE`, `--kerberos-service KERBEROS_SERVICE`

:   The Kerberos service used for authentication.

### SSH Connection Options

`-G GATEWAY`, `--ssh-gateway GATEWAY`

:   The SSH tunnel or gateway that is used to run a bootstrap action on
    a machine that is not accessible from the workstation.

`--ssh-gateway-identity SSH_GATEWAY_IDENTITY`

:   The SSH identity file used for gateway authentication.

`-A`, `--ssh-forward-agent`

:   Enable SSH agent forwarding.

`-i IDENTITY_FILE`, `--ssh-identity-file IDENTITY_FILE`

:   The SSH identity file used for authentication. Key-based
    authentication is recommended.

`ssh_verify_host_key`, `--ssh-verify-host-key VALUE`

:   Verify host key. Default is 'always'

### Chef Installation Options

`--bootstrap-version VERSION`

:   The version of Chef Infra Client to install.

`--bootstrap-install-command COMMAND`

:   Execute a custom installation command sequence for Chef Infra
    Client. This option may not be used in the same command with
    `--bootstrap-curl-options` or `--bootstrap-wget-options`.

`--bootstrap-curl-options OPTIONS`

:   Arbitrary options to be added to the bootstrap command when using
    cURL. This option may not be used in the same command with
    `--bootstrap-install-command`.

`--bootstrap-wget-options OPTIONS`

:   Arbitrary options to be added to the bootstrap command when using
    GNU Wget. This option may not be used in the same command with
    `--bootstrap-install-command`.

`--bootstrap-preinstall-command COMMANDS`

:   Custom commands to run before installing Chef Infra Client

`--bootstrap-url URL`

:   The URL to a custom installation script.

`-m URL`, `--msi-url URL`

:   Location of the Chef Infra Client MSI. The default templates will
    prefer to download from this location. The MSI will be downloaded
    from chef.io if not provided.

`--sudo`

:   Execute a bootstrap operation with sudo.

`--sudo-preserve-home`

:   Use to preserve the non-root user's `HOME` environment.

`--use-sudo-password`

:   Perform a bootstrap operation with sudo; specify the password with
    the `-P` (or `--ssh-password`) option.

`-t TEMPLATE`, `--bootstrap-template TEMPLATE`

:   The bootstrap template to use. This may be the name of a bootstrap
    template---`chef-full` for example---or it may be the full path to
    an Embedded Ruby (ERB) template that defines a custom bootstrap.
    Default value: `chef-full`, which installs Chef Infra Client using
    the Chef Infra installer on all supported platforms.

### Proxy Options

`--bootstrap-no-proxy NO_PROXY_URL_or_IP`

:   A URL or IP address that specifies a location that should not be
    proxied during the bootstrap.

`--bootstrap-proxy PROXY_URL`

:   The proxy server for the node that is the target of a bootstrap
    operation.

`--bootstrap-proxy-pass PROXY_PASS`

:   The proxy authentication password for the node being bootstrapped.

`--bootstrap-proxy-user PROXY_USER`

:   The proxy authentication username for the node being bootstrapped.

### Node Options

`-N NAME`, `--node-name NAME`

:   The name of the node.

    {{< note spaces=4 >}}

    This option is required for a validatorless bootstrap.

    {{< /note >}}

`-E ENVIRONMENT`, `--environment ENVIRONMENT`

:   The name of the environment to be applied.

`-r RUN_LIST`, `--run-list RUN_LIST`

:   A comma-separated list of roles and/or recipes to be applied.

`--secret SECRET`

:   The encryption key that is used for values contained within a data
    bag item.

`--secret-file FILE`

:   The path to the file that contains the encryption key.

`--hint HINT_NAME[=HINT_FILE]`

:   An Ohai hint to be set on the bootstrap target. See the
    [Ohai](/ohai/#hints) documentation for more information.
    `HINT_FILE` is the name of the JSON file. `HINT_NAME` is the name of
    a hint in a JSON file. Use multiple `--hint` options to specify
    multiple hints.

`-j JSON_ATTRIBS`, `--json-attributes JSON_ATTRIBS`

:   A JSON string that is added to the first run of a Chef Infra Client.

`--json-attribute-file FILE`

:   A JSON file to be added to the first run of Chef Infra Client.

`--[no-]fips`

:   Allows OpenSSL to enforce FIPS-validated security during Chef Infra
    Client runs.

`--policy-group POLICY_GROUP`

:   The name of a policy group that exists on the Chef Infra Server.

`--policy-name POLICY_NAME`

:   The name of a policy, as identified by the name setting in a Policyfile.rb file.

### chef-vault Options

`--bootstrap-vault-file VAULT_FILE`

:   The path to a JSON file that contains a list of vaults and items to
    be updated.

`--bootstrap-vault-item VAULT_ITEM`

:   A single vault and item to update as `vault:item`.

`--bootstrap-vault-json VAULT_JSON`

:   A JSON string that contains a list of vaults and items to be
    updated. --bootstrap-vault-json '{ "vault1": \["item1", "item2"\],
    "vault2": "item2" }'

### Key Verification Options

`--[no-]host-key-verify`

:   Use `--no-host-key-verify` to disable host key verification. Default
    setting: `--host-key-verify`.

`--[no-]node-verify-api-cert`

:   Verify the SSL certificate on the Chef Infra Server. When `true`,
    Chef Infra Client always verifies the SSL certificate. When `false`,
    Chef Infra Client uses the value of `ssl_verify_mode` to determine
    if the SSL certificate requires verification. If this option is not
    specified, the setting for `verify_api_cert` in the configuration
    file is applied.

`--node-ssl-verify-mode MODE`

:   Set the verify mode for HTTPS requests. Options: `none` or `peer`.

    Use `none` to do no validation of SSL certificates.

    Use `peer` to do validation of all SSL certificates, including the
    Chef Infra Server connections, S3 connections, and any HTTPS
    **remote_file** resource URLs used in a Chef Infra Client run. This
    is the recommended setting.

### Debug Options

`-V -V`

:   Run the initial Chef Infra Client run at the `debug` log-level (e.g.
    `chef-client -l debug`).

`-V -V -V`

:   Run the initial Chef Infra Client run at the `trace` log-level (e.g.
    `chef-client -l trace`).

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Validatorless Bootstrap

{{% knife_bootstrap_no_validator %}}

{{< note >}}

The `--node-name` option is required for a validatorless bootstrap.

{{< /note >}}

### FIPS Mode

{{% fips_intro_client %}}

**Bootstrap a node using FIPS**

{{% knife_bootstrap_node_fips %}}

## Custom Templates

The default `chef-full` template uses the Chef installer. For most
bootstrap operations, regardless of the platform on which the target
node is running, using the `chef-full` distribution is the best approach
for installing Chef Infra Client on a target node. In some situations, a
custom template may be required.

For example, the default bootstrap operation relies on an Internet
connection to get the distribution to the target node. If a target node
cannot access the Internet, then a custom template can be used to define
a specific location for the distribution so that the target node may
access it during the bootstrap operation. The example below will show
you how to create a bootstrap template that uses a custom artifact store
for Chef packages and installation scripts, as well as a RubyGem mirror:

1.  A custom bootstrap template file must be located in a `bootstrap/`
    directory, which is typically located within the `~/.chef/`
    directory on the local workstation. Navigate to the `.chef`
    directory, and create a `bootstrap` directory within it:

    ``` bash
    mkdir bootstrap
    ```

2.  Move to the `bootstrap` directory and create a blank template file;
    this example will use `template.erb` for the template name:

    ``` bash
    touch template.erb
    ```

3.  Still in the `bootstrap` directory, issue the following command to
    copy the `chef-full` configuration to your new template:

    ``` bash
    find /opt/chef-workstation/embedded/lib/ruby -type f -name chef-full.erb -exec cat {} \; > template.erb
    ```

    This command searches for the `chef-full` template file under
    `/opt/chef-workstation/embedded/lib/ruby`, and then outputs the
    contents of the file to `template.erb`. If you used a different
    template file name, be sure to replace `template.erb` with the
    template file you created during the last step.

4.  Update `template.erb` to replace `omnitruck.chef.io` with the URL of
    an `install.sh` script on your artifact store:

    ``` ruby
    install_sh="<%= knife_config[:bootstrap_url] ? knife_config[:bootstrap_url] : "http://packages.example.com/install.sh" %>"
    ```

5.  Still in your text editor, locate the following line near the bottom
    of your `template.erb` file:

    ``` ruby
    cat > /etc/chef/client.rb <<'EOP'
    <%= config_content %>
    EOP
    ```

    Beneath it, add the following, replacing `gems.example.com` with the
    URL of your gem mirror:

    ``` ruby
    cat >> /etc/chef/client.rb <<'EOP'
    rubygems_url "http://gems.example.com"
    EOP
    ```

    This appends the appropriate `rubygems_url` setting to the
    `/etc/chef/client.rb` file that is created during bootstrap, which
    ensures that your nodes use your internal gem mirror.

### Bootstrap a Custom Template

You can use the `--bootstrap-template` option with the `knife bootstrap`
subcommand to specify the name of your bootstrap template file:

``` bash
knife bootstrap 123.456.7.8 -x username -P password --sudo --bootstrap-template "template"
```

Alternatively, you can use the `knife[:bootstrap_template]` option
within `config.rb` to specify the template that `knife bootstrap` will
use by default when bootstrapping a node. It should point to your custom
template within the `bootstrap` directory:

``` ruby
knife[:bootstrap_template] = "#{current_dir}/bootstrap/template.erb"
```

## Examples

The following examples show how to use this knife subcommand:

**Bootstrap a node**

``` bash
knife bootstrap 192.0.2.0 -P vanilla -x root -r 'recipe[apt],recipe[xfs],recipe[vim]'
```

which shows something similar to:

``` none
...
192.0.2.0 Chef Infra Client finished, 12/12 resources updated in 78.942455583 seconds
```

Use `knife node show` to verify:

``` bash
knife node show debian-buster.int.domain.org
```

which returns something similar to:

``` none
Node Name:   debian-buster.int.domain.org
Environment: _default
FQDN:        debian-buster.int.domain.org
IP:          192.0.2.0
Run List:    recipe[apt], recipe[xfs], recipe[vim]
Roles:
Recipes:     apt, xfs, vim, apt::default, xfs::default, vim::default
Platform:    debian 10.0
Tags:
```

**Use an SSH password**

``` bash
knife bootstrap 192.0.2.0 -x username -P PASSWORD --sudo
```

**Use a file that contains a private key**

``` bash
knife bootstrap 192.0.2.0 -x username -i ~/.ssh/id_rsa --sudo
```

**Specify options when using cURL**

``` bash
knife bootstrap --bootstrap-curl-options "--proxy http://myproxy.com:8080"
```

**Specify options when using GNU Wget**

``` bash
knife bootstrap --bootstrap-wget-options "-e use_proxy=yes -e http://myproxy.com:8080"
```

**Specify a custom installation command sequence**

``` bash
knife bootstrap --bootstrap-install-command "curl -l http://mycustomserver.com/custom_install_chef_script.sh | sudo bash -s --"
```

**Bootstrap a Windows node via WinRM using a run list and environment**

``` bash
knife bootstrap -o winrm 123.456.7.8 -U username -P 'PASSWORD' --node-name NODE_NAME --run-list 'recipe[cookbook]' -E ENV_NAME
```

**Bootstrap a Windows node via WinRM using a policyfile and policy group**

``` bash
knife bootstrap -o winrm 123.456.7.8 -U username -P 'PASSWORD' --node-name NODE_NAME --policy-name PF_NAME --policy-group PG_NAME
```

**Bootstrap Windows node with shorthand syntax**
```bash
knife bootstrap winrm://username:PASSWORD@123.456.7.8 --run-list 'recipe[cookbook]' -E ENV_NAME
```
