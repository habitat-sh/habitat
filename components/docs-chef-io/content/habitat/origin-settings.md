+++
title = "Origin Settings"

date = 2020-10-12T14:02:01-07:00
draft = false

[menu]
  [menu.habitat]
    title = "Origin Settings"
    identifier = "habitat/origins/origin-settings Origin Settings"
    parent = "habitat/origins"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/origin-settings.md)

The _Origin Settings_ tab contains:

* Default Package Settings
* Origin Secrets

Everyone with origin membership can see the _Settings_ tab, but only origin administrators and owners can add, update, or delete settings content.

| Settings Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View settings | Y | Y | Y | Y | Y |
| Add/Update/Delete settings | N | N | N | Y | Y |
| **Origin Secrets Actions** |
| View secrets | N | N | Y | Y | Y |
| Add/Update/Delete secrets | N | N | N | Y | Y |

![The administrator or owner's view of the origin settings tab with a public default package setting and a saved origin secret](/images/habitat/origin-secrets.png)

## Default Package Settings

The _Default Package Settings_ define the visibility of build artifacts (.hart files). Everyone with origin membership can view the origin settings, but only origin administrators and owners can add, update, or delete settings.

* Public packages are visible in search results and can be used by every Chef Habitat Builder user
* Private artifacts do not appear in search results and are available only to users with origin membership

Change the default setting for an origin by switching from **Public Packages** to **Private Packages**. The default setting is required for each origin. Packages can have different default visibility settings than the origin to which they belong. You can change the default visibility setting in for an individual packages in the package setting tab (Builder > Origin > Package > Settings).

## Origin Secrets

Everyone with origin membership can view origin secrets, but only origin administrators and owners can add, update, or delete settings. _Origin Secrets_ are located at the bottom of the _Settings_ tab (Builder > Origin > Settings > Origin Secrets) and they let you encrypt and store secrets as environment variables. Origin secrets are useful for plans that require access to protected resources at build time, such as private source-code repositories and cloud storage providers.

Only Chef Habitat Builder can read encrypted origin secrets. The origin secrets in your local environment are encrypted with an origin encryption key. Origin secrets are retained by the origin and are available for any of its packages.

### Manage Origin Secrets with the Chef Habitat CLI

You can view the list of origin secrets and delete them in Chef Habitat Builder.
However, the primary way of interacting with origin secrets is with the Chef Habitat CLI.

#### List Secrets

To list all of the secrets in an origin, use:

```hab
hab origin secret list --origin <ORIGIN>
```

#### Set Origin Secrets as Environment Variables

Add your origin secrets as environment variables in your local environment:

```bash
export HAB_ORIGIN=<ORIGIN>
export HAB_AUTH_TOKEN=<TOKEN>
hab origin secret list
```

#### Save an Origin Secret

To save an origin secret give the secret a name and the key value:

```hab
hab origin secret upload AWS_ACCESS_KEY_ID <your-key-id>
hab origin secret upload AWS_SECRET_ACCESS_KEY <your-secret-access-key>
```

The output should similar to:

```bash
$ hab origin secret upload AWS_ACCESS_KEY_ID 1234567890EXAMPLE
↓ Downloading latest public encryption key
    79 B / 79 B | [========================================] 100.00 % 120.23 KB/s
☑ Cached habicat-20200123456789.pub
☛ Encrypting value for key AWS_ACCESS_KEY_ID.
✓ Encrypted AWS_ACCESS_KEY_ID=[REDACTED].
↑ Uploading secret for key AWS_ACCESS_KEY_ID.
✓ Uploaded secret for AWS_ACCESS_KEY_ID.
```

#### Delete an Origin Secret

To delete an origin secret from an origin with the CLI

```hab
hab origin secret delete AWS_ACCESS_KEY_ID
hab origin secret delete AWS_SECRET_ACCESS_KEY
```

See [Using Origin Secrets in Plans]({{< relref "plan-overview#buildtime-workflow" >}}) for guidance on using origin secrets.

See the [`hab origin secret`]({{< relref "habitat-cli#hab-origin-secret" >}}) CLI documentation for more information on these commands.
