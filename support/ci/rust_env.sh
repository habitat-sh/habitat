# This version corresponds to the build in `support/ci/compile_libsodium.sh`
export LIBSODIUM_PREFIX="$HOME/pkgs/libsodium/1.0.12"
echo "--> Setting LIBSODIUM_PREFIX='$LIBSODIUM_PREFIX'"
# This version corresponds to the build in `support/ci/compile_libarchive.sh`
export LIBARCHIVE_PREFIX="$HOME/pkgs/libarchive/3.2.0"
echo "--> Setting LIBARCHIVE_PREFIX='$LIBARCHIVE_PREFIX'"
# This version corresponds to the build in `support/ci/compile_zmq.sh`
export LIBZMQ_PREFIX="$HOME/pkgs/zeromq/4.1.4"
echo "--> Setting LIBZMQ_PREFIX='$LIBZMQ_PREFIX'"

# Used by Cargo builds using the `pkg-config` to find out compiled libs
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH:-}:$LIBARCHIVE_PREFIX/lib/pkgconfig:$LIBSODIUM_PREFIX/lib/pkgconfig:$LIBZMQ_PREFIX/lib/pkgconfig"
echo "--> Setting PKG_CONFIG_PATH='$PKG_CONFIG_PATH'"
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$LIBARCHIVE_PREFIX/lib:$LIBSODIUM_PREFIX/lib:$LIBZMQ_PREFIX/lib"
echo "--> Setting LD_LIBRARY_PATH='$LD_LIBRARY_PATH'"
export LD_RUN_PATH="${LD_RUN_PATH:-}:$LIBARCHIVE_PREFIX/lib:$LIBSODIUM_PREFIX/lib:$LIBZMQ_PREFIX/lib"
echo "--> Setting LD_RUN_PATH='$LD_RUN_PATH'"
export LIBRARY_PATH="${LIBRARY_PATH:-}:$LIBZMQ_PREFIX/lib"
echo "--> Setting LIBRARY_PATH='$LIBRARY_PATH'"
