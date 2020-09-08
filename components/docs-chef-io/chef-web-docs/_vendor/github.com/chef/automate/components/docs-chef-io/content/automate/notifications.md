+++
title = "Notifications"

date = 2018-05-18T13:19:02-07:00
draft = false
[menu]
  [menu.automate]
    title = "Notifications"
    parent = "automate/settings"
    identifier = "automate/settings/notifications.md Notifications"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/notifications.md)

## About Notifications

Chef Automate notifications uses webhooks to send alerts for failures of Chef Infra Client runs and InSpec compliance scans. You can send notifications to Slack channels, Slack users, or any other service that supports webhook integrations. Notifications are sent for all failures. To ignore a failure, set the `ignore_failure` attribute to `true` on the resource. 

By default only Admins of Chef Automate may create and manage notifications.

{{< warning >}}
Notifications sent by Chef Automate do not support retries; therefore, notifications sent while Slack or the external service receiving the notification is experiencing API issues, outages, or some other unplanned downtime may never be received. Undelivered notifications are not re-sent. Attempts to send notifications do generate log messages in your Chef Automate server.
{{< /warning >}}

## Slack Notifications

### Add a Slack Notification

To add a Slack notification for Chef Automate:

![Notifications Navigation](/images/automate/notifications-navigation.png)

1. In the **Settings** tab, navigate to the _Notifications_ page in the sidebar.
1. Select **Create Notification**.
1. Select **Slack**.
1. Enter a unique notification name.
1. Select the failure type to be notified on from the drop-down menu. Current options are Chef Infra Client run or InSpec scan
1. Get your Slack webhook address by using the **What's this?** link, which opens an external Slack site.
1. On the Slack page, select a channel or user for the notification. Slack will create the new webhook and then provide a webhook URL for you to copy. After entering a recipient, use the **Add Incoming WebHooks Integration** button.
1. Copy the URL, return to the Chef Automate page, paste the URL into the _Notifications_ form.
1. Use the **Send a test** button to try out your Slack notification. If your Slack notification does not appear, return to the Slack Webhooks Integration page to re-check the recipient and URL.
1. Use the **Save Notification** button to create the Slack notification.

### Edit Slack Notifications

To edit a Slack notification for Chef Automate:

1. From the _Notifications_ page, select the notification name to open its detail page.
1. Edit the notification type, name, failure type, or URL.
1. Use the **Save Notification** button to save the Slack notification.

### Delete Slack Notifications

To delete a Slack notification for Chef Automate:

1. From the Notifications page, select **Delete** from the menu at the end of the table row.
1. Confirm that you wish to permanently delete this notification.

## Webhook Notifications

### Add a Webhook Notification

To add a webhook notification for Chef Automate:

![Notifications Navigation](/images/automate/notifications-navigation.png)

1. In the **Settings** tab navigate to the _Notifications_ page in the sidebar.
1. Select **Create Notification**.
1. Select **Webhooks**.
1. Enter a unique notification name.
1. Select the failure type to be notified on from the drop-down menu. Current options are Chef Infra Client run or InSpec scan
1. Enter the webhook URL the notification should be sent to.
1. Use the **Send Test** button to try out your webhook notification.
1. Use the **Save Notification** button to create the webhook notification.

### Edit Webhook Notifications

To edit a webhook notification for Chef Automate:

1. From the _Notifications_ page, select the notification name to open its detail page.
1. Edit the notification type, name, failure type, or URL.
1. Use the **Save Notification** button to save the webhook notification.

### Delete Webhook Notifications

To delete a webhook notification for Chef Automate:

1. From the Notifications page, select **Delete** from the menu at the end of the table row.
1. Confirm that you wish to permanently delete this notification in the helper screen.

### Webhook Notification Payload

The body of the notification payload contains complex JSON data.
One attribute to note is `type`.
Chef Infra Client run failures have a `type` of `converge_failure` where the Chef InSpec scan failures have a `type` of `compliance_failure`.

The `controls` section of the Chef InSpec scan failures notification payload contains information about each failed control, so the payload can be rather large if the node has a lot of failed controls.

#### Chef Infra Client Run Failure Example Payload

