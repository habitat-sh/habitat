# Working on Habitat

## Rules for working on Habitat

1. The principle of least abstraction. When possible, we use the tooling that is closest to the native
tooling for the platform, and provide as little abstraction as necessary. When we do choose an abstraction,
we choose one - and we make it the one that is most user-serviceable.
1. Keep it light. The runtime component of Habitat is used as a process supervisor - it needs to stay lean. No run-times.
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

### Signing Your Commits

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

Habitat uses several bots to automate the review and merging of pull
requests. Messages to and from the bots are brokered via the account
@thesentinels. First, we use Facebook's [mention bot](https://github.com/facebook/mention-bot)
to identify potential reviewers for a pull request based on the `blame`
information in the relevant diff. @thesentinels can also receive
incoming commands from reviewers to approve PRs. These commands are
routed to a [homu](https://github.com/barosl/homu) bot that will
automatically merge a PR when sufficient reviewers have provided a +1
(or r+ in homu terminology).


### Delegating pull request merge access

A Habitat core maintainer can delegate pull request merge access to a contributor via

	@thesentinels delegate=username

If you've been given approval to merge, you can do so by appending a comment to the pull request containing the following text:

	@thesentinels r+

Note: **do not** click the Merge Pull Request button if it's enabled.


## Running a Builder service cluster locally

A service cluster can be started in your host machine with `make bldr-run`. The public API will be available on port 9636 and the admin API will be available on port 8080. The depot web site will be available on port 3000.

Please refer to the detailed setup instructions in the  [BUILDER_DEV.md](https://github.com/habitat-sh/habitat/blob/master/BUILDER_DEV.md) file for how to bring up a service cluster, as there are currently some manual steps involved.


### Documentation for Rust Crates

The Rust crates also have their own internal developer documentation. From the root of the project, type `make docs` to build the internal Rust documentation.

Run `make serve-docs` to run a small web server that exposes the documentation
on port `9633`. You can then read the docs at `http://<DOCKER_HOST>:9633/`
(with working JavaScript-based search).
