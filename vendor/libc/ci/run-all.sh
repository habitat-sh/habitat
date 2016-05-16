# This is **not** meant to be run on CI, but rather locally instead. If you're
# on a Linux machine you'll be able to run most of these, but otherwise this'll
# just attempt to run as many platforms as possible!

run() {
    _target=$1
    _cc=$2
    if [ "$_cc" != "" ]; then
        which $_cc > /dev/null
        if [ $? -ne 0 ]; then
           echo "$_cc not installed, skipping $_target"
           return
        fi
        export CC=$_cc
    fi
    if [ ! -d .cargo ]; then
        mkdir .cargo
        cp ci/cargo-config .cargo/config
    fi
    sh ci/run.sh $_target
    if [ "$?" != "0" ]; then
        exit 1
    fi
}

OS=`uname`
if [ "$OS" = "Linux" ]; then
    # For more info on where to get all these cross compilers see
    # ci/run-travis.sh and what packages are needed on ubuntu
    run x86_64-unknown-linux-gnu clang
    run i686-unknown-linux-gnu clang
    run x86_64-unknown-linux-musl musl-gcc
    run mips-unknown-linux-gnu mips-linux-gnu-gcc
    run aarch64-unknown-linux-gnu aarch64-linux-gnueabihf-gcc
    run arm-unknown-linux-gnueabihf arm-linux-gnueabihf-gcc-4.7

    # Prep for this by running `vagrant up freebsd` in the `ci` directory
    (cd ci && vagrant ssh freebsd -c \
        "cd /vagrant && sh ci/run.sh x86_64-unknown-freebsd")

    # Make sure you've run `docker pull alexcrichton/rust-libc-test` to get
    # this image ahead of time.
    docker run -itv `pwd`:/clone alexcrichton/rust-libc-test \
        sh ci/run.sh arm-linux-androideabi
elif [ "$OS" = "Darwin" ]; then
    cargo run --target x86_64-unknown-linux-gnu
    cargo run --target i686-unknown-linux-gnu
fi
