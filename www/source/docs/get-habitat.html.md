---
title: Habitat - Download
description: Download Habitat, an open source project created by Chef, so your apps can behave consistently in any runtime – bare metal, VMs, containers, and PaaS.

---

# Get Habitat
To get started with Habitat, you only need to download the `hab` command-line interface tool (CLI) for your specific operating system.

### For Mac
To start using Habitat on Mac OS X and macOS, download the following binary. Then unzip `hab` to `/usr/local/bin` and run `chmod a+x hab` to make it executable. By copying it into `/usr/local/bin`, it will be automatically added to your `PATH` variable. 
If you intend to build Habitat packages on your Mac, you will also need to install [Docker for Mac](https://www.docker.com/products/docker#/mac). 

_Habitat for Mac requires a 64-bit processor running Mac OS X version 10.9+._


<a class="button" href="https://api.bintray.com/content/habitat/stable/darwin/x86_64/hab-%24latest-x86_64-darwin.zip?bt_package=hab-x86_64-darwin">Download Habitat for Mac OS X</a>
<a class="button secondary" href="https://www.docker.com/products/docker#/mac">Download Docker for Mac</a>

### For Linux
To start using Habitat on Linux, download the following binary. Then unzip `hab` to `/usr/local/bin` and run `chmod a+x hab` to make it executable. As with the Mac instructions above, by copying the `hab` CLI into `/usr/local/bin`, it will automatically be added to your `PATH` variable.

_Habitat for Linux requires a 64-bit processor with a kernel greater than 2.6.32._

<a class="button" href="https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux">Download Habitat for Linux</a>

### For Windows
To start using Habitat on Windows, download the following binary.  Then unzip `hab` to `C:\habitat` and add `C:\habitat` to your `PATH`. Here's an example of how to do that in PowerShell:

    $env:PATH += "C:\habitat"

To keep from typing this in for every PowerShell window you use, add the above statement to your PowerShell profile. You can learn more about PowerShell profiles [here](https://msdn.microsoft.com/en-us/powershell/reference/5.1/microsoft.powershell.core/about/about_profiles).

If you intend to build Habitat packages on your Windows PC, you will also need to install [Docker for Windows](https://docs.docker.com/docker-for-windows/).

_Habitat for Windows requires 64bit Windows 10 Pro, Enterprise and Education (1511 November update, Build 10586 or later) and Microsoft Hyper-V._

> Note: Currently Habitat can only build and manage linux based packages. Support for Windows-based applications is coming.

<a class="button" href="https://api.bintray.com/content/habitat/stable/windows/x86_64/hab-%24latest-x86_64-windows.zip?bt_package=hab-x86_64-windows">Download Habitat for Windows</a>
<a class="button secondary" href="https://download.docker.com/win/stable/InstallDocker.msi">Download Docker for Windows</a>

<hr>

## Contribute

### Help build Habitat
Habitat is an open-source project. To learn more about contributing to the project visit the [Help Build Habitat](/docs/contribute-help-build) page.

### Browse the source code
You can also browse the Habitat repo on GitHub at [https://github.com/habitat-sh/habitat](https://github.com/habitat-sh/habitat); however, _you do not need to clone the source code_ in order to use Habitat.
