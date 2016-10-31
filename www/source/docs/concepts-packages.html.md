---
title: Packages
---

# Packages

A package refers to a binary distribution for a given piece of Habitat software that contains a software library or application as well as any configuration information for that software. It's a signed tarball with a .hart file extension created from a plan definition and built with Habitat tools that can be post-processed to be runtime specific, such as when creating a Docker container.

Packages are identified using a four-component scheme: `origin/name/version/release`, where origin, name, version, and release are replaced with corresponding values.

**Identifier components**

- `Origin`: A name that defines a set of related packages. For example, "sample", "core", or "mycompany".
- `Name`: The name of the application or service. For example, "postgres".
- `Version`: The version number designation by the author(s) of the application or service. For example, "3.1.1", or "20160118".
- `Release`: The unique Habitat id for a given version based on the timestamp pattern _YYYYMMDDhhmmss_. For example, "20160204220358" would be a package built at 22:03:58 on February 4th, 2016. Multiple releases of a given package version may exist using different dependencies and/or compiler options.


When referring to packages from either the depot or the `hab` command-line interface (CLI), you can refer to them in two ways: A package identifier and a fully qualified package identifier.

- A package identifier is typically specified using the two-component form `origin/name`. For example, `chef/redis` or `chef/openssl`.
- A fully-qualified package identifier includes all four components in the following format: `origin/name/version/release`. For example, `chef/glibc/2.22/20160310192356`.


If the package identifier isn't fully specified (having less than four components), then the missing components are assumed to be the most recent versions for that package. For example:

- `chef/glibc` assumes that version and release values are for the latest version of chef/glibc.

- `chef/glibc/2.22` assumes that the version of chef/glibc is 2.22 and that the release is for the latest version of chef/glibc/2.22.

- `chef/glibc/2.22/20160310192356` only refers to the specific package 20160310192356.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-keys">Keys</a></li>
</ul>
