Federal Information Processing Standards (FIPS) is a United States
government computer security standard that specifies security
requirements for cryptography. The current version of the standard is
FIPS 140-2. Chef Infra Client can be configured to allow OpenSSL to
enforce FIPS-validated security during a Chef Infra Client run. This
will disable cryptography that is explicitly disallowed in
FIPS-validated software, including certain ciphers and hashing
algorithms. Any attempt to use any disallowed cryptography will cause
Chef Infra Client to throw an exception during a Chef Infra Client run.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Chef uses MD5 hashes to uniquely identify files that are stored on the
Chef Infra Server. MD5 is used only to generate a unique hash identifier
and is not used for any cryptographic purpose.



</div>

</div>

Notes about FIPS:

-   May be enabled for nodes running on Microsoft Windows and Enterprise
    Linux platforms
-   Should only be enabled for environments that require FIPS 140-2
    compliance