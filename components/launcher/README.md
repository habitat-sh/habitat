# Launcher

Habitat's Launcher is a sidecar process for the Supervisor which provides a mechanism for launching processes on behalf of the Supervisor. It is the entry point for running the Supervisor and is the Supervisor for the Supervisor. It is designed to run as process 1; it's extremely minimal by design and contains as few dependencies and as little unsafe Rust code as possible. It's responsibilities are to:

* Launch the Habitat Supervisor
* Restart the Habitat Supervisor on shutdown (Supervisor for the Supervisor)
* Start, Stop, and Restart processes on behalf of the Supervisor

Launcher is specifically designed to enable the Supervisor to self update without shutting down or re-parenting the services that the Supervisor is supervising. It is versioned separately from the Supervisor and should be updated very infrequently since an update of the Launcher could require a system restart if it is running as process 1.

## How

The Supervisor is *always* started by the Launcher. Launcher will always attempt to start the latest version of the Supervisor package installed on the host. When the Launcher starts the Supervisor a handshake is made over a platform specific [IPC](https://en.wikipedia.org/wiki/Inter-process_communication) channel. The Supervisor communicates with the Launcher with a [binary protocol](../launcher-protocol) through the [Launcher Client](../launcher-client).

When the Supervisor needs to spawn a long lived process it sends a `Spawn` message to the Launcher and receives a message back with the PID of the running process or a failure message. The Supervisor will watch this process and ask the Launcher to stop or restart it as needed. If the Supervisor crashes or is restarted due to an automatic update it will reattach to all running services and continue operation normally.

## Dev Instructions

Since the launcher is updated infrequently relative to the rest of the habitat ecosystem, it has a separate build and release process. To build locally run
```
cargo build
```
from this directory. To make a build for release, run the `build` command from within the studio against this directory.

### Testing

Since the Supervisor requires the Launcher to start and it always attempts to start the latest version of the Supervisor packaged and installed on the host you may find it difficult to start your dev version of the Supervisor. You can force the Supervisor to start a specific version of the Supervisor by setting the `HAB_SUP_BINARY` environment variable to the file path of the desired Supervisor binary to start.

### Building Windows Release

The launcher may optionaly start a Windows process under an alternate identity declared via `svc_user`. Windows requires a password for this identity. We encrypt the identity when loading services and decrypt the password when the launcher spawns the new process. We ["complexify" this encryption with a secret key](https://docs.microsoft.com/en-us/windows/desktop/api/dpapi/nf-dpapi-cryptprotectdata). This key is stored in 1password and must be set in the `HAB_CRYPTO_KEY` environment variable when building `hab` or `launcher`. The `hab` build gets this from a secret appveyor key but the key must be manually set wnenever the launcher is built manually.

We have an appveyor build that will build and promote Windows launchers when code changes are merged into the `launcher` component. In order for a new launcher to get included in the Habitat Windows service and also built into the dockerized Windows Studio, one needs to invoke a new Appveyor build of the [windows-service](https://github.com/habitat-sh/windows-service) repo. This builds, uploads and promotes the `windows-service` Habitat package. The best way to do this is by using its rest API. First you will need a bearer token to authenticate with. You can get this by signing into the Appveyor site and then going to your [api-keys](https://ci.appveyor.com/api-keys) page. Choose "chef" from the accounts drop down menu and then copy your bearer token. With that, the following Powershell script will invoke the build:

```
$token = 'MY_BEARER_TOKEN'
$headers = @{
  "Authorization" = "Bearer $token"
  "Content-type" = "application/json"
}
$body = @"
{
    "accountName": "chef",
    "projectSlug": "windows-service",
    "branch": "master"
}
"@
curl -H  -Uri 'https://ci.appveyor.com/api/builds' -Headers $headers -Method Post -Body $body
```

In bash:

```
token=MY_BEARER_TOKEN
curl -H "Content-type: application/json" -H "Authorization: Bearer ${token}"  https://ci.appveyor.com/api/builds -X POST --data "{'accountName': 'chef','projectSlug': 'windows-service','branch': 'master'}"
```

You must invoke this build before starting a Windows release build.

### Building an x86_64-linux-kernel2 Release

If you are promoting launchers for x86_64-linux and x86_64-windows, you will also need to build a launcher for x86_64-linux-kernel2 by hand. You can do this by creating a CentOS-6 VM, either on your laptop or in a cloud instance. All further instructions are executed in the context of this VM. Once the VM is booted, you will then need to obtain an x86_64-linux-kernel2 hab binary, which can be downloaded by passing the -t argument to the install.sh script: `curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash -s -- -t x86_64-linux-kernel2`.  Then, you will need to obtain the core origin signing keys and [install them](https://www.habitat.sh/docs/habitat-cli/#hab-origin-key-import). Next, you will need to clone this repository. Once that is done, cd into the directory created and run `env HAB_ORIGIN=core hab pkg build components/launcher`.  The resulting hart file should be uploaded to builder to the `unstable` channel. Take note of the `$pkg_ident` for use in the pipeline.

### Releasing

To release a new version of the supervisor, upload the new `.hart` file and then [promote it to stable in builder](https://bldr.habitat.sh/#/pkgs/core/hab-launcher). If a new release contains important features or bug fixes, it will have to be communicated to the community and installed manually. Because of the nature of the launcher, this will require downtime in production environments, but it should be an exceedingly rare occurrence.
