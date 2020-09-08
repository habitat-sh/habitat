+++
title = "Integrate Workflow with SMTP"
draft = false
robots = "noindex"


aliases = ["/integrate_delivery_smtp.html", "/release/automate/integrate_delivery_smtp.html"]

[menu]
  [menu.legacy]
    title = "Workflow w/Email (SMTP)"
    identifier = "legacy/workflow/managing_workflow/integrate_delivery_smtp.md Workflow w/Email (SMTP)"
    parent = "legacy/workflow/managing_workflow"
    weight = 80
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/integrate_delivery_smtp.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Workflow may be configured to allow projects to send email notifications
to users about changes submitted to that project's pipeline, such as:

-   A change passed verification
-   A change was approved by a teammate
-   A comment was added to the change request
-   A change was accepted
-   A change was delivered
-   A change failed at a specific stage in the pipeline

Users may subscribe to notifications per project by using the **Watch
Project** button located on each project's home page in the Workflow web
UI.

## Configure Notifications

To configure Workflow for SMTP notifications:

1.  As an enterprise administrator on the Workflow server, select
    **Admin**.

2.  Select the **Email Setup** tab in the web UI and complete the form:

    <img src="/images/delivery_integrate_smtp.svg" class="align-left" width="300" alt="image" />

3.  Enter the URL for the SMTP server. For example: `smtp.gmail.com`.

4.  Enter the port number for the SMTP server. Most connections use port
    `25` by default. This setting will default to `25` if not specified.

5.  Enter the SMTP server credentials---login and password---for the
    account from which mail is sent. For example: `steved`, `p@ssw0rd!`.

6.  Optional. Enter the name for the sender from which mail is sent. For
    example: `Steve Danno`.

7.  Click the **Send Test** button. This will send an email to your
    email address, as specified in Workflow. The Workflow web UI will
    update the page if the email was sent successfully.

## Subscribe to Notifications

Once an SMTP server is configured for Workflow, users that belong to
that enterprise may subscribe to notifications that are sent from any
project that exists in the same enterprise.

To subscribe to SMTP notifications:

1.  Navigate to a project in Workflow.

2.  Click the **Watch Project** button.

3.  From the dropdown, select the categories of events for which
    notifications should be sent.

    The **Review** category enables notifications related to submitting,
    reviewing, and receiving feedback on a change.

    The **Deliver** category enables notifications related to changes
    that have passed acceptance (and may be delivered) or to changes
    that have been delivered.

    The **Observe** category enables notifications related to the status
    of the pipeline as changes move through it.

## Unsubscribe from Notifications

A user may unsubscribe from notifications at any time.

1.  Navigate to a project in Workflow.
2.  Click the **Watch Project** button.
3.  From the dropdown, de-select the categories of events for which
    notifications should no longer be sent. De-select all categories to
    stop receiving all notifications.
