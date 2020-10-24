+++
title = "Service Group Configuration"
description = "Update Services with File Uploads"

[menu]
  [menu.habitat]
    title = "Service Group Configuration"
    identifier = "habitat/services/file-uploads"
    parent = "habitat/services"

+++

## Uploading files to a service group

In addition to [configuration updates](/using-habitat#config-updates), you can upload files to a service group. Keep these small - we recommend 4k or less per file, and keep the count of files to a minimum.

### Usage

When submitting a file to a service group, you must specify a peer in the ring to connect to, the version number of the file, and the new path to the file itself.

File updates for service groups must be versioned. The version number must be an integer that starts at one and must be incremented with every subsequent update to the same service group. *If the version number is less than or equal to the current version number, the change(s) will not be applied.*

Here are some examples of how to upload a file to the ring:

```bash
$ hab file upload myapp.prod 1 /tmp/yourfile.txt --remote-sup=hab1.mycompany.com
```

  > Note: 1 is the version number. Increment this for subsequent updates.

    Your output would look something like this:

       » Uploading file yourfile.txt to 1 incarnation myapp.prod
       Ω Creating service file
       ↑ Applying via peer 172.0.0.3:9632
       ★ Uploaded file

  The services in the `myapp.prod` service group will receive the file, restarting if necessary:

       hab-sup(MR): Receiving new version 1 of file secret.txt for myapp.prod
       ...
       myapp.prod(SR): Service file updated, yourfile.txt

  > Note: The file will be put in your service's `svc/files` directory.

### Encryption

Files can be encrypted for the service group they are intended. To do so, pass the `--user` option with the name of your user key, and the `--org` option with the organization of the service group. If you have the public key for the service group, the data will be encrypted for that key, signed with your user key, and sent to the ring.

It will then be stored encrypted in memory, and decrypted on disk.
