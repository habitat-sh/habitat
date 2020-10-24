+++
title = "Building Packages"
description = "Building Packages in the Studio"

[menu]
  [menu.habitat]
    title = "Building Packages"
    identifier = "habitat/packages/pkg-build Build your Package"
    parent = "habitat/packages"
    weight = 10

+++

## Build

## Plan Build Process

When you have finished creating your plan and call `build` in Chef Habitat Studio, the build script does following steps:

1. Checks that Studio has the private origin key is available to sign the artifact
2. Downloads the source code from the location in `pkg_source`, if specified
3. Validates checksum of the downloaded file using the `pkg_shasum` value, if it is specified.
4. Extracts the source into a temporary cache.
5. Builds and installs the binary or library using `make` and `make install` for Linux based builds, and
  TODO: WHAT DOES WINDOWS USE? Invoke-Unpack function with Start-Process? Invoke-Install & Copy-Item? unless the callback methods are overridden in the plan.
6. Compresses the package contents (binaries, runtime dependencies, libraries, assets, etc.) into a tarball.
7. Signs the tarball with your private origin key and gives it a `.hart` file extension.

After the build script completes, you can then upload your package to Chef Habitat Builder, or install and start your package locally.

Packages need to be signed with a private origin key at buildtime. Generate an origin key pair manually by running the following command on your host machine:

```bash
hab origin key generate <ORIGIN>
```

The `hab-origin` subcommand will place the origin key files, originname-_timestamp_.sig.key (the private key) and originname-_timestamp_.pub files (the public key), in the `$HOME/.hab/cache/keys` directory. If you're creating origin keys in the Studio container, or you are running as root on a Linux machine, your keys will be stored in `/hab/cache/keys`.

Because the private key is used to sign your artifact, it should not be shared freely; however, if anyone wants to download and use your artifact, then they must have your public key (.pub) installed in their local `$HOME/.hab/cache/keys` or `/hab/cache/keys` directory. If the origin's public key is not present, Chef Habitat attempts to download it from the Builder endpoint specified by the `--url` argument (https://bldr.habitat.sh by default) to `hab pkg install`.

### Passing Origin Keys into the Studio

The Habitat Studio is a self-contained and minimal environment, which means that you'll need to share your private origin keys with the Studio to sign artifacts. You can do this in three ways:

1. Set `HAB_ORIGIN` to the name of the origin you intend to use before entering the Studio:

    ```bash
    export HAB_ORIGIN=originname
    ```

    This approach overrides the `HAB_ORIGIN` environment variable and imports your public and private origin keys into the Studio environment. It also overrides any `pkg_origin` values in the packages that you build. This is useful because you can use it to build your own artifact, as well as to build your own artifacts from other packages' source code, for example, `originname/node` or `originname/glibc`.

1. Set `HAB_ORIGIN_KEYS` to the names of your origins. If you're using more than one origin, separate them with commas:

    ```bash
    export HAB_ORIGIN_KEYS=originname-internal,originname-test,originname
    ```

    This imports the private origin keys, which must exactly match the origin names for the plans you intend to build.

1. Use the `-k` flag (short for "keys") which accepts one or more key names separated by commas with:

    ```bash
    hab studio -k originname-internal,originname-test enter
    ```

    This imports the private origin keys, which must exactly match the origin names for the plans you intend to build.

After you create or receive your private origin key, you can start up the Studio and build your artifact.

### Interactive Build

Any build that you perform from a Chef Habitat Studio is an interactive build. Studio interactive builds allow you to examine the build environment before, during, and after the build.

The directory where your plan is located is known as the plan context.

1. Change to the parent directory of the plan context.
1. Create and enter a new Chef Habitat Studio. If you have defined an origin and origin key during `hab cli setup` or by explicitly setting the `HAB_ORIGIN` and `HAB_ORIGIN_KEYS` environment variables, then type the following:

    ```bash
    hab studio enter
    ```

    The directory you were in is now mounted as `/src` inside the Studio. By default, a Supervisor runs in the background for iterative testing. You can see the streaming output by running <code>sup-log</code>. Type <code>Ctrl-C</code> to exit the streaming output and <code>sup-term</code> to terminate the background Supervisor. If you terminate the background Supervisor, then running <code>sup-run</code> will restart it along with every service that was previously loaded. You have to explicitly run <code>hab svc unload origin/package</code> to remove a package from the "loaded" list.

3. Enter the following command to create the package.

    ```bash
    build /src/planname
    ```

4. If the package builds successfully, it is placed into a `results` directory at the same level as your plan.

#### Managing the Studio Type (Docker/Linux/Windows)

Depending on the platform of your host and your Docker configuration, the behavior of `hab studio enter` may vary. Here is the default behavior listed by host platform:

* **Linux** - A local chrooted Linux Studio. You can force a Docker based studio by adding the `-D` flag to the `hab studio enter` command.
* **Mac** - A Docker container based Linux Studio
* **Windows** - A local Windows studio. You can force a Docker based studio by adding the `-D` flag to the `hab studio enter` command. The platform of the spawned container depends on the mode your Docker service is running, which can be toggled between Linux Containers and Windows Containers. Make sure your Docker service is running in the correct mode for the kind of studio you wish to enter.

