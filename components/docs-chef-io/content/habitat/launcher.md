+++
title = "Launcher"
description = "Launcher"

[menu]
  [menu.habitat]
    title = "Launcher"
    identifier = "habitat/supervisors/launcher"
    parent = "habitat/supervisors"

+++

Chef Habitat's Launcher is a sidecar process for the Supervisor which provides a mechanism for launching processes on behalf of the Supervisor. It is the entry point for running the Supervisor and is the Supervisor for the Supervisor. Whereas the Supervisor is able to automatically update itself, the Launcher is currently released a bit differently, by design; it should be rare that the Launcher ever needs to change.

To update your Launchers, run:

```bash
hab pkg install core/hab-launcher
```

Then restart the Supervisor. This will, by necessity, require a restart of supervised services, so factor that into your planning.

The Launcher is designed to run as process 1; it is minimal by design. Its responsibilities are simply to be the parent process for the Supervisor.

The Launcher enables the Supervisor to update itself without shutting down or re-parenting the services that the Supervisor is supervising. The Launcher is versioned separately from the Supervisor and should be updated very infrequently since an update of the Launcher could require a system restart if it is running as process 1.
""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''""""""""'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
