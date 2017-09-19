---
title: Running packages with security
---

# Supervisor Security

By default, a supervisor will run with no security. It will communicate with other supervisors in cleartext, and it will allow any user to apply new configuration without authentication. While this is beneficial for quickly illustrating the concepts of Habitat, users will want to run production deployments of Habitat supervisor networks with more security.

There are several types of security measures that can be undertaken by the operator:

* Wire encryption of inter-supervisor traffic
* Trust relationships between supervisors and users

## Wire Encryption

Supervisors running in a ring can be configured to encrypt all traffic between them. This is accomplished by generating a _ring key_, which is a symmetric shared secret placed into the supervisor environment prior to starting it.

### Generating a Ring Key

1. Generate a ring key using the `hab` command-line tool. This can be done on your workstation. The generated key has the `.sym.key` extension, indicating that it is a symmetric pre-shared key, and is stored in the `$HOME/.hab/cache/keys` directory.

    ```
    $ hab ring key generate yourringname
    ```

2. Copy the key file to the environment where the supervisor will run, into the `/hab/cache/keys` directory. Ensure that it has the appropriate permissions so only the supervisor can read it.
3. Start the supervisor with the `-r` or `--ring` parameter, specifying the name of the ring key to use.

    ```
    $ hab start --ring yourringname yourorigin/yourapp
    ```

    or, if using a standalone supervisor package:

    ```
    $ hab-sup start --ring yourringname yourorigin/yourapp
    ```

4. The supervisor becomes part of the named ring `yourringname` and uses the key for network encryption. Other supervisors that now attempt to connect to it without presenting the correct ring key will be rejected.
5. It is also possible to set the environment variable `HAB_RING_KEY` to the contents of the ring key; for example:

    ```
    $ env HAB_RING_KEY=$(cat /hab/cache/keys/ring-key-file) hab-sup start yourorigin/yourapp
    ```

### Using a Ring Key When Applying Configuration Changes

Users utilizing `hab config apply` or `hab file upload` will also need to supply the name of the ring key with the `-r` or `--ring` parameter, or supervisors will reject this communication.

## Service Group Encryption

Supervisors in a service group can be configured to require key-based authorization prior to allowing configuration changes. In this scenario, the supervisor in a named service group starts up with a key for that group bound to an _organization_. This allows for multiple service groups with the same name in different organizations.

As explained in the [security overview](/docs/internals-crypto), this process also requires the generation of a user key for every user making configuration updates to the supervisor network.

### Generating Service Group Keys

1. Generate a service group key using the `hab` command-line tool. This can be done on your workstation. Because asymmetric encryption is being used, two files will be generated: a file with a `.box.key` extension, which is the service group's private key, and a file with a `.pub` extension, which is the service group's public key.

    ```
    $ hab svc key generate servicegroupname.example yourorg
    ```

2. This generated a service group key for the service group `servicegroupname.example` in the organization `yourorg`. Copy the `.box.key` private key to the environment where the supervisor will run into the `/hab/cache/keys` directory. Ensure that it has the appropriate permissions so that only the supervisor can read it.
3. Start the supervisor, specifying both the service group and organization that it belongs to:

    ```
    $ hab start --org yourorg --group servicegroupname.example yourorigin/yourapp
    ```
    
4. Only users whose public keys that the supervisor already has in its cache will be allowed to reconfigure this service group. If you need to generate a user key pair, see the next section.

### Generating User Keys

The user key is used to encrypt configuration data targeted for a particular service group.

1. Generate a user key using the `hab` command-line tool. This can be done on your workstation. Because asymmetric encryption is being used, two files will be generated: a file with a `.box.key` extension, which is the user's private key, and a file with a `.pub` extension, which is the user's public key.
2. Distribute the user's public key to any supervisor that needs it, into the `/hab/cache/keys` directory. The user will be able to reconfigure that supervisor, provided they encrypted the configuration update using the service group's public key.

### Applying Configuration Changes

The `hab config apply` and `hab file upload` commands will work as usual when user/service group trust relationships are set up in this way.

If a running supervisor cannot decrypt a secret due to a missing key, it will retry with exponential backoff starting with a one-second interval. This allows an administrator to provide the supervisor with the key to resume normal operations, without taking down the supervisor.

## Identifying Key Types

To aid the user in the visual identification of the many varieties of keys in use by Habitat, a key itself is in plain text and contains a header on the first line indicating what kind of key it is. The file extension and, in some situations, the format of the file name, provide additional guidance to the user in identifying the type of key.

`YYYYMMDDRRRRRR` denotes the creation time and release identifier of that key.

| Key Type | Header | Filename Format |
|---|---|---|
| Private origin signing key | SIG-SEC-1 | originname-YYYYMMDDRRRRRR.sig.key |
| Public origin signing key | SIG-PUB-1 | originname-YYYYMMDDRRRRRR.pub.key |
| Ring wire encryption key | SYM-SEC-1 | ringname-YYYYMMDDRRRRRR.sym.key |
| Private service group key | BOX-SEC-1 | servicegroup.env@org-YYYYMMDDRRRRRR.box.key |
| Public service group key | BOX-PUB-1 | servicegroup.env@org-YYYYMMDDRRRRRR.pub |
| Private user key | BOX-SEC-1 | username-YYYYMMDDRRRRRR.box.key |
| Public user key | BOX-PUB-1 | username-YYYYMMDDRRRRRR.pub |

Keys that contain `SEC` in their header should be guarded carefully. Keys that contain `PUB` in their header can be distributed freely with no risk of information compromise.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-binding">Binding</a></li>
</ul>
