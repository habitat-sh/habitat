---
title: "Windows Container Studio"
date: 2017-11-16
author: Matt Wrock
tags: Windows, Studio
category: windows
classes: body-article
---

We launched a Windows Studio earlier this year that provides a "kind of" isolated environment for your Windows Habitat builds. It restricts your `path` to only include the Windows system root and vital Habitat Studio packages. It also creates a Powershell drive rooted in a separate Habitat Studio filesystem directory. This environment is certainly better than nothing but it is far from the isolation provided by a Linux chroot or container. Many programs tend to install bits to the `c:\windows\system32` directory, there is no Windows registry or feature isolation, and it's possible to navigate outside of the Studio file system root and taint files that exist outside of the Studio.

## Windows Containers to the Rescue!

A Windows container can provide an ideal level of isolation on par with the Linux Studio images we have been providing since Habitat's initial launch (at which time Microsoft had not yet released Windows containers). Here you have a locked down `C:` volume, a pristine Windows registry, no preinstalled software other than the Habitat Studio and Supervisor, and a known, very minimal set of Windows features.

Today's [0.39.0 Habitat release](https://github.com/habitat-sh/habitat/releases/tag/0.39.0) introduces a Windows container Studio for users developing on Windows 10 or Windows Server 2016 running [Docker Community Edition for Windows](https://store.docker.com/editions/community/docker-ce-desktop-windows) providing container support via Hyper-V.

## How to launch a Windows container Studio

You must be on a Windows 10 or Server 2016 host running Docker Community Edition for Windows. Also, your Docker settings must be **switched to Windows containers**. As long as Docker is in Windows container mode, running `hab studio enter` will pull down our Habitat Studio image and launch that container. The Studio will look just like a local Windows Studio which still exists today via `hab studio enter -w` but local my friend it is not! Oh its not far, but checking out `$env:COMPUTERNAME` should reveal a host name different from your local host.

Now if you still want to run a local Windows Studio, you can still do so by adding the `-w` switch: `hab studio enter -w`. This may still be desirable if you are on an older version of windows without Hyper-V enabled or if you want to avoid the longer Studio start up time (more on that in a bit).

## Known issues with the Windows container Studio

Please consider this Studio a "Beta" level Studio. There are indeed a few unexpected snags you may hit. Here are some things to look out for:

### Why does it take a million years to pull the image?

C'mon now...a million years? Well yeah it can take a very long time on the very first pull. We use the [`microsoft/windowsservercore`](https://hub.docker.com/r/microsoft/windowsservercore/) image as our base image which currently clocks in at a 5GB download. That is huge by Linux standards but should only be downloaded on the first pull and you should not need to download it again unless Habitat changes its base image.

### Why is the Studio missing IIS or my other required features?

As stated above, we use the `microsoft/windowsservercore` image as a base. This image only has critical features enabled. Thats actually the best possible Studio environment to have because you are never guaranteed that any runtime environment will have more than these features enabled. If you need IIS or other services, you may want to check for those features and enable them if they are not present in your `init` hooks.

### Why do my builds fail with OutOfMemoryExceptions?

When running a Windows container in [Hyper-V isolation](https://docs.microsoft.com/en-us/virtualization/windowscontainers/manage-containers/hyperv-container) (the only option on Windows 10), the container runs in an extremely minimal VM. By default this VM is given 1GB of memory. This can be adjusted with `docker run`'s `--memory` switch. You can have Habitat pass along increased memory by setting the `$env:HAB_DOCKER_OPTS` variable to `--memory 2GB` or whatever your desired memory amount may be.

### Why does it take several seconds to enter a Windows container Studio?

When running with Hyper-V isolation, there is a startup penalty incurred by starting the light weight VM and you will notice that the Studio takes longer to enter than a Linux Studio or local Windows Studio.

### Why is the Studio shell navigation so disorienting?

I have noticed this myself using the [ConEmu](https://conemu.github.io/) terminal emulator. The arrow keys do not work at all. If you encounter this kind of odd shell behavior, try using the standard `cmd` or `Powershell` based consoles. I have tested both of those on Windows 10 and found that the container Studio shell navigation behaves normally there.
