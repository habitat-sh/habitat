# Release Notes Draft

This document is the draft for our Habitat 2.0 Release Announcement. Items at the second level of heading are thought of as independent sections that may or may not be appropriate for publishing (but the title should make it obvious). It will be added to our repo via a PR so that others can review and enhance as appropriate.  After Habitat 2.0 is released and the release notes are published it should probably be removed as its utility has ceased.  We might keep it around as a live scratch pad but it's usually pretty easy just to look back and see what we should add.

Note that items prefixed with REMOVE: are "inlined source material" meant to be removed before the release notes are published.

TODO before publishing:

- remove all "REMOVE:" items update all links to handlebars update mitigation
- substitute actual trivial version for HAB_PATCH_VERSION placeholders
- substitute actual trivial version for LAUNCHER_VERSION placeholders
- substitute actual release date for RELEASE_DATE placeholders

## Habitat 2.0.HAB_PATCH_VERSION Release Notes (released on RELEASE_DATE)

The Progress Chef Habitat team is proud to announce the release of Habitat 2.0.HAB_PATCH_VERSION. This milestone release brings several new features and updates.

Our comprehensive documentation is available at <https://docs.chef.io/habitat/> which also includes our [installation instructions](https://docs.chef.io/habitat/install_habitat/). A comprehensive listing of all the changes may be found in our changelogs [CHANGELOG.md](https://github.com/habitat-sh/habitat/blob/main/CHANGELOG.md) and [CHANGELOG-before-2.x.md](https://github.com/habitat-sh/habitat/blob/main/CHANGELOG-before-2.x.md).

### Released Packages

This release updates the Habitat components to the following versions

| Package                        | Version            |
| ------------------------------ | ------------------ |
| chef/hab                       | HAB_PATCH_VERSION  |
| chef/hab-launcher              | LAUNCHER_VERSION   |
| chef/hab-pkg-export-container  | HAB_PATCH_VERSION  |
| chef/hab-pkg-export-tar        | HAB_PATCH_VERSION  |
| chef/hab-studio                | HAB_PATCH_VERSION  |
| chef/hab-sup                   | HAB_PATCH_VERSION  |

### New Features

- Linux ARM Support
  - Linux ARM support has been under development for some time but with the release of Habitat 2.0 it is fully supported.
- Upgraded PowerShell to 7.5.x release line

### Breaking Changes

