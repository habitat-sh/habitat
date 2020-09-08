+++
title = "Monitoring Chef Automate"

date = 2019-10-28T14:44:28-07:00
draft = false
[menu]
  [menu.automate]
    title = "Monitoring Chef Automate"
    parent = "automate/reference"
    identifier = "automate/reference/monitoring.md Monitoring Chef Automate"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/monitoring.md)

## Overview

The `/status` endpoint is an authenticated API endpoint for users who want to monitor their Automate installation by connecting to an http endpoint. Previously, users ran the `chef-automate status` command from the Automate node itself to monitor an Automate installation.

## Checking the status endpoint

The authenticated endpoint `/status` provides status for the overall Chef Automate installation as well as its component services. 
When all Chef Automate component services are up, `/status` returns a response code of 200. Otherwise, `/status` returns 500.

To use `/status`, set up an authentication token that can be used with your monitoring system by following the steps below:

1. Generate a token:

    ```bash
    chef-automate iam token create --id <token-id> <token-name>
    ```

2. Create a policy that allows your created token to access the `/status` endpoint.

    ```bash
    curl -k -H "api-token: <admin-token>" -d '{ "name": "Monitoring", "id": "monitoring", "members": [ "token:<token-id>" ], "statements": [ { "effect": "ALLOW", "actions": [ "system:status:get" ], "projects": [ "*" ] } ] }' -X POST https://automate.example.com/apis/iam/v2/policies?pretty
    ```

3. Test that your token and policy give you access to the `/status` endpoint by running the following command:
    ```bash
    curl -k -H "api-token: <token-id>" https://automate.example.com/api/v0/status?pretty
    ```
The output appears in the following JSON format:

```json
    {
      "ok": true,
      "services": [
        {
          "name": "deployment-service",
          "status": "OK"
        },
        {
          "name": "backup-gateway",
          "status": "OK"
        },
        {
          "name": "automate-postgresql",
          "status": "OK"
        },
        ...
      ]
    }
```
After establishing your authentication token and confirming access, connect to the `/status` endpoint.
