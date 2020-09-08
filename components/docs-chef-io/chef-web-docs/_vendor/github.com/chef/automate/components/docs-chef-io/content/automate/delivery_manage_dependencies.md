+++
title = "Dependencies"

draft = false
[menu]
  [menu.automate]
    title = "Dependencies"
    parent = "automate/workflow"
    identifier = "automate/workflow/delivery_manage_dependencies.md Dependencies"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/delivery_manage_dependencies.md)

Workflow is a legacy feature for Chef Automate, which was designed for managing changes to both infrastructure and application code, giving your operations and development teams a common platform for developing, testing, and deploying cookbooks, applications, and more.

{{< warning >}}
Workflow is available in Chef Automate for existing users. If you are not already using Workflow, but are interested in the solution it offers, please contact your sales or success representative for support with continuous integration pipelines.
{{< /warning >}}

A run-time dependency in Chef Automate is defined as an API-level
dependency between two distinct pieces of software that occurs after
both pieces have already compiled and started running. This type of
dependency is distinct from compile-time dependencies, which should be
handled through other means, such as local build verification tests and
through the publish phase. Dependencies are tracked in Chef Automate
because it is unsafe to deploy a project in an inter-dependent test
environment if other related projects are failing.

## Declare Dependencies

To declare a dependency, update your project's `config.json` file
located in your local project folder at `./delivery/config.json`. Once
the configuration change reaches Union, dependencies will be visible on
your source and dependent projects. When dependencies are defined for a
project, an additional tab will appear listing those dependencies.

![image](/images/automate/hero.png)

The projects who are declared as dependencies will also list which
projects depend on them.

![image](/images/automate/dragon.png)

If neither the **Dependencies** or **Required By** tabs are visible,
then that particular project is not part of a declared dependency graph.

### Configure Dependencies

The following example shows a run-time dependency against the master
branch of a project named `BackendAPI`:

```JSON
{
  "version": "2",
  "build_cookbook": {
    "name": "build-cookbook",
    "path": ".delivery/build-cookbook"
  },
  "skip_phases": [],
  "dependencies": ["BackendAPI"]
}
```

## Dependencies and Promotion

Dependencies impact two or more projects.
Those projects have their own pipelines up through Acceptance, but when a project's tests are run in the shared Union, Rehearsal, and Delivered pipeline for the organization, tests for all projects which depend on the currently-tested project will also run as part of the Union stage.
This is to ensure that no cross-project bugs were introduced, such as a breaking API change. Also, you should still run smoke and functional tests on a project that depends on other project(s) during the Acceptance stage (because you know what those dependencies are).
However, you may not know which projects depend on *your* project. Chef Automate uses the Union stage to runs tests against projects that depend on *your* project.

![image](/images/automate/consumer_tests.png)

In this example, you can see that a change on the project Eegah is in Union, and Mitchell depends on Eegah. As a result, both Eegah and Mitchell's tests are being run.

If any tests fail for either project, the entire Union run will fail and neither project will be automatically promoted. Additionally, if there's another failed Union run before the first one fails which includes some of the same projects, then all the projects from both Union runs must pass their tests before anything can be promoted.

It is important to note that you may have a situation where some projects are entirely independent and have no dependencies on other projects. In this case, it does not matter what state those other projects are in. If their tests pass, Chef Automate will allow their changes to promote through.

### Handle Failures

As described above, dependency failures are breakages in your dependency graph, which keep the current project's pipeline from being able to ship safely. You can see such failures as warnings on the change view in the Chef Automate server web UI. These failures are tracked because they allow Chef Automate to know which changes are safe to promote.

## Examples

To understand how dependency failures can affect a given project (or set of projects), here are some examples of different dependency failures. They progress from basic to complex and should give you an idea of how dependency graphs are constructed in Chef Automate.

Assume we have some projects with the following dependencies:

* objects B and C depend on the same pipeline of project A
* depends on a pipeline of B
* depends on a pipeline of C
* depends on a pipeline of E
* and Y depends on a pipeline of X

Here it is represented graphically:

![](/images/automate/dependency_graph_base_with_xy.png)

All the examples below are represented in graphical table form, where projects are denoted by uppercase letters and a test failure corresponding to a project is denoted by with a lowercase "x". For example, Bx would represent a test failure in project B.

### Simple Break and Clear

A change is made to project A, which causes an API incompatibility with project B, thus causing project B's tests to fail. To fix the problem, another change is made to correct the API in project A and is resubmitted. This allows all tests to pass. Project A can now promote. Projects B and C do not need to promote because no changes have been made to them.

