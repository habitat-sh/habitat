# Release Notes Draft

This document is the draft for our Habitat 2.0 Release Announcement. Items at the second level of heading are thought of as independent sections that may or may not be appropriate for publishing (but the title should make it obvious). It will be added to our repo via a PR so that others can review and enhance as appropriate.  After Habitat 2.0 is released and the release notes are published it should probably be removed as it's utility has ceased.  We might keep it around as a live scratch pad but it's usually pretty easy just to look back and see what we should add.

Note that items prefixed with REMOVE: are "inlined source material" meant to be removed before the release notes are published.

TODO before publishing:

- remove all "REMOVE:" items
- update all links to handlebars update mitigation

## Draft Habitat 2.0 Release Notes

The Progress Chef Habitat team is proud to announce the release of Habitat 2.0.  This milestone release brings a several new features and updates.  Comprehensive documentation may be found at <https://docs.chef.io/habitat/>.

### New Features

- Linux ARM Support
  - Linux ARM support has been under development for some time but with the release of Habitat 2.0 it is fully supported.
- Upgraded PowerShell to 7.5.x release line

### Breaking Changes

- Handlebars upgraded
  - Our Handlebars implementation was upgraded to the latest version. This work had been deferred for quite some time as doing so exposes potentially breaking changes.  Please see the details [in the Habitat manual](https://docs.chef.io/habitat/) to see what actions you may need to take.
- Removed support for "Linux 2 Kernels"
- Habitat package moved from the `core` origin to the `chef` origin.
  - This move better delineates "the system" from "software used within the system".  A specific example of this change is that the packages containing the habitat binary is now the `chef/habitat` as opposed to `core/habitat` package.  The other binaries and libraries that comprise habitat have made the same move.
- Default channel has changed from `core` to `base`.
  - In order to better facitilate dropping support for software that has reached it's end-of-life we've introduced new channels named `base-2025` and `base` that represent a grouping of supported packages. The `base-2025` channel is the first grouping.  The `base` channel will always point to the most recent package group so at the current time `base` and `base-2025` are synonyms. This isn't necessarily a breaking change but to leverage updated packages going forward you will need to use the new channels.  The hab cli now uses `base` as it's default value.
  - REMOVE: <https://www.chef.io/blog/long-term-support-progress-chef-providing-stability>
    - I thought we had something like this blog post discussing the different channels and the strategy but I can't find it.  If we don't have that it feels like it would be a good blog post at some point.

### Security Fixes

This is a list of security fixes in this release along with all known identifiers that detail them.

#### Vulnerability Mitigations

These are security vulnerability reported in established security databases.

1) crossbeam-channel: double free on Drop
   - CVE-2025-4574
   - GHSA-pg9f-39pc-qf8g
   - RUSTSEC-2025-0024
1) Use-After-Free in `Md::fetch` and `Cipher::fetch`
   - GHSA-4fcv-w3qc-ppgg
   - RUSTSEC-2025-0022
1) ssl::select_next_proto use after free
   - CVE-2025-24898
   - GHSA-rpmj-rpgj-qmpm
   - RUSTSEC-2025-0004
1) Crash due to uncontrolled recursion in protobuf crate
   - CVE-2025-53605
   - GHSA-2gh3-rmm4-6rq5
   - RUSTSEC-2024-0437
1) Regexes with large repetitions on empty sub-expressions take a very long time to parse
   - CVE-2022-24713
   - GHSA-m5pq-gvj9-9vr8
   - RUSTSEC-2022-0013
1) "Some AES functions may panic when overflow checking is enabled.",
   - CVE-2025-4432
   - GHSA-4p46-pwfr-66x6
   - GHSA-c86p-w88r-qvqr
   - RUSTSEC-2025-0009
1) "Data race in `Iter` and `IterMut`",
   - GHSA-9hpw-r23r-xgm5
   - RUSTSEC-2022-0006
1) Logging user input may result in poisoning logs with ANSI escape sequences
   - CVE-2025-58160
   - GHSA-xwfj-jgwm-7wp5
   - RUSTSEC-2025-0055

#### Unmaintained Mitigations

The Rust community [has a process](https://github.com/rustsec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md) by which crates can be identified as unmaintained. A crate being identified as an unmaintained crate does not automatically mean that it's vulnerable code but unmaintained code is a vector of concern. Therefore, when we become aware of unmaintained crates we migrate away from them as we are able to do so.

 1) `adler` crate is unmaintained, use `adler2` instead
    - RUSTSEC-2025-0056
 1) `ansi_term` is Unmaintained
    - RUSTSEC-2021-0139
 1) `atty` is unmaintained
    - RUSTSEC-2024-0375
 1) `derivative` is unmaintained; consider using an alternative
    - RUSTSEC-2024-0388
 1) `instant` is unmaintained
    - RUSTSEC-2024-0384
 1) `paste` - no longer maintained
    - RUSTSEC-2024-0436
 1) `proc-macro-error` is unmaintained
    - RUSTSEC-2024-0370
 1) `Rusoto` is unmaintained
    - RUSTSEC-2022-0071

#### Unsoundness Mitigations

The Rust community also raises advisories for code found to be unsound. For an in-depth discussion of "soundness" [see this post by dtolnay](https://docs.rs/dtolnay/0.0.7/dtolnay/macro._03__soundness_bugs.html) but a more succinct definition is found in [the glossary of Rust's unsafe code guidelines](https://github.com/rust-lang/unsafe-code-guidelines/blob/636d140ce9c74ffc4d1fc082bef0771f238f64c9/reference/src/glossary.md#soundness-of-code--of-a-library). Excerpting from the second link, in Rust

> we say that a library (or an individual function) is sound if it is impossible for safe code to cause Undefined Behavior using its public API. Conversely, the library/function is unsound if safe code can cause Undefined Behavior.

When we become aware of unsoundness advisories we move to address it in line with community guidance.

1) "Potential unaligned read" in `atty` package
   - RUSTSEC-2021-0145
   - GHSA-g98v-hv3f-hcfr
1) "Broadcast channel calls clone in parallel, but does not require `Sync`" in `tokio` package
   - RUSTSEC-2025-0023
   - GHSA-rr8g-9fpq-6wmg

## Data Sources

### Github Searches

`gh pr list --search "is:pr is:closed base:main created:>2024-12-25 sort:created-asc -author:app/dependabot" --limit 256`

`gh pr list --search "is:pr is:closed base:main created:>2024-12-25 sort:created-asc -author:app/dependabot" --limit 256 --json number,title --template '{{range .}}{{tablerow (printf "#%v" .number) .title}}{{end}}'`

### JIRA Pages

<https://progresssoftware.atlassian.net/projects/CHEF/versions/17382/tab/release-report-all-issues>

### Confluence Pages

<https://progresssoftware.atlassian.net/wiki/spaces/MasterChef/pages/2053210221/Hab+2.0+Testing+and+Documentation+Review>
