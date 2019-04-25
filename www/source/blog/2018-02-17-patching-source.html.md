---
title: Packaging Source that Needs a Patch
date: 2018-02-17
author: Franklin Webber
tags: blog, packaging, patching
category: supervisor
classes: build
---

Chef Habitat packages modern frameworks and legacy applications. In either case, dependencies within your application may bring you in contact with a legacy, lower level library that is not already packaged by someone else in the [Chef Habitat community](https://bldr.habitat.sh/#/pkgs).

An unmaintained library may require updates to correct security issues or address defects. Patching the source is often the preferred approach as it enables you to fix code inline without the creation of a maintenance fork.

Quite often a patch may already exist, but you may find yourself in an instance where that work falls on you. So let's walk through that scenario as an exercise. In this post, you'll learn how to:

* Use Chef Habitat's debugger with `attach`
* Develop and apply a patch with `diff` and `patch`

I created an initial Chef Habitat plan that downloads a Rust application that fails to build.

Move to a working directory, clone the project, move to the cloned directory, and enter the Chef Habitat studio.

```shell title:"Grab the source and enter the studio"
$ cd ~
$ git clone https://github.com/burtlo/habitat-patching-source
$ cd ~/hab-patching-source
$ hab studio enter
```

Run the `build` command.

```studio title:"Build the package within the Studio"
[1][default:/src:0]# build
: Loading /src/plan.sh
knock-knock: Plan loaded
knock-knock: Validating plan metadata
knock-knock: Using HAB_BIN=/hab/pkgs/core/hab/0.53.0/20180205213018/bin/hab for installs, signing, and hashing
[...]
knock-knock: Preparing to build
knock-knock: Building
Compiling knock-knock v0.1.0 (file:///hab/cache/src/knock-knock-0.1.0)
error[E0432]: unresolved import `std::cmp::Orderin`
--> src/main.rs:2:5
|
2 | use std::cmp::Orderin;
|     ^^^^^^^^^^^^^^^^^ no `Orderin` in `cmp`. Did you mean to use `Ordering`?
[...]
--> src/main.rs:105:13
 |
105 |             Ordering::Equal => println!("."),
 |             ^^^^^^^^ Use of undeclared type or module `Ordering`

warning: unused import: `std::cmp::Orderin`
--> src/main.rs:2:5
|
2 | use std::cmp::Orderin;
|     ^^^^^^^^^^^^^^^^^
|
= note: #[warn(unused_imports)] on by default

error: aborting due to 13 previous errors

error: Could not compile `knock-knock`.

To learn more, run the command again with --verbose.
knock-knock: Build time: 0m24s
knock-knock: Exiting on error
```

The `cargo` command fails to build the source code because there is an issue with the source code. Finding the solution to why source code fails to build is an entire career in itself. In this instance, a knowledge of the Rust language would make it clear what has to be accomplished. For other packages, it could be different languages, frameworks, or dependencies. If you are lost in trying to find a solution:

* reach out to the original maintainers
* find similar questions and answers posted to forums (e.g. Stack Overflow)
* ask within the [Chef Habitat Slack](http://slack.habitat.sh/)

For this problem, the solution is actually provided by the Rust compiler itself:

```studio title:"An error with a suggestion"
[...]
   Compiling knock-knock v0.1.0 (file:///hab/cache/src/knock-knock-0.1.0)
error[E0432]: unresolved import `std::cmp::Orderin`
 --> src/main.rs:2:5
  |
2 | use std::cmp::Orderin;
  |     ^^^^^^^^^^^^^^^^^ no `Orderin` in `cmp`. Did you mean to use `Ordering`?
[...]
```

The Rust compiler identified an unresolved import with the name `std::cmp::Orderin` and offers the suggestion that the author of the source may have intended to write `std::cmp::Ordering`.

The remaining errors in the source confirm that the original author did mean to write `Ordering` and not `Orderin`.

```studio title:"More errors support the suggestion"
[...]
error[E0433]: failed to resolve. Use of undeclared type or module `Ordering`
 --> src/main.rs:17:13
   |
17 |             Ordering::Equal => println!("."),
   |             ^^^^^^^^^^^^^^^ Use of undeclared type or module `Ordering`
[...]
```

The issue is within the `src/main.rs` file. To address this issue you will create a patch file. Patches represent the results of the `diff` command expressed in a file. The `patch` command is able to apply these diffs.

To create this patch file you will need:

* the original source
* the updated source
* the results from running the `diff` command

To view the original source requires you could download it and unpack it. Alternatively, you could view the original source while it is built in the `do_build` callback function. Chef Habitat provides a debugger that enables you to pause the build process during any phase.

The `attach` function can be added anywhere in a plan file to pause the execution, setting a break point, during a build. Using the `attach` function is a powerful debugging technique that will assist you when developing Chef Habitat packages.

Within `do_build` add a call to the `attach` function before `cargo` executes.

```bash title:~/hab-patching-source/habitat/plan.sh mark:13
pkg_name=knock-knock
pkg_origin=franklinwebber
pkg_version="0.1.0"
pkg_maintainer="Franklin Webber <franklin@chef.io>"
pkg_license=('Apache-2.0')
pkg_source="https://github.com/learn-chef/hab-patching-source/raw/master/${pkg_name}-${pkg_version}.tar.gz"
pkg_shasum=c35d0e7b4726f075545a93921c78853ee9c3ef47ed3a0793b4bb03b726812838
pkg_build_deps=( core/rust )
pkg_deps=( core/gcc-libs )
pkg_bin_dirs=( bin )

do_build() {
    attach
    cargo build --release
}

do_install() {
    cp target/release/$pkg_name $pkg_prefix/bin
}

```

Run the `build` command.

```studio title:"Re-building with a breakpoint in do_build" mark:18
[2][default:/src:0]# build
[...]
knock-knock: Building

### Attaching to debugging session

From: /src/plan.sh @ line 13 :

   3: pkg_version="0.1.0"
   4: pkg_maintainer="Franklin Webber <franklin@chef.io>"
   5: pkg_license=('Apache-2.0')
   6: pkg_source="https://github.com/learn-chef/hab-patching-source/raw/master/${pkg_name}-${pkg_version}.tar.gz"
   7: pkg_shasum=c35d0e7b4726f075545a93921c78853ee9c3ef47ed3a0793b4bb03b726812838
   8: pkg_build_deps=( core/rust )
   9: pkg_deps=( core/gcc-libs )
   10: pkg_bin_dirs=( bin )
   11:
   12: do_build() {
=> 13:     attach
   14:     cargo build --release
   15: }
   16:
   17: do_install() {
   18:     cp target/release/$pkg_name $pkg_prefix/bin
   19: }

[1] knock-knock(do_build)>
```

The flow of execution has been paused where you specified the `attach` function. Here you are placed into the scope of the `do_build` function.

Examine the current working directory.

```studio title:"Working directory within do_build callback"
[1] knock-knock(do_build)> pwd
/hab/cache/src/knock-knock-0.1.0
```

Examine the unpacked source.

```studio title:"Contents of the unpacked source"
[2] knock-knock(do_build)> ls
Cargo.toml  README      src
```

Examine the contents of the `main.rs` file.

```studio title:"Contents of the incorrect source code"
[3] knock-knock(do_build)> cat src/main.rs
use std::io;
use std::cmp::Orderin;

fn main() {
    println!("Knock, knock!");
[...]
```

To create the desired source, copy `src/main.rs` to `/src`.

```studio title:"Copying to the project directory"
[4] knock-knock(do_build)> cp src/main.rs /src
```

A copy of `main.rs` now exist in the project directory. Within your editor, update this file to fix the issue.

```rust title:~/hab-patching-source/main.rs mark:2
use std::io;
use std::cmp::Ordering;

[...]
```

Still within the paused build, run the `diff` command to generate the diff.

```studio title:"Show the difference between the source"
[5] knock-knock(do_build)> diff src/main.rs /src/main.rs
--- src/main.rs
+++ /src/main.rs
@@ -1,5 +1,5 @@
 use std::io;
-use std::cmp::Orderin;
+use std::cmp::Ordering;

 fn main() {
     println!("Knock, knock!");
```

The diff results show the path to the two files that were compared and the detected difference. The `-` shows the line to remove and the `+` shows the line to add.

Run `diff` again, but save the results to a file within the `/src` directory.

```studio title:"Create a patch file"
[6] knock-knock(do_build)> diff src/main.rs /src/main.rs > /src/$pkg_name-$pkg_version-ordering.patch
```

Now, let's apply the patch.

```studio title:"Apply the patch"
[7] knock-knock(do_build)> patch -p1 < /src/$pkg_name-$pkg_version-ordering.patch
patching file src/main.rs
```

With patching, the default behavior is to look for the file to patch within the directory you execute the `patch` command. For that to work, you would need to change to the `$HAB_CACHE_SRC_PATH/$pkg_name-$pkg_version/src` directory and run `patch < /src/$pkg_name-$pkg_version-ordering.patch`.

When you want to preserve some or all of the file path provided in the patch file you use `-p`, or strip flag. The `-p` flag will strip away the specified number of directory components from the path in the patch file. Recall the diff displayed on the second line `+++ /src/main.rs`. If you were to use:

* `-p0`, the path would remain intact and attempt to patch a file named `/src/main.rs`
* `-p1`, the path would remove the `/` and attempt to patch a file named `src/main.rs`
* `-p2`, the patch would remove the `/src/` and attempt to patch a file named `main.rs`

From within the paused build, run the `exit` command to resume execution of the build .The application successfully builds the package. This tells you that the patch you applied worked.

Update the plan to remove `attach` and include the `patch` command.

```bash title:~/hab-patching-source/habitat/plan.sh mark:13
pkg_name=knock-knock
pkg_origin=franklinwebber
pkg_version="0.1.0"
pkg_maintainer="Franklin Webber <franklin@chef.io>"
pkg_license=('Apache-2.0')
pkg_source="https://github.com/learn-chef/hab-patching-source/raw/master/${pkg_name}-${pkg_version}.tar.gz"
pkg_shasum=c35d0e7b4726f075545a93921c78853ee9c3ef47ed3a0793b4bb03b726812838
pkg_build_deps=( core/rust )
pkg_deps=( core/gcc-libs )
pkg_bin_dirs=( bin )

do_build() {
    patch -p1 < /src/$pkg_name-$pkg_version-ordering.patch
    cargo build --release
}

do_install() {
    cp target/release/$pkg_name $pkg_prefix/bin
}
```

While the package successfully builds it still does not provide access to the binary `knock-knock`. You can verify that by running the `hab pkg binlink` command.

```studio title:"Binlink and run the application"
[4][default:/src:0]# hab pkg binlink franklinwebber/knock-knock
[...]
[5][default:/src:0]# knock-knock
Knock, knock!
```

Type in `Who's there?` and then press enter. Enjoy a laugh at a bad joke!

## Summary

Chef Habitat excels at packaging modern frameworks and legacy applications. In either situation your package may require skills with debugging or patching. Knowing how to use `diff` and `patch` to fix source code is essential when packaging unmaintained libraries. Using Chef Habitat's `attach` function during build phase callbacks enables you to explore and interactively develop solutions.

Good luck and happy packaging!
