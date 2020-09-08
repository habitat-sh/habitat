+++
title = "Community Contributions"
draft = false

aliases = ["/community_contributions.html"]

[menu]
  [menu.overview]
    title = "Contributing"
    identifier = "overview/community/community_contributions.md Contributing"
    parent = "overview/community"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/community_contributions.md)

We're glad you want to contribute to a Chef project! This guide will
help answer common questions you may have during your first
contribution.

## Submitting Issues

Not every contribution comes in the form of code. Submitting,
confirming, and triaging issues is an important task for any project. At
Chef we use GitHub to track all project issues.

If you are familiar with Chef projects and know the component that is
causing you a problem, you can file an issue in the corresponding GitHub
project. All of our Open Source Software can be found in our GitHub
organizations at <https://github.com/chef>, <https://github.com/inspec>,
and <https://github.com/habitat-sh>. All projects include GitHub issue
templates to help gather information needed for a thorough review.

We ask you not to submit security concerns via GitHub. For details on
submitting potential security issues please see
<https://www.chef.io/security/>.

In addition to GitHub issues, we also utilize a feedback site that helps
our product team track and rank feature requests. If you have a feature
request, this is an excellent place to start:
<https://feedback.chef.io>.

The following repositories are the preferred locations for the creation
of issues and bug reports:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Repo</th>
<th>URL</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Infra Client</td>
<td><a href="https://github.com/chef/chef">https://github.com/chef/chef</a></td>
</tr>
<tr class="even">
<td>Ohai</td>
<td><a href="https://github.com/chef/ohai">https://github.com/chef/ohai</a></td>
</tr>
<tr class="odd">
<td>Chef Workstation</td>
<td><a href="https://github.com/chef/chef-workstation">https://github.com/chef/chef-workstation</a></td>
</tr>
<tr class="even">
<td>Chef InSpec</td>
<td><a href="https://github.com/inspec/inspec">https://github.com/inspec/inspec</a></td>
</tr>
<tr class="odd">
<td>Chef Infra Server</td>
<td><a href="https://github.com/chef/chef-server">https://github.com/chef/chef-server</a></td>
</tr>
<tr class="even">
<td>Chef Habitat</td>
<td><a href="https://github.com/habitat-sh/habitat">https://github.com/habitat-sh/habitat</a></td>
</tr>
<tr class="odd">
<td>Chef Automate</td>
<td><a href="https://github.com/chef/automate">https://github.com/chef/automate</a></td>
</tr>
</tbody>
</table>

