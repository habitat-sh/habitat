+++
title = "Workflow Overview"

draft = false
[menu]
  [menu.automate]
    title = "Workflow Overview"
    parent = "automate/workflow"
    identifier = "automate/workflow/workflow.md Workflow Overview"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/workflow.md)

Workflow is a legacy feature for Chef Automate, which was designed for managing changes to both infrastructure and application code.

{{< warning >}}
Workflow is only available in Chef Automate for existing users. If you are not already using Workflow, but are interested in the solution it offers, please contact your sales or success representative for support with continuous integration pipelines.
{{< /warning >}}

Chef Automate accelerates the adoption of continuous delivery and
encourages DevOps collaboration. It provides a proven, reproducible
workflow for managing changes as they flow through the pipeline from a
local workstation, through automated tests, and out into production.

Chef Automate handles many types of software systems. Use it to:

* Upload new and updated cookbooks to the Chef Infra Server that manages your infrastructure and applications
* Publish new and updated cookbooks to a Chef Supermarket installation
* Release source code or build artifacts to a repository such as GitHub
* Push build artifacts to production servers in real time

## Pipelines

A pipeline is series of automated and manual quality gates that take
software changes from development to delivery. The goal of a pipeline is
to move changes from your workstation into production quickly and
safely.

Pipelines in Chef Automate have six stages: Verify, Build, Acceptance,
Union, Rehearsal, and Delivered. Changes progress from one stage to
another by passing a suite of automated tests. For the Verify and
Acceptance stages, explicit approval by a designated person is required
(in addition to the tests).

Here are the stages of a Chef Automate pipeline.

![](/images/automate/delivery_partial_workflow.png)

The tests within each stage are organized into phases. The stages and
the phases are fixed for all pipelines in Chef Automate. However, what
happens within any given phase is completely up to you---if you can
describe the activity in a Chef recipe, then you can make it happen in a
phase.

The following illustration shows the phases of each pipeline stage.

![](/images/automate/delivery_full_workflow.png)

## Projects

Chef Automate relies on git and uses its lightweight feature branches as
the mechanism for handling changes before they merge, as well as its
ability to perform merges automatically. Each pipeline has a designated
target branch into which it will merge approved changes. Chef Automate
uses a "gated master" model that manages merges to the target branch.
(In preparation for using Chef Automate, it is helpful if team members
understand how to use feature branches.)

Chef Automate uses projects to organize work across multiple teams. You
can create as many projects as you need. A common approach is to have
one project for each major component of your system. Each project has
its own git repository. (Chef Automate includes a git server for hosting
project repositories. It is also possible to integrate with GitHub and
GitHub Enterprise for the git-related aspects of the workflow.)

Organizations allow you to group related projects and provide scope for
authorization rules.

Each project has one or more pipelines. The typical setup is for each
project to have a single pipeline that targets the master branch.

Having multiple pipelines allows the project to target different
branches for different changes. A potential use case is maintaining
different versions of a project on different branches, enabling you to
target a change (for instance, a security fix) against multiple versions
quickly and easily.

## Changes and Project Pipelines

Let's walk through what happens as a change makes its way through Chef
Automate. We'll assume you have created a project in Chef Automate and
want to make a change.

You start with a local checkout of the project's git repository. You
create a feature branch, make a change in that branch and test it
locally. When you're ready, submit the change using the
`delivery review` command (part of the Chef Automate command line tool).
This command submits the change to Chef Automate and kicks off the
pipeline. The command is the equivalent to `git push`, although it also
creates a change in Chef Automate that is similar to a pull request in
GitHub and other git-based version control systems.

### Verification

When Chef Automate receives the change, it triggers the Verify stage.
The purpose of Verify is to run checks so that the system can decide if
it's worth the time of a human to review the change.

When the Verify phases have completed successfully, the change is ready
for code review. Chef Automate provides integrated code review through
its web UI. There is also an integration with GitHub and Bitbucket
Server (by Atlassian) for teams with existing code review workflows.

In code review, team members can comment on the diffs. If more changes
are required, they can be made either as additional commits on top of
the originally submitted feature branch, or the commit(s) can be
reworked using `git commit --amend` and `git rebase`.

To submit the updates on a feature branch for review, simply run
`delivery review` again. There's no need to worry about force pushing
if you've squashed commits. Chef Automate patchset handling will work
with your workflow. When you resubmit a change with updates from code
review, Chef Automate triggers a fresh run of the Verify stage using the
updated feature branch. This can be repeated as necessary. When Verify
has passed and the team is happy with the change, it can be approved.
Changes are approved by selecting the Approve button in the web UI.

### Approval

When someone selects the Approve button, the feature branch that contains
your change is merged into the target branch of the pipeline (usually
this is master). At this point, the Build stage begins and the same
tests that were run in Verify are run again. This is because the target
branch may have moved ahead by other approvals. Assuming these tests
pass, the Build stage proceeds with the quality and security phases. The
Build stage is also a good place to run additional test suites, as well
as security scanning checks, that might be too time consuming to run
during Verify.

