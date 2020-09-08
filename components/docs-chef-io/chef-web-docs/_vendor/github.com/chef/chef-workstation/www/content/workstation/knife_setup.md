+++
title = "Setting up Knife"
draft = false

aliases = ["/knife_setup.html", "/knife_setup/"]

[menu]
  [menu.workstation]
    title = "Setting up Knife"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_setup.md Setting up Knife"
    parent = "chef_workstation/chef_workstation_tools/knife"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_setup.md)

The knife command line tool must be configured to communicate with the
Chef Infra Server as well as any other infrastructure within your
organization. This is done initially during the workstation setup, but
subsequent modifications can be made using the config.rb configuration
file.

## config.rb Configuration File

Knife is configured using a config.rb configuration, which contains
configuration for both the knife command line tool as well as any
installed knife plugins. See [config.rb](/workstation/config_rb/) for a complete
list of configuration options in the config.rb file.

### Load Path Priority

The config.rb file is loaded every time the knife command is invoked
using the following load order:

-   From a specified location given the `--config` flag
-   From a specified location given the `$KNIFE_HOME` environment
    variable, if set
-   From a `config.rb` file within the current working directory, e.g.,
    `./config.rb`
-   From a `config.rb` file within a `.chef` directory in the current
    working directory, e.g., `./.chef/config.rb`
-   From a `config.rb` file within a `.chef` directory located one
    directory above the current working directory, e.g.,
    `../.chef/config.rb`
-   From `~/.chef/config.rb` (macOS and Linux platforms) or
    `c:\Users\<username>\.chef` (Microsoft Windows platform)

{{< note >}}

When running Microsoft Windows, the config.rb file is located at
`%HOMEDRIVE%:%HOMEPATH%\.chef` (e.g. `c:\Users\<username>\.chef`). If
this path needs to be scripted, use `%USERPROFILE%\chef-repo\.chef`.

{{< /note >}}

### config.rb Configuration Within a Chef Repository

{{% chef_repo_many_users_same_knife %}}

## Generating a config.rb File

The knife command <span class="title-ref">knife configure</span> can be
used to generate your initial config.rb configuration file in your home
directory. See [knife configure](/workstation/knife_configure/) for details.

## Knife Profiles

**Profile Support since Chef 13.7**

Knife profiles provide an alternative to using the `config.rb` files for
configuring your knife client. This makes it easier to switch knife
between multiple Chef Infra Servers or between multiple organizations on
the same Chef Infra Server. Configure knife profiles by adding them to
the `.chef/credentials` file in your home directory on your workstation.
The `credentials` file is TOML formatted. Each profile is listed as a
separate 'table' name of your choice, and is followed by key-value
pairs. The keys correspond to any setting permitted in the
[config.rb](/workstation/config_rb/) file.

File paths, such as `client_key` or `validator_key`, will be relative to
`~/.chef` unless absolute paths are given. Clients can be identified
with either `node_name` or `client_name`, with `client_name` being
preferred.

Credentials for use with Target Mode (e.g.
`chef-client --target switch.example.org`) can also be stored as a
separate profile in the credentials file. The name of the profile should
match the DNS name of the target, and must be surrounded by single
quotes when the name contains a period. For example:
`['switch.example.org']`. Keys that are valid configuration options will
be passed to train, such as `port`.

``` none
# Example .chef/credentials file
[default]
node_name = "barney"
client_key = "barney_rubble.pem"
chef_server_url = "https://api.chef.io/organizations/bedrock"

# a 'config context' such as knife can be is configured as a separate table
[default.knife]
ssh_user = 'ubuntu' # this would have been knife[:ssh_user] in your config.rb
aws_profile = 'engineering'
use_sudo = true

# a client_key may also be specified inline as in this example
[dev]
client_name = "admin"
client_key = """
-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQCqGKukO1De7zhZj6+H0qtjTkVxwTCpvKe4eCZ0FPqri0cb2JZfXJ/DgYSF6vUp
wmJG8wVQZKjeGcjDOL5UlsuusFncCzWBQ7RKNUSesmQRMSGkVb1/3j+skZ6UtW+5u09lHNsj6tQ5
1s1SPrCBkedbNf0Tp0GbMJDyR4e9T04ZZwIDAQABAoGAFijko56+qGyN8M0RVyaRAXz++xTqHBLh
3tx4VgMtrQ+WEgCjhoTwo23KMBAuJGSYnRmoBZM3lMfTKevIkAidPExvYCdm5dYq3XToLkkLv5L2
pIIVOFMDG+KESnAFV7l2c+cnzRMW0+b6f8mR1CJzZuxVxx6xx2fvLi55/mbSYxECQQDeAw6fiIQX
GukBI4eMZZt4nscy2o12KyYner3VpoeE+Np2q+Z3pvAMd/aNzQ/W9WaI+NRfcxUJrmfPwIGm63il
AkEAxCL5HQb2bQr4ByorcMWm/hEP2MZzROV73yF41hPsRC9m66KrheO9HPTJuo3/9s5p+sqGxOlF
L0NDt4SkosjgGwJAFklyR1uZ/wPJjj611cdBcztlPdqoxssQGnh85BzCj/u3WqBpE2vjvyyvyI5k
X6zk7S0ljKtt2jny2+00VsBerQJBAJGC1Mg5Oydo5NwD6BiROrPxGo2bpTbu/fhrT8ebHkTz2epl
U9VQQSQzY1oZMVX8i1m5WUTLPz2yLJIBQVdXqhMCQBGoiuSoSjafUhV7i1cEGpb88h5NBYZzWXGZ
37sJ5QsW+sJyoNde3xH8vdXhzU7eT82D6X/scw9RZz+/6rCJ4p0=
-----END RSA PRIVATE KEY-----
"""
validator_key = "test-validator.pem"
chef_server_url = "https://api.chef-server.dev/organizations/test"

['web.preprod']
node_name = "brubble"
client_key = "preprod-brubble.pem"
chef_server_url = "https://preprod.chef-server.dev/organizations/preprod"

['switch.example.org']
user = "cisco"
password = "cisco"
enable_password = "cisco"
```

