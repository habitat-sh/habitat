+++
title = "Create an Origin on Builder"
description = "Create an Origin on Builder"

[menu]
  [menu.habitat]
    title = "Origins"
    identifier = "habitat/builder/origins"
    parent = "habitat/builder"
    weight = 30

+++

An origin is a place on Chef Habitat Builder where you can store, share, and build packages. It is a unique namespace within Chef Habitat Builder, and while you can delete or transfer an origin, you can't rename an origin after it is created. One example of an origin is the "core" origin, which is the set of foundational packages managed and versioned by the core Chef Habitat maintainers.

You can join existing origins by invitation and you can create your own origins.
For more on invitations, see [origin membership and RBAC](#origin-membership).

### Create an Origin

![Chef Habitat Builder without origins](/images/habitat/create-origin.png)

To create an origin, select the **Create origin** button on the _My Origins_ page which opens the _Create New Origin_ form. (Chef Habitat Builder > My Origins )

![Creating an origin](/images/habitat/create-origin-form.png)

First, enter a unique name that you want to associate with your packages.  Chef Habitat will only let you create an origin with a unique name. Some examples that you'll see in Chef Habitat Builder are team names, user names, and abstract concepts.

Next, choose a privacy setting to set as the default for new packages. You can override this setting when uploading individual packages from the CLI or by connecting a plan file that declares a package as private. The difference between public and private packages is:

  - Anyone can find and use public packages
  - Only users with origin membership can find and use private packages

When you select **Save and Continue**, Chef Habitat Builder:

1. Creates your origin
1. Creates an [origin key pair](#origin-keys)
1. Redirects Chef Habitat Builder to the origin page

![Origin successfully created](/images/habitat/create-origin-done.png)

#### Create an Origin with the Chef Habitat CLI

Use the [hab origin](../habitat-cli/#hab-origin) commands to manage your origins from the command line.

Create an origin from the command line with the [hab origin create](/habitat-cli/#hab-origin-create/) command

```
hab origin create <origin>
```

The results of this command differ slightly from creating an origin on the Chef Habitat Builder site. The CLI command:

1. Creates an origin on the Chef Habitat Builder site
1. Does _not_ generate an origin key pair

For more information, see the [`hab origin create`](/habitat-cli/#hab-origin-create) CLI documentation.

## Origin Membership & RBAC

Prerequisites:

* [Download the Chef Habitat CLI](/install-habitat)
* [Create a Chef Habitat Builder account](#builder-account)
* [Generate a personal access token](#builder-token)
* [Create an origin](#create-origin) or accept an invitation to an existing origin
* [Get origin keys](#origin-keys)


### Role-Based Access Control (RBAC) for Chef Habitat Builder (SaaS and on-prem)

New in: 1.6.140

RBAC provides your organization with better operational safety by letting you assign specific levels of access to each user that belongs to an origin. With RBAC in place, existing standard origin 'members' from earlier versions are assigned the 'Maintainer' role. This role has similar permissions of the previous generic 'member' role, and the areas of difference are detailed below. The origin owner role remains unchanged.

When you join or create an origin, Chef Habitat Builder identifies your personal access token and assigns it a membership role for that origin. Your membership role defines the level of access that you have to the resources in an origin. By default, you're assigned the "read-only" role when you join an origin, and you're assigned the 'owner' role when you create an origin.

RBAC Origin Member Roles:

* Read-Only: This user can read an origin's packages, channels, origin membership, jobs, keys, integrations, invitations, roles, settings but cannot add to, change, or delete anything else in the origin, including uploading packages and inviting users to the origin. Read-Only is the default membership role for users joining the origin.
* Member: In addition to read-only access, an origin 'Member' can upload and build packages in the 'unstable' channel, but they cannot promote packages to other channels.
* Maintainer: Existing origin 'members' are now 'Maintainers'. This role has full read and write access to packages, channels, origin membership, jobs, integrations, invitations, settings. However, the 'Maintainer' role is more limited than the past role, in that 'Maintainers' only have read access to packages, channels, origin membership, jobs, keys, integrations, and settings. Origin 'Maintainers' can read origin membership roles and see and send invitations, but they cannot otherwise change origin membership--their own or anybody else's. Finally, 'Maintainers' can neither read nor write origin secrets.
* Administrator: In addition to 'Maintainer' access, the 'Administrator' role adds the missing privileges for writing origin keys and membership roles, as well as for reading and writing origin secrets. Administrators have full read and write access to packages, channels, origin membership, jobs, keys, integrations, invitations, roles, secrets, settings.
* Owner: As in the past, the origin 'Owner' has full read and write access to the origin. Only Owners can delete the origin or transfer ownership to another member.

### Comparison of RBAC Member Roles and Actions

| Action | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| **Packages** |
| View packages | Y | Y | Y | Y | Y |
| Upload packages to `unstable` | N | Y | Y | Y | Y |
| Promote packages from `unstable` | N | N | Y | Y | Y |
| **Build Jobs** |
| View build jobs | Y | Y | Y | Y | Y |
| Trigger `unstable` build job | N | Y | Y | Y | Y |
| **Channels** |
| View channels | Y | Y | Y | Y | Y |
| Add/Update/Delete channels | N | N | Y | Y | Y |
| **Origin Keys** |
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |
| **Origin Membership** |
| View origin membership | Y | Y | Y | Y | Y |
| View invitations | Y | Y | Y | Y | Y |
| Send Invitations | N | N | Y | Y | Y |
| Revoke Invitations | N | N | Y | Y | Y |
| **Member Roles** |
| View member roles | Y | Y | Y | Y | Y |
| Update member roles | N | N | N | Y | Y |
| **Origin Settings** |
| View settings | Y | Y | Y | Y | Y |
| Add/Update/Delete settings | N | N | N | Y | Y |
| **Origin Secrets** |
| View secrets | N | N | N | Y | Y |
| Add/Update/Delete secrets | N | N | N | Y | Y |
| **Cloud Integrations** |
| View integrations | Y | Y | Y | Y | Y |
| Add/Update/Delete integrations | N | N | Y | Y | Y |
| **Ownership** |
| Transfer Origin | N | N | N | N | Y |
| Delete Origin | N | N | N | N | Y |

### Manage Origin Membership

In tandem with the changes to the Builder membership roles, we've also updated the `hab` CLI to support RBAC. We're working on adding role management to the Chef Habitat Builder site, but in the meantime, you'll need to use the CLI for now.

![Manage origin membership](/images/habitat/origin-members.png)

#### Manage origin membership with `hab origin invitations`

Manage Chef Habitat Builder origin membership with the Chef Habitat CLI, using the [hab origin invitations](/habitat-cli/#hab-origin-invitations) command.

All Chef Habitat Builder users can accept, ignore, and see invitations for their accounts.

View origin invitations:

```bash
hab origin invitations list
```

Accept origin invitations:

```bash
hab origin invitations accept <ORIGIN> <INVITATION_ID>
```

Ignore origin invitations:

```bash
hab origin invitations ignore <ORIGIN> <INVITATION_ID>
```

Send origin membership invitations:

```bash
hab origin invitations send <ORIGIN> <INVITEE_ACCOUNT>
```

Origin administrators and owners can see all pending origin membership invitations:

```bash
hab origin invitations pending <ORIGIN>
```

Origin administrators and owners can rescind an origin membership invitation:

```bash
hab origin invitations rescind <ORIGIN> <INVITATION_ID>
```

Origin owners can transfer origin ownership to another member:

```bash
hab origin transfer [OPTIONS] <ORIGIN> <NEW_OWNER_ACCOUNT>
```

#### Manage membership roles with `hab rbac`

You can use role based access control (RBAC) from the command line.
An origin `MEMBER_ACCOUNT` is the name used to sign in to Chef Habitat builder. You can find the list of user names on an origin's _Members Tab_. (Builder > Origin > Members)

The RBAC command syntax is:

```bash
hab origin rbac <SUBCOMMAND>
```

The syntax for the `show` subcommand is:

```bash
hab origin rbac show <MEMBER_ACCOUNT> --origin <ORIGIN>
```

See an origin member's RBAC role:

```
hab origin rbac show bluewhale --origin two-tier-app
```

The syntax for the `set` subcommand is:

```bash
hab origin rbac set [FLAGS] [OPTIONS] <MEMBER_ACCOUNT> <ROLE> --origin <ORIGIN>
```

Set an origin membership RBAC role with:

```bash
hab origin rbac set bluewhale admin --origin two-tier-app
```

## Origin Keys

Prerequisites:

* [Download the Chef Habitat CLI](/install-habitat)
* [Create a Chef Habitat Builder account](#builder-account)
* [Generate a personal access token](#builder-token)
* [Create an origin with `hab origin create` or join an origin by invitation](#create-origin)

When you create an origin, Chef Habitat Builder automatically generates _origin keys_.
Origin key cryptography is asymmetric: it has a public origin key that you can distribute freely, and a private origin key that you should distribute only to users belonging to the origin.
All Chef Habitat Builder users with access to the origin can view the origin public key revisions in the origin key tab (Builder > Origin > Keys) and download the origin public key, but only users with the origin 'administrator' or 'owner' roles can view or download the origin private key, or change the origin key pair.

| Keys Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |

Chef Habitat uses origin keys:

* When you build an artifact in your local environment, Chef Habitat signs the artifact with a public key
* When you upload an artifact to Chef Habitat Builder or Builder on-prem, Chef Habitat verifies that the artifact was signed with its public key
* When you install an artifact on a Chef Habitat Supervisor, Chef Habitat uses the public origin key to authorize the artifact's installation; Chef Habitat only installs artifacts for which it has the public origin key
* When you download an artifact to your local environment, Chef Habitat uses the public origin key to verify the artifact's integrity before it starts the installation

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

* "testorigin" is the origin's name
* "20190416223046" is the date and time of the key's creation, which was 2019-04-16 22:30:46.
* `.pub` is the file extension for the public key
* `.sig.key` is the file extension for the private key, which is also called a "signing key"

### <a name="key-tab" id="key-tab" data-magellan-target="key-tab"> The Keys Tab </a>

When you create an origin, Chef Habitat Builder automatically generates an origin key pair and saves both keys. To view your origin keys on Chef Habitat Builder, navigate to your origin and select the **Keys** tab. (Builder > Origins > Keys) You will always be able to view and download origin public keys, but you will only see the private keys for origins in which you are an "administrator" or "owner".

![Viewing your origin keys](/images/habitat/origin-keys.png)

#### Download Origin Keys

Download your private or public origin key by selecting the **download** icon from the right end of the key details, under the _Actions_ heading.

![Detail of the download icon](/images/habitat/origin-key-download.png)

#### Upload Origin Keys

You can upload origin keys that you generate on the command line to Chef Habitat Builder by selecting either the **Upload a private key** or **Upload a public key** icon, and copy your key into the form that appears.

![Example form content for uploading an origin key in Builder](/images/habitat/builder-key-upload.png)

### <a name="key-cli" id="key-cli" data-magellan-target="key-cli"> Managing Origin Keys with the CLI </a>

Run Chef Habitat CLI commands from your local environment or from within the Chef Habitat Studio.

See the CLI documentation for more information on the [`hab origin key`](/habitat-cli/#hab-origin-key) commands.

#### Find Your Local Origin Keys

Chef Habitat stores your public and private origin keys at `~/.hab/cache/keys` on Linux systems, `C:\hab\cache\keys` on Windows, and at `/hab/cache/keys` inside of the Chef Habitat Studio environment.

##### To find your origin keys in your local environment:

On Windows:

```PowerShell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la ~/.hab/cache/keys
```

##### To find your existing origin keys from inside of the Chef Habitat Studio:

On Windows:

```powershell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la /hab/cache/keys
```

#### Generate Origin Keys with the CLI

When you create an origin through the site, Chef Habitat Builder automatically generates an origin key pair.

The Chef Habitat CLI creates origin key pairs through two different commands, for two different uses:

* Use [`hab setup`](/install-habitat) to generate your first origin key pair as part of setting up the `hab` CLI
* Use the `hab origin key generate <ORIGIN>` command to create an key pair for an origin created with the `hab origin create` command

Create origin keys with the `hab` command:

```hab
hab origin key generate <ORIGIN>
```

#### Download Origin Keys with the CLI

To get your public origin key using the command line, use:

```hab
hab origin key download <ORIGIN>
```

#### Upload Origin Keys with the CLI

Creating an origin with the `hab origin create` command registers the origin on Chef Habitat Builder without creating an origin key pair. The `hab origin key generate` command creates the key pair and saves them in your local environment, but it does not upload either origin key to Chef Habitat Builder.

* Only "administrators" and "owners" can upload new keys to an origin.
* Builder requires the public origin key to upload artifacts for that origin, so you'll need to upload it.
* Builder requires the private origin key to enable new artifact builds from packages with plans linked to that origin.

Upload origin keys with the `hab` command:

```hab
hab origin key upload <ORIGIN>
```

Upload the origin private key:

```hab
hab origin key upload --secret <ORIGIN>
```

Upload both origin keys at the same time:

```hab
hab origin key upload  --secfile <PATH_TO_PRIVATE_KEY> --pubfile <PATH_TO_PUBLIC_KEY>
```

#### Import Origin Keys with the CLI

Use `hab origin key import` to read the key from a standard input stream into Chef Habitat Builder:

```hab
hab origin key import <enter or paste key>
hab origin key import <PATH_TO_KEY>
curl <URL_THAT_RETURNS_KEY> | hab origin key import
```

##### Troubleshoot Origin Key Import

On a macOS, you may encounter an upload failure.
To remediate this failure:

 * Check that your `HAB_AUTH_TOKEN` environment variable is properly set and initialized
 * Add your `SSL_CERT_FILE` to the environment variables in your interactive shell configuration file, such as your `.bashrc`.


```bash
  export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

Initialize the setting from the command line with:

```bash
 source ~/.bashrc
```

## Origin Settings

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

### Default Package Settings

The _Default Package Settings_ define the visibility of build artifacts (.hart files). Everyone with origin membership can view the origin settings, but only origin administrators and owners can add, update, or delete settings.

* Public packages are visible in search results and can be used by every Chef Habitat Builder user
* Private artifacts do not appear in search results and are available only to users with origin membership

Change the default setting for the origin by switching from **Public Packages** to **Private Packages**. The default setting required for each origin and users with more than one origin can set some as public and others as private. Packages can have different default visibility settings than their origin's. Change the default visibility setting in for individual packages in that package's setting tab (Builder > Origin > Package > Settings).

### Origin Secrets

Everyone with origin membership can view origin secrets, but only origin administrators and owners can add, update, or delete settings. _Origin Secrets_ are located at the bottom of the _Settings_ tab (Builder > Origin > Settings > Origin Secrets) and they let you encrypt and store secrets as environment variables. Origin secrets are useful for plans that require access to protected resources at build time, such as private source-code repositories and cloud storage providers.

Origin secrets are retained by the origin and are available for any of that origin's packages. The origin secrets in your local environment are encrypted with an origin encryption key. Only Chef Habitat Builder can read encrypted origin secrets.

#### Manage Origin Secrets with the Chef Habitat CLI

You can view the list of origin secrets and delete them in Chef Habitat Builder.
However, the primary way of interacting with origin secrets is with the Chef Habitat CLI.

##### List Secrets

To list all of the secrets in an origin, use:

```hab
hab origin secret list --origin <ORIGIN>
```

##### Set Origin Secrets as Environment Variables

Add your origin secrets as environment variables in your local environment:

```bash
export HAB_ORIGIN=<ORIGIN>
export HAB_AUTH_TOKEN=<TOKEN>
hab origin secret list
```

##### Save an Origin Secret

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

##### Delete an Origin Secret

To delete an origin secret from an origin with the CLI

```hab
hab origin secret delete AWS_ACCESS_KEY_ID
hab origin secret delete AWS_SECRET_ACCESS_KEY
```

See [Using Origin Secrets in Plans](/plan-overview/#buildtime-workflow) for guidance on using origin secrets.

See the [`hab origin secret`](/habitat-cli/#hab-origin-secret) CLI documentation for more information on these commands.
