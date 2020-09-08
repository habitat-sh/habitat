+++
title = "Troubleshooting"

date = 2018-05-15T17:27:57-07:00
draft = false
[menu]
  [menu.automate]
    title = "Troubleshooting"
    parent = "automate"
    identifier = "automate/troubleshooting.md Troubleshooting"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/troubleshooting.md)

## chef-automate CLI Errors

### Error: Unable to make a request to the deployment-service

The `chef-automate` CLI emits this error when the CLI is unable to communicate with a Chef Automate deployment. In particular, when Chef Automate 2 (as distinct from Chef Automate 1) is not deployed, running `chef-automate` CLI commands such as `version` or `status` causes this error.

## File exists (os error 17)

It's possible for the following error to be emitted when deploying Chef Automate:

```bash
DeploymentServiceCallError: A request to the deployment-service failed: Request to configure deployment failed: rpc error: code = Unknown desc = failed to binlink command "chef-automate" in pkg "chef/automate-cli/0.1.0/20181212085335" - hab output: >> Binlinking chef-automate from chef/automate-cli/0.1.0/20181212085335 into /bin
xxx
xxx File exists (os error 17)
xxx
: exit status 1
```

This problem can be fixed by removing the `chef-automate` binary from the `/bin` directory. The binary
should not be placed in the PATH manually as the deployment process will do it.

## Low Disk Space

Chef Automate emits a warning when the available disk space on the system drops below 1 GB, for example:

```bash
es-sidecar-service.default(O): time="2018-05-16T00:07:16Z" level=error msg="Disk free below critical threshold" avail_bytes=43368448 host=127.0.0.1 mount="/ (overlay)" threshold_bytes=536870912 total_bytes=31361703936
```

### Recovering from Low Disk Conditions

Chef Automate disables disk writes if available disk space drops below 250 MB and logs a message similar to:

```bash
ingest-service.default(O): time="2018-05-16T00:10:09Z" level=error msg="Message failure" error="rpc error: code = Internal desc = elastic: Error 403 (Forbidden): blocked by: [FORBIDDEN/12/index read-only / allow delete (api)]; [type=cluster_block_exception] elastic: Error 403 (Forbidden): blocked by: [FORBIDDEN/12/index read-only / allow delete (api)]; [type=cluster_block_exception]"
```

After freeing up disk space, you will need to remove the write block on the Elasticsearch indices by running:

```bash
curl -X PUT "localhost:10141/_all/_settings" -H 'Content-Type: application/json' -d'
{
  "index.blocks.read_only_allow_delete": null
}
'
```

To confirm that you've successfully removed the blocks, run:

```bash
curl 'localhost:10141/_all/_settings'
```

Verify that the output does not contain `"blocks":{"read_only_allow_delete":"true"}`.

## Uninstalling Chef Automate

The following procedure will remove Chef Automate from your system,
including all data. If you wish to preserve the data, make a backup
before uninstalling.

With the `chef-automate` CLI:

```bash
chef-automate uninstall
```

## Resetting the Admin Password

Use the following command to completely reset a lost, forgotten, or compromised admin password:

```bash
chef-automate iam admin-access restore NEW_PASSWORD
```

This command causes Automate to inspect your A2 IAM resource and initiates a series of steps for applying the new password to the "admin" user--in effect reconstituting the admin--and connecting it with full administrative permissions.

The process Automate follows for resetting the admin password may include: recreating the user record, recreating the "admins" local team, and recreating the default policy that grants access to all resources for the newly reconstituted local admin team.

To see what exactly will happen in your system, pass `--dry-run`:

```bash
chef-automate iam admin-access restore NEW_PASSWORD --dry-run
```