``` json
{
  "type": "converge_failure",
  "start_time_utc": "2020-05-28T18:43:33.000000Z",
  "node_name": "winnode1",
  "failure_snippet": "Chef client run failure on [localhost] winnode1 : https://automate.example.com/infrastructure/client-runs/e9c01d6d46543c19fba3b7bfc17a22d/runs/advff4a3-7890-4b74-98c1-fb00ae6dd251\nError executing action `run` on resource 'execute[name]'\nexecute[name] (cookbook::default line 10) had an error: Mixlib::ShellOut::ShellCommandFailed: Expected process to exit with [0], but received '1'\n---- Begin output of command ----\nSTDOUT: \nSTDERR: 'command' is not recognized as an internal or external command,\r\noperable program or batch file.\n---- End output of command ----\nRan command returned 1 \n",
  "exception_title": "Error executing action `run` on resource 'execute[name]'",
  "exception_message": "execute[name] (cookbook::default line 10) had an error: Mixlib::ShellOut::ShellCommandFailed: Expected process to exit with [0], but received '1'\n---- Begin output of command ----\nSTDOUT: \nSTDERR: 'command' is not recognized as an internal or external command,\r\noperable program or batch file.\n---- End output of command ----\nRan command returned 1",
  "exception_backtrace": [
    "C:/hab/pkgs/chef/chef-infra-client/15.6.10/20191210013214/vendor/gems/mixlib-shellout-3.0.7-universal-mingw32/lib/mixlib/shellout.rb:300:in `invalid!'",
    "C:/hab/pkgs/chef/chef-infra-client/15.6.10/20191210013214/vendor/gems/mixlib-shellout-3.0.7-universal-mingw32/lib/mixlib/shellout.rb:287:in `error!'",
    .
    .
    .
    "C:/hab/pkgs/chef/chef-infra-client/15.6.10/20191210013214/bin/chef-client:172:in `load'",
    "C:/hab/pkgs/chef/chef-infra-client/15.6.10/20191210013214/bin/chef-client:172:in `<main>'"
  ],
  "end_time_utc": "2020-05-28T18:43:34.000000Z",
  "automate_fqdn": "automate.example.com",
  "automate_failure_url": "https://automate.example.com/infrastructure/client-runs/e9c01d6d46543c19fba3b7bfc17a22d/runs/advff4a3-7890-4b74-98c1-fb00ae6dd251"
}
```

#### Chef InSpec Scan Failure Example Payload

``` json
{
    "type": "compliance_failure",
    "total_number_of_tests": 436,
    "total_number_of_skipped_tests": 142,
    "total_number_of_passed_tests": 79,
    "total_number_of_failed_tests": 215,
    "number_of_failed_critical_tests": 212,
    "number_of_critical_tests": 372,
    "node_uuid": "e9c01d6d46543c19fba3b7bfc17a22d",
    "node_name": "winnode1",
    "inspec_version": "4.18.97",
    "failure_snippet": "InSpec found a critical control failure on [winnode1](https://automate.example.com/compliance/reporting/nodes/e9c01d6d46543c19fba3b7bfc17a22d)",
    "failed_critical_profiles": [
        {
            "version": "1.0.1",
            "title": "Windows Audit",
            "supports": [],
            "summary": "Windows Audit Baseline",
            "sha256": "2e2a41ee574b4ffbe9e190de85e01c57a49b35f047d60bb0e541e95d776fd30e",
            "number_of_controls": 436,
            "name": "windows-audit",
            "maintainer": "user@example.com",
            "license": "Apache-2.0",
            "copyright_email": "user@example.com",
            "copyright": "Code Owners",
            "controls": [
                {
                    "title": "All important updates are installed",
                    "status": "failed",
                    "source_location": {
                        "ref": "C:/Windows/System32/config/systemprofile/.inspec/cache/f4167795b8659a357e0991a731b3a08321e36f1b29dc2c8ac90e2035ea9db9b6/windows-patch-baseline-0.4.0/controls/patches.rb",
                        "line": 28
                    },
                    "results": [
                        {
                            "status": "failed",
                            "start_time": "2020-05-28T19:02:53+00:00",
                            "skip_message": "",
                            "run_time": 0.00010469999688211828,
                            "message": "expected that `Windows Update 'Windows Malicious Software Removal Tool x64 - v5.82 (KB890830)'` is installed",
                            "code_desc": "Windows Update 'Windows Malicious Software Removal Tool x64 - v5.82 (KB890830)' is expected to be installed"
                        },
                        {
                            "status": "failed",
                            "start_time": "2020-05-28T19:02:53+00:00",
                            "skip_message": "",
                            "run_time": 0.00013299999409355223,
                            "message": "expected that `Windows Update '2020-02 Security Update for Adobe Flash Player for Windows Server 2019 for x64-based Systems (KB4537759)'` is installed",
                            "code_desc": "Windows Update '2020-02 Security Update for Adobe Flash Player for Windows Server 2019 for x64-based Systems (KB4537759)' is expected to be installed"
                        }
                    ],
                    "refs": [],
                    "number_of_tests": 2,
                    "number_of_failed_tests": 2,
                    "impact": 1,
                    "id": "important-patches",
                    "desc": "",
                    "code": "control 'important-patches' do\n  impact 1.0\n  title 'All important updates are installed'\n  win_update.important.each { |update|\n    describe update do\n      it { should be_installed }\n    end\n  }\nend\n"
                }
            ],
            "attributes": []
        }
    ],
    "automate_fqdn": "automate.example.com",
    "automate_failure_url": "https://automate.example.com/compliance/reporting/nodes/e9c01d6d46543c19fba3b7bfc17a22d"
}
```
