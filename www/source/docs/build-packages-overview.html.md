---
title: Building packages
---

# Build packages

Habitat packages are cryptographically-signed tarballs with a .hart extension that are built from plans. You can build a package in two ways: interactively from inside a studio, and non-interactively.

In both scenarios, you'll need to have the signing key for the origin of your package, which is defined using the `pkg_origin` setting inside your plan. If you haven't created an origin signing key yet, see [Keys](/docs/keys).

## Interactive Build

An interactive build is one in which you enter a Habitat studio to perform the build. Doing this allows you to examine the build environment before, during, and after the build. The studio is destroyed after you exit it.

The directory where your plan is located is known as the Plan Context.

1. Change to the parent directory of the Plan Context.
2. Create an enter a new Habitat studio and pass the origin key into it. We'll assume your origin key is named `yourname`.

       hab studio -k yourname enter

3. The directory you were in is now mounted as `/src` inside the studio. Enter the following command to create the package.

       build /src/planname

4. If the package builds successfully, it is placed into a `results` directory at the same level as your plan.

## Non-Interactive Build

A non-interactive build is one in which Habitat creates a studio for you, builds the package inside it, and then destroys the studio, leaving the resulting `.hart` on your computer. Use a non-interactive build when you are sure the build will succeed, or in conjunction with a continuous integration system.

1. Change to the parent directory of the Plan Context.
2. Build the package in an unattended fashion, passing the name of the origin key to the command.

        hab pkg build yourpackage -k yourname

3. The resulting package is inside a directory called `results`, along with any build logs and a build report (`last_build.env`) that includes machine-parseable metadata about the build.

For more information on how to define a plan and build a package, how to create origin signing keys, and how to run a Habitat service, see the [getting started tutorial](/tutorials/getting-started-overview).

For information on the contents of an installed package, see [Package contents](/docs/package-contents).

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-overview">Run packages</a></li>
</ul>
