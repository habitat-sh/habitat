+++
title = "Supervisor Cryptography"
date = 2020-10-26T19:09:25-07:00
draft = false
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Supervisor Cryptography"
    identifier = "habitat/supervisors/Supervisor Cryptography"
    parent = "habitat/supervisors"
    weight = 70
+++

Chef Habitat implements cryptography using a Rust [implementation](https://github.com/jedisct1/libsodium) of [NaCl](https://nacl.cr.yp.to/) called `libsodium`. `libsodium` provides a fast, modern framework for encryption, decryption, signing, and verification.

Chef Habitat uses both symmetric encryption (for wire encryption) and asymmetric encryption (for everything else). If you are not familiar with the difference between the two, please consult [this article](https://support.microsoft.com/en-us/topic/a082a391-dee8-6265-9ce6-77c7f07c48dd).

### Message Encryption

When you have either wire encryption or service group encryption turned on, the messages use the Curve25519, Salsa20, and Poly1305 ciphers specified in [Cryptography in NaCl](https://nacl.cr.yp.to/valid.html).

### Package Signing

Chef Habitat packages are signed using [BLAKE2b](https://blake2.net/) checksums. BLAKE2b is a cryptographic hash function faster than MD5, SHA-1, SHA-2 and SHA3, yet provides at least as much security as the latest standard SHA-3.

You can examine the first four lines of a `.hart` file to extract the signature from it, because it is an `xz`-compressed tarball with a metadata header. The `hab pkg header` command will do this for you.

```bash
hab pkg header somefile.hart
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
tail -n +6 somefile.hart | xzcat | tar x
```