The Build stage concludes with the publish phase. The purpose of the
publish phase is to assemble one or more potentially releasable
artifacts and make them available to the remaining stages of the
pipeline. You can, for example, publish to a Chef Infra Server, to Chef
Supermarket, and to JFrog Artifactory.

If the pipeline succeeds in generating and publishing the artifacts,
then the Acceptance stage begins. This is the first phase that assesses
build artifacts rather than source code.

### Acceptance

The Acceptance stage is where your team decides whether the change
should ship all the way out to its final destination.

During the Acceptance stage, infrastructure is provisioned (if needed),
and the artifacts published at the end of the Build stage are deployed.
The deployment is verified with automated smoke tests, and then the
health of the resulting system is verified by running a functional test
suite. At this point, the pipeline pauses and waits for explicit
approval from someone who has the "shipper" role. The Acceptance stage
is where you can run ad-hoc tests, and perform manual user acceptance
testing. For the internal use of Chef Automate at Chef, we have our
product owners review changes in Acceptance and decide whether or not to
select the Deliver button.

When you select the Deliver button, the change begins its final journey
into production. This journey consists of three stages: Union,
Rehearsal, and Delivered. These three stages are special for two
reasons.

1. The first reason is that they are fully automated. Once you ship a
    change into Union, it will automatically move through the Rehearsal
    and Delivered stages if all of the automated checks are successful.
2. The second reason is that Union, Rehearsal, and Delivered form the
    shared pipeline. In these stages you evaluate a change in the
    context of your system as a whole. Ultimately, it is the health of
    the entire system---not a particular application---that matters. The
    Union stage gives you a place to evaluate the impact of a change on
    the consumers of the application being shipped. Each stage in the
    shared pipeline has the same set of phases: provision, deploy,
    smoke, and functional.

How stages of the pipeline are associated with actual infrastructure
environments is flexible. For example, you can have dedicated
infrastructure for each stage. This allows each stage to operate
independently.

## Pipeline Stages

In this section, we go into more detail about the pipeline. As we've
said, the Chef Automate pipeline is made up of six stages: Verify,
Build, Acceptance, Union, Rehearsal, and Delivered.

Each stage consists of phases that perform a particular task, such as
running some type of test.

One way to think about the stages is whether the set of potentially
releasable artifacts has been produced or not. The pipeline creates
these artifacts at the end of the Build stage. The remaining stages of
the pipeline focus on gaining confidence in those artifacts. Another way
to understand the stages is by whether they are isolated at the project
level or shared across the system. This diagram shows the relationships
among the different stages.

![](/images/automate/delivery_pipeline_attributes.png)

To summarize:

* Each project pipeline has an associated Verify, Build and Acceptance stage. These are called acceptance pipelines
* The Union, Rehearsal, and Delivered stages constitute the shared delivery pipeline
* The Verify and Build stages perform tests on the source code
* The Acceptance, Union, Rehearsal and Delivered stages test potentially releasable artifacts

### Verify Stage

The Verify stage runs automatically when someone submits a new change or
updates an existing change that hasn't yet been approved. It is made up
of the following phases. (Remember that you can skip phases that do not
apply to your project and you have complete control over what happens in
a given phase job.)

Lint
: Run tools that analyze your source code to identify stylistic problems.

Syntax
: Check that the code can be parsed and, if applicable, that it compiles.

Unit
: Run unit tests.

### Build Stage

When a change is approved, Chef Automate merges the change into the
pipeline's target branch and triggers the Build stage. The Build stage
repeats the lint, syntax, and unit phases from the Verify stage. This is
because the target branch may have moved ahead since the Verify stage
ran on this change (this occurs if there are multiple open changes on a
project and another change is approved before yours).

In addition to the Verify checks, the Build stage provides three
additional phases:

Quality
: A place to run additional test suites and code analysis tools. Some tests are too time consuming to run in Verify and are better reserved for changes that have received approval.

Security
: In many organizations, a suite of security tests must be run before a change can be deployed. The Build phase is the place to run such scans and checks. (You can also add compliance checks into the functional test suites that run against the deployed artifacts.)

Publish
: The goal of the publish phase is to produce the potentially releasable artifacts and to make them available to the rest of the pipeline.

### Acceptance Stage

Beginning with the Acceptance stage, the pipeline switches from
analyzing the project's source code to verifying the set of artifacts
that were produced in the Build stage. The goal of the Acceptance stage
is for the team to make a decision about whether the change should go
all the way out to production or not. There are four phases in
Acceptance:

Provision
: Provision infrastructure needed to test the artifact(s). Examples include instantiating new infrastructure with Chef provisioning (or another API-accessible mechanism) and manipulating Chef Infra Server environments to designate the nodes used by the current stage. Of course, what executes in any phase is up to you and determined by the project's build cookbook.

Deploy
: Deploy the artifacts published in the Build stage to the
portion of your infrastructure that has been set aside for acceptance testing.

Smoke
: Smoke tests should be relatively short-running tests that verify that the code that should have been deployed has indeed been deployed and that the system passes minimal health checks.

Functional
: The functional tests should give you confidence that the system is meeting its business requirements.

### Union Stage

