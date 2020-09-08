+++
title = "Integrate Workflow with Slack"
draft = false
robots = "noindex"


aliases = ["/integrate_delivery_slack.html", "/release/automate/integrate_delivery_slack.html"]

[menu]
  [menu.legacy]
    title = "Workflow w/Slack"
    identifier = "legacy/workflow/managing_workflow/integrate_delivery_slack.md Workflow w/Slack"
    parent = "legacy/workflow/managing_workflow"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/integrate_delivery_slack.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Workflow may be configured to allow projects to send Slack notifications
to users about changes submitted to that project's pipeline, such as:

-   A change passed verification
-   A change was approved by a teammate
-   A comment was added to the change request
-   A change was accepted
-   A change was delivered
-   A change failed at a specific stage in the pipeline

Integrating Workflow with Slack requires a webhook to be created in
Slack, and then saving that webhook in Workflow. A webhook may be
specified at two levels:

-   By organization. A webhook that is specified at the organization
    level enables Slack notifications for all projects that belong to
    that organization.
-   By project. A webhook that is specified at the project level enables
    Slack notifications only for that project.

{{< note >}}

Notifications sent to Slack by Workflow do not support retries.
Therefore, notifications sent while Slack is experiencing API issues,
outages, or some other unplanned downtime may never be received by the
channel. Undelivered notifications are not re-sent.

{{< /note >}}

## Create a Webhook

To create a webhook in Slack:

1.  [Create an incoming
    webhook](https://slack.com/apps/A0F7XDUAZ-incoming-webhooks) in
    Slack and be sure to use the team in Slack to be associated with
    Workflow.
2.  Select a team, and then click the **Configure** button.
3.  Select **Add Configuration** (if that team already has at least one
    webhook) or **Install** to add a webhook.
4.  Under **Post to Channel** select the channel in Slack to which
    Workflow will send notifications.
5.  Click **Add Incoming Webhooks Integration**. Slack will create
    webhook, and then provide a location from which the URL for that
    webhook can be copied.
6.  Copy the URL.

## Add a Webhook to Workflow

To add a Slack webhook for Workflow:

1.  On the Workflow server, select **Organizations**.
2.  Select an organization or a project.
3.  Click **Edit** to open the details for that organization or project.
4.  Pick a meaningful name for the webhook, and then paste the webhook
    URL.
5.  Click **Send a Test**. If a test notification is successful, click
    **Save**.

## Disable Slack Notifications

Slack notifications are enabled by default, but they may be disabled.

1.  On the Workflow server, select **Organizations**.
2.  Select an organization or a project.
3.  Click **Edit** to open the details for that organization or project.
4.  De-select the **Enabled** checkbox, and then click **Save**.

## Delete Slack Notifications

Slack notifications are enabled by default, but they may be disabled.

1.  On the Workflow server, select **Organizations**.
2.  Select an organization or a project.
3.  Click **Edit** to open the details for that organization or project.
4.  Delete the URL for the Slack webhook and click **Save** or click the
    trash can button.
