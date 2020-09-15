+++
title = "Glossary"
draft = false

[menu]
  [menu.habitat]
    title = "Glossary"
    identifier = "habitat/reference/glossary.md Glossary"
    parent = "habitat/reference"
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/glossary.md)

Artifact
: A Chef Habitat artifact refers to a binary distribution for a given package built with Chef Habitat. A Chef Habitat artifact is a signed tarball with a `.hart` file extension. Chef Habitat artifacts are composed of a software library or application, the configuration information for that software, and lifecycle hooks. They are created from a the plan file, a `plan.sh` on Linux systems or a `plan.ps1` on Windows systems, and are built with Chef Habitat tools. Chef Habitat artifacts can be exported to a specific format, such as when creating a Docker image.

Artifact export formats
: Chef Habitat `.hart` files can be exported in a number of different formats depending on what you need and where you need it. This is powerful because you can use the same immutable Chef Habitat artifact by exporting it into a format that you need for a specific job. For example, when you can use one format for iterating locally in a Docker container, another to deploy that Chef Habitat artifact to an environment running Kubernetes, and a third to deploy it to a data center that's running virtual machines, but the Chef Habitat artifact is identical in each location. it's simply exported to the correct format for the job you are trying to do. You can read more about how to export Chef Habitat artifacts, and what exporters are currently available, [here](/docs/plan-overview/#pkg-exports).

Application lifecycle hooks
: Used in plans to define application lifecycle events, which allows the Supervisor to take actions in response to specific lifecycle events. Includes a set of TOML variables for generating configuration files with [configuration templates](/docs/reference/configuration-templates).

Binary wrapper
: While Chef Habitat provides the best behavior for applications that can be compiled from source into the Chef Habitat ecosystem, it can also bring the same management benefits to applications distributed in binary-only form.
You can write plans to package up these binary artifacts with minimal special handling. This article covers some tips and tricks for getting this software into Chef Habitat.

Builder
: Users have the option to connect their GitHub repositories to Builder to enable continuous builds of their plans. Checking in new code to GitHub initiates a new build through a GitHub hook. If you've added your Chef Habitat plan to the root of your source code repository and your project depends on any of the Chef Habitat Core Packages (for example, openssl, ruby, or node), when these packages are updated, Builder automatically rebuilds your software and posts the updated package to your project's `unstable` channel, where it will wait until you review and promote it according to your regular release procedure.

Builder on-prem
: In addition to our hosted service, we also support installing and running a Chef Habitat Builder Depot on-premises, using your own network and infrastructure, which allows you to choose from a wider selection of authentication providers and to manage how Builder fits into your existing CI/CD processes. Currently, our on-premises Builder depot only stores packages for download and upload by Supervisors and Studios. We intend to bring the full capabilities of Chef Habitat Builder to our on premises option in the future.   For a detailed explanation of features, requirements and setup instructions, [see the GitHub repository](https://github.com/habitat-sh/on-prem-builder).

Channel
: Channels are a best practice in CI/CD, they allow you to gate a package for testing  before making it the default version of the package that users should consume. You can think of this split as the difference between test and production, or nightly releases versus stable releases of products. A channel is a tagged package location in Builder, which is used for managing the application lifecycle. Supervisors can subscribe to channels and take actions in response to changes within channels that are described in a plan's application lifecycle hooks.

Fully-qualified package identifier (FQPI)
: Four component name for a package, in the format: `origin/name/version/release`. For example, `core/glibc/2.22/20160310192356`.

`hab`
: The Chef Habitat CLI (command-line interface). `hab` subcommands for executing package builds, loading services into the process Supervisor, uploading packages to Builder, and entering the Studio. For more information on all of the functionality of `hab` checkout our [CLI command documentation](/habitat/reference/habitat-cli).

Hooks
: Each plan can specify application lifecycle event handlers, or hooks, to perform certain actions during a service's runtime. Each hook is a script with a shebang defined at the top to specify the interpreter to be used. To see a full list of available hooks and how to use them check out our [hooks documentation](/docs/plans/application-lifecycle-hooks).

Launcher
: The sidecar process for launching processes from the Chef Habitat Supervisor. It is the entry point for running the Supervisor and is the Supervisor for the Supervisor. Whereas the Supervisor is able to automatically update itself, the Launcher is currently released a bit differently, by design; it should be rare that the Launcher ever needs to change.

Leader/follower
: This topology allows a distributed application running on at least three Chef Habitat nodes to use a leader/follower configuration. Leaders are elected with Chef Habitat's leader election algorithm, and followers are restarted to reflect a configuration that follows the new leader. Subsequent elections due to leader failure will update both leader and follower configuration data.

Leader election
: A algorithm for selecting a leader deterministically for the Supervisor group. We rely on the eventual consistency of every Supervisor's census entry to elect a new leader in a reasonable amount of time.

Origin key pairs
: TODO:

Package identifier
: specified using the two-component form `origin/name`. For example, `core/redis` or `core/openssl`. Use the three-component form `origin/name/version`, such as `core/redis/5.0.4`, when you need a specific version of an artifact.

Plan
: A plan is a set of files that describe how to build a Chef Habitat package. At the heart of the plan is a configurable script named `plan.sh` for Linux and `plan.ps1` for Windows, containing instructions on how to download, compile, and install its software.

Callbacks
: Used in plans to override build phase defaults.
Chef Habitat's build phase defaults.

Census
: The census is the core of the service discovery mechanism in Chef Habitat. It keeps track of every Supervisor in the ring, and handles reading, writing, and serializing it with the discovery backend. Each Supervisor in the system is a *census entry* that together form a *census*. Operations to discover or mutate the state of the census happen through algorithms that arrive at the same conclusion given the same inputs, such as leader elections.

Control gateway
: The Supervisor control gateway is used to issue commands to a remote Supervisor. When a new Supervisor is created, a key for the `HAB_CTL_SECRET` environment variable is generated for it by default, if one is not already present; this key is used to authenticate requests that are made via the control gateway. See the [control gateway](/docs/internals#control-gateway) documentation for more details.

Ring encryption key
: Use to encrypt *all* supervisor-to-supervisor communication. Type: shared symmetric key.

Scaffolding
: Standardized plans for automated building and running your application for Ruby, Node.js, and Gradle. Each scaffolding is tuned to your application and helps you automatically create the appropriate [application lifecycle hooks](/docs/glossary/glossary-hooks) and runtime dependencies for building the package from your plan. Scaffoldings provide some default health check hooks where appropriate to ensure your application is functioning reliably.

Service
: TODO:

Service group
: A set of one or more running services with a shared configuration and topology makes up a service group. If a service is started without explicitly naming the group, it's assigned to the `default` group for the name of that package. For example:

  - `redis.default`
  - `postgres.financialdb` (possibly running in a cluster)
  - `postgres.userdb` (possibly running in a cluster)

Service group key
: TODO:

Standalone
: This is the default topology, useful for services inside a group that are completely independent from one another. Note that this still means they can share the same configuration.

Studio
: A clean, self-contained, minimal environment in which you can develop, build, and package software that is free from any upstream operating system distribution. All tools and dependencies included in the Studio are installed through Chef Habitat packages, thus preventing any unwanted dependencies from being used by your package.

Supervisor
: The Supervisor is a process manager that has two primary responsibilities. First, it starts and monitors child services defined in the plan it is running. Second, it receives and acts upon information from the other Supervisors to which it is connected. A service will be reconfigured through application lifecycle hooks if its configuration has changed.

Supervisor API
: The Chef Habitat Supervisor provides a HTTP API to expose cluster metadata, statistics, and general diagnostic information useful for monitoring and support in the form of a JSON document. It also provides detailed information about the Chef Habitat package that it is supervising, including metadata such as the build and runtime dependencies and their versions.

Ring
: Supervisors typically run in a network, which we refer to as a *ring* (although it is more like a peer-to-peer network rather than a circular ring). The ring can be very large; it could contain hundreds or thousands of supervisors. The membership list of this ring is maintained independently by each Supervisor and is known as the *census*.

Topology
: Chef Habitat allows you to define the behavior of your service groups as "standalone" or "leader/follower"

User key
: TODO:
