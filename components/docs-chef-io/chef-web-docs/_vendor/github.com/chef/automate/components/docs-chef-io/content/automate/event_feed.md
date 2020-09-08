+++
title = "Event Feed"

date = 2018-03-26T16:01:47-07:00
draft = false
[menu]
  [menu.automate]
    title = "Event Feed"
    parent = "automate"
    identifier = "automate/event_feed.md Event Feed"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/event_feed.md)

Use the _Event Feed_ for actionable insights and operational visibility.
The _Event Feed_ shows the time of the event, its type, the object acted upon, the action, and the initiating action.
The Guitar Strings visualization helps you drill into infrastructure and compliance automation events and quickly isolate errors.

Users require permission for the `event:*` action to view and interact with _Event Feed_.
Filter and search ability in _Event Feed_ requires user permission for the `infra:nodes:list` action.

![Event feed](/images/automate/event-feed.png)

## Event Guitar Strings

The Event Guitar Strings is a timeline representing Chef Infra Server and Compliance events.
The events are separated into create, update, and delete.
Blue circles show create events, red circles show delete events, and purple diamonds show update events.
The icons inside of the shapes represent different types of events, and a multi-event icon denotes a case where multiple events happened within the same 4-hour window.
Hovering over the icon shows a text box summarizing the events for that 4-hour window.

## Icon Legend

![Event feed icon legend](/images/automate/event_icons.png)

## Grouped Events

The _Event Feed_ groups events of the same type by the same user.
The event list entry shows how many events are grouped into an individual entry, which is also a link.
Selecting this link opens a side panel showing the details of the collapsed events.

## Filtering Events

To filter the event feed and event timeline by Event Type, Chef Infra Server, or Chef Organization, use the search bar.
The search bar appears to users with permission for the `infra:nodes:list` action.
Available event type filters are clients, cookbooks, data bags, environments, nodes, policyfiles, profiles, roles, and scan jobs.

To filter, select Event Type, Chef Infra Server, or Chef Organization in the search bar and start typing the name.
You cannot filter compliance events -- profiles and scan jobs -- by organization or Chef Infra Server.
Compliance events are not visible when either of these filters are applied.

You can also filter the _Event Feed_ event timeline by day or by set of days within the past week.
The _Event Feed_ defaults to show all events in the past week.
Moving the indicators to the right or left will activate the filter.
You can move the indicators back to their start position to reset the time scale.
