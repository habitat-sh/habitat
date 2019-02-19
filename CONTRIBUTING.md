# Contributing to Habitat

Thank you for your interest in contributing to Habitat! There are many ways to contribute, and we appreciate all of them.

You are currently in the core Habitat repo, which is primarily written in Rust code and holds the core functionality of the Core Habitat, Habitat Builder Web Application, Habitat Builder, and Documentation. If you are interested in contributing a new plan to core plans (which is a great way to get started as a new contributor!), check out the [core-plans repo](https://github.com/habitat-sh/core-plans) instead.

If you have questions, join the [Habitat Slack Channel](http://slack.habitat.sh) to talk directly to the community and core maintainers. All experience levels of questions are welcome in the general channel.

As a reminder, all participants are expected to follow the [Code of Conduct](https://github.com/habitat-sh/habitat/blob/master/CODE_OF_CONDUCT.md).

* [Support Channels](#support-channels)
* [Feature Requests](#feature-requests)
* [Bug Reports](#bug-reports)
* [Report a security vulnerability](#report-a-security-vulnerability)
* [Pull Requests](#pull-requests)
* [Writing Documentation and blogs](#writing-documentation-and-blogs)
* [Issue Triage](#issue-triage)
* [Related Articles](#related-articles)
* [Development priniciples for working on Habitat](#development-principles-for-working-on-habitat)
* [Workstation Setup](#workstation-setup)
* [Writing new features](#writing-new-features)
* [Signing Your Commits](#signing-your-commits)
* [Pull Request Review and Merge Automation](#pull-request-review-and-merge-automation)
* [Delegating pull request merge access](#delegating-pull-request-merge-access)
* [Running a Builder service cluster locally](#running-a-builder-service-cluster-locally)
* [Documentation for Rust Crates](#documentation-for-rust-crates)

# Support Channels

Whether you are a user or contributor, official support channels include:

* GitHub issues: https://github.com/habitat-sh/habitat/issues
* Slack: http://slack.habitat.sh

Before opening a new issue or submitting a new pull request, it's helpful to search the project - it's likely that another user has already reported the issue you're facing, or it's a known issue that we're already aware of.

# Feature Requests

To request a change to the way that Habitat works, please [open an issue](https://github.com/habitat-sh/habitat/issues).

# Bug Reports

Bugs are a reality in software. We can't fix what we don't know about, so please report liberally. If you're not sure if something's a bug or not, feel free to file a bug anyway.

If you believe your bug represents a security issue for Habitat users, please follow our instructions for [reporting a security vulnerability.](#report-a-security-vulnerability)

If you have the chance, before reporting a bug please [search existing issues](https://github.com/habitat-sh/habitat/issues), as it's possible someone has already reported your error.

Opening an issue is as easy as following [this link](https://github.com/habitat-sh/habitat/issues/new) and filling out the form with as much information as you have. It is not necessary to follow the template exactly.

# Report a security vulnerability

Safety is one of the core principles of Habitat, and to that end, we would like to ensure that Habitat has a secure implementation. Thank you for taking the time to responsibly disclose any issues you find.

All security bugs in the distribution should be reported by email to security@habitat.sh. This list is delivered to a small security team. Your email will be acknowledged within 24 hours, and you'll receive a more detailed response to your email within 48 hours indicating the next steps in handling your report.

This email address receives a large amount of spam, so be sure to use a descriptive subject line to avoid having your report be missed. After the initial reply to your report, the security team will endeavor to keep you informed of the progress being made towards a fix and full announcement.

If you have not received a reply to your email within 48 hours, or have not heard from the security team for the past five days, there are a few steps you can take:

* Contact the current security coordinator (Jamie Winsor) directly.
* Contact the back-up contacts (Fletcher Nichol, Tasha Drew) directly.
* Post on the [slack channel](http://slack.habitat.sh) requesting an update.

Please note that the discussion forums and slack channel are public areas. When escalating in these venues, please do not discuss your issue. Simply say that you're trying to get a hold of someone from the security team.

# Pull Requests

Pull requests are the primary mechanism we use to write Habitat. GitHub itself has some great documentation on using [the Pull Request feature](https://help.github.com/articles/about-pull-requests/). We use the "fork and pull" model [described here](https://help.github.com/articles/about-collaborative-development-models/), where contributors push changes to their personal fork and create pull requests to bring those changes into the source repository.

Please make pull requests against the `master` branch.

# Writing Documentation and Blogs

Documentation improvements are very welcome.

To find documentation-related issues, [search by the A-Documentation label](https://github.com/habitat-sh/habitat/issues?q=is%3Aissue+is%3Aopen+label%3AA-documentation).

Blogs from community members are also very welcome. You can open a pull request to submit a blog for the website by [following these instructions](https://github.com/habitat-sh/habitat/tree/master/www/source/blog).

# Issue Triage

The Habitat core team does issue triage every Tuesday, at 10am Pacific time. A link to a public video is made available in the #general channel of Slack when triage occurs, and a recording is shared afterwards on the [Habitat youtube channel](https://www.youtube.com/channel/UC0wJZeP2dfPZaDUPgvpVpSg).

You can also help triage by using the following labeling system adopted by the Habitat core team:

| *TAGS Groups*                     |               Meaning                             |
|:----------------------------------|:--------------------------------------------------|
| *AREA TAGS* Prefix "A-"           | What part of the codebase does the issue refer to.|
| *CATEGORIZATION TAGS* Prefix "C-" | What type of work does the issue refer to.        |
| *EFFORT TAGS* Prefix "E-"         | Should a contributor undertake, what is the estimated effort level. |
| *LANGUAGE TAGS* Prefix "L-"       | Pretty straightforward right? What language the work will involve.  |
| *STATUS TAGS* Prefix "S-"         | The current status of an issue or pull request.   |
| *PLATFORM TAGS* Prefix "P-"       | Issue is in regards to specific platform.         |

[Read more about issue triage here.](https://www.habitat.sh/blog/community/)

# Related Articles
* [Current Habitat Maintainers](https://github.com/habitat-sh/habitat/blob/master/MAINTAINERS.md)
* [Maintainers Policy, how to become a Maintainers](https://github.com/habitat-sh/habitat/blob/master/maintenance-policy.md)
* [ReadMe](https://github.com/habitat-sh/habitat/blob/master/README.md)
* [CLI UX Principes](https://github.com/habitat-sh/habitat/blob/master/UX_PRINCIPLES.md)
* [Habitat Main Website: Browse docs, do a tutorial, check out Builder](https://www.habitat.sh/)

## Development principles for working on Habitat

1. The principle of least abstraction. When possible, we use the tooling that is closest to the native
tooling for the platform, and provide as little abstraction as necessary. When we do choose an abstraction,
we choose one - and we make it the one that is most user-serviceable.
1. Keep it light. The runtime component of Habitat is used as a process Supervisor - it needs to stay lean. No run-times.
1. Convention over configuration, with reasonable defaults. Where possible, we remove the need to configure things
by having a convention cover it. When we do need to configure things, we set reasonable defaults.
1. Call things what they are.
1. It has to feel great to the end user. If it doesn't feel great, it's a bug.
1. Write documentation as you go. Internal and external.

## Workstation Setup

See [BUILDING.md](BUILDING.md) for platform specific info on getting your workstation configured to contribute.

## Writing new features

1. Start a new feature branch
1. Open a terminal and run `make shell`
1. Change directory to a component `cd components/x`
1. Build with `cargo build` or `cargo test`
1. You can use `cargo run -- foobar` to pass options to the built binary
1. Sign and commit your change
1. Push your feature branch to GitHub, and create a Pull Request

## Signing Your Commits

This project utilizes a Developer Certificate of Origin (DCO) to ensure that each commit was written by the
author or that the author has the appropriate rights necessary to contribute the change.  The project
utilizes [Developer Certificate of Origin, Version 1.1](http://developercertificate.org/)

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
660 York Street, Suite 102,
San Francisco, CA 94110 USA

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

Each commit must include a DCO which looks like this

`Signed-off-by: Joe Smith <joe.smith@email.com>`

The project requires that the name used is your real name.  Neither anonymous contributors nor those
utilizing pseudonyms will be accepted.

Git makes it easy to add this line to your commit messages.  Make sure the `user.name` and
`user.email` are set in your git configs.  Use `-s` or `--signoff` to add the Signed-off-by line to
the end of the commit message.

## Pull Request Review and Merge Automation

Habitat uses several bots to automate the review and merging of pull requests. Messages to and from the bots are brokered via the account @thesentinel. First, we use Facebook's [mention bot](https://github.com/facebook/mention-bot) to identify potential reviewers for a pull request based on the `blame` information in the relevant diff. @thesentinels can also receive
incoming commands from reviewers to approve PRs. These commands are routed to a [homu](https://github.com/barosl/homu) bot that will automatically merge a PR when sufficient reviewers have provided a +1 (or r+ in homu terminology).


## Delegating pull request merge access

A Habitat core maintainer can delegate pull request merge access to a contributor via

	@thesentinels delegate=username

If you've been given approval to merge, you can do so by appending a comment to the pull request containing the following text:

	@thesentinels r+

Note: **do not** click the Merge Pull Request button if it's enabled.


## Running a Builder service cluster locally

A service cluster can be started in your host machine with `make bldr-run`. The public API will be available on port 9636 and the admin API will be available on port 8080. The depot web site will be available on port 3000.

Please refer to the detailed setup instructions in the [builder repo](https://github.com/habitat-sh/builder)'s [DEVELOPING.md](https://github.com/habitat-sh/builder/blob/master/DEVELOPING.md) file for how to bring up a service cluster, as there are currently some manual steps involved.


## Documentation for Rust Crates

The Rust crates also have their own internal developer documentation. From the root of the project, type `make docs` to build the internal Rust documentation.

Run `make serve-docs` to run a small web server that exposes the documentation on port `9633`. You can then read the docs at `http://<DOCKER_HOST>:9633/` (with working JavaScript-based search).
