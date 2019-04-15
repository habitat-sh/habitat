# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Monday.

# Create an issue to track progress from the template

1. Create an issue which will track the progress of the release with [this template](https://github.com/habitat-sh/habitat/issues/new?template=release-checklist.md).
1. Check off the items in the list as you go.
1. If you make any changes to the release automation or documentation to include in the next release, you can mark those PRs as resolving the issue. Otherwise, just close it when the release is done.

## Releasing Launcher

The [`core/hab-launcher` package](https://bldr.habitat.sh/#/pkgs/core/hab-launcher), which contains
the `hab-launch` binary has a separate release process. See
[its README](components/launcher/README.md) for details. The Buildkite pipeline does not build a `core/hab-launcher` package; that must currently be built out-of-band and uploaded to Builder. The Buildkite pipeline _will_ however take care of _promoting_ it for you. The pipeline will guide you as appropriate.

## Prepare Master Branch for Release

1. Call for a "Freeze" on all merges to master and set the topic in #hab-team to indicate that it
is in effect

1. Clone the Habitat repository if you do not already have it

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/habitat
    ```

1. Ensure you are on the master branch and have the latest of `~/habitat`:

    ```
    $ cd ~/habitat
    $ git checkout master
    $ git pull origin master
    ```

1. Create a new release branch in the Habitat repo. You can call this branch whatever you wish:

    ```
    $ cd ~/habitat
    $ git checkout -b <branch>
    ```

1. Remove the `-dev` suffix from the version number found in the `VERSION` file. *Note*: there must not be a space after the `-i`.

    ```
    $ sed -i'' -e 's/-dev//' VERSION
    ```
1. If necessary, fix up any issues with `CHANGELOG.md`, such as PRs that were missing the `X-` label and didn't get put in the correct category.

1. Commit `VERSION` changes and push your branch.
1. Issue a new PR with the `Expeditor: Exclude from Changelog` label and await approval (in the form of a [dank gif](http://imgur.com/X0sNq)) from two maintainers.
1. Pull master once again once the PR is merged into master.
1. Create & push a Git tag:

    ```
    $ make tag-release
    ```

If there are problems discovered during validation, or you need to modify the tag to include
additional commits, see [Addressing issues with a Release](#addressing-issues-with-a-release).

Once the release tag is pushed, a Buildkite build will be triggered on the release tag.

You can view/adminster Buildkite builds [here](https://buildkite.com/chef/habitat-sh-habitat-master-release).
When you get to the "Ensure that Builder is stable on the new release" step, you need to ssh to the various
hosts and confirm that the new version of the supervisor is running and check the builder dashboards
(see https://forums.habitat.sh/t/on-call-engineering-duties/626). To quickly check the versions of the supervisor,
you can run ([`hab-instances`](https://github.com/habitat-sh/builder/blob/master/tools/ssh_helpers/hab-instances) is in the `builder` repo and assumes you already set up the [ssh-helpers](https://github.com/habitat-sh/builder/blob/master/tools/ssh_helpers/Usage.md)):
```bash
for host in $(./tools/ssh_helpers/hab-instances live | jq -r '.Reservations[] | .Instances[0] | .PublicDnsName + ";" + (.Tags | from_entries | ."X-Environment" + "-" + .Name)' | cut -d';' -f2); do
    ssh "$host" pgrep -afl hab-sup | grep -E "/\d+\.\d+\.\d+/"
done
```
This won't work for Windows workers (they require RDP), so you can confirm Windows worker services are running via the Builder dashboard.

The release tag builds will upload all release binaries to a channel named `rc-[VERSION]` and the `hab` cli will be uploaded but _not_ published to the `stable` Bintray repository. These builds can take about 45 minutes to fully complete. Keep an eye on them so we can validate the binaries when they finish.

## Validate the Release

For each platform ([darwin](https://bintray.com/habitat/stable/hab-x86_64-darwin), [linux](https://bintray.com/habitat/stable/hab-x86_64-linux), [linux-kernel2](https://bintray.com/habitat/stable/hab-x86_64-linux-kernel2), [windows](https://bintray.com/habitat/stable/hab-x86_64-windows)), download the latest stable cli version from [Bintray](https://bintray.com/habitat/stable) (you will need to be signed into Bintray and a member of the "Habitat" organization). These can be downloaded from the version files page but are unpublished so that our download page does not yet include them. There may be special behavior related to this release that you will want to validate but at the very least, do the following basic tests.

You need to set `HAB_INTERNAL_BLDR_CHANNEL` and `CI_OVERRIDE_CHANNEL` to the name of the release channel (you _may_ also need to set `HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL` and `HAB_STUDIO_SECRET_CI_OVERRIDE_CHANNEL` for non-Docker-based studio). If a new Launcher is in the release channel, you should be fine; however, since that should be rare, you may have some additional work.

NOTE: If you are running `sudo hab studio enter` with all the required environmental variables set, but it's still telling you that it cannot find the package in stable, try `sudo -E hab studio enter`.

In a previous release, we were able to validate things on Linux by re-using a chroot studio and installing a Launcher out-of-band. You can probably create a new studio, enter it with `HAB_STUDIO_SUP=false`, manually install the latest stable Launcher (if a new one isn't part of the current release), exit the studio, then re-enter with `HAB_STUDIO_SUP` unset (but with all the override variables mentioned above set). This should reuse the Launcher you just installed, but pull in additional artifacts as needed from your release channel.

See https://github.com/habitat-sh/habitat/issues/4656 for further context and ideas.

Then you can actually exercise the software as follows:

1. It pulls down the correct studio image
1. That studio's `hab` is at the correct version (`hab --version`)
1. A `sup-log` shows a running supervisor (if `sup-log` does not show a supervisor running, run `hab install core/hab-sup --channel release_channel` then `hab sup run`)
1. Verify that the supervisor is the correct version (`hab sup --version`)

When testing the linux studio, you will need to `export CI_OVERRIDE_CHANNEL` to the rc channel of the release. So if you are releasing 0.75.2, the channel would be `rc-0.75.2`.

### Validating x86_64-linux-kernel2

For this PackageTarget it is important that you perform validation on a Linux system running a 2.6 series kernel. CentOS 6 is recommended because it ships with a kernel of the appropriate age,  but any distro with a Kernel between 2.6.32 and 3.0.0 can be used. Included in the `support/validation/x86_64-linux-kernel2` directory in this repository is a Vagrantfile that will create a CentOS-6 VM to perform the validation. You can also run a VM in EC2.

The Vagrantfile is configured to grab the [core-plans](https://github.com/habitat-sh/core-plans) repository (to give you something to build), as well as grab the secret key for your `HAB_ORIGIN` (using the `HAB_ORIGIN` and `HAB_AUTH_TOKEN` variables in your environment). You'll need to manually install the release-candidate `hab` binary and set your various channel overrides, but other than that you should have all you need to test things out.

As an example, immediately after provisioning you can SSH into the machine and run `HAB_ORIGIN=<my_origin> hab pkg build core-plans/redis`.

### Addressing issues with a Release

If you find issues when validating the release binaries that must be fixed before promoting the release, you will need to fix those issues and then have Buildkite and AppVeyor rerun the deployment. After you merge the necessary PRs to fix the release issues:

```
     $ make re-tag-release
```

# Post-Release Tasks
The Buildkite release is fairly-well automated at this point, but once it is complete, there are still a few remaining manual tasks to perform. In time, these will be automated as well.

## Update Builder Bootstrap Bundle

Once the Buildkite linux deployment has completed, we generate a release bundle of all Habitat and Builder components which are uploaded to an S3 bucket which we read from when we bootstrap new nodes. This bundle is useful if you are bootstrapping in an environment which doesn't have access to Builder or there simply isn't a Builder instance in existence (ah, those were the days).

NOTE: Do this step from either a Linux VM or in a studio.

1. Configure your AWS credentials in your environment.

In general, this means ensuring that `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` are present in your environment.

_However_, if you are using [okta_aws](https://github.com/chef/okta_aws) (and if you're working at Chef, you should be!), things are a little bit different.

In this case, you will need to run the following:

```sh
okta_aws habitat
export AWS_DEFAULT_PROFILE=habitat
```

This ensures that the script can access your appropriate Okta-mediated credentials.

1. Execute the script that currently lives in the [builder](https://github.com/habitat-sh/builder) repository:

    ```
    $ cd /path/to/builder-repo
    $ sudo terraform/scripts/create_bootstrap_bundle.sh <HABITAT_VERSION>
    ```

## Update Homebrew Tap

This should be automatically handled by Buildkite. You can find manual instructions in [a previous version of this file](https://github.com/habitat-sh/habitat/blob/267d31f03a00dfa3b1b8e0ba00c20efa4913a7a8/RELEASE.md).

Validate the update by running `brew upgrade hab` on Mac OS X and checking the version is correct.

## Push Chocolatey package

Until Buildkite integrates the Chocolatey package creation and upload, we need to run `support/ci/choco_push.ps1` from a Windows machine that has the `choco` cli installed.

The `choco` cli can be installed via:
```
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
```

Now run:
```
.\support\ci\choco_push.ps1 -Version [VERSION] -Release [RELEASE] -ApiKey [CHOCO_API_KEY] -Checksum [BINTRAY_PUBLISHED_CHECKSUM]
```

`CHOCO_API_KEY` can be retrieved from 1password. `BINTRAY_PUBLISHED_CHECKSUM` should be the checksum in `windows.zip.sha256sum` file uploaded to bintray.


## Publish Release

Create release in [GitHub](https://github.com/habitat-sh/habitat/releases)

On the GitHub releases page, there should already be a tag for the release (pushed up previously).
Draft a new Release, specify the tag, and title it with the same (eg, 0.18.0). Then hit Publish Release.

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

Verify the diff looks reasonable and matches the newly released version, then submit your PR. Until
https://github.com/habitat-sh/habitat/issues/5948 is fixed, this may require some manual fixup. In particular,
make sure that https://github.com/habitat-sh/habitat/blob/master/www/source/partials/docs/_reference-template-data.html.md.erb doesn't remove the `sys` section. If https://github.com/habitat-sh/habitat/issues/5948 is fixed,
update these instructions.

## Verify the Docs

After the PR that updates the generated docs is merged, the [deploy_website.sh](https://github.com/habitat-sh/habitat/blob/master/support/ci/deploy_website.sh)
script will run to redeploy the website with the updated content. Verify that this was successful
at https://www.habitat.sh/docs. If not, check https://travis-ci.org/habitat-sh/habitat/builds
and find the "Merge pull request #XXX" run where `XXX` was the number of your PR from the
previous step to see if there were any errors during the deploy process.

## Update the Changelog

We currently use Expeditor (an internal tool) to _partially_ manage our changelog. It adds items to `CHANGELOG.md` for every PR that is merged, based on certain labels that are attached to the PR. This is all well and good.

However, due to our versioning scheme (specifically, the use of the `-dev` suffix), we can't yet take advantage of Expeditor's built-in version bumping capabilities. This will change soon, but in the meantime, this means that we must manually add the release header to the changelog and do some re-arranging of additional headers.

In a nutshell, the top of the `CHANGELOG.md` file should be modified to look something like this:

```
# Habitat CHANGELOG

<!-- latest_release unreleased -->

## Unreleased

// Any merges after the release tag should be in this section

<!-- latest_release -->

## [<JUST_RELEASED_VERSION>](https://github.com/habitat-sh/habitat/tree/<JUST_RELEASED_VERSION>) (YYYY-MM-DD)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/LAST_VERSION...<JUST_RELEASED_VERSION>)
```

These are the only places in the file that the `latest_release unreleased` and `latest_release` comment lines should be.

For additional background, please consult [Expeditor's CHANGELOG documentation](https://expeditor.chef.io/docs/reference/changelog/).

# Drink. It. In.

## Bump Version

1. Update the version number found in the `VERSION` file to the next target release and append the `-dev` suffix to that number
1. Issue a PR and merge it yourself

> Example: If the release version was `0.9.0` then the contents of `VERSION` might read `0.10.0-dev` if your next target is `0.10.0`.

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
1. Repeat with the [core](https://github.com/habitat-sh/core) and [builder](https://github.com/habitat-sh/builder) repos (omit the `habitat-launcher` build).

# Update rustfmt
1. Using https://mexus.github.io/rustup-components-history/, find the most recent date that all the Tier1 platforms have a present `rustfmt`. For example: `nightly-2019-03-04`.
1. Update `get_current_toolchain` in [`support/ci/shared.sh`](https://github.com/habitat-sh/habitat/blob/master/support/ci/shared.sh#L16) to output the new nightly date.
1. Locally install the nightly toolchain and update the formatting. For example:
    ```
    ➤ rustup toolchain install nightly-2019-03-04
    ➤ rustup component add --toolchain nightly-2019-03-04 rustfmt
    ➤ cargo +nightly-2019-03-04 fmt
    ```
1. Open a PR and merge the toolchain update as well as any formatting changes.
1. Repeat with the `core` and `builder` repos.

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