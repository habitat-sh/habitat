# Supervisor

The supervisor is a process manager that has two primary responsibilities. First, it starts and monitors the child app service defined in the package payload. Second, it receives and acts upon configuration changes from the rest of the Habitat cluster. A service will be reconfigured through hooks if its configuration has changed.
