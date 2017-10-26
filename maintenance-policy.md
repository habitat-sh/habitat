
# Maintenance Policy

The Maintenance Policy defines how we make decisions about what happens with
Habitat and associated software projects. It provides the process by which:

* Patches are merged
* Disputes are resolved

It is intended to be short, flexible, and clear.

This file is related to the [MAINTAINERS
file](https://github.com/habitat-sh/habitat/blob/master/MAINTAINERS.md).

# How the project is maintained

This file is the canonical source for how the Habitat project is maintained.

# Roles

## Project Lead

* Resolves disputes
* Provides vision and roadmap
* Has universal veto power
* There can be only one

## Lieutenant

* Each component in the project may have at most one Lieutenant
* Provides guidance on future direction for their component
* Resolves disputes within their component
* Has localized veto power
* Plus all the responsibilities of a Maintainer

## Maintainer

* Each component may have multiple Maintainers
* Handles contributions on GitHub - first response to a PR within 48 hours
* Is available on [Slack](http://habitat-sh.slack.com/)
* Is available to answer mailing list questions within 48 hours
* Weekends and local holidays in the Maintainer’s jurisdiction are not counted
  for timeliness requirements. Absences for reasonable causes such as vacations,
  illness, etc. are also acceptable; Maintainers should notice of absences via
  slack list whenever possible.
* Committed to 100% tests passing for your component
* Has full commit/merge access to the relevant repositories

# Contributing Patches

## How a patch gets merged

* Open a Developer Certificate of Origin-signed (DCO-signed) Pull Request
  (anyone)
* Code reviewed by a Maintainer, Lieutenant, or Project Lead. Approval is
  indicated by :+1: on the pull request.
* Merged after :+1: vote or `r+` by at least one Maintainer for the component(s)
  affected by your patch.  The merge is initiated by an `r+` from an approved
  maintainer.

### Pull Request Review and Merge Automation

Habitat uses several bots to automate the review and merging of pull requests.
Messages to and from the bots are brokered via the account @thesentinels. First,
we use [Facebook's mention bot](https://github.com/facebook/mention-bot) to
identify potential reviewers for a pull request based on the blame information
in the relevant diff. @thesentinels can also receive incoming commands from
reviewers to approve PRs. These commands are routed to a [homu
bot](https://github.com/barosl/homu) that will automatically merge a PR when
sufficient reviewers have provided a +1 (or `r+` in homu terminology).

Any Maintainer may vote :-1: on a patch, which increases the requirement for a
patch to be merged to an absolute majority of Maintainers for the affected
component(s), unless that Maintainer later changes their vote.

## Patch Appeals Process

There may be cases where someone wishes to appeal a Maintainer decision. In this
event, the "chain of command" for the appeals process is as follows.

* In the event that the actions of a Maintainer are to be appealed, the appeal
  should be directed to the Lieutenant for that component. As stated above, a
  Lieutenant retains veto power for the component(s) for which they are
  responsible.

* In the event that the actions of a Lieutenant are to be appealed, the appeal
  should be directed to the Project Lead. As stated above, the Project Lead
  retains universal veto power over all components.

Although Lieutenants and the Project Lead retain veto powers over certain
components, use of this veto power is not guaranteed by the submission of an
appeal to that person. It is expected that the majority decisions of component
Maintainers and Lieutenants will be respected in all but the most exceptional
circumstances.

# How to become a...

## Maintainer

* Have patches merged into the relevant component
* Be willing to perform the duties of a Maintainer
* Issue a pull request adding yourself to the MAINTAINERS file for your
  component
* Receive an absolute majority of existing Maintainers and Lieutenants for your
  component via :+1:s on the pull request
* No veto from the component Lieutenant
* No veto from the current Project Lead

## Lieutenant

* Issue a pull request to the MAINTAINERS file making yourself the Lieutenant
* Be willing to perform the duties of a Lieutenant
* Receive an absolute majority of existing Lieutenants via :+1:s on the pull
  request
* No veto from the current Project Lead

## Project Lead

* Issue a pull request to the MAINTAINERS file making yourself the Project Lead
* Be willing to perform the duties of the Project Lead
* Receive an absolute majority of existing Lieutenants via :+1:s on the pull
  request
* No veto from Chef Software, Inc., as held by their current Chief Executive
  Officer.

# Removing a Maintainer, Lieutenant or Project Lead

If a Maintainer, Lieutenant, or Project Lead consistently fails to maintain
their responsibilities or becomes disruptive, they can be removed by:

* Issue a pull request removing them from the MAINTAINERS file
* Receive an absolute majority of existing Lieutenants via :+1:s on the pull
  request
* No veto from the current Project Lead

OR

* Issue a pull request removing them from the MAINTAINERS file
* The current Project Lead unilaterally decides to merge pull request

# How to add a component

* Issue a pull request to the MAINTAINERS file describing the component, and
  making yourself Lieutenant
* Be willing to perform the duties of a Lieutenant
* Receive an absolute majority of existing Lieutenants via :+1:s on the pull
  request
* No veto from the current Project Lead

# How to change the rules by which the project is maintained

* Issue a pull request to this file.
* Receive an absolute majority of existing Lieutenants from the Habitat
  repository MAINTAINERS file via :+1:s on the pull request
* No veto from the current Project Lead

# The MAINTAINERS file in Habitat

The current
[MAINTAINERS](https://github.com/habitat-sh/habitat/blob/master/MAINTAINERS.md)
file resides in the [habitat](https://github.com/habitat-sh/habitat/) repository
on GitHub.
