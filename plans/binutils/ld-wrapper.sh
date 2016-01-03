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
# so that additional options can be added before invoking it. There are 2
# primary responsibilities for this program:
#
# 1. Ensure that correct dynamic linker is used, by setting a `-dynamic-linker`
#    option
# 1. Add an `-rpath` option for each entry in the `$LD_RUN_PATH` environment
#    variable, if it is set
#
# The idea and implementation which this is based upon is thanks to the nixpkgs
# project, specifically in the `cc-wrapper` package. For more details, see:
# https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/cc-wrapper/ld-wrapper.sh
#
# # Environment Variables
#
# There are several environment variables that are consumed by this program:
#
# * `$LD_RUN_PATH` (*Optional*): Each path entry in this variable will get a
#   corresponding `-rpath $path` option added to the real program invocation
# * `$DEBUG` (*Optional*): If set, this program will output the original and
#    extra flags added to standard error
#
#

# # Main program

# Fail whenever a command returns a non-zero exit code.
set -e

# Populate a `$params` variable with all arguments passed to this program.
params=("$@")

# Create an empty array for extra arguments.
extra=()

# Add the dynamic linker.
extra+=("-dynamic-linker @dynamic_linker@")

# Add `-rpath` switches.
#
# Sidenote: why the process substitution strangeness? Learn more:
# http://mywiki.wooledge.org/BashFAQ/024
while read path; do
  if [ -n "$path" ]; then
    extra+=("-rpath $path")
  fi
done < <(echo $LD_RUN_PATH | tr : '\n')

# Optionally print debug info.
if [ -n "$DEBUG" ]; then
  echo "original flags to @program@:" >&2
  for i in "${params[@]}"; do
    echo "  $i" >&2
  done
  echo "extra flags to @program@:" >&2
  for i in ${extra[@]}; do
    echo "  $i" >&2
  done
fi

# Become the underlying real program
exec @program@ "${params[@]}" ${extra[@]}
