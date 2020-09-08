+++
title = "Troubleshooting"
draft = false

[menu]
  [menu.workstation]
    title = "Troubleshooting"
    identifier = "chef_workstation/troubleshooting.md Troubleshooting"
    parent = "chef_workstation"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/troubleshooting.md)

## Chef Workstation Logs

Chef Workstation logs are stored in ` ~/.chef-workstation/logs`.

## Uninstall instructions

Follow the steps provided under [Uninstalling]({{< ref "install_workstation.md#uninstalling" >}}).

## Common Error Codes

### CHEFINT001

```
CHEFINT001

An remote error has occurred:

  Your SSH Agent has no keys added, and you have not specified a password or a key file.
```

This error now appears as CHEFTRN007.  If you're running an older version of chef-run
it will appear as CHEFINT001 with the message above.  Follow the steps detailed under
CHEFTRN007 below to resolve.

### CHEFTRN007

`No authentication methods available`

This error occurs when there are no available ssh authentication methods to provide to the server.
chef-run requires a password, a key file, or a `.ssh/config` host entry containing a KeyFile.
Information about each option is below.

#### resolve via chef-run flags

Use `--password` to provide the password required to authenticate to the host:

```
chef-run --password $PASSWORD myhost.example.com --password
```

Alternatively, explicitly provide an identity file using '--identity-file':

```
chef-run --identity-file /path/to/your/ssh/key
```

#### resolve by adding key(s) to ssh-agent
```
## ensure ssh-agent is running.  This may report it is already started:
$ ssh-agent

## Add your key file(s):
$ ssh-add
Identity added: /home/timmy/.ssh/id_rsa (/home/timmy/.ssh/id_rsa)
```

### resolve by adding a host entry to ~/.ssh/config

Add an entry for this host to your .ssh/config:

```
host example.com
  IdentityFile /path/to/valid/key
```
