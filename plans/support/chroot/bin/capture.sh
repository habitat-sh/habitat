#
# # Usage
#
# ```
# $ capture.sh <SESSION_NAME>
# ```
#
# # Synopsis
#
# `capture.sh` provides an easy way to start logging a terminal session in
# order to capture a full session transctipt. It is designed to execute in a
# minimal chrooted build environment, and therefore only yields a minimal
# set of environment variables. At the end of the session, simply type `exit`
# and you are returned to the previous session context.
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

# The name of session, used for file naming and prompt decoration
name="$1"
# Determine the path to the `env(1)` command
env_cmd="`command -v env`"
# The directory that will hold all the session log files
log_dir="/plans/logs"

# Determine which path to `bash(1)` we should use--if the stage1 `/tools`
# version is present, use this, and otherwise assume that `/bin/bash` is
# correct and exists.
if [ -x /tools/bin/bash ]; then
  bash_cmd="/tools/bin/bash"
else
  bash_cmd="/bin/bash"
fi

# Build the base environment variable set to be passed into `script(1)`. We
# propagate the `$PATH` of our caller, but set `$HOME` explictly. If either
# `$http_proxy` or `$https_proxy` environment variables are present, pass
# these along as well.
env="HOME=/root TERM=$TERM PATH=$PATH"
if [ -n "${http_proxy:-}" ]; then
  env="$env http_proxy=$http_proxy"
fi
if [ -n "${https_proxy:-}" ]; then
  env="$env https_proxy=$https_proxy"
fi

# The final command that will be passed to `script(1)`
command="$env_cmd -i $env 'PS1=[chroot:$name] \u:\w\$ ' $bash_cmd --login +h"
# The current timestamp, in UTC--the only true time
timestamp="`date -u +%Y%m%d%H%M%S`"

# Display the final setup and execution for the user
set -x

# Create the log directory, if not present
mkdir -pv "$log_dir"

# Finally, become the `script(1)` process
exec script -c "$command" "$log_dir/${name}.${timestamp}.log"
