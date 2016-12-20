# Building Habitat from source

## Mac OS X

These install instructions assume you want to develop, build, and run the
various Habitat software components in a Linux environment. The Habitat core
team suggests that you use our consistent development environment that we can
the "devshell" as the easiest way to get started.

1. [Install Docker for Mac](https://www.docker.com/products/docker)
1. Checkout the source by running `git clone git@github.com:habitat-sh/habitat.git; cd habitat`
1. Run `make` to compile all Rust software components (this will take a while)
1. (Optional) Run `make test` if you want to run the tests. This will take a while.

Everything should come up green. Congratulations - you have a working Habitat
development environment.

You can enter a devshell by running `make shell`. This will drop you in a
Docker container at a Bash shell. The source code is mounted in under `/src`,
meaning you can use common Rust workflows such as `cd components/sup; cargo
build`.

**Note:** The Makefile targets are documented. Run `make help` to show the
output. Habitat requires `perl`.

**Optional:** This project compiles and runs inside Docker containers so while
installing the Rust language isn't strictly necessary, you might want a local
copy of Rust on your workstation (some editors' language support require an
installed version). To [install stable
Rust](https://www.rust-lang.org/install.html), run: `curl -sSf
https://static.rust-lang.org/rustup.sh | sh`. Additionally, the project
maintainers use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) for
code formatting. If you are submitting changes, please ensure that your work
has been run through the latest version of rustfmt. An easy way to install it
(assuming you have Rust installed as above), is to run `cargo install rustfmt`
and adding `$HOME/.cargo/bin` to your `PATH`.


## Ubuntu: Latest (16.10/Yakkety)

This installation method uses as many packages from Ubuntu as possible. This
will closely reproduce the state of the Docker-based "devshell" as it also uses
an Ubuntu base image.

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_latest.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
```

These docs were tested with a Docker image, created as follows:

```
docker run --rm -it ubuntu:yakkety bash
apt-get update && apt-get install -y sudo git-core
useradd -m -s /bin/bash -G sudo jdoe
echo jdoe:1234 | chpasswd
sudo su - jdoe
```


## Ubuntu: 14.04+ (Trusty+)

This can be used to build and install on older versions of Ubuntu where
libsodium and czmq aren't available.

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_14.04.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
```

These docs were tested with a Docker image, created as follows:

```
docker run --rm -it ubuntu:trusty bash
apt-get update && apt-get install -y sudo git-core
useradd -m -s /bin/bash -G sudo jdoe
echo jdoe:1234 | chpasswd
sudo su - jdoe
```


## Centos 7

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_centos_7.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
```

If you have issues with libsodium at runtime, ensure that you've set
`LD_LIBRARY_PATH` and `PKG_CONFIG_PATH`:

    export LD_LIBRARY_PATH=/usr/local/lib
    export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig

These docs were tested with a Docker image, created as follows:

```
docker run --rm -it centos:7 bash
yum install -y sudo git
useradd -m -s /bin/bash -G wheel jdoe
echo jdoe:1234 | chpasswd
sudo su - jdoe
```


## Windows

These instructions are based on Windows 10 1607 (Anniversary update) or newer.
Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you
have `git` installed and configured.  Posh-Git is a handy PowerShell module for
making `git` better in your PowerShell console (`install-module posh-git`).

```
# Clone the Windows build script
git clone https://github.com/smurawski/hab-build-script.git

# Clone the Habitat source
git clone https://github.com/habitat-sh/habitat.git

# Install the psake PowerShell module
import-module PowerShellGet -force
install-module psake -force

# Change into build script directory
cd ./hab-build-script

# Build Habitat
invoke-psake
```


## General build notes

- Once make has finished, executables will exist in `/src/target/debug/foo`,
  where `foo` is the name of an executable (`hab`, `hab-sup`, `hab-depot`,
  etc).
- Executable names are specified in each components `Cargo.toml` file in a TOML
  table like this:

		[[bin]]
		name = "hab-depot"


## Windows build notes

The Windows build scripts referenced above and the default build task
`invoke-psake` attempts to build the full habitat project as well as validates
pre-reqs are installed. This is not always and ideal way to build or test. In
some cases the following tasks may be more appropriate.

```
# Build all the currently ported crates
invoke-psake -tasklist build_all

# Build only the current crate in progress
invoke-psake -tasklist current_build

# Test all the currently ported crates
invoke-psake -tasklist test_all

# Run tests on the current crate in progress
invoke-psake -tasklist current_test
```


#### Building the native dependencies

You'll want to start in a fresh PowerShell instance, with the Visual C++ Build
Tools paths and environment variables set.

I use a handy `Start-VsDevShell` function in my profile.

```
function Start-VsDevShell {
  cmd.exe --% /k ""C:\Program Files (x86)\Microsoft Visual C++ Build Tools\vcbuildtools.bat" amd64" & powershell
}
```

then

```
Start-VsDevShell
cd ~/source/hab-build-script
invoke-psake -tasklist build_native_deps
```