> Note: For more details related to Windows containers see [Running Chef Habitat Windows Containers](/best-practices/#running-habitat-windows-containers).

#### Building Dependent Plans in the Studio

Writing plans for multiple packages that are dependent on each other can prove cumbersome when using multiple studios, as you need update dependencies frequently. On the other hand, using a single studio allows you to quickly test your changes by using locally built packages. To do so, you should use a folder structure like this:

```bash
tree projects

projects/
├── project-a
└── project-b
```

This way, you can `hab studio enter` in `projects/`. If `project-b` depends on `project-a`, you can call `build project-a && build project-b` for example.

### Non-interactive Build

A non-interactive build is one in which Chef Habitat creates a Studio for you, builds the package inside it, and then destroys the Studio, leaving the resulting `.hart` on your computer. Use a non-interactive build when you are sure the build will succeed, or in conjunction with a continuous integration system.

1. Change to the parent directory of the plan context.
1. Build the artifact in an unattended fashion, passing the name of the origin key to the command.

    ```bash
    hab pkg build yourpackage -k yourname
    ```

    > Similar to the `hab studio enter` command above, the type of studio where the build runs is determined by your host platform and `hab pkg build` takes the same `-D` flag to force a Docker environment if desired.

1. The resulting artifact is inside a directory called `results`, along with any build logs and a build report (`last_build.env`) that includes machine-parsable metadata about the build.

By default, the Studio is reset to a clean state after the package is built; however, *if you are using the Linux version of `hab`*, you can reuse a previous Studio when building your package by specifying the `-R` option when calling the `hab pkg build` subcommand.

For information on the contents of an installed package, see [Package contents](/reference/#package-contents).

## Troubleshooting Builds

### Bash Plans: `attach`

While working on plans, you may wish to stop the build and inspect the environment at any point during a build phase (e.g. download, build, unpack, etc.). In Bash-based plans, Chef Habitat provides an `attach` function for use in your plan.sh that functions like a debugging breakpoint and provides an easy <acronym title="Read, Evaluation, Print Loop">REPL</acronym> at that point. For PowerShell-based plans, you can use the PowerShell built-in `Set-PSBreakpoint` cmdlet prior to running your build.

To use `attach`, insert it into your plan at the point where you would like to use it, e.g.

```bash
 do_build() {
   attach
   make
 }
```

Now, perform a [build](/plan-overview/#plan-builds) -- we recommend using an interactive studio so you do not need to set up the environment from scratch for every build.

```bash
hab studio enter
```

```studio
build yourapp
```

The build system will proceed until the point where the `attach` function is invoked, and then drop you into a limited shell:

```bash
## Attaching to debugging session
From: /src/yourapp/plan.sh @ line 15 :

    5: pkg_maintainer="The Chef Habitat Maintainers <humans@habitat.sh>"
    6: pkg_source=http://download.yourapp.io/releases/${pkg_name}-${pkg_version}.tar.gz
    7: pkg_shasum=c2a791c4ea3bb7268795c45c6321fa5abcc24457178373e6a6e3be6372737f23
    8: pkg_bin_dirs=(bin)
    9: pkg_build_deps=(core/make core/gcc)
    10: pkg_deps=(core/glibc)
    11: pkg_exports=(
    12:   [port]=srv.port
    13: )
    14:
    15: do_build() {
 => 16:   attach
    17:   make
    18: }

[1] yourapp(do_build)>
```

You can use basic Linux commands like `ls` in this environment. You can also use the `help` command the Chef Habitat build system provides in this context to see what other functions can help you debug the plan.

```studio
[1] yourapp(do_build)> help
Help
  help          Show a list of command or information about a specific command.

Context
  whereami      Show the code surrounding the current context
                (add a number to increase the lines of context).

Environment
  vars          Prints all the environment variables that are currently in scope.

Navigating
  exit          Pop to the previous context.
  exit-program  End the /hab/pkgs/core/hab-plan-build/0.6.0/20160604180818/bin/hab-plan-build program.

Aliases
  @             Alias for `whereami`.
  quit          Alias for `exit`.
  quit-program  Alias for `exit-program`.
```

  Type `quit` when you are done with the debugger, and the remainder of the build will continue. If you wish to abort the build entirely, type `quit-program`.

### PowerShell Plans: `Set-PSBreakpoint`

While there is no `attach` function exposed in a `plan.ps1` file, one can use the native Powershell cmdlet `Set-PSBreakpoint` to access virtually the same functionality. Instead of adding `attach` to your `Invoke-Build` function, enter the following from inside your studio shell:

```powershell
[HAB-STUDIO] Habitat:\src> Set-PSBreakpoint -Command Invoke-Build
```

Now upon running `build` you should enter an interactive prompt inside the context of the Invoke-Build function:

```powershell
   habitat-aspnet-sample: Building
Entering debug mode. Use h or ? for help.

Hit Command breakpoint on 'Invoke-Build'

At C:\src\habitat\plan.ps1:26 char:23
+ function Invoke-Build {
+                       ~
[HAB-STUDIO] C:\hab\cache\src\habitat-aspnet-sample-0.2.0>>
```

You can now call Powershell commands to inspect variables (like `Get-ChildItem variable:\`) or files to debug your build.