| Changed Project | Test Results | Blocked Projects |Promoted |
| :-------------- | ------------ | ---------------- | ------- |
| A               | (A, Bx, C)   |    (A, Bx, C)    |   None  |
| A               | (A, B, C)    |    None          |   A     |

### Unrelated Changes

Again, a breaking change is made to project A. Before it can be fixed,
someone from the other side of the company makes a change to X, which is
an unrelated project. X and Y (which depends on X) pass their tests
cleanly. Project X promotes without issue.

| Changed Project | Test Results | Blocked Projects | Promoted |
|-----------------|--------------|------------------|----------|
| A               | (A, Bx, C)   | (A, Bx, C)       | None     |
| X               | (X,Y)        | (A, Bx, C)       | X        |
|                 |              |                  |          |

In a very similar example, a change can still ship if it is related to a
breakage as long as that breakage does not occur in an immediate
upstream dependency. Again, assume the same change is made to project A
that broke project B. Then, a change is made to project E that passes
its project tests. A project with changes can only impact projects that
consume it, so project E is safe to promote although its dependency
project C is blocked due to project C's dependency on project A. This
promotion of project E's change can happen because the version of
project C in Union and Rehearsal is still what it was before the change
to project A was pushed through the pipeline and broke project B's
run-time tests in Union.

| Changed Project | Test Results | Blocked Projects | Promoted |
|-----------------|--------------|------------------|----------|
| A               | (A, Bx, C)   | (A, Bx, C)       | None     |
| E               | (E)          | (A, Bx, C)       | E        |
|                 |              |                  |          |

### Expanding the Blocked Set

The same broken change is made to project A, which causes project B's
tests to fail. This time, instead of changing project A, project B is
updated instead to use the new API. That worked, but the change in
project B inadvertently broke project D during testing. Now, none of
projects A through D can ship. It might look like project A should be
able to ship as it isn't broken, and neither are the projects which
depend on it; however, to ship this version of project A, a new version
of project B would have to ship, which would cause project D to break in
production.

The end result is all projects are kept from promotion until project D
is fixed, at which point everything can ship.

| Changed Project | Test Results | Blocked Projects | Promoted |
|-----------------|--------------|------------------|----------|
| A               | (A, Bx, C)   | (A, Bx, C)       | None     |
| B               | (B, Dx)      | (A, B, C, Dx)    | None     |
| D               | (D)          | None             | A, B     |

### Clearing Blockages

So far the examples have shown cases where there is a single set of
blocked projects. Sometimes it makes sense to have blockages clear
independently.

For example, that breaking change to project A got merged and broke
project B again. Then, a bad change to project X was merged, which
caused project X to fail. If project B is updated to address the
breaking change from project A, one would expect projects A, B, and C to
be able to ship, while X and Y would still be blocked. Because those
project sets are not part of the same dependency graph, that is exactly
what happens.

| Changed Project | Test Results | Blocked Projects | Promoted |
|-----------------|--------------|------------------|----------|
| A               | (A, Bx, C)   | (A, Bx, C)       | None     |
| X               | (Xx,Y)       | (A, B, C), (Xx,Y)| None     |
| B               | (B,D)        | (Xx, Y)          | A, B     |

### Overlapping Dependencies

This final example describes how disjointed and broken project sets may
merge when a new test set introduces overlap. It is similar to the
previous one, but instead of projects X and Y, which only have an
isolated dependency between X and Y, projects F and E have some overlap
with the project set (A, B, C) because project E is dependent on project
C. Making a change to project E which breaks project F does not lump F
and E with the existing blocked project set (A, B, C) since F is not a
dependency of A, B or C.

Suppose a change is made to project C in an attempt to make it
compatible with the change to project A. Recall that project E was
dependent on project C, and is broken by the latest change to project F.
The dependent project set (C, E) is considered not safe to promote. The
blocked project set (A, B, C), and the set (C, E), have project C in
common and are merged to form the superset (A, B, C, E).

Similarly, the blocked project sets (C, E) and (E, F) merge to form (C,
E, F). The blocked project supersets join together to form the final
blocked set (A, B, C, E, F).

A final change to fix project E will unblock itself and the projects A,
C, and F.

| Changed Project | Test Results | Blocked Projects  | Promoted |
|-----------------|--------------|-------------------|----------|
| A               | (A, B, Cx)   | (A, B, Cx)        | None     |
| F               | (F, Ex)      | (A, B, Cx), (F,Ex)| None     |
| C               | (C, Ex)      | (A, B, C, F, Ex)  | None     |
| E               |  (E)         |     None          |A, C, E, F|
