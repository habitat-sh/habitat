+++
title = "Desktop Dashboard"

date = 2018-03-26T16:01:47-07:00
draft = false
[menu]
  [menu.automate]
    title = "Desktop Dashboard"
    parent = "automate"
    identifier = "automate/desktop.md Desktop Dashboard"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/desktop.md)

The Chef Automate _Desktop_ dashboard displays status information about all desktops connected to Chef Automate.
Desktop information populates this dashboard after a Chef Infra Client run has executed.

## Setting Up the Desktop Dashboard

Enable the Desktop dashboard with: `chef-automate deploy --product automate --product infra-server --product desktop`.
For more installation information, see [Install Chef Infra Server with Chef Automate](/automate/infra-server/).
The Desktop dashboard has no supported compliance profiles, and installation with the `--product desktop` flag includes no compliance profiles.

{{< note >}}
When installing Chef Automate with the `--product desktop` flag, _Data Lifecycle_ settings will not mark nodes as missing and not delete missing nodes by default.
We encourage users to not change these specific settings and not defeat the monitoring purpose of the Desktop dashboard.
{{< /note >}}

## Desktop Dashboard Display

Within Chef Automate, the _Desktop_ dashboard uses four panels to summarize information: _Daily Check-in_ _Check-in History_, _Top 10 Errors_, and _Time since Last Check-in_.
Selecting rows within these displays will list relevant desktops and selecting a singular desktop reveals its detailed information.

The _Desktop_ dashboard does not support project filtering.
Node counts in the _Desktop_ dashboard may include liveness agents.

### Daily Check-in

The _Daily Check-in_ display shows a top-level view of daily desktop check-in statistics.  
A bar graphic illustrates the proportion of desktops with `unknown` and `checked-in` statuses.
Below the bar, boxes display counts of all desktops, desktops with an `unknown` status, and desktops with a `checked-in` status.
`Checked-in` refers to desktops reporting into Chef Automate.
`Unknown` desktops did not report to Chef Automate during the last twenty-four hours.

Selecting a box will display a Filtered Desktop List of desktops for the selected status.
The selected filters will not persist when selecting a different box.

### Check-in History

_Check-in History_ shows the history of checked-in desktop counts as graphed over a selected period of time.
Use the drop-down menu to select a different period of time.
The time period options are "Last 3 Days," "Last 7 Days," and "Last 14 Days".

### Time Since Last Check-in

The _Time since Last Check-in_ display shows a count of desktops with an `unknown` status for defined timefames.
Selecting a row will reveal a Filtered Desktop List of desktops for the selected timeframe.

### Top 10 Errors

The _Top 10 Errors_ display shows the ten most common errors that have occurred across all desktops over the last twenty-four hours, and a count of the machines that experienced each error.
Selecting an error opens the Filter Desktop List with the applied error filter to enable further investigation.

### Filtered Desktop Lists

Selecting rows in _Daily Check-in_, _Time since Last Check-in_, or _Top 10 Errors_ activates the Filtered Desktop List.
The Filtered Desktop List displays a list of desktops filtered according to the selected status, timeframe, or error in the previous display.

Apply more filters to this initial list if desired.
Specific filter values populate according to available desktop information and include the categories of "platform," "environment," "domain," and "last run status."

Select an individual desktop row to display the individual desktop's node details.

#### Node Details

The _Node Details_ display shows the details of a single desktop and includes a visualization of its check-in history.

The individual check-in history displays the desktop's latest status for each day with an associated icon.
Hover over a status icon to see the status label.
Statuses include "converged" (check box icon), "unchanged" (dash icon), "error" (exclamation point icon), or "missing" (question mark icon).
The timescale of the check-in history can also switch between "Last 2 Weeks" and "Last 4 Weeks."

The individual desktop has an information panel that displays the following data: Overview, Chef Infra Client, System, Virtualization, and Kernel.

Select **Download** and choose the output format -- **JSON** or **CSV** -- to download the individual desktop's historical data.