For additional support about Chef products, including documentation,
learning, and opening tickets with the support team, see [Chef
Support](https://www.chef.io/support/) .

{{< warning >}}

If you require a response from Chef per the terms of a support level
agreement (SLA), please contact [Chef
Support](https://www.chef.io/support/) directly.

{{< /warning >}}

{{< note >}}

Questions on how to use Chef Infra should be sent as Support Tickets if
you have an SLA, or asked on the [Chef Software Mailing
List](https://discourse.chef.io/), or [Chef Community
Slack](https://community-slack.chef.io/). Bug Trackers are not
appropriate for general purpose questions that are not bugs.

{{< /note >}}

## Contribution Process

We have a 4 step process for contributions:

1.  Fork the project repository to your own GitHub account.
2.  Commit your changes to your fork, making sure to sign-off those
    changes for the Developer Certificate of Origin with `git commit -s`
3.  Create a GitHub Pull Request for your change, following the
    instructions in the pull request template.
4.  Perform a Code Review with the project maintainers on the pull
    request.

## About the Apache License

Chef uses the Apache License Version 2 because it provides the same
level of freedom for our users that we desire for ourselves. Based upon
the Apache Licensing FAQ, it allows you to:

-   freely download and use Chef software, in whole or in part, for
    personal, company internal, or commercial purposes;
-   use Chef software in packages or distributions that you create.

It forbids you to:

-   redistribute any piece of Chef-originated software without proper
    attribution;
-   use any marks owned by Chef in any way that might state or imply
    that Chef endorses your distribution;
-   use any marks owned by Chef in any way that might state or imply
    that you created the Chef software in question.

It requires you to:

-   include a copy of the license in any redistribution you may make
    that includes Chef software;
-   provide clear attribution to Chef for any distributions that include
    Chef software; attribution can be done in the NOTICE file for an
    application, by adding yourself as an author/copyright holder to the
    HEADER for an individual file, and by placing text in a header file
    saying that new work is based on previous work
-   reuse work as long as the licensing terms of the reused work remains
    unchanged (i.e. The Apache License Version 2 also applies to the
    reused work)

It does not require you to:

-   include the source of the Chef software itself, or of any
    modifications you may have made to it, in any redistribution you may
    assemble that includes it;
-   submit changes that you make to the software back to Chef (though
    such feedback is encouraged).

It is our goal to run a successful, truly open source business. To that
end, we are protecting our own rights by making them explicit in our
choice of licensing: you have the same rights to our open source
software that we do.

## Developer Certification of Origin (DCO)

To make a good faith effort to ensure the criteria of the Apache License
are met, Chef requires the Developer Certificate of Origin (DCO) process
to be followed.

The DCO is an attestation attached to every contribution made by every
developer. In the commit message of the contribution, the developer
simply adds a Signed-off-by statement and thereby agrees to the DCO,
which you can find below or at <https://developercertificate.org/>.

``` console
Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the
    best of my knowledge, is covered under an appropriate open
    source license and I have the right under that license to
    submit that work with modifications, whether created in whole
    or in part by me, under the same open source license (unless
    I am permitted to submit under a different license), as
    Indicated in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including
    all personal information I submit with it, including my
    sign-off) is maintained indefinitely and may be redistributed
    consistent with this project or the open source license(s)
    involved.
```

Chef does not merge any pull requests made against a Chef-managed open
source repository until each commit has been signed for the DCO, with
three exceptions:

-   "Obvious Fixes" (as described below)
-   Pull requests made against the docs.chef.io documentation repository
    (<https://github.com/chef/chef-web-docs>)
-   Pull requests that contain only documentation updates made against
    projects where the documentation is embedded in the project's
    repository (i.e. the `docs` directory in the `chef/inspec`
    repository)

## The "Obvious Fix" Rule

Chef's contribution policy is aimed at encouraging broad participation
from our community, and minimizing risks to Chef and our community due
to inappropriate contributions of the intellectual property of others.

As a general standard, Chef requires every contribution to by signed for
the Developer Certificate of Origin (DCO).

HOWEVER, very small contributions such as fixing spelling errors, where
the content is small enough to not be considered intellectual property,
can be submitted by a contributor as a patch, without a DCO sign-off. If
you submit an obvious fix without a DCO sign-off, then you are agreeing
that your submission is not independently copyrightable. The purpose of
this exception is to lower the barrier for new contributors to make
contributions while retaining the integrity of the project and our
community.

### How does the Obvious Fix Rule Work?

Any committer may commit fixes without a DCO sign-off for obvious typos,
grammar mistakes, and formatting problems wherever they may be - in the
web pages, API documentation, source code, etc.

Whenever you invoke the "obvious fix" rule, please say so in your commit
message. For example:

``` bash
------------------------------------------------------------------------
commit 370adb3f82d55d912b0cf9c1d1e99b132a8ed3b5
Author: Julia Child <juliachild@chef.io>
Date:   Wed Sep 18 11:44:40 2015 -0700

  Fix typo in the README.

  Obvious fix.

------------------------------------------------------------------------
```

### What qualifies as an Obvious Fix?

An obvious fix is a pull request that does not contain creative work. We
rely on your judgment to determine what is "obvious"; if you're not
sure, just ask by sending an email to: oss AT getchef DOT com.

As a rule of thumb, changes are obvious fixes if they do not introduce
any new functionality or creative thinking. As long as the change does
not affect functionality, some likely examples include the following:

-   Spelling/grammar fixes;
-   Correcting typos;
-   Cleaning up comments in the code;
-   Changes to white space or formatting;
-   Bug fixes that change default return values or error codes stored in
    constants, literals, or simple variable types;
-   Adding logging messages or debugging output;
-   Changes to 'metadata' files like Gemfile, rebar.config, Makefile,
    app.config, sys.config, .gitignore, example configuration files,
    build scripts, etc.;
-   Changes that reflect outside facts, like renaming a build directory
    or changing a constant;
-   Changes in build or installation scripts;
-   Re-ordering of objects or subroutines within a source file (such as
    alphabetizing routines);
-   Moving source files from one directory or package to another, with
    no changes in code;
-   Breaking a source file into multiple source files, or consolidating
    multiple source files into one source file, with no change in code
    behavior;
-   Changes to words or phrases isolated from their context;
-   Changes to typeface.

Things that would still require a DCO sign-off before submitting would
likely include stuff like the following:

-   Any of the above actions that result in a change in functionality;
-   A new feature;
-   A translation;
-   Extensive or creative comments.
