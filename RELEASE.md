# Releasing Habitat

This document contains step-by-step details for how to release Habitat.

Note: this document covers what to do once you decide you want to make
an official release. If you would like further details on how the
overall pipeline works, please go [here](./.expeditor/README.md).


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


### Installation / Update
For each platform
([darwin](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-darwin.zip),
[linux](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-linux.tar.gz),
[windows](https://packages.chef.io/files/staging/habitat/latest/hab-x86_64-windows.zip)),
download the latest release candidate CLI from `packages.chef.io`. You
**must** have run the `/expeditor` Slack command above _before_
downloading the package!

Alternatively, you can either use our "curlbash" installation script
to install the above packages, or (if you're running on a system that
already has and older version of `hab` installed) you can upgrade
using `hab` itself, since the same CLI is available from
`packages.chef.io` as well as Builder.


#### Linux

Run either of the following:

``` sh
curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh \
    | sudo bash -s -- -c staging
```

```sh
sudo hab pkg install chef/hab --binlink --force --channel=staging
```

#### macOS

``` sh
curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh \
    | sudo bash -s -- -c staging
```

You cannot (yet) update using `hab` itself due to how the CLI is
currently installed on macOS.

#### Windows

``` powershell
iex "& { $(irm https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.ps1) } -Channel staging"
```
### What to Test

There may be special behavior related to this release that you will
want to validate but at the very least, you should validate the version,
try running some services, and exercising the Studio.

To ensure everything is working properly, you will need to have the
following environment variables set:

```sh
export HAB_INTERNAL_BLDR_CHANNEL=staging
export HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL=staging
```

On Windows, only `HAB_INTERNAL_BLDR_CHANNEL` needs to be set:

``` pwsh
$env:HAB_INTERNAL_BLDR_CHANNEL="staging"
```

`HAB_INTERNAL_BLDR_CHANNEL` is needed *outside* the Studio in order to
install the correct `chef/hab-studio` package to begin with. If you already
have this installed when executing `hab studio enter`, you can leave
this variable out.

`HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL` is needed so that the
inside the studio can install the Launcher and Supervisor
packages for the internal Supervisor from the appropriate place.

See https://github.com/habitat-sh/habitat/issues/4656 for further context and ideas.


Here is how you can validate the version of the cli, supervisor and the supervisor version inside the studio.

On Linux:
``` sh
hab --version
sudo -E hab sup --version

hab studio rm
hab studio enter
hab --version
sup-log
hab studio enter -D
hab --version
sup-log
```

On Windows:
``` pwsh
hab --version
hab sup --version
hab studio rm
hab studio enter
hab --version
Get-Supervisorlog
hab studio enter -D
hab --version
Get-Supervisorlog
```


Here are examples of what you might do with a Studio.

First, set up the studio properly. We clone `core-plans` to have
things to build, and we run `hab studio rm` to ensure a clean slate.

``` sh
mkdir testing
cd testing
git clone https://github.com/habitat-sh/core-plans
export HAB_INTERNAL_BLDR_CHANNEL=staging
export HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL=staging
hab studio rm
hab studio enter
```
Then, once inside the Studio, you could try these:

On Linux:

``` sh
# Does the version of the cli inside the studio match staging?
hab --version
# Does the version of the supervisor inside the studio match staging?
sup-log
^C
# build the redis plan
build core-plans/redis
source results/last_build.env
hab svc load $pkg_ident
sup-log
# Is redis running and accepting connections
^C
# Is it connectable?
hab pkg exec $pkg_ident redis-cli --stat
```

On Windows:
``` sh
# Does the version of the cli inside the studio match staging?
hab --version
# Does the version of the supervisor inside the studio match staging?
# The supervisor log will come up in a different window in a non-docker
# Windows Studio. Make sure to "accept" any windows firewall requests.
Get-SupervisorLog
# build the nginx plan
build core-plans/nginx
. results/last_build.ps1
hab svc load $pkg_ident
# Look at the log window to see if nginx running
# Is it responding?
Invoke-WebRequest http://localhost
```

Testing the Docker studio is identical, except you enter
using the following command instead:

```sh
hab studio enter -D
```

Test both in the native and docker studios on x86 linux and windows.

## Promote from Staging to Current

If all your manual validation works out, you should promote the
release candidate from the `staging` channel to the `current`
channel. Once again, we use the Slack command:

```
/expeditor promote habitat staging
```
This will result in the Supervisors in our Production Builder updating
themselves to the release candidate.

## Update Pending Release Notes

The `Pending Release Notes` on the habitat githib repo's wiki is a placeholder for content that is published to the chef.io release notes and the discource release announcement. The `finish_release` pipeline which runs after promoting habitat to stable in expeditor automates the process of extracting the pending notes and publishing to chef.io and discource. It is important to make sure that the [wiki page](https://github.com/habitat-sh/habitat/wiki/Pending-Release-Notes) is updated before promoting to stable.

## Promote from Current to Stable

Once you're satisfied with the new Supervisors in Production (you can access and navigate around https://bldr.habitat.sh/), you can
finish the release process by promoting from the `current` channel to
the `stable` channel.

```
/expeditor promote habitat current
```
This places all the release candidates into the `stable` channel,
making them "officially" available to the world, and thus "released".

# Post-Release Tasks
The Buildkite release is fairly-well automated at this point, but once it is complete, there are still a few remaining manual tasks to perform. In time, these will be automated as well.

## Update Homebrew

The [Habitat Homebrew](https://github.com/habitat-sh/homebrew-habitat) repository will automatically generate a PR to update the Homebrew tap after a release is promoted to stable. This PR will be tested to ensure the updated version is able to install.  If the PR checks are green, it is safe to merge and this will update our `brew install`ed version to the current stable release.

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

   (These examples are for the 2.0.134 release; change the versions as required.)

   On Linux:

       hab pkg uninstall chef/hab/2.0.134
       hab pkg uninstall chef/hab-sup/2.0.134

       rm /hab/cache/artifacts/chef-hab-2.0.134-*
       rm /hab/cache/artifacts/chef-hab-sup-2.0.134-*

   If a Launcher release must be pulled, make note of the version and
   delete any installed versions of it:

       hab pkg uninstall chef/hab-launcher/${VERSION}
       rm /hab/cache/artifacts/chef-hab-launcher-${VERSION}-*

   On Windows (Chocolatey) :

       hab pkg exec chef/windows-service uninstall
       hab pkg uninstall chef/windows-service

       choco uninstall habitat --version 2.0.134
       hab pkg uninstall chef/hab-sup/2.0.134
       choco install habitat

   On macOS (Homebrew):

       brew uninstall hab
       brew install hab
