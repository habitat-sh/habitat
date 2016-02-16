#!@shell@
#
# # Usage
#
# ```
# $ @program@ [ARG ..]
# ```
#
# # Synopsis
#
# This program takes the place of
# @program@
# so that additional options can be added before invoking it.
#
# The idea and implementation which this is based upon is thanks to the nixpkgs
# project, specifically in the `cc-wrapper` package. For more details, see:
# https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/cc-wrapper/cc-wrapper.sh
#
# # Environment Variables
#
# There is one environment variables that is consumed by this program:
#
# * `$DEBUG` (*Optional*): If set, this program will output the original and
#    extra flags added to standard error
#
#

# # Main program

# Fail whenever a command returns a non-zero exit code.
set -e

# The `-B$libc/lib/` flag is a quick hack to force gcc to link against the
# crt1.o from our own glibc, rather than the one in `/usr/lib`.
#
# Unfortunately, setting `-B` appears to override the default search path.
# Thus, the gcc-specific `../includes-fixed` directory is now longer searched
# and glibc's `<limits.h>` header fails to compile, because it uses
# `#include_next <limits.h>` to find the limits.h file in ../includes-fixed. To
# remedy the problem, another `-idirafter` is necessary to add that directory
# again.
libc_cflags="-B@glibc@/lib/"
libc_cflags="$libc_cflags -idirafter @glibc@/include"
libc_cflags="$libc_cflags -idirafter @gcc@/lib/gcc/*/*/include-fixed"

# Force gcc to use our ld wrapper from binutils when calling `ld`
libc_cflags="$libc_cflags -B@binutils@/bin/"

# Figure out if linker flags should be passed.  GCC prints annoying
# warnings when they are not needed.
dontLink=0
getVersion=0
nonFlagArgs=0

# Determine is we add dynamic linker arguments to the extra arguments by
# looking at the calling arguments to this program. This may not work 100% of
# the time, but it has shown to be fairly reliable
for i in "$@"; do
  if [ "$i" = -c ]; then
    dontLink=1
  elif [ "$i" = -S ]; then
    dontLink=1
  elif [ "$i" = -E ]; then
    dontLink=1
  elif [ "$i" = -E ]; then
    dontLink=1
  elif [ "$i" = -M ]; then
    dontLink=1
  elif [ "$i" = -MM ]; then
    dontLink=1
  elif [ "$i" = -x ]; then
    # At least for the cases c-header or c++-header we should set dontLink.
    # I expect no one use -x other than making precompiled headers.
    dontLink=1
  elif [ "${i:0:1}" != - ]; then
    nonFlagArgs=1
  fi
done

# If we pass a flag like -Wl, then gcc will call the linker unless it
# can figure out that it has to do something else (e.g., because of a
# "-c" flag).  So if no non-flag arguments are given, don't pass any
# linker flags.  This catches cases like "gcc" (should just print
# "gcc: no input files") and "gcc -v" (should print the version).
if [ "$nonFlagArgs" = 0 ]; then
  dontLink=1
fi

params=("$@")

# If we are calling a c/g++ style program, set additional flags.
if [[ "$(basename @program@)" = *++ ]]; then
  if  echo "$@" | grep -qv -- -nostdlib; then
    libc_cflags="$libc_cflags -isystem @gcc@/include/c++/*"
    libc_cflags="$libc_cflags -isystem @gcc@/include/c++/*/$(@gcc@/bin/gcc -dumpmachine)"
  fi
fi

# Add the flags for the C compiler proper.
extraBefore=()
extraAfter=($libc_cflags)

if [ "$dontLink" != 1 ]; then
  extraBefore+=("-Wl,-dynamic-linker" "-Wl,@dynamic_linker@")
fi

# As a very special hack, if the arguments are just `-v', then don't
# add anything.  This is to prevent `gcc -v' (which normally prints
# out the version number and returns exit code 0) from printing out
# `No input files specified' and returning exit code 1.
if [ "$*" = -v ]; then
  extraAfter=()
  extraBefore=()
fi

# Optionally print debug info.
if [ -n "$DEBUG" ]; then
  echo "original flags to @program@:" >&2
  for i in "${params[@]}"; do
    echo "  $i" >&2
  done
  echo "extraBefore flags to @program@:" >&2
  for i in ${extraBefore[@]}; do
    echo "  $i" >&2
  done
  echo "extraAfter flags to @program@:" >&2
  for i in ${extraAfter[@]}; do
    echo "  $i" >&2
  done
fi

# Become the underlying real program
exec @program@ ${extraBefore[@]} "${params[@]}" "${extraAfter[@]}"
