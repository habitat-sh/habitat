+++
title = "Origin Membership & RBAC"

date = 2020-10-12T13:53:50-07:00
draft = false

[menu]
  [menu.habitat]
    title = "Origin Membership & RBAC"
    identifier = "habitat/origins/origin-rbac Builder RBAC"
    parent = "habitat/origins"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/builder-rbac.md)

Prerequisites:

- [Download the Chef Habitat CLI]({{< relref "install-habitat.md" >}})
- [Create a Chef Habitat Builder account]({{< relref "builder-account" >}})
- [Generate a personal access token]({{< relref "builder-profile#create-a-personal-access-token" >}})
- [Create an origin]({{< relref "origins.md#create-origin" >}}) or accept an [invitation]({{< relref "#manage-origin-membership-with-hab-origin-invitations" >}}) to an existing origin
- [Get origin keys]({{< relref "origins.md#origin-keys" >}})

## Role-Based Access Control (RBAC) for Chef Habitat Builder (SaaS and on-prem)

**New in: 1.6.140**
{{< note >}}
RBAC improves operational safety by letting you enable specific levels of access to each user of an origin. With this upgrade, existing origins and their users are automatically converted to RBAC. The RBAC upgrade assigns the 'Maintainer' role to existing users with a generic 'Member' role. The differences between 'Member' and 'Maintainer' are detailed below. The origin owner role remains unchanged.
{{< /note >}}

RBAC membership is a token-based authentication process that works at the origin level. Your membership role defines the level of access that you have to the resources within an origin. When you first join or create an origin, Chef Habitat Builder identifies your personal access token and assigns a membership role to it for that origin. By default, when you join an origin you're assigned the "read-only" role and when you create an an origin you're assigned the 'owner' role. Role access is cumulative and progressive--each RBAC role includes all of the privileges of the previous roles and adds new access privileges.

RBAC Origin Member Roles:

Read-Only
: The default membership role for any user joining an origin. 'Read-Only' users can read an origin's packages, channels, origin membership, jobs, keys, integrations, invitations, roles, settings but cannot add to, change, or delete anything in the origin, including uploading packages and inviting users to the origin.

Member
: In addition to read-only access, an origin 'Member' can upload and build packages in the 'unstable' channel, but they cannot promote packages to other channels.

Maintainer
: Origin 'Members' from Chef Habitat versions before 1.6.140 are now called 'Maintainers'. This role has full read and write access to packages, channels, origin membership, jobs, integrations, invitations, settings. However, the 'Maintainer' role is more limited than the past role, in that 'Maintainers' only have read access to packages, channels, origin membership, jobs, keys, integrations, and settings. Origin 'Maintainers' can read origin membership roles and see and send invitations, but they cannot otherwise change origin membership--their own or anybody else's. Finally, 'Maintainers' can neither read nor write origin secrets.

Administrator
: In addition to 'Maintainer' access, the 'Administrator' role adds the privileges for writing origin keys and membership roles, as well as for reading and writing origin secrets. Administrators have full read and write access to packages, channels, origin membership, jobs, keys, integrations, invitations, roles, secrets, settings.

Owner
: As in the past, the origin 'Owner' has full read and write access to the origin. Only Owners can delete the origin or transfer ownership to another member.

## Comparison of RBAC Membership Roles and Actions

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

## Manage Origin Membership

In tandem with the changes to the Builder membership roles, we've also updated the `hab` CLI to support RBAC. We're working on adding role management to the Chef Habitat Builder site, but in the meantime, you'll need to use the CLI for now.

![Manage origin membership](/images/habitat/origin-members.png)

### Manage origin membership with `hab origin invitations`

Use the [hab origin invitations]({{< relref "habitat-cli.md#hab-origin-invitations" >}}) command to invite users to join your origin and to respond to invitations. Origin Administrators and Owners can use this command to manage invitations.

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

### Manage membership roles with `hab origin rbac`

Use the [hab origin rbac]({{< relref "habitat-cli.md#hab-origin-rbac" >}}) command to see and set role based access control (RBAC) from the command line.
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

```bash
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
