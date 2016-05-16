Pending merging of https://github.com/rust-lang/libc/pull/289

This is used in order to build the `x86_64-unknown-linux-musl` targets for the `hab` component.

The vendoring was accomplised by:

```
(cd /tmp; curl -L -o /tmp/libc.tar.gz https://github.com/fnichol/rust-lang-libc/archive/fix-musl-ioctl-constants.tar.gz)
mkdir -p vendor/libc
tar xfz /tmp/libc.tar.gz -C vendor/libc --strip-components=1
rm -f /tmp/libc.tar.gz
```
