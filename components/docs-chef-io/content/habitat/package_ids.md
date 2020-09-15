+++
title = "Package Identifiers"
description = "How to call a package in Chef Habitat Builder, Studio, and plan files"

[menu]
  [menu.habitat]
    title = "Packages"
    identifier = "habitat/reference/package_ids"
    parent = "habitat/reference"

+++

A Chef Habitat artifact the binary distribution for a given package built with Chef Habitat. A Chef Habitat artifact is a signed tarball with a `.hart` file extension. Chef Habitat artifacts are composed of a software library or application, the configuration information for that software, and lifecycle hooks. They are created from a the plan file, a `plan.sh` on Linux systems or a `plan.ps1` on Windows systems, and are built with Chef Habitat tools. Chef Habitat artifacts can be exported to a specific format, such as when creating a Docker image.

When referring to Chef Habitat artifacts from either Builder or the Studio, you can refer to them in two ways:

- Fully-qualified package identifier (FQPI)
: Uses four components in the following format: `origin/name/version/release`. For example, `core/glibc/2.22/20160310192356`.

- Short package identifier
: Uses two components in the following format `origin/name`. For example, `core/redis` or `core/openssl`. Use the three-component form `origin/name/version`, such as `core/redis/5.0.4`, when you need a specific version of an artifact.

## Package Identifier Components

- `Origin`: A name that defines a set of related Chef Habitat packages and the artifacts they generate. For example, "sample", "core", or "mycompany".
- `Name`: The name of the application or service. For example, "postgres".
- `Version`: The version number designation by the author(s) of the application or service. For example, "3.1.1", or "20160118".
- `Release`: The unique Chef Habitat id for a given version based on the timestamp pattern _YYYYMMDDhhmmss_. For example, "20160204220358" would be a Chef Habitat artifact built at 22:03:58 on February 4th, 2016.

## Package Lookup

If the Chef Habitat artifact identifier isn't fully qualified (having fewer than four components), and exactly one artifact is required, then the missing components are assumed to be the most recent values. For example:

- `core/glibc` assumes that version and release values are for the latest version of core/glibc.

- `core/glibc/2.22` assumes that the version of core/glibc is 2.22 and that the release is for the most recent value of core/glibc/2.22.

- `core/glibc/2.22/20160310192356` only refers to the specific Chef Habitat artifact 20160310192356.
