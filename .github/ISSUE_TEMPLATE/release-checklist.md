---
name: Release Checklist
about: Track the status of a release
labels: C-release

---

Here are all the steps for the release process. Create a new issue at the beginning and check them off as you go. This issue exists to track progress on the release and ensure all the steps are carried out. When you're done just close the issue.

As you encounter discrepancies between this list and reality, or as you encounter problems with a release, please add a comment to this issue with a checklist of these things, and add to it as the release progresses. This will give you a list of things to update at the end, as well as provide a historical record of what happened, which can help us drive improvements in the future. As you fix things or file formal issues, please add the relevant links back to the checklist item that triggered it to provide further context.

- [ ] Determine if you need a new Launcher release, based on changes since the last release. The pipeline will ask you if it needs to build a new one.
- [ ] Declare merge freeze and update slack status
- [ ] [Create PR](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#prepare-master-branch-for-release) to update [`VERSION`](https://github.com/habitat-sh/habitat/blob/master/VERSION)
- [ ] Fix up any miscategorization in [CHANGELOG.md](https://github.com/habitat-sh/habitat/blob/master/CHANGELOG.md) and add to PR updating [`VERSION`](https://github.com/habitat-sh/habitat/blob/master/VERSION)
- [ ] [Merge PR](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#prepare-master-branch-for-release) to update [`VERSION`](https://github.com/habitat-sh/habitat/blob/master/VERSION)
- [ ] Tag the repo when the above PR merges and push. This is the trigger for the Buildkite release pipeline.
- [ ] [Buildkite](https://buildkite.com/chef/habitat-sh-habitat-master-release) run success
- [ ] [Validate](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#validate-the-release) [darwin binaries](https://bintray.com/habitat/stable/hab-x86_64-darwin)
- [ ] [Validate](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#validate-the-release) [linux binaries](https://bintray.com/habitat/stable/hab-x86_64-linux)
- [ ] [Validate](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#validate-the-release) [linux-kernel2 binaries](https://bintray.com/habitat/stable/hab-x86_64-linux-kernel2)
- [ ] [Validate](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#validate-the-release) [windows binaries](https://bintray.com/habitat/stable/hab-x86_64-windows)
- [ ] Confirm validation in Buildkite to complete promotion
- [ ] [Update builder bootstrap bundle](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-builder-bootstrap-bundle)
- [ ] [Update chocolately package](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#rerun-chocolatey-validation-tests)
- [ ] [Publish release](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#publish-release) on GitHub
- [ ] [Update docs](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-the-docs)
- [ ] [Verify updated docs are deployed to https://www.habitat.sh/docs/](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#verify-the-docs)
- [ ] [Update CHANGELOG.md](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-the-changelog)
- [ ] [Bump version](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#bump-version) to new `-dev` version
- [ ] Declare merge thaw and update slack status
- [ ] [Update `hab-backline` to new `-dev` version in acceptance environment](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-the-acceptance-environment-with-the-new-hab-backline-1)
- [ ] [Promote Linux builder worker](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#promote-the-builder-worker) and confirm build uses new version
- [ ] [Promote Linux2 builder worker](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#promote-the-builder-worker) and confirm build uses new version
- [ ] [Promote Windows builder worker](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#promote-the-builder-worker) and confirm build uses new version
- [ ] Post announcement in [Chef discourse](https://discourse.chef.io/c/habitat)
- [ ] Post announcement in [Habitat forums](https://forums.habitat.sh/c/announcements)
- [ ] Tweet a release announcement from `@habitatsh`
- [ ] [Update `Cargo.lock`](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-cargolock) for [`habitat`](https://github.com/habitat-sh/habitat)
- [ ] [Update `Cargo.lock`](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-cargolock) for [`builder`](https://github.com/habitat-sh/builder)
- [ ] [Update `rustfmt`](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-rustfmt) for [`habitat`](https://github.com/habitat-sh/habitat)
- [ ] [Update `rustfmt`](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#update-rustfmt) for [`builder`](https://github.com/habitat-sh/builder)
- [ ] Review release automation and/or [`RELEASE.md`](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md) and add necessary fixes
- [ ] [Schedule release postmortem](https://github.com/habitat-sh/habitat/blob/master/RELEASE.md#release-postmortem)
