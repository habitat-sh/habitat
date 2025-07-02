Habitat CI/CD Pipeline Overview
===============================

The Habitat project practices continuous delivery for generating releases of the core Habitat packages (the `hab` CLI, the Habitat Supervisor, the Habitat Launcher, the Habitat Studio, various package exporters, and so on). This is achieved through a number of related Buildkite pipelines that feed into each other to create a larger overall pipeline. This overall pipeline is governed by the contents of Expeditor's [config.yml](config.yml) file, as well as the various `*.pipeline.yml` files in the [.expeditor](.) directory. This document serves to provide a friendly overview of how everything flows together, but the code is the definitive source of truth.

For additional background, please consult the documentation for [Expeditor](https://expeditor.chef.io), Chef's release engineering coordination system, as well as that of [Buildkite](https://buildkite.com/docs), the CI/CD execution engine we use.

# Verify Pipeline

Defined in [verify.pipeline.yml](verify.pipeline.yml), this pipeline is run for every PR that is created in Github (you will need a Habitat team member to "unblock" the pipeline run in Buildkite if you yourself are not an authorized Buildkite user in the Chef account). It runs linting, static analysis, unit tests, and builds Habitat packages (to ensure they still _can_ be built; none of these packages will ultimately be release candidates, though.)

As new commits are pushed to an existing PR, any currently running pipeline for that PR will be cancelled, and a new one will be started.

If any stages fail in this pipeline, you can freely retry them. Similarly, you can freely retry the entire pipeline.

A PR should only be merged if its corresponding verify pipeline has run successfully (this is the default behavior, but repository administrators have the option to override this at their discretion).

[Buildkite Page for verify pipeline](https://buildkite.com/chef/habitat-sh-habitat-master-verify)

# Release Pipeline

Defined in [release_habitat.pipeline.yml](release_habitat.pipeline.yml), this pipeline runs after a PR has been merged to the `master` branch of the repository. Here, we build all the Habitat packages in the repository, for all the platforms we support (currently, `x86_64-linux`, `aarch64-linux`, and `x86_64-windows`). In particular, we build the packages in dependency order. Everything up to and including the Habitat Studio are built with the currently-released stable Habitat toolchain. Then, once we've built a Studio, we switch to building the rest of the packages with the new toolchain. (This also serves as a kind of implicit acceptance test for the build system itself.)

We also create the various Studio containers for this release and upload them to Docker Hub. This is a safe operation currently because the specific container image being used depends on the version of Habitat that is requesting it (i.e., when a Habitat Studio starts up in Docker-mode, it doesn't just pull the `latest` version of the image).

In addition to creating packages, we also create slimmed-down tarballs for the `hab` CLI. These contain only the `hab` CLI binary (and, on Windows, any needed DLLs). These are ultimately what is pulled down when installing Habitat via the ["curl | bash"](../components/hab/install.sh) installer script.

Finally, we also create a tarball of any documentation that is dynamically-generated based on the state of the code at this commit. This will be used later to update our documentation site.

After a successful release pipeline run, all artifacts are promoted to the `dev` channel for subsequent validation.

[Buildkite Page for release_habitat pipeline](https://buildkite.com/chef/habitat-sh-habitat-master-release-habitat)
## "Staging Area" Concept

Since we can merge several PRs in rapid succession, and since the entire release pipeline may take on the order of an hour, we have to have some way to manage when new release pipeline runs are triggered. Fortunately, Expeditor provides the notion of [staging areas](https://expeditor.chef.io/docs/patterns/staging-areas/). Here, this effectively means that any PRs that are merged while a release pipeline is running are grouped together until the pipeline finishes (whether successfully or not). Then, a single new release pipeline run is triggered on all the code that has been merged since the last pipeline was triggered.

This means that there should only ever be one instance of the release pipeline running at any given time, and that it should only be started by Expeditor itself in response to a PR being merged. It also generally means that an overall pipeline failure should not be retried, but rather initiated by merging another PR. You can retry a failed _stage_ in the pipeline if the overall pipeline itself is still running, but attempting to re-run a failed pipeline won't work out like you want. Expeditor may have already kicked off another pipeline run based on newly-merged PRs. Additionally, pipelines are granted time-limited credentials by Expeditor which may have already expired. Finally, each pipeline run is associated with an ephemeral Builder channel (see [below](#where-things-are-uploaded-to) for more) that is deleted at the end of the pipeline run; attempting to re-run stages after the channel has been deleted will not end well. In general, unless you know _precisely_ how everything is wired up, your best bet is to simply run more code through the pipeline.

Despite this, there are some safeguards in place to prevent bad things from happening if you need to manually trigger a pipeline run (perhaps to test some changes to the pipeline logic itself). In particular, stages that upload packages or containers, or otherwise have "global" effects outside the pipeline itself, are guarded such that they will only execute if Expeditor is the entity that started the pipeline run. Even so, only do this if you _know_ that you need to do this.

## Where Things are Uploaded To

During the release pipeline, as Habitat packages are created, they will be uploaded into a "release channel". This is an ephemeral channel that exists only for the duration of the pipeline run, and will be deleted when it ends, whether it succeeded, failed, or was cancelled. The channel is named `habitat-release-${BUILDKITE_BUILD_ID}`, where `${BUILDKITE_BUILD_ID}` is the unique identifier provided by Buildkite for this run of the pipeline. This serves as a holding area for the packages as they are built. If the pipeline is successful, all these packages are promoted to the `dev` channel, where they will be further tested in the [End-to-End Pipeline](#end-to-end-pipeline).

If the pipeline fails, any packages that were successfully built still remain in Builder, but the channel itself will be deleted.

However, we create more than just Habitat packages; we also have `hab` CLI binary-only tarballs, documentation tarballs, and container images. The container images are uploaded to [the `habitat` organization on DockerHub](https://hub.docker.com/orgs/habitat/). Everything else is uploaded to the S3 bucket that backs https://packages.chef.io. In particular, they are uploaded to `s3://chef-automate-artifacts/files/habitat/${version}` (the bucket name is a historical artifact), where `${version}` is the version of the Habitat packages being built (see the [VERSION](../VERSION) file).

These artifacts are also promoted, but this has a slightly different connotation, since these are not stored in Builder (they are not `*.hart` files, after all). Instead, the contents of the `files/habitat/${version}` directory in the S3 bucket are copied (_not_ moved!) into `s3://chef-automate-artifacts/${channel}/latest/habitat`. At the end of the release pipeline, `${channel}` is set to "dev". This pattern is common to other pipelines within Chef engineering; feel free to talk with anyone from the Release Engineering team for additional details.

### Manifest File

In addition to uploading the various tarballs to S3, we also create a "manifest" JSON file that describes the packages that this release comprises. This is an ad-hoc format we have created, generated from [create_manifest.rb](./scripts/create_manifest.rb). It looks like this:

```json
{
  "schema_version": "1",
  "version": "2.0.134",
  "sha": "cf0e228b01d76440487593641e6b732da4b0e193",
  "packages": {
    "x86_64-linux": [
      "chef/hab/2.0.134/20201030172917",
      "chef/hab-backline/2.0.134/20201030180709",
      "chef/hab-launcher/14772/20201030181158",
      "chef/hab-pkg-export-container/2.0.134/20201030181204",
      "chef/hab-pkg-export-tar/2.0.134/20201030181205",
      "chef/hab-plan-build/2.0.134/20201030180522",
      "chef/hab-studio/2.0.134/20201030180823",
      "chef/hab-sup/2.0.134/20201030181202"
    ],
    "aarch64-linux": [
      "chef/hab/2.0.134/20201030172917",
      "chef/hab-backline/2.0.134/20201030180714",
      "chef/hab-launcher/14772/20201030181209",
      "chef/hab-pkg-export-tar/2.0.134/20201030181209",
      "chef/hab-plan-build/2.0.134/20201030180519",
      "chef/hab-studio/2.0.134/20201030180830",
      "chef/hab-sup/2.0.134/20201030181209"
    ],
    "x86_64-windows": [
      "chef/hab/2.0.134/20201030173308",
      "chef/hab-launcher/14772/20201030181258",
      "chef/hab-pkg-export-container/2.0.134/20201030181259",
      "chef/hab-pkg-export-tar/2.0.134/20201030181255",
      "chef/hab-plan-build-ps1/2.0.134/20201030180619",
      "chef/hab-studio/2.0.134/20201030180915",
      "chef/hab-sup/2.0.134/20201030181309",
      "core/windows-service/0.6.1/20201030182923"
    ],
    "x86_64-darwin": [
      "chef/hab/2.0.134/20201030173740"
    ]
  }
}
```
This file is generated at the very end of the release pipeline, and is used by subsequent pipelines to ensure that we are operating on the precise Habitat packages we expect.

(This approach is similar to that taken with Automate.)

# Questions You May Have

## Why aren't these Habitat packages being built in Builder?

The main reason is that many of these packages have "hidden" dependencies on each other that aren't expressed as either Habitat build-time or run-time dependencies. For instance, if all you have is the `hab` binary and you run `hab sup run`, we transparently download the appropriate `chef/hab-sup/${HAB_BINARY_VERSION}` package from Builder, install it, and then execute the `hab-sup` binary from that package. This relationship is currently only expressed in the actual Rust code of the CLI, meaning that it is simply not available to Builder. Similar logic holds for the Studio, the Launcher, and the exporters. These dependencies could be expressed as formal Habitat dependencies, allowing Builder to act upon them, but would require some rethinking of how we onboard new users (since there wouldn't be any "download a single binary to get started" workflow). Alternatively, if we sorted out a way to package everything into a single binary, the point would be moot (though then we'd need to take a look at how we handle Supervisor self-updates, among other things). These aren't impossible to address, but it does explain the current state of the world.

# End-to-End Pipeline

After the release pipeline completes successfully, Expeditor will trigger a new run of the end-to-end testing pipeline, defined in [end_to_end.pipeline.yml](end_to_end.pipeline.yml). This pipeline defines a number of testing scenarios that exercise the packages that were built in the release pipeline (specifically, it uses packages that are in the `dev` channel on Builder).

[Buildkite Page for end-to-end pipeline](https://buildkite.com/chef/habitat-sh-habitat-master-end-to-end)

## Why so many stages?

There are numerous stages in this pipeline. Since there isn't an off-the-shelf framework for running arbitrary system tests in parallel, in isolation, in a cross-platform way (at least, not one that we're aware of), we have twisted Buildkite to our needs, along with a dash of Powershell.

Each stage (with exception of the "Docker End-to-End Supervisor Tests"; more on that later) essentially runs a single script that tests a particular scenario (or a handful of closely-related scenarios). This is how we get parallelism. Additionally, the tests themselves are _generally_ written in Powershell, using [Pester](https://pester.dev). This allows us to run the same tests on both Linux and Windows, thus unlocking cross-platform testing.

(There are also a small number of tests written as an experiment in [Expect](https://core.tcl-lang.org/expect/index), which is kind of fun, but a bit difficult to scale with current team resources and knowledgebase.)

This makes it relatively easy to write new tests, but managing the overhead of pipeline definition file is becoming a bit burdensome. Such extreme parallelism can also be a bit wasteful of resources.

All these tests are, broadly speaking, testing individual interactions with the `hab` CLI or studio. In order to test some more interesting interactions of the Supervisor, we have a small suite of `docker-compose`-based tests that set up small, self-contained networks of Supervisors and makes various assertions on their interactions. Please see [the documentation](../test/end-to-end/multi-supervisor/README.md) for more detail on these tests. (These tests do _not_ currently run in parallel, though they could be made to do so with a little effort. They only run on Linux at the moment; making them also run on Windows would take more effort.)

If all tests run successfully, all artifacts in both Builder and packages.chef.io are promoted to their respective `acceptance` channels to await further processing.

## Manual Runs

As with the release pipeline, it's best to let Expeditor run the end-to-end pipeline in response to successful runs of the release pipeline. However, it is sometimes useful for developers to trigger individual runs in order to test changes to the test code itself. There are guards in place here to block promotion to the `acceptance` channel if the pipeline is not triggered by Expeditor. Be aware, though, that the packages being tested will come from the `dev` channel, while your tests will be coming from whatever branch you are triggering the pipeline from; adjust your expectations accordingly.

## Source of Tests

At the moment, Expeditor does _not_ tie an end-to-end pipeline run to the same git SHA that was used to build packages in the release pipeline; it instead runs from whatever is currently on the `master` branch. Thus, the packages being tested will _not necessarily_ come from the same SHA that the end-to-end testing code comes from. For instance, you may merge some code that changes the tests while a release pipeline is running. Those packages will then end up being tested by code from a _later_ commit.

In general, this is not a problem, but the possibility should be kept in mind. This is something that Release Engineering will likely address at some point in the future.

# Promotion to Subsequent Channels and Ultimate Release

This covers the broad strucutre of the pipeline to this point. Subsequent actions are under manual control of team members, and are documented in [RELEASE.md](../RELEASE.md).


# Finish Release Pipeline

Once the release candidates have been promoted to the stable channel, Expeditor will kick off a ["finish release"](finish_release.pipeline.yml) pipeline. This performs a handful of maintenance tasks, including creating new release artifacts for our Windows build in [Chocolatey](https://chocolatey.org/packages/habitat).

[Buildkite Page for finish_release Pipeline](https://buildkite.com/chef/habitat-sh-habitat-master-finish-release)

# Other Pipelines
## Homebrew
We maintain a [Homebrew tap](https://www.github.com/habitat-sh/homebrew-habitat) for our macOS Habitat releases. Unlike the explicit triggering of the update process that we do for our Windows Chocolatey package in our "finish release" pipeline, the updating of the Homebrew tap is handled implicitly, using [an Expeditor subscription](https://github.com/habitat-sh/homebrew-habitat/blob/03bcd2cc03ed3e2a41ea5b382b2ff88ecbc93568/.expeditor/config.yml#L12-L15).

This process ultimately results in a new PR being opened in the `habitat-sh/homebrew-habitat` repository that updates the release. Once this PR is merged, the new Habitat CLI will be available via Homebrew.

[Buildkite Page for homebrew-habitat verify Pipeline](https://buildkite.com/chef/habitat-sh-homebrew-habitat-master-verify)