Union is the first of the three shared pipeline stages. The purpose of
the Union stage is to assess the impact of the change in the context of
a complete (or as close as possible) installation of the set of projects
that comprise the system as a whole. Union is where you are able to test
for interactions between interdependent projects. The phases in Union
and the remaining stages in the pipeline are the same: provision,
deploy, smoke, and functional.

When an artifact is in Union, Chef Automate ensures that any projects
that depend on it can only pass their own Acceptance stages by proving
their compatibility with that artifact. Chef Automate does this by
pinning the versions of the dependencies to the version of the artifact
in Union. In this way, Chef Automate forces projects to consume updates
to their dependencies as early as possible and prevents them from
shipping before proving that they are compatible with the latest
version.

If a problem is discovered in Union (it will happen, that is what Union
is for), the cooperating teams need to have a conversation about the
right fix. Sometimes the fix may require a change on a different project
than the one that initiated the break. To fix the break, you submit a
new change through the pipeline. Chef Automate is fundamentally a
roll-forward system.

Chef Automate ensures that only one change is active in each of the
Union, Rehearsal, and Delivered stages at any one time. This
orchestration increases safety by encouraging small batch change. In
complex systems, identifying root causes of issues in the context of a
single change is much easier than trying to analyze larger batches of
changes across many different projects. In the future, Chef Automate's
dependency management features will be enhanced to include all
concurrent deploys in Union, Rehearsal, and Delivered, as long as they
map to completely unrelated dependency sets.

### Rehearsal Stage

If all phases of Union succeed, then the Rehearsal stage is triggered.
Rehearsal increases confidence in the artifacts and the deployment by
repeating the process that occurred in Union in a different environment.

If a failure occurs in Union, Rehearsal serves a different and critical
purpose. When you submit a new change and it fixes the break in Union,
you will have proved that a sequence of two changes, one that breaks the
system, and one that comes after and fixes it, results in a healthy
system. You do not yet know what happens when you apply the cumulative
change to an environment that never saw the failure. Sometimes a fix's
success depends upon state left behind as a result of a preceding
failure. The Rehearsal stage is an opportunity to test the change in an
environment that didn't see the failure.

### Delivered Stage

Delivered is the final stage of the pipeline. What "delivered" means
for your system is up to you. It could mean deploying the change so that
it is live and receiving production traffic, or it might mean publishing
a set of artifacts so they are accessible for your customers.

## Components

The following diagram shows the servers that are involved in a Chef
Automate installation.

![](/images/automate/automate_architecture_workflow.png)

The build cookbook, hosted on the Chef Infra Server, determines what happens
during each phase job. Runners, under control of the Chef Infra Server, run
the phase jobs. It's a good idea to have at least three runners so that
the lint, syntax and unit phases can run in parallel.

## Environments

As changes flow through the Chef Automate pipeline, they are tested in a
series of runtime environments that are increasingly similar to the
final runtime target environment.

Chef Automate allows you to define the infrastructure that participates
in each stage. How you map infrastructure environments to pipeline
phases is controlled by the build cookbook. In other words, whether a
given phase job distributes work to other infrastructure is up to you.
There are many ways to map infrastructure environments to pipeline
phases, but here are some possible approaches.

Because they test source code, the Verify and Build stages ordinarily
run exclusively on the runners and don't involve other infrastructure.
The necessary runtime environments are created and destroyed during the
execution of the stage. For example, they can be established using
virtual machines created by testing frameworks such as Kitchen.

The stages that test artifacts---Acceptance, Union, Rehearsal and
Delivered---almost always need access to additional infrastructure to
perform their tests.

For the Acceptance stage, a common approach is to provision one or more
nodes that test the deployment. The Acceptance stage nodes for a project
are usually dedicated to that project and can be either persistent, or
they can be created and destroyed every time the Acceptance stage runs.

For the shared pipeline (Union, Rehearsal, and Delivered), it makes
sense to have persistent infrastructure dedicated to each of the stages.
Infrastructure environments mapped to Union and Rehearsal should ideally
be identical in topology and should correspond as closely as possible to
the live infrastructure of the Delivered stage.

You can set up the infrastructure environments either manually or by
using automated, on-the-fly provisioning upon first use. The manual
approach is simple, but it has the disadvantage of not having an initial
run-list for the nodes in the environment. Automated provisioning
requires adding code to the build cookbook, but it is more replicable
than the manual approach. Automated provisioning also bootstraps the
initial run-list for each node in the environment. The **delivery-truck**
cookbook makes it easy to customize your pipeline's build cookbook for
the environments you want to use for each stage of the pipeline.

Currently, Chef Automate manages cookbook version and application
attribute version pins using environment objects of the Chef Infra Server. The
names of the environments in the Chef Infra Server correspond to the stages of
a pipeline. (This doesn't mean, however, that the nodes that
participate in a given stage need to remain fixed over time.)

It is also possible to share infrastructure among pipeline stages. For
example, you can provision infrastructure needed for performing
acceptance tests while relying on enterprise services provided by
another pipeline stage or even a production environment. Another
possibility is to reserve a portion of infrastructure from production to
run acceptance testing.
