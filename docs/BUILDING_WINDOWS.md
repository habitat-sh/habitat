# Building Habitat from source

*NOTE:* This document is likely to change a lot over the coming months.

*NOTE:* For Rust editing, I'd recommend Visual Studio Code and have [a blog post on how to set up the best Rust editing experience](http://stevenmurawski.com/powershell/2016/08/rustify).

This document is based on Windows 10 1607 (Anniversary update) or newer.  Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you have `git` installed and configured.  Posh-Git is a handy PowerShell module for making `git` better in your PowerShell console (`install-module posh-git`).

```
# Move to where you want to download the source
# for this example, I'll use ~/source
cd ~/source

# Grab the current build scripts
git clone https://github.com/smurawski/hab-build-script.git

# Grab the latest source
git clone https://github.com/habitat-sh/habitat.git

# Install the psake PowerShell module
# I'm currently using psake until we can transition to 
# habitat builds natively
import-module PowerShellGet -force
install-module psake -force

# We are going to move into the build script directory
cd ./hab-build-script

# Option 1 - Install the pre-reqs for building
# the pre-req checker will use chocolatey (installing if missing)
invoke-psake -taskname pre_reqs

# Option 2 - Or just start the default build task (see below)
# (which will verify the pre-reqs and install if missing)
```
## To Build

You'll want to start in a fresh PowerShell instance, with the Visual C++ Build Tools paths and environment variables set.

I use a handy `Start-VsDevShell` function in my profile.

```
function Start-VsDevShell {
  cmd.exe --% /k ""C:\Program Files (x86)\Microsoft Visual C++ Build Tools\vcbuildtools.bat" amd64" & powershell
}
```


### Get in the right working directory

```
Start-VsDevShell
cd ~/source/hab-build-script
```

### Common Build Tasks

#### Build

```
# The default task attempts to build the current project
# that is in the process of being ported.  It also 
# validates pre-reqs are installed and builds native dependencies. 

invoke-psake
```

```
# Build all the currently ported crates

invoke-psake -taskname build
```

```
# Build the current crate in progress

invoke-psake -taskname current_build
```

#### Test

```
# Test all the currently ported crates

invoke-psake -taskname test
```


```
# Run tests on the current crate in progress

invoke-psake -taskname current_test
```