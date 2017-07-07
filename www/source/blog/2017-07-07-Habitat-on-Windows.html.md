---
title: Habitat on Windows!
date: 2017-07-07
author: Matt Wrock
tags: supervisor, packaging, windows
category: windows
classes: body-article
---

We are pleased to announce that the latest release of Habitat - [v0.25.0](https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-%24latest-x86_64-windows.zip?bt_package=hab-x86_64-windows) - brings Habitat to Windows! This functionality has been evolving steadily for the last several months and has been available in various pieces along the way. Perhaps you have [seen my demo](https://www.youtube.com/watch?v=pzcmXivVRYI&t=615s) showing where we were back in March. Well with v0.25.0, we ship everything you need to author, package and run Habitat packages inside of a Windows supervisor.

Our Windows work is very much in the Alpha stages, but we are hungry for feedback from others who would like to start building Windows applications with Habitat. This post will walk you through building and running Habitat packages on Windows with an emphasis on how the Windows experience differs from building and running Linux based packages. Finally I will cover the few things that are not yet implemented for Windows and what is coming up next.

## A working Windows Habitat service example

Before diving into details, let me point the impatient reader to a git repo that contains a full Windows application that includes Habitat plans and configuration.

[https://github.com/habitat-sh/habitat-aspnet-sample/](https://github.com/habitat-sh/habitat-aspnet-sample/)

This packages a VERY minimal ASP.NET Core MVC application using Habitat that runs against a MySQL database on Windows. Have a look at its [readme](https://github.com/habitat-sh/habitat-aspnet-sample/blob/master/readme.md) for details.

## Building packages on Windows

The best way to build packages on Windows is to enter the Windows studio. Currently running `hab studio enter` on Windows will enter a Linux based studio in a Docker container. You can override this behavior by including the `-w` flag. `hab studio enter -w` will drop you into a Habitat studio built for Windows:

    C:\dev\habitat-aspnet-sample [win_studio +0 ~1 -0]> hab studio enter -w
      hab-studio: Creating Studio at /hab/studios/dev--habitat-aspnet-sample
    » Importing origin key from standard input
    ★ Imported secret origin key core-20170417214814.
      hab-studio: Entering Studio at /hab/studios/dev--habitat-aspnet-sample
    ** The Habitat Supervisor has been started in the background.
    ** Use 'hab svc start' and 'hab svc stop' to start and stop services.
    ** Use the 'Get-SupervisorLog' command to stream the supervisor log.
    ** Use the 'Stop-Supervisor' to terminate the supervisor.

    [HAB-STUDIO] Habitat:\src>

So how does this environment differ from a Linux based studio or a normal Windows shell?

### Runs inside Habitat packaged Powershell

We have packaged the [open sourced Powershell core](https://github.com/PowerShell/PowerShell) as a [Habitat package](https://bldr.habitat.sh/#/pkgs/core/powershell/6.0.0-alpha.17/20170329104549). This allows us to make certain assumptions as to what cmdlets are available in your shell and saves package authors from having to write Habitat plans that maintain compatibility with different versions of Powershell.

### A disposable Habitat environment.

Just as we do in a Linux shell, we copy your keys from your primary local hab keys but the cached artifacts you would typically find in `c:\hab\cache\artifacts` are in an isolated Habitat environment as well as the supervisor data files and habitat service directories. You can find this disposable `/hab` directory under `c:\hab\studios` in a child directory named after the full path from where you entered the studio. This directory is the target of a "Powershell Drive" named `habitat`. A `hab studio rm -w` will remove this entire environment and eliminate all artifacts created during the lifetime of a studio.

### The path is modified.

Habitat attempts to set your path to the most minimal path feasible to reduce the possibility of outside applications bleeding into your studio build environment. Just like a Linux studio, you want all of your build and runtime dependencies to exist as Habitat packages.

    [HAB-STUDIO] Habitat:\src> $env:path
    C:\hab\pkgs\core\hab-studio\0.25.0-dev\20170705214714\bin\hab;C:\hab\pkgs\core\hab-studio\0.25.0-dev\20170705214714\bin\7zip;C:\hab\pkgs\core\hab-studio\0.25.0-dev\20170705214714\bin;C:\WINDOWS\system32;C:\WINDOWS;C:\hab\pkgs\core\hab-studio\0.25.0-dev\20170705214714\bin\powershell
    [HAB-STUDIO] Habitat:\src>

As you can see the only thing in your path besides Habitat packaged artifacts is the core Windows system paths. We add those because otherwise Windows might not act like Windows.

Be aware that this does not quite promise the same guarantees of a Linux `chroot`ed environment. It is very possible for user installed applications to bleed into the system root directory. For example the C++ redistributable runtimes tend to save `dll`s to this location.

### A `src` directory is created linking to the studio root

This is really no different from what happens in a Linux studio. Habitat creates a `Junction` directory called `src` in the root of the studio environment that targets the directory from which you entered the studio. Build artifacts will be saved to `src/results` allowing you to access them both from inside and outside the studio.

### Added Habitat Powershell functions

The following functions are available in a Windows Habitat studio:

* `build` - this is identical to its Linux cousin. It builds a Habitat package. However instead of looking for a `plan.sh` it looks for a `plan.ps1`. We'll look more closely at plan.ps1 files soon.

* `Get-SupervisorLog` - Just like one expects in a Linux studio, a Habitat supervisor is started in the Windows studio environment and runs in the background. `Get-SupervisorLog` streams the output of the supervisor to a new console window.

* `Stop-Supervisor` - This will stop the above mentioned background supervisor.

## Authoring a `plan.ps1`

We have tried to strike a balance between making a `plan.ps1` closely resemble a `plan.sh` but still "feeling" like Powershell. This means that the basic structure of a `plan.ps1` should be nearly identical to a `plan.sh`.

For an example Powershell based plan see [this plan](https://github.com/habitat-sh/core-plans/blob/master/visual-cpp-build-tools-2015/plan.ps1) for building the Visual Studio C++ Build tools.

Here is how Powershell plans differ from their Bash counterparts:

### The Habitat build variables remain exactly the same

All Habitat plan variables like `pkg_version`, `pkg_origin` and others have the exact same names. Of course because this is Powershell, variables are always prefixed with a `$` even when initializing them.

### The build lifecycle functions are prefixed with `Invoke-`

All of the Habitat build functions like `do_build` and `do_install` are renamed to replace `do_` with `invoke-`. Therefore your Powershell plan's build function might look like this:

    function Invoke-Build {
      cp $PLAN_CONTEXT/../* $HAB_CACHE_SRC_PATH/$pkg_dirname -recurse -force -Exclude ".vagrant"
      & "$(Get-HabPackagePath dotnet-core-sdk)\bin\dotnet.exe" restore
      & "$(Get-HabPackagePath dotnet-core-sdk)\bin\dotnet.exe" build
      if($LASTEXITCODE -ne 0) {
          Write-Error "dotnet build failed!"
      }
    }

In addition to the name change, you will clearly recognize that the function body is Powershell. We hope that those comfortable with Powershell will be comfortable in a Habitat `plan.ps1`.

### Bash arrays and associative arrays use Powershell arrays and hashtables

So...

    pkg_deps=(
      core/dotnet-core
    )

    pkg_build_deps=(
      core/dotnet-core-sdk
      core/patchelf
    )
    pkg_exports=(
        [port]=port
    )

becomes

    $pkg_deps=@("core/dotnet-core")
    $pkg_build_deps=@("core/dotnet-core-sdk")

    $pkg_exports=@{
        "port"="port"
    }

## Running packages in the Windows Supervisor

All `hab svc` and `hab sup` based commands are exactly the same on Windows as they are on Linux and much of the mechanics of how they are supervised remains the same as well. However there are some environmental issues unique to Windows that we should highlight here:

### The Supervisor Path

The behavior of creating the path for a service started by the Windows supervisor is actually identical to Linux. When a Habitat package is built, metadata is produced which states what directories to add to the path at runtime. Those directories are added to the path of the process that the supervisor starts as well as the same `PATH` metadata of all dependent packages. This is no different than how the Linux supervisor behaves. However I bring it up here to point out how dynamic linking differs on Windows opposed to Linux [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) based binaries.

While ELF binaries can have their headers manipulated at build time to link to dependencies at specific paths at runtime, native Windows binaries do not share this quality. Habitat packages for Linux highly leverage ELF header patching to enforce that a Habitat built binary points to the correct Habitat dependencies. This is great because it solidifies a Habitat guarantee which is that packages you build your top level application against will be the *EXACT SAME* packages that run along side your application at runtime - no surprises. Windows in contrast highly leverages the directories in an environment's `path` ([this post](https://msdn.microsoft.com/en-us/library/7d83bc18.aspx) covers the details of Windows DLL searching). The binaries of an application's dependencies need to be located in the path in order to be used at runtime. So the path, while it is constructed the same as on Linux is absolutly critical to dynamic linking of binaries at runtime.

### Gossiping over the ring and the Windows Firewall

The Habitat supervisors can talk to other supervisor peers on the same gossip ring over ports 9631 (TCP) and 9638 (TCP/UDP). In order for supervisors on a ring to listen to each other, these ports must be opened on the Windows Firewall where the supervisor is running. You can open them by running:

    New-NetFirewallRule -DisplayName "Habitat TCP" -Direction Inbound -Action Allow -Protocol TCP -LocalPort 9631,9638
    New-NetFirewallRule -DisplayName "Habitat UDP" -Direction Inbound -Action Allow -Protocol UDP -LocalPort 9638

### Running services under different accounts - `pkg_svc_user`

Just like on Linux, if a plan includes a `pkg_svc_user`, the supervisor will start that service by spawning a new process running as that user. If no `pkg_svc_user` is specified then the supervisor will use the `hab` user if that account exists. Otherwise, the supervisor will start the service under the same account that the supervisor itself is running under.

Here are some significant ways this behavior differs on Windows:

**`pkg_svc_group` is ignored**

There are fundamental differences between the Linux and Windows concept of Group. On Windows we only use the user.

**Use the `--password` argument to specify the user's password**

Unlike Linux, Windows processes cannot simply start processes setting the `UID` for the process. Instead, a token for that user must be generated. To generate this token, the user's credentials are required. Thus the Habitat supervisor needs that user's password.

Both the `hab svc load` and `hab svc start` commands accept a `--password` argument on Windows. Pass the password of the `pkg_svc_user` to this argument so that Habitat can generate the logon token for that user.

**The `pkg_svc_user` must have the `SE_SERVICE_LOGON_NAME` right**

This can be set using the Local Security Policy GUI and give the user the right to logon as a service.

**The Supervisor's account must posses the `SE_INCREASE_QUOTA_NAME` and `SE_ASSIGNPRIMARYTOKEN_NAME` privileges to start services as a different user**

Like the `SE_SERVICE_LOGON_NAME` right above, both of these privileges can be set using the Local Security Policy GUI by assigning the supervisor's user to the "Adjust memory quotas for a process" right (likely already assigned) and "Replace a process level token" right.

For an example of setting user right assignments programatically via Powershell see [this Vagrantfile](https://github.com/habitat-sh/habitat-aspnet-sample/blob/master/Vagrantfile).

## What kind of Windows applications can Habitat package and run?

Throughout the development of these features, we have been using [the ASP.NET Core application](https://github.com/habitat-sh/habitat-aspnet-sample) mentioned at the beginning of this post. .NET Core applications are extremely Habitat friendly because the runtime can be easily packaged and isolated to the Habitat environment. We have also converted the Node tutorial app and packaged several utilities and platforms in Windows Powershell based plans.

### So can I package my .NET Full framework IIS applications and Windows Services?

I get this question a lot! Probably thats because its these types of applications that most Windows developers actually build and run. Experimenting with bringing these applications to Habitat is high on our Windows priority list. These kinds of applications present some challenges to Habitat because IIS, the full .NET framework and the Windows Service Manager are very much coupled to the OS and not friendly to supervisor intervention.

These challenges do not at all mean that they cannot run with Habitat and we have some ideas we plan to test to get them running with Habitat. Keep an eye out for more on this in the weeks to come!

### How can I discover what packages are available on Windows?

Today there is not yet a way to filter packages by platform. What we do have today are maintained in the [core-plans](https://github.com/habitat-sh/core-plans) and you could search for all `plan.ps1` files to discover them. For those interested in authoring Windows packages, I'd encourage you to look them over since they may help answer questions related to how to package your applications and libraries in Habitat packages. Today these include plans for:

* dotnet-core
* node
* powershell
* lessmsi
* wix
* mysql
* 7zip
* rust
* visual-cpp-build-tools-2015
* visual-cpp-redist-2013

While these packages largely represent libraries and supporting runtimes, as mentioned at the beginning of this post, see our [habitat-aspnet-sample](https://github.com/habitat-sh/habitat-aspnet-sample) application for an example of what a full habitat service on Windows would look like.

## What features on Linux are missing on Windows

This list is very small.

The main feature missing on Windows is the ability to export Windows `.hart` packages to a Windows Docker container. Obviously this would be an extremely cool feature to have. I personally want this very much.

Finally, while this is not a feature, per se, we do not yet support running the supervisor as a Windows Service. While you can run Habitat under Systemd on Linux, on Windows you still need to run the supervisor in a console. That is clearly a blocker for many production scenarios and is toward the top of the list of things we plan to support.

## What's coming up for Windows

The top items I have mentioned in this post:

* Running the supervisor as a service
* Creating patterns for packaging IIS/.NET Full framework and Windows service applicatrions
* Exporting Windows services to Windows based docker containers
* Various "polish" chores in the CLI and the docs

## Try it out and tell us what you think!

What we need the most of right now, even more than implementing the above features, is your feedback. We need your input as to what works and what doesn't work, what you would need to package and run your applicationis in Habitat and perhaps above all, does Habitat provide a compelling value proposition to your application development lifecycle and what would make it even better.

If Habitat on Windows interests you, I invite you to take Habitat on Windows for a spin and reach out to us with feedback. The best way to provide your feedback or to ask questions is using the [#windows Slack channel](https://habitat-sh.slack.com/messages/C1GQZNACF) where we are actively listening.

We think Habitat opens up some very exciting aplication management scnarios for Windows and we hope you do too!
