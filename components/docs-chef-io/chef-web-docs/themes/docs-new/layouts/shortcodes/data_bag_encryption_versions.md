The manner by which a data bag item is encrypted depends on the Chef
Infra Client version used. See the following:

![image](/images/essentials_data_bags_versions.png)

where R is read, W is write, and D is disable. (Disabling support for
older encryption version formats will be in the next version and, if
desired, will require a configuration change.)

For version 0 (default, through Chef Client 10.18):

-   An encrypted data bag item is written using YAML as the
    serialization format
-   Base64 encoding is used to preserve special characters in encrypted
    contents
-   Data is encrypted using AES-256-CBC (as defined by the OpenSSL
    package in the Ruby Standard Library)
-   Chef Infra Client uses [shared secret
    encryption](https://en.wikipedia.org/wiki/Symmetric-key_algorithm);
    an encrypted file can only be decrypted by a node or a user with the
    same shared secret
-   A recipe can load encrypted data as long as the shared secret is
    present in a file on the node or is accessible from a URI path
-   Only the values of a data bag item are decrypted; keys are still
    searchable. The values associated with the `id` key of a data bag
    item are not encrypted (because they are needed when tracking the
    data bag item)

For version 1 (default, starting with Chef Client 11.0):

-   An encrypted data bag item is written using JSON as the
    serialization format
-   Base64 encoding is used to preserve special characters in encrypted
    contents
-   Data is encrypted using AES-256-CBC (as defined by the OpenSSL
    package in the Ruby Standard Library)
-   A data bag item is encrypted using a random initialization vector
    each time a value is encrypted, which helps protect against some
    forms of cryptanalysis
-   Chef Infra Client uses [shared secret
    encryption](https://en.wikipedia.org/wiki/Symmetric-key_algorithm);
    an encrypted file can only be decrypted by a node or a user with the
    same shared secret
-   A recipe can load encrypted data as long as the shared secret is
    present in a file on the node or is accessible from a URI path
-   Only the values of a data bag item are decrypted; keys are still
    searchable. The values associated with the `id` key of a data bag
    item are not encrypted (because they are needed by Chef Infra Client
    when tracking the data bag item)

For version 2 (available, starting with Chef Client 11.6):

-   Same as version 1
-   Can disable version 0 and version 1 data bag item encryption formats
-   Adds Encrypt-then-MAC(EtM) protection