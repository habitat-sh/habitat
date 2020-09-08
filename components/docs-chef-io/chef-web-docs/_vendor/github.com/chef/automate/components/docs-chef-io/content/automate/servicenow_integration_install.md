+++
title = "ServiceNow Integration"

draft = false

[menu]
  [menu.automate]
    title = "ServiceNow Integration"
    parent = "automate/reference"
    identifier = "automate/reference/servicenow_integration_install.md ServiceNow Integration"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/servicenow_integration_install.md)

## Overview

This guide helps you set up a ServiceNow instance that creates incidents from Chef Automate failure notifications on failed Chef Infra Client runs and Compliance scans.

### Chef Automate - ServiceNow Integration

The integration between a Chef Automate server and a ServiceNow instance requires the following:

* Chef Automate
* Chef Automate Incident Creation Application in ServiceNow

The Chef Automate Incident Creation application is a ServiceNow certified scoped application available from the ServiceNow store. [Chef Automate](https://www.chef.io/automate/) provides a full suite of enterprise capabilities for workflow, node visibility, and compliance. Once configured, the Chef Automate server sends HTTPS JSON notifications to the Chef Automate Incident Creation Application in a ServiceNow instance to create or update incidents.

![ServiceNow and Automate Flow](/images/automate/SNOW_Automate_diagram.png)

#### Prerequisites

* The ServiceNow instance must be publicly reachable on https port 443
* [Chef automate server installation](/docs/install/)
* ServiceNow package - System Import Sets com.glide.system_import_set, min version 1.0.0

## Configuration

### Install Chef Automate Incident Creation Application in ServiceNow

The Chef Automate Incident Creation application exposes the REST API endpoints that facilitate communication between Chef Automate and the ServiceNow instance.

* Visit the ServiceNow store at [https://store.servicenow.com](https://store.servicenow.com)
* Get the Chef Automate Incident Creation application
* In the ServiceNow instance, navigate to the System Applications > Applications menu
* From the Downloads tab, install the Chef Automate Incident Creation application

### Create Application Users

The application provides several roles appropriate for integration, which can be assigned to existing or new ServiceNow users. The roles are as follows:

* `x_chef_incident.admin`
* `x_chef_incident.user`
* `x_chef_incident.api`

For more information, see [Creating users in ServiceNow](https://docs.servicenow.com/bundle/kingston-platform-administration/page/administer/users-and-groups/task/t_CreateAUser.html) and [Assigning roles in ServiceNow](https://docs.servicenow.com/bundle/kingston-platform-administration/page/administer/users-and-groups/task/t_AssignARoleToAUser.html)

#### Role x_chef_incident.admin

This `x_chef_incident.admin` role can be assigned to a user other than a systems administrator to allow administration of the application properties and logs. Thus, administration can be carried out by a user who is not the system administrator. Note that a systems administrator can perform all tasks that a `x_chef_incident.admin` role can.

The admin role grants a user access to the:

* Chef incidents menu
* Client runs menu item
* InSpec scans menu item
* Properties menu item
* Support menu item
* Logs menu item

#### Role x_chef_incident.user

The role is suitable for users that require application access without administration access. The role grants a user access to the:

* Chef Incidents menu
* Chef Infra Client runs menu item
* InSpec scans menu item
* Support menu item

Client run and InSpec scan records are linked to incidents and it is appropriate to assign this role to users that manage incidents in ServiceNow.

#### Role x_chef_incident.api

The `x_chef_incident.api` role should be assigned to a user that is responsible for integrating the Chef Automate data into the application. Create a new user specifically for this task. Communication with the application requires this user's credentials.

#### Application Properties

The application properties can be configured by users with admin or `x_chef_incident.admin` roles. Select the Chef Incidents > Properties menu item to navigate to the properties configuration UI.

![ServiceNow Config Page](/images/automate/SNOW_config_page.png)

The application system properties are:

* `x_chef_incident.association`
* `x_chef_incident.scan_association`
* `x_chef_incident.assigned_to`
* `x_chef_incident.assignment_group`
* `x_chef_incident.impact`
* `x_chef_incident.urgency`
* `x_chef_incident.retention_days`
* `x_chef_incident.logging.enabled`
* `x_chef_incident.logging.verbosity`

#### Property x_chef_incident.association

The `x_chef_incident.association` property defines how a Chef Infra Client run record is associated with an Incident record. The possible values are:

* `cookbook` (default)
* `node`

Setting the value to `cookbook` creates an incident for the failed cookbook. All failing Chef Infra Client runs on nodes are associated with the corresponding incident. `cookbook` is the default value because the number of nodes is likely to exceed the number of cookbooks. The short description of the incident will indicate the failed cookbook:

![CCR Failed Cookbook Description](/images/automate/SNOW_Failed_Cookbook.png)

The associated client runs are shown on the 'Chef Infra Client runs' tab of the incident.

Setting the value to `node` creates an incident for each failed node. All failing Chef Infra Client runs for a node are associated with the corresponding incident. The short description of the incident will indicate the failed node:

![CCR Failed Node Description](/images/automate/SNOW_Failed_Node_CCR.png)

#### Property x_chef_incident.scan_association

The `x_chef_incident.scan_association` property defines how an InSpec scan record is associated with an Incident record. The possible values are:

* `profile` (default)
* `node`

Setting the value to `profile` creates an incident for the failed InSpec compliance profile. All InSpec scans on failing nodes will be associated with the corresponding incident. `profile` is the default value because the number of nodes is likely to exceed the number of profiles.

The short description of the incident indicates the failed profile:

![Scan Failed Profile Description](/images/automate/SNOW_Failed_Profile_Scan.png)

The associated InSpec scans are shown on the 'Chef InSpec scans' tab of the incident.

Setting the value to `node` creates an incident for each failed node. All InSpec scans failing for a node will be associated with the corresponding incident.

The short description of the incident indicates the failed node:

![Scan Failed Node Description](/images/automate/SNOW_Failed_Node_Scan.png)

The associated InSpec scans are shown on the 'Chef InSpec scans' tab of the incident.

#### Property x_chef_incident.assigned_to

The `x_chef_incident.assigned_to property` has no default value. It should be set to the ServiceNow user ID to assign incidents raised by the application to. If the user is in an assignment group, that group will also be assigned automatically to the incident. This property can be left blank and an assignment group used instead.

#### Property x_chef_incident.assignment_group

The `x_chef_incident.assignment_group` property has no default value. It can be set rather than an individual user in the `x_chef_incident.assigned_to` property to assign the incident to a group.

#### Property x_chef_incident.impact

The `x_chef_incident.impact` property is the value to set in an incident raised by the application. This should be set as either `1`, `2` or `3`.

#### Property x_chef_incident.urgency

The `x_chef_incident.urgency` property is the value to set in an incident raised by the application. This should be set as either `1`, `2` or `3`.

#### Property x_chef_incident.retention_days

The `x_chef_incident.retention_days` property is the number of days to retain Chef Infra Client run and Chef InSpec scan records in ServiceNow after the corresponding incident is closed or after the association with an incident is removed. The default is set to 30 days and Chef Infra Client run and InSpec scan records will remain in ServiceNow and be deleted after 30 days when the following conditions are met:

* The corresponding incident is closed
* The corresponding incident is deleted
* The incident has been removed from the Chef Infra Client run or InSpec scan record by a user update

#### Property x_chef_incident.logging.enabled

The `x_chef_incident.logging.enabled` property allows application logging to be enabled. Once enabled, logs can be viewed from the Chef incidents > Logs menu and the System logs > Application logs menu by authorized users. The default value is `no` and logs are disabled by default.

#### Property x_chef_incident.logging.verbosity

The `x_chef_incident.logging.verbosity` property sets the logging level when logging is enabled. The default value is `error` and the following levels are available:

* `debug`
* `warn`
* `info`
* `error`

### Configure Chef Automate

Chef Automate can be configured to send a notification to the REST APIs exposed by the application. There are two REST APIs:

* Chef Infra Client run API /api/x_chef_incident/client_run
* InSpec scan API /api/x_chef_incident/inspec_scan

#### Chef Infra Client Run API

The Chef Automate notifications configuration UI for client runs

![Automate Notification for CCR Failures](/images/automate/SNOW_CCR_Setup.png)

To report failed client runs on nodes managed by Chef Automate:

* Navigate to the notification settings page in the Chef Automate UI
* Select the 'Add Notification' button
* Select the ServiceNow notification type
* Assign a unique name for the notification
* Select 'Chef Infra Client run failures' as the failure type
* Enter the ServiceNow client run API address in the ServiceNow webhook field
* Enter the credentials of a user with the x_chef_incident.api role
* Save the notification

#### InSpec scan API

The Chef Automate notifications configuration UI for InSpec scans

![Automate Notification for InSpec Scan Failures](/images/automate/SNOW_Scan_Setup.png)

To report failed InSpec scans on node managed by Chef Automate:

* Navigate to the notification settings page in the Chef Automate UI
* Select the 'Add Notification' button
* Select the ServiceNow notification type
* Assign a unique name for the notification
* Select 'InSpec scan failures' as the failure type
* Enter the ServiceNow InSpec scan API address in the ServiceNow webhook field
* enter the credentials of a user with the x_chef_incident.api role
* Save the notification

# Uninstalling

To uninstall the application:

* In the ServiceNow instance, navigate to the System Applications > Applications menu
* From the Downloads tab, select the Chef Automate Incident Creation link
* In the Related Links section, select uninstall
