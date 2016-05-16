#!/bin/sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

TARGET=$1
case "$TARGET" in
  *-apple-ios)
    cargo rustc --manifest-path libc-test/Cargo.toml --target $TARGET -- \
        -C link-args=-mios-simulator-version-min=7.0
    ;;

  *)
    cargo build --manifest-path libc-test/Cargo.toml --target $TARGET
    ;;
esac

case "$TARGET" in
  arm-linux-androideabi)
    emulator @arm-21 -no-window &
    adb wait-for-device
    adb push /tmp/$TARGET/debug/libc-test /data/libc-test
    adb shell /data/libc-test 2>&1 | tee /tmp/out
    grep "^PASSED .* tests" /tmp/out
    ;;

  arm-unknown-linux-gnueabihf)
    qemu-arm -L /usr/arm-linux-gnueabihf libc-test/target/$TARGET/debug/libc-test
    ;;

  mips-unknown-linux-gnu)
    qemu-mips -L /usr/mips-linux-gnu /tmp/$TARGET/debug/libc-test
    ;;

  aarch64-unknown-linux-gnu)
    qemu-aarch64 -L /usr/aarch64-linux-gnu/ \
      libc-test/target/$TARGET/debug/libc-test
    ;;

  *-rumprun-netbsd)
    rumprun-bake hw_virtio /tmp/libc-test.img /tmp/$TARGET/debug/libc-test
    qemu-system-x86_64 -nographic -vga none -m 64 \
        -kernel /tmp/libc-test.img 2>&1 | tee /tmp/out &
    sleep 5
    grep "^PASSED .* tests" /tmp/out
    ;;

  *)
    libc-test/target/$TARGET/debug/libc-test
    ;;
esac
