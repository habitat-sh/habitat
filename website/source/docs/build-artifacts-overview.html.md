---
title: Building artifacts
---

# Build artifacts

Habitat artifacts are signed tarballs with a .hart extension that are built from plans using the hab-plan-build script file. To build an artifact, do the following:

1. Open a terminal window with the Docker CLI connected to the shell.  
2. Change directory to your local instance of the habitat repo.
3. Open a Habitat dev shell container with `make shell`.
4. Make sure you have the origin signature key that matches the pkg_origin value in the plan for the artifact you are trying to build.
5. Enter the following command to create the artifact.

       build /src/plans/planname

For more information on how to define a plan and build an artifact, how to create origin keys, and how to run a Habitat service, see the [getting started tutorial](/tutorials/getting-started-overview).

For information on the contents of an installed package, see [Package contents](/docs/package-contents).