There are four ways to select which profile to use and are listed in
priority order:

1.  Pass the `--profile` option to knife, e.g.
    `knife node list --profile dev`.
2.  Set the profile name in the `CHEF_PROFILE` environment variable.
3.  Write the profile name to the `~/.chef/context` file.
4.  Otherwise, knife will use the 'default' profile.

## Knife Config

**knife config support since Chef 14.4**

Your knife profiles can be managed with the `knife config` command.

You can list your profiles using the `knife config list-profiles`
command, for example:

``` bash
## Profile              Client   Key                          Server
 default             barney   ~/.chef/barney_rubble.pem    https://api.chef.io/organizations/bedrock
 dev                 admin    ~/.chef/admin.pem            https://api.chef-server.dev/organizations/test
 web.preprod         brubble  ~/.chef/preprod-brubble.pem  https://preprod.chef-server.dev/organizations/preprod
 switch.example.org  btm      ~/.chef/btm.pem              https://localhost:443
```

The line that begins with the asterisk is the currently selected
profile.

To change the current profile, run the `knife config use-profile NAME`
command, which will write the profile name to the `~/.chef/context`
file.

Running `knife config get-profile` will print out the name of the
currently selected profile.

If you need to troubleshoot any settings, you can verify the value that
knife is using with the `knife config get KEY` command, for example:

``` bash
knife config get chef_server_url
Loading from credentials file /home/barney/.chef/credentials
chef_server_url: https://api.chef-server.dev/organizations/test
```

## Setting Your Text Editor

Some knife commands, such as `knife data bag edit`, require that
information be edited as JSON data using a text editor. For example, the
following command:

``` bash
knife data bag edit admins admin_name
```

will open up the text editor with data similar to:

``` javascript
{
  "id": "admin_name"
}
```

Changes to that file can then be made:

``` javascript
{
  "id": "Justin C."
  "description": "I am passing the time by letting time pass over me ..."
}
```

The type of text editor that is used by knife can be configured by
adding an entry to your config.rb file, or by setting an `EDITOR`
environment variable. For example, to configure knife to open the `vim`
text editor, add the following to your config.rb file:

``` ruby
knife[:editor] = "/usr/bin/vim"
```

When a Microsoft Windows file path is enclosed in a double-quoted string
(" "), the same backslash character (`\`) that is used to define the
file path separator is also used in Ruby to define an escape character.
The config.rb file is a Ruby file; therefore, file path separators must
be escaped. In addition, spaces in the file path must be replaced with
`~1` so that the length of each section within the file path is not more
than 8 characters. For example, if EditPad Pro is the text editor of
choice and is located at the following path:

    C:\\Program Files (x86)\EditPad Pro\EditPad.exe

the setting in the config.rb file would be similar to:

``` ruby
knife[:editor] = "C:\\Progra~1\\EditPa~1\\EditPad.exe"
```

One approach to working around the double- vs. single-quote issue is to
put the single-quotes outside of the double-quotes. For example, for
Notepad++:

``` ruby
knife[:editor] = '"C:\Program Files (x86)\Notepad++\notepad++.exe" -nosession -multiInst'
```

for Sublime Text:

``` ruby
knife[:editor] = '"C:\Program Files\Sublime Text 2\sublime_text.exe" --wait'
```

for TextPad:

``` ruby
knife[:editor] = '"C:\Program Files (x86)\TextPad 7\TextPad.exe"'
```

and for vim:

``` ruby
knife[:editor] = '"C:\Program Files (x86)\vim\vim74\gvim.exe"'
```

### Using Quotes

The text editor command cannot include spaces that are not properly
wrapped in quotes. The command can be entered with double quotes (" ")
or single quotes (' '), but this should be done consistently as shown in
the examples above.
