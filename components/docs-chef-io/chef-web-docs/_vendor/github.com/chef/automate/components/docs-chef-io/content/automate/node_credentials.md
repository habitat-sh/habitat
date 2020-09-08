+++
title = "Node Credentials"

date = 2018-05-22T17:23:24-07:00
weight = 20
draft = false
[menu]
  [menu.automate]
    title = "Node Credentials"
    identifier = "automate/settings/node_credentials.md Node Credentials"
    parent = "automate/settings"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/node_credentials.md)

The Chef Automate Credentials page allows you to add, edit, and delete ``SSH``, ``WinRm``, and ``Sudo`` credentials for remotely access to your nodes.

To manage your credentials, navigate to the _Node Credentials_ page from the **Settings** tab.

![Node Credentials](/images/automate/node-credentials.png)

Adding SSH, WinRM, and Sudo credentials is the first step for using the Chef Automate Compliance Scanner. After adding credentials, you'll be able to add nodes and create scan jobs.

Depending on how you've set up your nodes, you may need to set up more than one key that uses the same SSH Private Key with different usernames. For example, AWS EC2 Amazon Linux nodes require the username ``ec2-user``, while AWS EC2 Ubuntu nodes require the username ``ubuntu`` or ``root``. The **Credentials** page enables saving two different sets of credentials, both using the same SSH Private Key and different user names.  However, credentials with different content may also reuse identical key names; it may be advisable to reduce confusion by follow a naming pattern specifying the key name and platform to distinguish between similar credentials.

{{< warning >}}
A credential name may be reused, even when it contains different usernames or keys.
{{< /warning >}}

## Add a Credential

### Add a SSH Credential

![SSH Credential Form](/images/automate/credentials-ssh.png)

**SSH** requires a credential name, a user name and either a SSH password **or** a SSH Private key, but not both.

### Add a WinRM Credential

![WINRM Credential Form](/images/automate/credentials-winrm.png)

**WinRM** requires a credential name, a user name, and a WinRM password.

### Add a Sudo Credential

![Sudo Credential Form](/images/automate/credentials-sudo.png)

**Sudo** requires a credential name, a user name, and a password **or** sudo options, but not both.

Credentials will be visible in the _Node Credentials_ view after using the **Save Credential** button. If you are not redirected to the credentials list, then review the credential you are attempting to add.

## Manage Credentials

* Edit a credential by selecting the credential's name, which opens the credential's detail page.
* Delete a credential by selecting **Delete** from the menu at the end of the table row.
