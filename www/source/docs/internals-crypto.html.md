---
title: Cryptography
---

# Cryptography

Habitat implements cryptography using a Rust [implementation](https://github.com/jedisct1/libsodium) of [NaCl](https://nacl.cr.yp.to/) called `libsodium`. `libsodium` provides a fast, modern framework for encryption, decryption, signing, and verification.

## Message Encryption

When you have either wire encryption or service group encryption turned on, the messages use the Curve25519, Salsa20, and Poly1305 ciphers specified in [Cryptography in NaCl](http://nacl.cr.yp.to/valid.html).

## Package Signing

Habitat packages are signed using [BLAKE2b](https://blake2.net/) checksums. BLAKE2b is a cryptographic hash function faster than MD5, SHA-1, SHA-2 and SHA3, yet provides at least as much security as the latest standard SHA-3.

You can examine the first four lines of a `.hart` file to extract the signature from it, because a `.hart` is an `xz`-compressed tarball with a metadata header:

       head -n +4 somefile.hart

The `.hart` file format is designed in this way to allow you to extract both the signature and the payload separately for inspection. To extract only the `xz`-compressed content, bypassing the signature, you could type this:

       tail -n +6 somefile.hart | xzcat | tar x

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/internals-supervisor">Supervisor Internals</a></li>
</ul>
