---
title: Habitat.sh Governance and Issue Tracking
date: 2017-06-16
author: Ian Henry
tags: Governance,Community
category: Community
classes: body-article
---
Hopefully our previous [blog](https://www.habitat.sh/blog/2017/06/Triage-and-Contributions/) and [discourse](https://forums.habitat.sh/t/habitat-community-triage/287) posts have given you some general understanding around the way we govern the Habitat project. But, as a group of people hoping to share what we're building with a larger audience we know we can always be doing more to "pull back the curtain". We can be doing more to [expose our work](https://ext.prodpad.com/ext/roadmap/d2938aed0d0ad1dd62669583e108357efd53b3a6) and keep our community informed and educated on the ways we've chosen to operate.

Thus, I'm here to explain as much as I can about the way we currently govern the Habitat Project and how the Issue Tracker fits in. Generally this blog post is going to build on top of the Triage and Contribution blog post linked.

## Core Team
Habitat is governed by a core team, which is ultimately responsible for all decision-making in the project. Specifically, the core team alongside a Chef Inc. Product Owner:

* Sets the overall direction and vision for the project
* Sets the priorities and release schedule
* Makes final decisions
You can track the core maintainers in our [MAINTAINERS.md](https://github.com/habitat-sh/habitat/blob/master/MAINTAINERS.md) file at the root of the habitat-sh/habitat repository.

## Tag Categorization
We discussed this in [another blog post](https://www.habitat.sh/blog/2017/06/Triage-and-Contributions/) but it bares keeping in mind.

| *TAGS Groups*                     |               Meaning                             |
|:----------------------------------|:--------------------------------------------------|
| *AREA TAGS* Prefix "A-"           | What part of the codebase does the issue refer to.|
| *CATEGORIZATION TAGS* Prefix "C-" | What type of work does the issue refer to.        |
| *EFFORT TAGS* Prefix "E-"         | Should a contributor undertake, what is the estimated effort level. |
| *LANGUAGE TAGS* Prefix "L-"       | Pretty straightforward right? What language the work will involve.  |
| *STATUS TAGS* Prefix "S-"         | The current status of an issue or pull request.   |
| *PLATFORM TAGS* Prefix "P-"       | Issue is in regards to specific platform.         |

## The Triage Process
Currently any member of the Core-Maintainers group on github can triage an issue in any of the habitat-sh organization repositories. A maintainer may assign any tags they wish at any time they please. Typically when a core maintainer triages an issue they will tag it with _at least_ an "Area" tag and a "Category" tag (for a full list of tags and their definitions you can check out our [wiki entry](https://github.com/habitat-sh/habitat/wiki/Label-Names-and-Definitions) on the subject). Ideally we prefer the maintainer to also tag the issue with an "Effort" tag so as to maximize accessibility for external contributions. "Language" tags and "Status" tags are both fairly optional and exist again to lower the barrier to entry for new contributors. Since we're using Github projects for a majority of our prioritized issues, community members and users should be able to track the status of prioritized issues on [the project board](https://github.com/habitat-sh/habitat/projects/1). Basically, the process for applying these tags is at the discretion of the core maintainers. Maintainers are encouraged to make triage a regular part of their process and apply these tags to all issues as they have time.

A final note on our triage process is about milestones. Currently the Habitat project uses milestones as buckets for sorting our triaged issues. Any issues that get added into the "Accepted Minor" milestone will be transferred to the Core Engineering backlog at a later date. Any issues in the "Help Wanted" milestone are issues that we would happily merge but that are unlikely to get prioritized by the core engineering team. Now that doesn't mean those issues will never get prioritization, they will be revisited at regular intervals, they are just typically features that don't align directly with our [product roadmap](https://ext.prodpad.com/ext/roadmap/d2938aed0d0ad1dd62669583e108357efd53b3a6). The accepted-major issues are issues that we intend to build but that we know in advance will have breaking changes for our users. Those issues will be revisited and likely moved off to other milestones at later dates but will stay in that milestone until an appropriate time for implementation can be determined.

## Public Project Triage
Every Tuesday afternoon there is a public, open triage process focused on making sure that all new issues for the week that have not yet been placed into a milestone, get sifted, sorted, and labeled. The focus for this triage event (at least right now) is to ensure that new issues have been labeled correctly and with the most useful labels to anyone who may decide to work them whether those individuals are core maintainers or not. Those in leadership roles for the project are encouraged to attend and lend their expertise. We track the minutes of those calls in [a thread on the forums](https://forums.habitat.sh/t/weekly-issue-triage).

# Contributing Code

## Feature Pipeline
A majority of the feature work the core team is doing currently is prioritized by the Habitat product owner. Those features will be given issues and labels like all other work in the codebase and added onto the [core engineering project board](https://github.com/habitat-sh/habitat/projects/1). Many of the issues on this board are created by the core maintainers. But this board will also include features and bugs that have the Accepted Minor milestone.

The project and our community is currently an easily managed size so we do not currently have an overly complex RFC process. For features that might include breaking changes, or that are a bit more complex or touch important habitat subsystems we suggest contributors to open an issue that includes [RFC] in the title. Doing so will put the issue on the maintainers radar. When the issue is triaged it will be labeled with the `C-RFC` tag. RFCs will get reviewed once per week by the whole team, but maintainers are encouraged to add discussion to these issues outside of public triage as they have the time.

Lots of new features might require community input. So, we suggest that the contributor whom submits the issue make sure to post about it in community channels to drum up user input. Contributors should also feel comfortable using the `@core-maintainers` slack notifier to notify the maintainers that the RFC has been opened.

In the case of an RFC being opened the core maintainers may request separate issues to be created for various aspects of the feature (dependent wholly on the size of the incoming work) which will be linked back to the original RFC. In the case that the RFC is a manageable chunk of work for an individual the issue will get sorted to Accepted Minor or Accepted Major milestones, the core engineering project board and the RFC prefix will be removed.

## Merging Code
* Open a Pull Request that includes a Developer Certificate of Origin-signed (DCO-signed) commit (anyone).
* Code is reviewed by a Maintainer, Lieutenant, or Project Lead. Approval is indicated by a thumbs up in a comment on the PR. Sometimes that thumbs up comes in the form of a dank gif.
* Merged after a thumbs up vote by at least one Maintainer in the component(s) affected by your patch (Area Tag). The merge will be initiated by approved maintainer
* Dank gifs are an important part of every pull request.

## Pull Request Review and Merge Automation
Habitat uses several bots to automate the review and merging of pull requests. Messages to and from the bots are brokered via the account @thesentinels. First, we use Facebook's mention bot to identify potential reviewers for a pull request based on the blame information in the relevant diff. @thesentinels can also receive incoming commands from reviewers to approve PRs. The bot will first fire pre-commit checks across your PR to ensure that a DCO signature is provided and that the code doesn't have easily corrected issues. Once the PR has approval from a maintainer they will issue an `@thesentinels approve` command which will be routed to a bot that will automatically merge the patch.

Any Maintainer may downvote on a PR, which increases the requirement for a patch to be merged to an absolute majority of Maintainers for the affected component(s), unless that Maintainer later changes their vote.

## Patch Appeals Process

There may be cases where someone wishes to appeal a Maintainer decision. In this event, the "chain of command" for the appeals process is as follows.

> In the event that the actions of a Maintainer are to be appealed, the appeal should be directed to the Lieutenant for that component. As stated above, a Lieutenant retains veto power for the component(s) for which they are responsible.

> In the event that the actions of a Lieutenant are to be appealed, the appeal should be directed to the Project Lead. As stated above, the Project Lead retains universal veto power over all components.

Although Lieutenants and the Project Lead retain veto powers over certain components, use of this veto power is not guaranteed by the submission of an appeal to that person. It is expected that the majority decisions of component Maintainers and Lieutenants will be respected in all but the most exceptional circumstances.

## Release Notes
All merged PRs will be added into our release notes which are generated automatically when we cut a release. The Habitat product manager typically will aggregate that PR list into a post on the [forums](https://forums.habitat.sh/c/habitat-announcements) with any qualifying information in regards to new features that will be included in that release.


This is the majority of the process around governance that the project has in place today. If you're interested in reading more make sure to keep track of the habitat wiki page on GitHub, and check back here on our blog for updates in the future!
