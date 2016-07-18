---
title: Upload files to a service group
---

# Upload files to a service group
In addition to [configuration updates](/docs/run-packages-apply-config-updates), you can upload files to a service group. The file upload process is very similar to applying configuration updates; however, there is one important difference: You must use [service group encryption](/docs/run-packages-security#service-group-encryption) to upload files to a service group. This requires creating service keys and user keys. File uploads cannot be in the clear.

When uploading a file, the encryption operation requires that the service public key and the user secret key be in the `$HOME/.hab/cache/keys` directory if uploading from the host machine and `/hab/cache/keys` if uploading from the studio environment, unless those paths are overridden using the `HAB_CACHE_KEY_PATH` environment variable.

To decrypt the file, the supervisor needs the service secret key and the user public key to be in the `hab/cache/keys` directory accessible by the service's supervisor.

If a supervisor receives an encrypted file through a gossip rumor but is not the intended recipient, the file remains encrypted in memory. If the supervisor is the recipient and has the service secret key and the user public key, then the decrypted file will be written out to disk in the running service's directory (for example, /hab/svc/redis/files/foo). If a service is the recipient but does not have the appropriate keys to decrypt, the operation will be retried with an exponential backoff starting at one second, then doubling the time between retries (two seconds, four seconds, and so on). A max retry limit has not been set yet.

> Note: Both the secret and public keys have their permissions set to allow the owners to read only (0400). Also, subdirectories on uploads are not supported. All files are uploaded to `/hab/svc/servicename/files`, and are owned by `$pkg_svc_user:$pkg_svc_group` if specified in a plan. Otherwise, the user and group defaults to `hab:hab`. Uploaded file permissions are set to `0770`.

## Usage
Follow this basic flow to upload files into a service group.

When running on a Mac OSX machine, because service and user keys generated with the native `hab` CLI tool cannot be imported into the studio, you can only do this through the studio or by exporting your services to an external format, such as a Docker or ACI container, at this time. Also, the studio environment is primarily meant for building packages. Using the `hab file upload` functionality in the studio should be for testing purposes only.

1. Run `hab user key generate myname` specifying your user name.
2. Run `hab service key generate myapp.default myorg` specifying your organization.
3. Start up your services in multiple windows or by [using the director](/docs/run-packages-director). They must specify a group and the same organization specified in step two.

       hab start myorigin/myapp --group test --org myorg
       hab start myorigin/myapp --peer IPaddress --group test --org myorg

4. Place the service secret key and user public key into the `/hab/cache/keys` directories of those services. If exporting these services and starting them from within a container, you will have to copy the keys in after the container starts running.
5. Run `hab file upload` from a different window or container, passing in your org, a peer to connect to, service group, file, version, and username. Like configuration updates, the version number must be an integer, start with one, and increment any time you want to update the same file to the same service group.

       hab file upload --org myorg --peer 172.17.0.2 myapp.test test.txt 1 myname

    > Note: You must make sure all supervisors for your services are accessible by the `hab` CLI. Also, the maximum file size that you can upload is 4096 bytes (4k).

   If successful, you should see output similar to the following from where you ran the `hab` CLI:

       » Uploading file test.txt
       ↑ Uploading status.json for myapp.test@myorg into ring via ["172.17.0.2:9634"]
       Joining peer: 172.17.0.2:9634
       Configuration applied to: 172.17.0.2:9634
       ★ Upload of test.txt complete.

     The output text will update as each peer in the service group receives the file.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-security">Security</a></li>
</ul>