- Handlebars upgraded
  - Our Handlebars implementation was upgraded to the latest version. This work had been deferred for quite some time, as doing so exposes potentially breaking changes.  Please see the details [in the Habitat manual](https://docs.chef.io/habitat/) to see what actions you may need to take.
- Removed support for "Linux 2 Kernels"
- Habitat package moved from the `core` origin to the `chef` origin.
  - This move better delineates "the system" from "software used within the system".  A specific example of this change is that the package containing the habitat binary is now the `chef/hab` as opposed to `core/hab` package.  The other binaries and libraries that comprise habitat have made the same move.
- Default channel has changed from `core` to `base`.
  - In order to better facilitate dropping support for software that has reached its end-of-life we've introduced new channels named `base-2025` and `base` that represent a grouping of supported packages. The `base-2025` channel is the first grouping.  The `base` channel will always point to the most recent package group so at the current time `base` and `base-2025` are synonyms. This isn't necessarily a breaking change, but to leverage updated packages going forward you will need to use the new channels.  The hab cli now uses `base` as its default value.
  - REMOVE: <https://www.chef.io/blog/long-term-support-progress-chef-providing-stability>
    - I thought we had something like this blog post discussing the different channels and the strategy but I can't find it.  If we don't have that it feels like it would be a good blog post at some point.

### Security Fixes

This is a list of security fixes in this release along with all known identifiers that detail them.

#### Vulnerability Mitigations

These are security vulnerabilities reported in established security databases.

1) Updated [crossbeam-channel](https://crates.io/crates/crossbeam-channel) to 0.5.15 which fixes
   - [CVE-2025-4574](https://nvd.nist.gov/vuln/detail/CVE-2025-4574)
   - [GHSA-pg9f-39pc-qf8g](https://github.com/advisories/GHSA-pg9f-39pc-qf8g)
   - [RUSTSEC-2025-0024](https://rustsec.org/advisories/RUSTSEC-2025-0024.html)
1) Updated [openssl](https://crates.io/crates/openssl) to 0.10.73 which fixes
   - [GHSA-4fcv-w3qc-ppgg](https://github.com/advisories/GHSA-4fcv-w3qc-ppgg)
   - [RUSTSEC-2025-0022](https://rustsec.org/advisories/RUSTSEC-2025-0022.html)
   - [CVE-2025-24898](https://nvd.nist.gov/vuln/detail/CVE-2025-24898)
   - [GHSA-rpmj-rpgj-qmpm](https://github.com/advisories/GHSA-rpmj-rpgj-qmpm)
   - [RUSTSEC-2025-0004](https://rustsec.org/advisories/RUSTSEC-2025-0004.html)
1) Deactivated the default features of the [prometheus](https://crates.io/crates/prometheus) crate which fixes
   - [CVE-2025-53605](https://nvd.nist.gov/vuln/detail/CVE-2025-53605)
   - [GHSA-2gh3-rmm4-6rq5](https://github.com/advisories/GHSA-2gh3-rmm4-6rq5)
   - [RUSTSEC-2024-0437](https://rustsec.org/advisories/RUSTSEC-2024-0437.html)
1) Updated [regex](https://crates.io/crates/regex) to 1.12.2 which fixes
   - [CVE-2022-24713](https://nvd.nist.gov/vuln/detail/CVE-2022-24713)
   - [GHSA-m5pq-gvj9-9vr8](https://github.com/advisories/GHSA-m5pq-gvj9-9vr8)
   - [RUSTSEC-2022-0013](https://rustsec.org/advisories/RUSTSEC-2022-0013.html)
1) Updated [ring](https://crates.io/crates/ring) to 0.17.14 which fixes
   - [CVE-2025-4432](https://nvd.nist.gov/vuln/detail/CVE-2025-4432)
   - [GHSA-4p46-pwfr-66x6](https://github.com/advisories/GHSA-4p46-pwfr-66x6)
   - [GHSA-c86p-w88r-qvqr](https://github.com/advisories/GHSA-c86p-w88r-qvqr)
   - [RUSTSEC-2025-0009](https://rustsec.org/advisories/RUSTSEC-2025-0009.html)
1) Updated [thread_local](https://crates.io/crates/thread_local) to 1.1.9 which fixes
   - [GHSA-9hpw-r23r-xgm5](https://github.com/advisories/GHSA-9hpw-r23r-xgm5)
   - [RUSTSEC-2022-0006](https://rustsec.org/advisories/RUSTSEC-2022-0006.html)
1) Updated [tracing-subscriber](https://crates.io/crates/tracing-subscriber) to 0.3.20 which fixes
   - [CVE-2025-58160](https://nvd.nist.gov/vuln/detail/CVE-2025-58160)
   - [GHSA-xwfj-jgwm-7wp5](https://github.com/advisories/GHSA-xwfj-jgwm-7wp5)
   - [RUSTSEC-2025-0055](https://rustsec.org/advisories/RUSTSEC-2025-0055.html)

#### Unmaintained Mitigations

The Rust community [has a process](https://github.com/rustsec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md) by which crates can be identified as unmaintained. A crate being identified as an unmaintained crate does not automatically mean that it's vulnerable code but unmaintained code is a vector of concern. Therefore, when we become aware of unmaintained crates we migrate away from them as we are able to do so.  Note that in some cases the unmaintained code was pulled in via a dependency so the connection to the updated crate may not be immediately obvious.

1) Switched to using [adler2](https://crates.io/crates/adler2) which fixes
   - [RUSTSEC-2025-0056](https://rustsec.org/advisories/RUSTSEC-2025-0056.html)
1) Updated [clap](https://crates.io/crates/clap) to 4.5.51 which fixes
   - [RUSTSEC-2021-0139](https://rustsec.org/advisories/RUSTSEC-2021-0139.html)
   - [RUSTSEC-2024-0375](https://rustsec.org/advisories/RUSTSEC-2024-0375.html)
   - [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370.html)
1) Updated [log4rs](https://crates.io/crates/log4rs) to 1.4.0 which fixes
   - [RUSTSEC-2024-0388](https://rustsec.org/advisories/RUSTSEC-2024-0388.html)
1) Updated [notify](https://crates.io/crates/notify) to 8.2.0 which fixes
   - [RUSTSEC-2024-0384](https://rustsec.org/advisories/RUSTSEC-2024-0384.html)
1) Eliminated use of [paste](https://github.com/dtolnay/paste) which fixes
   - [RUSTSEC-2024-0436](https://rustsec.org/advisories/RUSTSEC-2024-0436.html)
1) Switched to using the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) fixing
   - [RUSTSEC-2022-0071](https://rustsec.org/advisories/RUSTSEC-2022-0071.html)

#### Unsoundness Mitigations

The Rust community also raises advisories for code found to be unsound. For an in-depth discussion of "soundness" [see this post by dtolnay](https://docs.rs/dtolnay/0.0.7/dtolnay/macro._03__soundness_bugs.html) but a more succinct definition is found in [the glossary of Rust's unsafe code guidelines](https://github.com/rust-lang/unsafe-code-guidelines/blob/636d140ce9c74ffc4d1fc082bef0771f238f64c9/reference/src/glossary.md#soundness-of-code--of-a-library). Excerpting from the second link, in Rust

> we say that a library (or an individual function) is sound if it is impossible for safe code to cause Undefined Behavior using its public API. Conversely, the library/function is unsound if safe code can cause Undefined Behavior.

When we become aware of unsoundness advisories we move to address it in line with community guidance.

1) Updated [clap](https://crates.io/crates/clap) to 4.5.51 which fixes
   - [RUSTSEC-2021-0145](https://rustsec.org/advisories/RUSTSEC-2021-0145.html)
   - [GHSA-g98v-hv3f-hcfr](https://github.com/advisories/GHSA-g98v-hv3f-hcfr)
1) Updated [tokio](https://crates.io/crates/tokio) to 1.48.0 which fixes
   - [RUSTSEC-2025-0023](https://rustsec.org/advisories/RUSTSEC-2025-0023.html)
   - [GHSA-rr8g-9fpq-6wmg](https://github.com/advisories/GHSA-rr8g-9fpq-6wmg)

## Data Sources

### Github Searches

`gh pr list --search "is:pr is:closed base:main created:>2024-12-25 sort:created-asc -author:app/dependabot" --limit 256`

`gh pr list --search "is:pr is:closed base:main created:>2024-12-25 sort:created-asc -author:app/dependabot" --limit 256 --json number,title --template '{{range .}}{{tablerow (printf "#%v" .number) .title}}{{end}}'`

### JIRA Pages

<https://progresssoftware.atlassian.net/projects/CHEF/versions/17382/tab/release-report-all-issues>

### Confluence Pages

<https://progresssoftware.atlassian.net/wiki/spaces/MasterChef/pages/2053210221/Hab+2.0+Testing+and+Documentation+Review>
