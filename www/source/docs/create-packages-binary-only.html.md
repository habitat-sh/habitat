---
title: Packaging Binary-Only Software
---

# Packaging Binary-Only Software

While Habitat provides the best behavior for applications that can be compiled from source into the Habitat ecosystem, one of its other major features is that it can bring the same management benefits to legacy applications. Frequently, these applications are off-the-shelf software purchased from an independent software vendor, distributed in binary-only form.

You can write plans to package up these binary artifacts with minimal special handling. This article covers some tips and tricks for getting this software into the Habitat realm.

## Override The Phases You Don't Need

A Habitat package build proceeds in phases: download, verification, unpacking (where you would also patch source code, if you had it), build, and finally installation. You can override the behavior of any of these phases by redefining the corresponding `do_` function. The following is an extreme example of overriding all the phases except install:

```
do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_build() {
  return 0
}

do_install() {
  mkdir -p $pkg_prefix/bin
  cp $PLAN_CONTEXT/bin/hello_world $pkg_prefix/bin/hello_world
  chmod +x $pkg_prefix/bin/hello_world
}
```

Typically, when working with binary artifacts, you would start with a plan like this and work backwards to define the correct contents of the phases.

## Relocate Hard-Coded Library Dependencies If Possible

Many binaries hardcode library dependencies to `/lib` or `/lib64` inside their ELF symbol table. Unfortunately, this means that Habitat is unable to provide dependency isolation guarantees if packages are dependent on any operating system's libraries in those directories. The built Habitat packages will also fail to run in minimal environments like containers built using `hab-pkg-dockerize`, because there will not be a `glibc` inside `/lib` or `/lib64`.

Most binaries compiled in a full Linux userland have a hard dependency on `/lib/ld-linux.so` or `/lib/ld-linux-x86_64.so`. In order to relocate this dependency to the Habitat-provided variant, which is provided by `core/glibc`, use the `patchelf(1)` utility within your plan:

1. Declare a build-time dependency on `core/patchelf` as part of your `pkg_build_deps` line.
2. Invoke `patchelf` on any binaries with this problem during the `do_install()` phase. For example:

```
  patchelf --interpreter "$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2" \
           ${pkg_prefix}/bin/somebinary
```

3. The binary may have other hardcoded dependencies on its own libraries that you may need to relocate using other flags to `patchelf` like `--rpath`. For example, Oracle Java provides additional libraries in `lib/amd64/jli` that you will need to relocate to the Habitat location:

```
  export LD_RUN_PATH=$LD_RUN_PATH:$pkg_prefix/lib/amd64/jli
  patchelf --interpreter "$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2" \
           --set-rpath ${LD_RUN_PATH} \
           ${pkg_prefix}/bin/java
```

4. For more information, please see the [patchelf](https://nixos.org/patchelf.html) documentation.

## If You Cannot Relocate Library Dependencies

In some situations it will be impossible for you to relocate library dependencies using `patchelf` as above. For example, if the version of `glibc` the software requires is different than that provided by an available version of `glibc` in a Habitat package, attempting to `patchelf` the program will cause execution to fail due to ABI incompatibility.

Your software vendor's support policy might also prohibit you from modifying software that they ship you.

In these situations, you will have to give up Habitat's guarantees of complete dependency isolation and continue to rely on the library dependencies provided by the host operating system. However, you can continue to use the features of the Habitat supervisor that provide uniform manageability across your entire fleet of applications.

## Fix Hardcoded Interpreters

Binary packages often come with other utility scripts that have their interpreter, or "shebang", line (first line of a script) hardcoded to a path that will not exist under Habitat. Examples are: `#!/bin/sh`, `#!/bin/bash`, `#!/bin/env` or `#!/usr/bin/perl`. It is necessary to modify these to point to the Habitat-provided versions, and also declare a runtime dependency in your plan on the corresponding Habitat package (for example, `core/perl`).

Use the `fix_interpreter` function within your plan to correct these interpreter lines during any phase, but most likely your `do_build` phase. For example:

       fix_interpreter ${target} core/coreutils bin/env

The arguments to `fix_interpreter` are the file (represented here by `${target}`) you are trying to fix, the origin/name pair of the Habitat package that provides that interpreter, and the interpreter pattern to search and replace in the target.

If you have many files you need to fix, or the binary package automatically generates scripts with hardcoded shebang lines, you may need to simply symlink Habitat's version into where the binary package expects it to go:

       ln -sv $(pkg_path_for coreutils)/bin/env /usr/bin/env

This is a last resort as it breaks the dependency isolation guarantees of Habitat.
