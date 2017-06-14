---
title: Triage and Contribution Process
date: 2017-06-07
author: Ian Henry
tags: Governance,Community
category: Community
classes: body-article
---
A super common question that comes up in lots of FOSS communities is "How do I contribute?". While most projects have a file like a [CONTRIBUTING.md](https://github.com/habitat-sh/habitat/blob/master/CONTRIBUTING.md) or instructions in a README on how to join in on the open source fun, this question still gets asked quite frequently. This is an interesting problem to have for a group of people that are actively attempting to grow their community and base of contributors. The question project maintainers really need to ask themselves is quite simply: Why is this question coming up in the first place? Which, I think points to an inherent problem that doesn't get discussed frequently/directly. It's not a problem specific to the habitat project but rather a side effect of collaborating on a thing with geo-spatially distributed actors. What I mean by that is: Collaborating over the internet is _hard_ and we should be perpetually attempting to find ways to make that experience less painful.

Many of us have probably watched other FOSS communities adopt different strategies to minimize the barrier to contribution and the habitat project is not an exception. One of our project beliefs is that building humane software requires our processes, our code of conduct, our Governance and our developer tooling to reflect that humanity. Which, is why we're opting to make some simple governance-ish changes that we hope to be the first steps towards minimizing the barrier to contribution in the [habitat.sh codebase](https://github.com/habitat-sh/habitat).

## Tag Categorization
The first change is relatively low impact but if you spend time tracking our development in GitHub then you might have noticed it. We've opted to change the system by which we are tagging and tracking our issues. "Wow... what a build up for a such a trivial change there, Ian." I know, I know but, hear me out. First let's look at how this is going to work now and then we can discuss where we hope to lead this forward. We've broken all of our GitHub Labels into groups which I'll define here:

| *TAGS Groups*                     |               Meaning                             |
|:----------------------------------|:--------------------------------------------------|
| *AREA TAGS* Prefix "A-"           | What part of the codebase does the issue refer to.|
| *CATEGORIZATION TAGS* Prefix "C-" | What type of work does the issue refer to.        |
| *EFFORT TAGS* Prefix "E-"         | Should a contributor undertake, what is the estimated effort level. |
| *LANGUAGE TAGS* Prefix "L-"       | Pretty straightforward right? What language the work will involve.  |
| *STATUS TAGS* Prefix "S-"         | The current status of an issue or pull request.   |
| *PLATFORM TAGS* Prefix "P-"       | Issue is in regards to specific platform.         |

Obviously these specific labels might change in the future and for a full breakdown of what each individual tag means you can check out the [Label Names and Definitions](https://github.com/habitat-sh/habitat/wiki/Label-Names-and-Definitions) post in the [wiki](https://github.com/habitat-sh/habitat/wiki) (side note on the wiki: this is another item we plan on fleshing out with content as we continue forward with the project!).

The goal of these changes is to quickly and efficiently bubble up snippets of work for new and recurring project contributors. We hope that during triage we can start estimating level of effort, providing theoretical starting points, and flagging the piece of the project an issue refers to in order to give our community a really low friction way to get involved in the hacking! But we don't plan to stop here. We are always looking for ways to expose and encourage community members to get involved and we feel like taking this step first really enables us to build out some community work we'd like to be doing like social-coding, mentoring new contributors, and providing better metrics and tooling around the contribution process.

Keep an eye out as any of these changes are flexible, we hope to have another post on the wiki soon that will 100% define how we handle the issue tracker in full. That post will likely be cross-posted here too. For now, happy habitat-ing!  
