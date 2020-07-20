# supplemental-group-tester

This is a silly testing package that can be used to ensure that the
Supervisor and Launcher are both passing supplementary group
information on to spawned hook processes on Linux.

This package has an `init` hook (run by the Supervisor) and a `run`
hook (run by the Launcher) that both try to read a single file
`/tmp/supplemental-group-tester-file`.

The package has service user and group of `hab`, as is common practice
with Habitat packages.

The testing file should not be directly readable by the `hab` user or
group, but should instead be readable by some other group. The `hab`
user should be made a member of that group. If all goes well, then
both the `init` and `run` hooks should be able to read this file,
allowing them to progress. A successful `init` hook run will allow the
`run` hook to begin, and a successful file check in the `run` hook
will then allow it to continue into an infinite sleep loop. Before
doing that, though, it will write the string `SUCCESS` to the file
`/tmp/supplemental-group-tester-file-sentinel`; if that file exists,
things started up properly.

```sh
echo "Hello World" > /tmp/supplemental-group-tester-file
sudo groupadd extra_group
chgrp extra_group /tmp/supplemental-group-tester-file
sudo chown 740 /tmp/supplemental-group-tester-file
sudo usermod -G extra_group hab
```
