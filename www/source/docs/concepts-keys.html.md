---
title: Keys
---

# Keys

Habitat has strong cryptography built into both the build system and the supervisor. This means there are several different kinds of keys.

## Origin Keys

As described in a <a href="/docs/concepts-packages">previous topic</a>, every package in Habitat belongs to an origin, and is cryptographically signed with that origin's private key.

Origin key cryptography is asymmetric; it has a public key that you can distribute freely, and a private key that you should keep safe.

Supervisors, by default, will refuse to run packages for which they do not have the public key. They use this public key to verify the integrity of the Habitat package they download, before running it. Supervisors can be provided the public key by pointing them at a depot that has it, or by putting the key on disk outside of Habitat.

## User and Service Group Keys

User and service group keys are used to set up trust relationships between these two entities. Service groups can be set up to reject communication (e.g. applying new configuration via `hab config apply`) from untrusted users.

By default, service groups will trust *any* communication, so for a production deployment of Habitat, setting up these relationships is essential.

User and service group keys also utilize asymmetric cryptography. To apply configuration changes to service groups when running in this mode, a user uses their own private key to encrypt configuration information for a service group, using that service group's public key. The service group then uses its private key to decrypt the configuration information, and the user's public key to verify.

## Ring Encryption Key

A supervisor ring can be optionally set up to encrypt *all* communication across the network. This requires the use of a symmetric pre-shared key. Any supervisor joining the ring that does not present this key will be rejected.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-studio">Studio</a></li>
</ul>
