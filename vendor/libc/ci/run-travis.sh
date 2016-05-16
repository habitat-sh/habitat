# Entry point for all travis builds, this will set up the Travis environment by
# downloading any dependencies. It will then execute the `run.sh` script to
# build and execute all tests.
#
# For a full description of how all tests are run, see `ci/README.md`

set -ex

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  OS=unknown-linux-gnu
else
  OS=apple-darwin
fi

export HOST=$ARCH-$OS
if [ "$TARGET" = "" ]; then
  TARGET=$HOST
fi

MAIN_TARGETS=https://static.rust-lang.org/dist
DATE=$(echo $TRAVIS_RUST_VERSION | sed s/nightly-//)
if [ "$DATE" != "nightly" ]; then
    MAIN_TARGETS=$MAIN_TARGETS/$DATE
    TRAVIS_RUST_VERSION=nightly
fi

install() {
  if [ "$TRAVIS" = "true" ]; then
    sudo apt-get update
    sudo apt-get install -y $@
  fi
}

# If we're going to run tests inside of a qemu image, then we don't need any of
# the scripts below. Instead, download the image, prepare a filesystem which has
# the current state of this repository, and then run the image.
#
# It's assume that all images, when run with two disks, will run the `run.sh`
# script from the second which we place inside.
if [ "$QEMU" != "" ]; then
  # Acquire QEMU and the base OS image
  install qemu-kvm
  tmpdir=/tmp/qemu-img-creation
  mkdir -p $tmpdir
  if [ ! -f $tmpdir/$QEMU ]; then
    curl https://people.mozilla.org/~acrichton/libc-test/qemu/$QEMU.gz | \
      gunzip -d > $tmpdir/$QEMU
  fi

  # Generate all.{c,rs} on the host which will be compiled inside QEMU. Do this
  # here because compiling syntex_syntax in QEMU would time out basically
  # everywhere.
  rm -rf $tmpdir/generated
  mkdir -p $tmpdir/generated
  CARGO_TARGET_DIR=$tmpdir/generated-build \
    cargo build --manifest-path libc-test/generate-files/Cargo.toml
  (cd libc-test && TARGET=$TARGET OUT_DIR=$tmpdir/generated SKIP_COMPILE=1 \
    $tmpdir/generated-build/debug/generate-files)

  # Create a mount a fresh new filesystem image that we'll later pass to QEMU,
  # this contains the checkout of libc and will be able to run all tests
  rm -f $tmpdir/libc-test.img
  dd if=/dev/null of=$tmpdir/libc-test.img bs=1M seek=5
  mkfs.ext2 -F $tmpdir/libc-test.img
  rm -rf $tmpdir/mount
  mkdir $tmpdir/mount
  sudo mount -t ext2 -o loop $tmpdir/libc-test.img $tmpdir/mount

  # Copy this folder into the mounted image, the `run.sh` entry point, and
  # overwrite the standard libc-test Cargo.toml with the overlay one which will
  # assume the all.{c,rs} test files have already been generated
  sudo mkdir $tmpdir/mount/libc
  sudo cp -r * $tmpdir/mount/libc/
  sudo cp ci/run-qemu.sh $tmpdir/mount/run.sh
  echo $TARGET | sudo tee -a $tmpdir/mount/TARGET
  sudo cp $tmpdir/generated/* $tmpdir/mount/libc/libc-test
  sudo cp libc-test/run-generated-Cargo.toml $tmpdir/mount/libc/libc-test/Cargo.toml

  sudo umount $tmpdir/mount

  # If we can use kvm, prefer that, otherwise just fall back to user-space
  # emulation.
  if kvm-ok; then
    program="sudo kvm"
  else
    program=qemu-system-x86_64
  fi

  # Pass -snapshot to prevent tampering with the disk images, this helps when
  # running this script in development. The two drives are then passed next,
  # first is the OS and second is the one we just made. Next the network is
  # configured to work (I'm not entirely sure how), and then finally we turn off
  # graphics and redirect the serial console output to out.log.
  $program \
    -m 1024 \
    -snapshot \
    -drive if=virtio,file=$tmpdir/$QEMU \
    -drive if=virtio,file=$tmpdir/libc-test.img \
    -net nic,model=virtio \
    -net user \
    -nographic \
    -vga none 2>&1 | tee out.log
  exec grep "^PASSED .* tests" out.log
fi

mkdir -p .cargo
cp ci/cargo-config .cargo/config

# Next up we need to install the standard library for the version of Rust that
# we're testing.
if [ "$TRAVIS" = "true" ]; then
  curl https://static.rust-lang.org/rustup.sh | \
    sh -s -- --add-target=$TARGET --disable-sudo -y \
      --prefix=`rustc --print sysroot`
fi

# If we're testing with a docker image, then run tests entirely within that
# image. Note that this is using the same rustc installation that travis has
# (sharing it via `-v`) and otherwise the tests run entirely within the
# container.
#
# For the docker build we mount the entire current directory at /checkout, set
# up some environment variables to let it run, and then run the standard run.sh
# script.
if [ "$DOCKER" != "" ]; then
  args=""

  case "$TARGET" in
    mips-unknown-linux-gnu)
      args="$args -e CC=mips-linux-gnu-gcc-5"
      ;;

    *)
      ;;
  esac

  exec docker run \
    --entrypoint bash \
    -v `rustc --print sysroot`:/usr/local:ro \
    -v `pwd`:/checkout \
    -e LD_LIBRARY_PATH=/usr/local/lib \
    -e CARGO_TARGET_DIR=/tmp \
    $args \
    -w /checkout \
    -it $DOCKER \
    ci/run.sh $TARGET
fi

# If we're not running docker or qemu, then we may still need some packages
# and/or tools with various configurations here and there.
case "$TARGET" in
  x86_64-unknown-linux-musl)
    install musl-tools
    export CC=musl-gcc
    ;;

  arm-unknown-linux-gnueabihf)
    install gcc-4.7-arm-linux-gnueabihf qemu-user
    export CC=arm-linux-gnueabihf-gcc-4.7
    ;;

  aarch64-unknown-linux-gnu)
    install gcc-aarch64-linux-gnu qemu-user
    export CC=aarch64-linux-gnu-gcc
    ;;

  *-apple-ios)
    ;;

  *)
    # clang has better error messages and implements alignof more broadly
    export CC=clang

    if [ "$TARGET" = "i686-unknown-linux-gnu" ]; then
      install gcc-multilib
    fi
    ;;

esac

# Finally, if we've gotten this far, actually run the tests.
sh ci/run.sh $TARGET

if [ "$TARGET" = "x86_64-unknown-linux-gnu" ] && \
   [ "$TRAVIS_RUST_VERSION" = "nightly" ] && \
   [ "$TRAVIS_OS_NAME" = "linux" ]; then
  sh ci/dox.sh
fi
