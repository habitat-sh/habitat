# Building Habitat from source

## Mac OS X for Linux Development

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
output (this target requires `perl`).

**Optional:** This project compiles and runs inside Docker containers so while
installing the Rust language isn't strictly necessary, you might want a local
copy of Rust on your workstation (some editors' language support require an
installed version). To [install stable
Rust](https://www.rust-lang.org/install.html), run: `curl -sSf
https://sh.rustup.rs | sh`. Additionally, the project maintainers use
[rustfmt](https://github.com/rust-lang-nursery/rustfmt) for code formatting. If
you are submitting changes, please ensure that your work has been run through
the latest version of rustfmt. An easy way to install it (assuming you have
Rust installed as above), is to run `cargo install rustfmt` and adding
`$HOME/.cargo/bin` to your `PATH`.


## Mac OS X for Native Development

These instructions assume you want to develop, build, and run the various
Habitat software components in a macOS environment. While components other than
the `hab` CLI itself aren't officially supported on Mac, it is possible and
sometimes useful to run parts of the Habitat ecosystem without virtualization.

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/mac/install_dev_0_mac_latest.sh
sh support/mac/install_dev_9_mac.sh
. ~/.profile
export PKG_CONFIG_PATH="/usr/local/opt/libarchive/lib/pkgconfig:/usr/local/opt/openssl/lib/pkgconfig:$PKG_CONFIG_PATH"
export IN_DOCKER=false
make
```

For the builds to find the libarchive and openssl libraries, you will need to
set the `PKG_CONFIG_PATH` environment variable as above before running `cargo
build`, `cargo test`, etc. Additionally to use the Makefile on Mac and not have
the tasks execute in the Docker-based devshell (see above), you will need to
set `IN_DOCKER=false` in your environment. If you use an environment switcher
such as [direnv](https://direnv.net/), you can set up the following in the root
of the git repository:

```
echo 'export PKG_CONFIG_PATH="/usr/local/opt/libarchive/lib/pkgconfig:/usr/local/opt/openssl/lib/pkgconfig:$PKG_CONFIG_PATH"' > .direnv
direnv allow
```


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


## Arch Linux

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_arch.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
```

These docs were tested with a Docker image, created as follows:

```
docker run --rm -it greyltc/archlinux bash
pacman -Syy --noconfirm
pacman -S --noconfirm sudo git
echo "%wheel ALL=(ALL) ALL" > /etc/sudoers.d/01_wheel
useradd -m -s /bin/bash -G wheel jdoe
echo jdoe:1234 | chpasswd
sudo su - jdoe
```

## VM vs. Docker development

While the available docker images can provide a convenient contributor onboarding experience, they may not prove ideal for extensive habitat development. Building habitat components is a disk intensive operation. Mounted file systems accross network boundaries, even when confined to a single local host, can yield build times dramatically slower than building against a locally attached file system. Build times will be best on either a bare metal linux environment or a dedicated linux vm. Also note that when using vagrant to provision a vm for habitat development, it is strongly advised that you do not put your habitat repo on a synced folder or at least do not use that folder as your build target. You should copy or clone the repository to a local directory in order to realize much faster build times.

Here is a [vagrantfile](https://github.com/mwrock/vagrantfile/blob/master/UbuHab) that provisions an ubuntu environment for habitat development. It leverages a Hyper-V provisioner but its provisioning script can be used in any hypervisor that uses an ubuntu 14.04 box.

## Windows

These instructions are based on Windows 10 1607 (Anniversary update) or newer.
Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you
have `git` installed and configured.  Posh-Git is a handy PowerShell module for
making `git` better in your PowerShell console (`install-module posh-git`).

```
# Clone the Habitat source
git clone https://github.com/habitat-sh/habitat.git

cd habitat

./build.ps1 components/hab -configure

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

The `-configure` switch will make sure you have all the necessary dependencies to build the `hab` CLI tool, including Rust, the Visual Studio Build Tools, and all the native dependencies.

Not all crates have been fully ported to Windows.

Currently the `hab` command will build (as well as the dependent crates).

Work is in progress on the supervisor and other parts of the toolchain.

# Building and running individual components

When you are working on an individual component in the /components directory, you may wish to build, install, then use that individual component.

Let's say you want to do this with the supervisor (which lives in the components/sup directory).

## Building

Change directories into the component you want to build

```
  $ cd habitat/components/sup
```

Then run

```
  $ cargo build
```

Once it is finished compiling, you can find the new build in root hab_repo/target/debug

Head back to the root of the Habitat repo

```
  $ cd ../..
```

And you will find your build in target/debug

If you built the sup component, this is where you would find the new build

```
  $ target/debug/hab-sup
```

## Running

You can now run this newly built component with

```
  $ ./target/debug/hab-sup
```
