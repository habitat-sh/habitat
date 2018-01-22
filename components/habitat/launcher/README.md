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

Since the Supervisor requires the Launcher to start and it always attempts to start the latest version of the Supervisor packaged and installed on the host you may find it difficult to start your dev version of the Supervisor. You can force the Supervisor to start a specific version of the Supervisor by setting the `HAB_SUP_BINARY` environment variable to the file path of the desired Supervisor binary to start.
