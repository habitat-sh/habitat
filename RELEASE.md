# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Monday.

## Promote a Candidate for Manual Testing

In Slack, execute the following command:

```
/expeditor promote habitat acceptance
```
This will take whatever packages are currently in the `acceptance`
channel and promote them into the `staging` channel. This manual
action is the only way things get into the `staging` channel, so you
can perform whatever manual validation you need without worrying about
new packages coming in and invalidating your efforts.

## Validate the Release

For each platform
([darwin](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-darwin.zip),
[linux](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-linux.tar.gz),
[linux-kernel2](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-linux-kernel2.tar.gz),
[windows](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-windows.zip)),
download the latest release candidate CLI from `packages.chef.io`. You
**must** have run the `/expeditor` Slack command above _before_
downloading the package!

Alternatively, you can use our "curlbash" installer, specifying the
`staging` channel as an argument:

``` sh
curl
https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh
| sudo bash -s -- -c staging`
```

There may be special behavior related to this release that you will want to validate but at the very least, do the following basic tests.

You need to set `HAB_INTERNAL_BLDR_CHANNEL` and `CI_OVERRIDE_CHANNEL`
to the value `staging` (you _may_ also need to set `HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL` and `HAB_STUDIO_SECRET_CI_OVERRIDE_CHANNEL` for non-Docker-based studio).

NOTE: If you are running `sudo hab studio enter` with all the required environmental variables set, but it's still telling you that it cannot find the package in stable, try `sudo -E hab studio enter`.

See https://github.com/habitat-sh/habitat/issues/4656 for further context and ideas.

Then you can actually exercise the software as follows:

1. It pulls down the correct studio image
1. That studio's `hab` is at the correct version (`hab --version`)
1. A `sup-log` shows a running supervisor (if `sup-log` does not show
   a supervisor running, run `hab install core/hab-sup --channel=staging` then `hab sup run`)
1. Verify that the supervisor is the correct version (`hab sup --version`)

When testing the Linux studio, you will also need to `export CI_OVERRIDE_CHANNEL=staging`.

### Validating x86_64-linux-kernel2

For this PackageTarget it is important that you perform validation on a Linux system running a 2.6 series kernel. CentOS 6 is recommended because it ships with a kernel of the appropriate age,  but any distro with a Kernel between 2.6.32 and 3.0.0 can be used. Included in the `support/validation/x86_64-linux-kernel2` directory in this repository is a Vagrantfile that will create a CentOS-6 VM to perform the validation. You can also run a VM in EC2.

The Vagrantfile is configured to grab the [core-plans](https://github.com/habitat-sh/core-plans) repository (to give you something to build), as well as grab the secret key for your `HAB_ORIGIN` (using the `HAB_ORIGIN` and `HAB_AUTH_TOKEN` variables in your environment). You'll need to manually install the release-candidate `hab` binary and set your various channel overrides, but other than that you should have all you need to test things out.

As an example, immediately after provisioning you can SSH into the machine and run `HAB_ORIGIN=<my_origin> hab pkg build core-plans/redis`.

## Promote from Staging to Current

If all your manual validation works out, you should promote the
release candidate from the `staging` channel to the `current`
channel. Once again, we use the Slack command:

```
/expeditor promote habitat staging
```
This will result in the Supervisors in our Production Builder updating
themselves to the release candidate.

## Promote from Current to Stable

Once you're satisfied with the new Supervisors in Production, you can
finish the release process by promoting from the `current` channel to
the `stable` channel.

```
/expeditor promote habitat current
```
This places all the release candidates into the `stable` channel,
making them "officially" available to the world, and thus "released".


# Post-Release Tasks
The Buildkite release is fairly-well automated at this point, but once it is complete, there are still a few remaining manual tasks to perform. In time, these will be automated as well.

## Verify the Acceptance environment is using the new hab-backline

Running [`update-hab-backline.sh`](https://github.com/habitat-sh/habitat/blob/master/update-hab-backline.sh)
is handled by buildkite. If it is necessary to do manually, you can find instructions in [a previous release of this file.](https://github.com/habitat-sh/habitat/blob/bebf0fdfb738e1304ea201717fb6054733b17939/RELEASE.md#update-the-acceptance-environment-with-the-new-hab-backline)

## Update the Docs

Assuming you've got a locally installed version of the `hab` CLI you just released, you can update the CLI documentation in a separate PR. To do that run the following commands on OS X (other platforms may work as well):

```
cd www
make cli_docs
make template_reference
```

Verify the diff looks reasonable and matches the newly released version, then submit your PR.

## Verify the Docs

After the PR that updates the generated docs is merged, the [deploy_website.sh](https://github.com/habitat-sh/habitat/blob/master/support/ci/deploy_website.sh)
script will run to redeploy the website with the updated content. Verify that this was successful
at https://www.habitat.sh/docs. If not, check https://travis-ci.org/habitat-sh/habitat/builds
and find the "Merge pull request #XXX" run where `XXX` was the number of your PR from the
previous step to see if there were any errors during the deploy process.

## Update the Changelog

We currently use Expeditor (an internal tool) to _partially_ manage our changelog. It adds items to `CHANGELOG.md` for every PR that is merged, based on certain labels that are attached to the PR. This is all well and good.

However, due to how our pipeline is structured, we can't really use
Expeditor's built-in changelog rollup functionaltiy.

(The rollup is something Expeditor does at the current head of master;
when we release, we are almost always going to be dealing with
artifacts that trail the current head of master by one or more
commits. If we were to use Expeditor, we would either be doing a
rollup for a "release" that hadn't been fully validated yet (and might
indeed never be formally released), or doing a rollup for a "release"
that claims to include features that were only merged after the
validated release candidates were built!)

All this means that we must manually manage the rollup headers to
match the contents of the release candidate. In practice, this should
amount to moving around a few of Expeditor's HTML comments, and
creating a new header for the just-released version. Take a look at
the contents of the `CHANGELOG.md` file for the pattern. You should
also consult [Expeditor's CHANGELOG
documentation](https://expeditor.chef.io/docs/reference/changelog/)
for additional details.

## Update the Acceptance environment with the new hab-backline

While buildkite handles adding the new stable backline version to acceptance, updating the new unstable
version must be done manually. In order to do this, (from a Linux machine):

```
./update-hab-backline.sh unstable $(< VERSION)
```

If your auth token isn't specified in your environment, you can add `-z <AUTH_TOKEN>`
(or any other arguments to pass to the `hab pkg upload` command) to the
`update-hab-backline.sh` script after the channel and version arguments.

NOTE: Until Builder automatically builds linux2 packages in response to web hook activity, you may need to manually trigger a build after you've merged the version bump PR. If that is the case, you can use the CLI:

```sh
hab bldr job start core/hab-backline x86_64-linux-kernel2
```

Once the Acceptance Builder is doing this, then we will no longer need to worry about this step.

Make sure the commands from the trace output look correct when the script executes:
1. The version is the new dev version after the one we just released; there should be a `-dev` suffix
1. The install is from the `unstable` channel
1. The upload is to the `stable` channel

## The Builder Worker

Now that the release is stable, we need to build a new version of builder-worker and promote it. The easiest way to do this is to use the CLI to trigger builds for all three platforms:

```sh
hab bldr job start habitat/builder-worker x86_64-linux
hab bldr job start habitat/builder-worker x86_64-linux-kernel2
hab bldr job start habitat/builder-worker x86_64-windows
```

When these are all done, promote the resulting artifacts to the `stable` channel (do this for each of the three build jobs):

```sh
hab bldr job promote ${BUILD_GROUP_ID} stable
```
(`$BUILD_GROUP_ID` is given in the output of each `hab bldr job start` command.)


Wait for a few minutes so that supervisors on all the workers can update to the newly promoted version, then perform a test build. Check the build log for the test build to confirm that the version of the Habitat client being used is the desired version.

# Release Notification

1. Create new posts in [Habitat Announcements](https://discourse.chef.io/c/habitat) on the Chef discourse as well as [Announcements](https://forums.habitat.sh/c/announcements) in the Habitat forums.
1. Tweet a release announcement from `@habitatsh`.
1. Link forum posts to the github release
1. Link github release to forum post
1. Announce that the "Freeze" on merges to master is lifted in both the Chef internal slack team and in the Habitat slack team.

# Update Cargo.lock

1. In the [habitat](https://github.com/habitat-sh/habitat) repo, run `cargo update`, `cargo check --all --tests`.
1. If there are warnings or errors that are simple, fix them. Otherwise, lock the appropriate versions in `Cargo.toml` files that lets the build succeed and file an issue to resolve the failure and relax the version lock.
1. Open a PR for the `Cargo.lock` updates and any accompanying fixes which are necessary.
1. Repeat with the [builder](https://github.com/habitat-sh/builder) repo (omit the `habitat-launcher` build).

# Release postmortem

If there were any problems with the release process that may benefit from changes to code
(including release automation itself), write up a brief description of what happened and
schedule a meeting with appropriate stakeholders to scope and prioritize the work. This isn't
a full-blown postmortem as described in https://github.com/chef/oc_post_mortems, so it should
be quick and relatively informal, but the fundamental goals are the same:
1. While context is fresh, help the team understand the what happened and why without placing blame
or speaking in counter-factuals (🙆: "I did…/I thought…", 🙅‍♂️: "I should've…/I would've…")
1. Agree on, assign and prioritize remediation items to ensure continuous improvement of our release process and codebase more generally

If the release truly had no problems at all, add a "Yay!" to [the retro board](https://trello.com/b/H3ysuKy9/habitat-retro) and celebrate our success as a team.

# "Yanking" a Release

In general, if an escaped defect is discovered in a Habitat release,
the first solution should be to "roll forward". That is, a patch that
addresses the defect (either by fixing it or reverting the code
responsible) should be merged to master, and then a new release is
built from that. This is the desired solution because it requires no
extraordinary measures by end-users to take advantage of the fix; they
only need to update their software as they would for any "normal"
release.

In the rare case that that is _not_ possible (e.g., the cause of the
defect is not readily apparent), you can remove or "yank" the
offending release, and fall back to the previous release. This is not
a perfect solution, however. The process and drawbacks are detailed
here.

It should be noted that yanking a release is a *last resort* action;
all efforts should be made to roll forward first. Here is a
(non-exhaustive) list of possible situations that may call for yanking
a release:

* The cause of the defect isn't apparent, but definitely isn't present
  in the previous release.
* The defect is severe enough that waiting for a new release isn't
  acceptable (becomes less an issue as our release automation matures).
* A simple revert wouldn't work due to infrastructure dependencies
  such as schema changes and properly testing the forward fix will be
  too time-consuming.

## Steps for Yanking a Release

1. Demote all packages produced during a release from the `stable` channel in Builder.

   This includes not only the `hab` CLI and Supervisor-related
   binaries, but _everything_, including Studio and exporters, and for
   all supported platforms. The list of specific package releases can
   be found in the `manifest.json` file located at

   ```
   https://chef-automate-artifacts.s3-us-west-2.amazonaws.com/files/habitat/${VERSION}/manifest.json
   ```
2. Put the last-known-good artifacts into the `stable` directory in
   S3. This amounts to copying the contents of

       https://chef-automate-artifacts.s3-us-west-2.amazonaws.com/files/habitat/${LAST_GOOD_VERSION}/

   into

       https://chef-automate-artifacts.s3-us-west-2.amazonaws.com/stable/latest/habitat/

   This makes it so our "curlbash" installer will pull the correct
   packages.

3. Revert the version's change for the Homebrew tap

   Clone the repository from
   https://github.com/habitat-sh/homebrew-habitat and revert the
   commit for the offending release (see [this
   example](https://github.com/habitat-sh/homebrew-habitat/commit/ee8d5d6857879ce067a84ad7819446f1bfff35f3)).

4. Unlist the Chocolatey version for Windows

   Chocolatey versions cannot, strictly speaking, be deleted, but only
   unlisted. However, since the Chocolatey release ultimately pulls
   binaries from Bintray currently, it ends up being a distinction
   without a difference.

   To unlist a version, go to https://chocolatey.org/packages/habitat/
   and scroll down to the `Version History` listing. Find the
   offending version and click the link for it in the `Listed` column
   (the link text will be "yes"). This will take you to a page where
   you can choose to unlist the package.

5. Container Images

   There are no steps currently needed to yank any container images,
   since the binaries pull them in specifially by version tag.

6. User Remediation Steps

   Though the above steps will prevent _future_ users from installing
   the offending release, it does nothing to fix users that have
   already downloaded the release. These are the steps they will need
   to take to "roll back" to the last stable release. These steps
   should be included in any communications that are sent out about
   the release.

   Rolling forward is preferable to yanking a release, because rolling
   forward does not require these manual remediation steps.

   (These examples are for the 0.80.0 release; change the versions as required.)

   On Linux:

       hab pkg uninstall core/hab/0.80.0
       hab pkg uninstall core/hab-sup/0.80.0

       rm /hab/cache/artifacts/core-hab-0.80.0-*
       rm /hab/cache/artifacts/core-hab-sup-0.80.0-*

   If a Launcher release must be pulled, make note of the version and
   delete any installed versions of it:

       hab pkg uninstall core/hab-launcher/${VERSION}
       rm /hab/cache/artifacts/core-hab-launcher-${VERSION}-*

   On Windows (Chocolatey) :

       hab pkg exec core/windows-service uninstall
       hab pkg uninstall core/windows-service

       choco uninstall habitat --version 0.80.0
       hab pkg uninstall core/hab-sup/0.80.0
       choco install habitat

   On macOS (Homebrew):

       brew uninstall hab
       brew install hab

# Creating a One-Off Release

See the [one-off-release tool](tools/one-off-release/README.md).
