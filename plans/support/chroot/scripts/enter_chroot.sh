#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ enter_chroot.sh [<COMMAND> [<ARG>...]]
# ```
#
# # Synopsis
#
# `enter_chroot.sh`
#
# # Environment Variables
#
# There are several enviroment variables that are used with this program:
#
# * `$CHROOT` (**Required**): The root directory of the chroot filesystem
# * `$DEBUG` (*Optional*): If set, the program will output the shell commands
#    as they are being executed
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2015 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ```
#
#

# # Main program

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
fi

# Determine which path to `env(1)` we should use--if the stage1 `/tools`
# version is present, use this, and otherwise assume that `/usr/bin/env` is
# correct and exists.
if [ -x "$CHROOT/tools/bin/env" ]; then
  env_cmd="/tools/bin/env"
else
  env_cmd="/usr/bin/env"
fi

# Determine which path to `bash(1)` we should use--if the stage1 `/tools`
# version is present, use this, and otherwise assume that `/bin/bash` is
# correct and exists.
if [ -x "$CHROOT/tools/bin/bash" ]; then
  bash_cmd="/tools/bin/bash"
else
  bash_cmd="/bin/bash"
fi

# Build a minimal `$PATH` for use inside the chroot environment.
path="/bin:/usr/bin:/sbin:/usr/sbin"
# `/tools/bin` is added, but at the end of `$PATH` to that any other version
# of a command will be found first
if [ -d "$CHROOT/tools/bin" ]; then
  path="${path}:/tools/bin"
fi
# The `bin/` directory of the chroot support files is added at the very
# end of the `$PATH` to make programs such as `capture.sh` available
if [ -d "$CHROOT/plans/support/chroot/bin" ]; then
  path="${path}:/plans/support/chroot/bin"
fi

# Determine the command to run inside the chroot environment. If no arguments
# to this program are provided then a `bash(1)` will be run. Otherwise the
# provided command will be executed instead.
if [ -z "${*:-}" ]; then
  cmd="$bash_cmd --login +h"
else
  cmd="$*"
fi

# Build the base environment variable set to be passed into `script(1)`. We
# propagate the `$PATH` of our caller, but set `$HOME` explictly. If either
# `$http_proxy` or `$https_proxy` environment variables are present, pass
# these along as well.
env="HOME=/root TERM=$TERM PATH=$path"
if [ -n "${http_proxy:-}" ]; then
  env="$env http_proxy=$http_proxy"
fi
if [ -n "${https_proxy:-}" ]; then
  env="$env https_proxy=$https_proxy"
fi

echo "==> Entering CHROOT"
echo

# Display the final setup and execution for the user
set -x

# Finally, become the `chroot(8)` process
exec chroot "$CHROOT" "$env_cmd" -i $env 'PS1=[chroot] \u:\w\$ ' $cmd
