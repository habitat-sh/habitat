+++
title = "Origin Keys"

date = 2020-10-12T13:59:46-07:00
draft = false

[menu]
  [menu.habitat]
    title = "Origin Keys"
    identifier = "habitat/origins/origin-keys Origin Keys"
    parent = "habitat/origins"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/origin-keys.md)

Prerequisites:

- [Get Chef Habitat]({{< relref "install-habitat.md" >}})
- [Create a Chef Habitat Builder account]({{< relref "builder-account#builder-account" >}})
- [Generate a personal access token]({{< relref "builder-profile#create-a-personal-access-token" >}})
- [Create an origin]({{< relref "origins" >}}) or join an origin by [invitation]({{< relref "origin-rbac#manage-origin-membership-with-hab-origin-invitations" >}})

When you create an origin, Chef Habitat Builder automatically generates _origin keys_.
Origin key cryptography is asymmetric: it has a public origin key that you can distribute freely, and a private origin key (also called a "signing key") that you should distribute only to users of the origin.
All Chef Habitat Builder users with access to the origin can view the public origin key revisions in the origin key tab (Builder > Origin > Keys) and download the public origin key, but only users with the origin 'administrator' or 'owner' roles can view or download the private origin key, or change the origin key pair.

| Keys Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |

Chef Habitat uses origin keys:

- When you build an artifact in your local environment, Chef Habitat signs the artifact with the private origin key
- When you upload an artifact to Chef Habitat Builder or Builder on-prem, Chef Habitat uses the public origin key to verify that the artifact was signed with the private origin key
- When you install any package onto a Chef Habitat Supervisor, Chef Habitat uses the public origin key to verify the package's integrity before it starts the installation
- When you download an artifact to your local Chef Habitat Studio, Chef Habitat uses the public origin key to verify the artifact's integrity before it starts the installation

Chef Habitat Builder origin key names follow the format:

```hab
<origin>-<datetime>.pub (public key)
<origin>-<datetime>.sig.key (private key, also called a "signing key")
```

For example, in:

```hab
testorigin-20190416223046.pub
testorigin-20190416223046.sig.key
```

- "testorigin" is the origin's name
- "20190416223046" is the date and time of the key's creation, which was 2019-04-16 22:30:46.
- `.pub` is the file extension for the public key
- `.sig.key` is the file extension for the private key, which is also called a "signing key"

## The Keys Tab

When you create an origin, Chef Habitat Builder automatically generates an origin key pair and saves both keys. To view your origin keys on Chef Habitat Builder, navigate to your origin and select the **Keys** tab. (Builder > Origins > Keys) You will always be able to view and download public origin keys, but you will only see the private keys for origins in which you are an "administrator" or "owner".

![Viewing your origin keys](/images/habitat/origin-keys.png)

### Download Origin Keys from the Keys Tab

Download your private or public origin key by selecting the **download** icon from the right end of the key details, under the _Actions_ heading.

![Detail of the download icon](/images/habitat/origin-key-download.png)

### Upload Origin Keys from the Keys Tab

You can upload origin keys that you generate on the command line to Chef Habitat Builder by selecting either the **Upload a private key** or **Upload a public key** icon, and copy your key into the form that appears.

![Example form content for uploading an origin key in Builder](/images/habitat/builder-key-upload.png)

## Managing Origin Keys with the CLI

Run Chef Habitat CLI commands from your local environment or from within the Chef Habitat Studio.

See the CLI documentation for more information on the [`hab origin key`]({{< relref "habitat-cli.md/#hab-origin-key" >}}) commands.

### Find Your Origin Keys

Chef Habitat stores your public and private origin keys at `~/.hab/cache/keys` on Linux systems, `C:\hab\cache\keys` on Windows, and at `/hab/cache/keys` inside of the Chef Habitat Studio environment.

#### Find Origin Keys in a Local Environment

On Windows:

```PowerShell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la ~/.hab/cache/keys
```

#### Find Origin Keys in the Chef Habitat Studio

On Windows:

```powershell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la /hab/cache/keys
```

### Generate Origin Keys

When you create an origin through the site, Chef Habitat Builder automatically generates an origin key pair.

The Chef Habitat CLI creates origin key pairs through two different commands, for two different uses:

- Use [`hab setup`]({{< relref "install-habitat.md" >}}) to generate your first origin key pair as part of setting up the `hab` CLI
- Use the `hab origin key generate <ORIGIN>` command to create a key pair for an origin

Create origin keys with the `hab` command:

```hab
hab origin key generate <ORIGIN>
```

### Download Origin Keys

To get your public origin key using the command line, use:

```hab
hab origin key download <ORIGIN>
```

### Upload Origin Keys

Creating an origin with the `hab origin create` command registers the origin on Chef Habitat Builder without creating an origin key pair. The `hab origin key generate` command creates the key pair and saves them in your local environment, but it does not upload either origin key to Chef Habitat Builder.

- Only "administrators" and "owners" can upload new keys to an origin.
- Builder requires the public origin key to upload artifacts for that origin, so you'll need to upload it.
- Builder requires the private origin key to enable new artifact builds from packages with plans linked to that origin.

Upload origin keys with the `hab` command:

```hab
hab origin key upload <ORIGIN>
```

Upload the private origin key:

```hab
hab origin key upload --secret <ORIGIN>
```

Upload both origin keys at the same time:

```hab
hab origin key upload  --secfile <PATH_TO_PRIVATE_KEY> --pubfile <PATH_TO_PUBLIC_KEY>
```

### Import Origin Keys

Use `hab origin key import` to read the key from a standard input stream into Chef Habitat Builder:

```hab
hab origin key import <enter or paste key>
hab origin key import <PATH_TO_KEY>
cat <PATH_TO_KEY> | hab origin key import
```

#### Troubleshoot Origin Key Import

On a macOS, you may encounter an upload failure.
To remediate this failure:

- Check that your `HAB_AUTH_TOKEN` environment variable is properly set and initialized
- Add your `SSL_CERT_FILE` to the environment variables in your interactive shell configuration file, such as your `.bashrc`.

```bash
  export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

Initialize the setting from the command line with:

```bash
 source ~/.bashrc
```
