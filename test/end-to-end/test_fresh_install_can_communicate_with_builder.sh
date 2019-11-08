#!/bin/bash

# Given a fresh install of Habitat we should be able to install packages from builder
# Ensure that we don't have any ssl certificates cached that might influence our ability
# to connect. 
# Note: since we're testing pre-release this won't be a "pure" fresh install, as we 
# have to curlbash install stable first, in order to get the pre-release version. 

set -euo pipefail

echo "--- Inspecting ssl cache directories"
# The assumption here is that the directory is not created on a fresh install until 
# a certificate needs to be cached. If something in the provided images or setup_environment 
# break that assumption this test could start failing. 
test ! -d /hab/cache/ssl

# This test is expected to run in a container, so will be executed as root.  
# Hab will always use /hab/cache/ssl when run as root, but in development scenarios
# hab will be run as a normal user. Run the test as the `hab` user, provided by
# setup_environment.sh
su hab -c "test ! -d ~/.hab/cache/ssl"

echo "--- Installing package from builder" 
hab pkg install core/redis --channel stable 
