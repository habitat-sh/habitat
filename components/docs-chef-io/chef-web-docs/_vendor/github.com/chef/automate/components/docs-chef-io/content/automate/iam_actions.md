+++
title = "IAM Actions"

draft = false
[menu]
  [menu.automate]
    title = "IAM Actions"
    parent = "automate/authorization"
    identifier = "automate/authorization/iam_actions.md IAM Actions"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/iam_actions.md)

Reference the chart on this page when creating a *Role* to know which action grants access to what page in the browser.

*IAM Action* lists the associated action or actions required to access that page in the browser. 
Use `*` in these actions to give broad permissions to perform all associated actions such as get, list, create, delete, etc.
Specify the action to restrict user access to the specific action.

{{% responsive-table %}}
|  Task           | Browser Tab     | IAM Action       | API endpoint  | URL       |
| --------------- | --------------- | ---------------- | ------------- | --------- |
| View Events | Dashboards | event:* | /event_feed | https://{{< example_fqdn "automate" >}}/dashboards/event-feed |
| View and Search Events | Dashboards | [event:*, infra:nodes:list] | /event_feed | https://{{< example_fqdn "automate" >}}/dashboards/event-feed |
| View Service Group Data | Applications | applications:*  | /applications/service-groups | https://{{< example_fqdn "automate" >}}/applications/service-groups |
| View Client Runs | Infrastructure | infra:nodes:*   | /cfgmgmt/nodes | https://{{< example_fqdn "automate" >}}/infrastructure/client-runs |
| View Chef Servers | Infrastructure | infra:infraServers:* | /infra/servers | https://{{< example_fqdn "automate" >}}/infrastructure/chef-servers |
| List Reports | Compliance | compliance:reporting:*  | /compliance/reporting/reports | https://{{< example_fqdn "automate" >}}/compliance/reports/overview |
| List Scan Jobs | Compliance | compliance:scannerJobs:* | /compliance/scanner/jobs | https://{{< example_fqdn "automate" >}}/compliance/scan-jobs/jobs |
| Manage Scan Jobs | Compliance | [compliance:scannerJobs:* , infra:nodes:* , infra:nodeManagers:* , compliance:profiles:* ] | /compliance/scanner/jobs | https://{{< example_fqdn "automate" >}}/compliance/scan-jobs/jobs |
| Manage Compliance Profiles | Compliance | compliance:profiles:* | /compliance/profiles | https://{{< example_fqdn "automate" >}}/compliance/compliance-profiles |
| Manage Notifications | Settings | notifications:* | /notifications | https://{{< example_fqdn "automate" >}}/settings/notifications |
| Manage Data Feed | Settings | datafeed:* | /data_feed/destination | https://{{< example_fqdn "automate" >}}/settings/data-feed |
| Manage Node Integrations | Settings | [infra:nodeManagers:* , infra:nodes:* , secrets:* ] | /nodemanagers , /cfgmgmt/nodes , /secrets | https://{{< example_fqdn "automate" >}}/settings/node-integrations |
| Manage Node Credentials | Settings | secrets:* | /secrets | https://{{< example_fqdn "automate" >}}/settings/node-credentials |
| Manage Data Lifecycle | Settings | dataLifecycle:* | /data-lifecycle | https://{{< example_fqdn "automate" >}}/settings/data-lifecycle |
| Manage Users | Settings | iam:users:* | /iam/v2/users | https://{{< example_fqdn "automate" >}}/settings/users |
| Manage Teams | Settings | iam:teams:* | /iam/v2/teams | https://{{< example_fqdn "automate" >}}/settings/teams |
| Manage API Tokens | Settings | iam:tokens:* | /iam/v2/tokens | https://{{< example_fqdn "automate" >}}/settings/tokens |
| Manage Policies | Settings | iam:policies:* | /iam/v2/policies | https://{{< example_fqdn "automate" >}}/settings/policies |
| Manage Roles | Settings | iam:roles:* | /iam/v2/roles | https://{{< example_fqdn "automate" >}}/settings/roles |
| Manage Projects | Settings | iam:projects:* | /iam/v2/projects | https://{{< example_fqdn "automate" >}}/settings/projects |
{{% /responsive-table %}}
