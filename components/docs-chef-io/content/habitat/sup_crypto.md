+++
title = "Supervisor Cryptography"

date = 2020-10-26T19:09:25-07:00
draft = false

[menu]
  [menu.habitat]
    title = "Supervisor Cryptography"
    identifier = "habitat/supervisors/Supervisor Cryptography"
    parent = "habitat/supervisors"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/sup_crypto.md)

## Leader Election

The Chef Habitat Supervisor performs leader election natively for service group [topologies]({{< relref "about_services" >}}) that require one, such as _leader-follower_.

Because Chef Habitat is an eventually-consistent distributed system, the role of the leader is different than in strongly-consistent systems. It only serves as the leader for *application level semantics*, e.g. a database write leader. The fact that a Supervisor is a leader has no bearing upon other operations in the Chef Habitat system, including rumor dissemination for configuration updates. It is _not_ akin to a [Raft](https://raft.github.io/) leader, through which writes must all be funneled. This allows for very high scalability of the Chef Habitat Supervisor ring.

Services grouped using a leader need to have a minimum of three supervisors in order to break ties. It is also strongly recommended that you do not run the service group with an even number of members. Otherwise, in the event of a network partition with equal members on each side, both sides will elect a new leader, causing a full split-brain from which the algorithm cannot recover. Supervisors in a service group will warn you if you are using leader election and have an even number of supervisors.

### Protocol for Electing a Leader

When a service group starts in a leader topology, it will wait until there are sufficient members to form a quorum (at least three). At this point, an election cycle can happen. Each Supervisor injects an election rumor into ring, targeted at the service group, with the _exact same_ rumor, which demands an election and insists that the peer itself is the leader. This algorithm is known as [Bully](https://en.wikipedia.org/wiki/Bully_algorithm).

Every peer that receives this rumor does a simple lexicographic comparison of its GUID with the GUID of the peer contained in that rumor. The winner is the peer whose GUID is higher. The peer then adds a vote for the GUID of the winner, and shares the rumor with others, including the total number of votes of anyone who previously voted for this winner.

An election ends when a candidate peer X gets a rumor back from the ring saying that it (X) is the winner, with all members voting. At this point, it sends out a rumor saying it is the declared winner, and the election cycle ends.

### Related Reading

* For more information about the Bully algorithm, see [Elections in a Distributed Computing System](https://dl.acm.org/doi/10.1109/TC.1982.1675885) by Héctor García-Molina.

## Cryptography

Chef Habitat implements cryptography using a Rust [implementation](https://github.com/jedisct1/libsodium) of [NaCl](https://nacl.cr.yp.to/) called `libsodium`. `libsodium` provides a fast, modern framework for encryption, decryption, signing, and verification.

Chef Habitat uses both symmetric encryption (for wire encryption) and asymmetric encryption (for everything else). If you are not familiar with the difference between the two, please consult [this article](https://support.microsoft.com/kb/246071).

### Message Encryption

When you have either wire encryption or service group encryption turned on, the messages use the Curve25519, Salsa20, and Poly1305 ciphers specified in [Cryptography in NaCl](https://nacl.cr.yp.to/valid.html).

### Package Signing

Chef Habitat packages are signed using [BLAKE2b](https://blake2.net/) checksums. BLAKE2b is a cryptographic hash function faster than MD5, SHA-1, SHA-2 and SHA3, yet provides at least as much security as the latest standard SHA-3.

You can examine the first four lines of a `.hart` file to extract the signature from it, because it is an `xz`-compressed tarball with a metadata header. The `hab pkg header` command will do this for you.

```bash
$ hab pkg header somefile.hart
```

outputs:

```bash
» Reading package header for somefile.hart

Package        : somefile.hart
Format Version : HART-1
Key Name       : myorigin-19780608081445
Hash Type      : BLAKE2b
Raw Signature  : a8yDoiA0Mv0CcW6xVyfkSOIZ0LW0beef4RPtvKL56MxemgG6dMVlKG1Ibplp7DUByr5az0kI5dmJKXgK6KURDzM1N2Y2MGMxYWJiMTNlYjQxMjliZTMzNGY0MWJlYTAzYmI4NDZlZzM2MDRhM2Y5M2VlMDkyNDFlYmVmZDk1Yzk=
```

The `.hart` file format is designed in this way to allow you to extract both the signature and the payload separately for inspection. To extract only the `xz`-compressed content, bypassing the signature, you could type this:

```bash
$ tail -n +6 somefile.hart | xzcat | tar x
```
