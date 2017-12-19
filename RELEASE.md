# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Thursday.

## Prepare Master Branch for Release

1. Clone the Habitat repository if you do not already have it

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/habitat
    ```

1. Ensure you are on the master branch and have the latest of `~/habitat`

    ```
    $ cd ~/habitat
    $ git checkout master
    $ git pull origin master
    ```

1. Create a new release branch in the Habitat repo

    ```
    $ cd ~/habitat
    $ git checkout -b <branch>
    ```

1. Remove the `-dev` suffix from the version number found in the `VERSION` file

    ```
    $ sed -i '' -e 's/-dev//' VERSION
    ```

1. Generate a new `CHANGELOG.md`

    ```
    $ export GITHUB_TOKEN=<your-token>
    $ make changelog
    ```

The changelog generator will only process the last 1000 issues to prevent the generator from exceeding Github's rate limit threshold. This may produce a truncated changelog. If so, simply copy the last release in the generated changelog and paste it on top of the former changelog. Terrible we know but we have a better plan coming!

1. Commit the `CHANGELOG.md` and `VERSION` changes and push your branch
1. Issue a new PR await approval (in the form of a [dank gif](http://imgur.com/X0sNq)) from two maintainers
1. Pull master once again once the PR is merged into master
1. Create & push a Git tag

    ```
    $ make tag-release
    $ git push origin --tags
    ```

Once the release tag is pushed, Travis and Appveyor builds will be triggered that will upload all release binaries to a channel named `rc-[VERSION]` and the hab cli will be uploaded BUT not published to the `stable` bintray repository. These builds can take nearly an hour to fully complete. Keep an eye on them so we can validate the binaries when they finish.

## Validate the Release

For each platform, download the latest stable cli version from [Bintray](https://bintray.com/habitat/stable). These can be downloaded from the version files page but are unpublished so that our download page does not yet include them. There may be special behavior related to this release that you will want to validate but at the very least, run `hab studio enter` and make sure:

1. It pulls down the correct studio image
1. That studio's `hab` is at the correct version
1. A `sup-log` shows a running supervisor and the supervisor is the correct version

## Publish the release

```
$ export HAB_AUTH_TOKEN=<your-token>
$ export BINTRAY_USER=<your-bintray-user>
$ export BINTRAY_KEY=<your-bintray-api-key>
$ make publish-release
```

This should promote all RC packages to stable and publish the hab cli for each platform (linux, mac and windows).

## Update Builder Bootstrap Bundle

Once the travis linux deployment has completed, we generate a release bundle of all Habitat and Builder components which are uploaded to an S3 bucket which we read from when we bootstrap new nodes. This bundle is useful if you are bootstrapping in an environment which doesn't have access to Builder or there simply isn't a Builder instance in existence (ah, those were the days).

1. Configure your AWS credentials in your environment
1. Run the bundle build make task

    ```
    $ make bundle
    ```

## Update Homebrew Tap

We have our own [Homebrew tap](https://github.com/habitat-sh/homebrew-habitat) that will need
updating. You will need the following bits of information for the latest stable MacOS Bintray artifact:

* the new version number
* the new release
* the SHA256 of the Bintray zip file

With those in hand, update the
[formula](https://github.com/habitat-sh/homebrew-habitat/blob/5adccfd7bf7657e64abda659160ca116d8bdff1a/Formula/hab.rb#L3-L5),
and merge the changes to the master branch.

(This will be a temporary state of affairs; I'll be talking with Engineering Services soon to get their help with automating this, as well as other parts of our release process.)

## Publish Release

Create release in [GitHub](https://github.com/habitat-sh/habitat/releases)

On the Github releases page, there should already be a tag for the release (pushed up previously).
Draft a new Release, specify the tag, and title it with the same (eg, 0.18.0). Then hit Publish Release.

# Drink. It. In.

## Bump Version

1. Update the version number found in the `VERSION` file to the next target release and append the `-dev` suffix to that number
1. Issue a PR and merge it yourself

> Example: If the release version was `0.9.0` then the contents of `VERSION` might read `0.10.0-dev` if your next target is `0.10.0`.

# Troubleshooting Mac deployments

Travis runs the Mac deployment scripts via SSH to a headless Mac builder affectionately called `74.80.245.236`. You can SSH to this machine as `admin` using the `habitat-srv-admin` key available in 1password.

# If your Release is going to cause downtime

1. Put a scheduled maintanence window into PagerDuty so the on-call doesn't go off.
1. Pre-announce outage on Twitter & Slack #general channel. There is no hard rule for a length of time you need to do this ahead of a release outage.
1. When your release begins, post announcement in our statuspage.io with outage information.
1. Per [ON_CALL.md](#ON_CALL.md) you are responsible for updating status changes in statuspage.io as the downtime proceeds. You are not responsible for regular minutes or responding in any other venue.
1. When the downtime is over, announce the end of the outage via statuspage.io. It will automatically post an announcement to #general and twitter.

# Release Notification

1. Create a new post in [Habitat Announcements](https://discourse.chef.io/c/habitat)
1. Link forum post to the github release
1. Link github release to forum post
1. Tweet a link to the announcement @habitatsh
