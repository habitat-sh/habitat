#!/bin/bash

# Downloads the latest builder bootstrap tarball from S3 and installs
# the desired packages into /hab/cache/artifacts. All
# Supervisor-related packages are always installed; additional
# packages from the archive can be installed by specifying them as
# arguments.

set -euo pipefail

########################################################################
# Command Line Arguments
#
# Any argument given to this script will be interpreted as a builder
# package to install on the machine. All Supervisor-related packages
# are always installed, and depending on the role the machine will
# play, additional packages should also be installed.
#
# To set up a machine as a worker, for example, you would call the
# script with the argument "core/builder-worker" (You could even just
# pass "builder-worker"; both are recognized.)
#
# Multiple packages can be specified.
#
# Do *not* provide version or release information.
declare -a services_to_install='()'
if [ "$#" > 0 ]
then
    # Bash 4 adds the command to this array; let's slice it off
    services_to_install=( "${@:1}" )
fi

########################################################################
# Preliminaries, Helpers, Constants

self=${0}
log() {
  >&2 echo "${self}: $1"
}

find_if_exists() {
    command -v ${1} || { log "Required utility '${1}' cannot be found!  Aborting."; exit 1; }
}

# These are the key utilities this script uses. If any are not present
# on the machine, the script will exit.
awk=$(find_if_exists awk)
curl=$(find_if_exists curl)
shasum=$(find_if_exists shasum)
tar=$(find_if_exists tar)

# This is where we ultimately put all the things. All contents of the
# bucket will be publicly readable, so we can just use curl to grab them.
s3_root_url="https://s3-us-west-2.amazonaws.com/habitat-builder-bootstrap"

# We're always going to need all the packages for running the
# Supervisor.
sup_packages=(hab-launcher
              hab
              hab-sup
              hab-butterfly)

########################################################################
# Download bootstrap archive from S3

# Pull down the most recent tarball manifest file from S3. The name of
# the corresponding tarball is the first line of the file.
manifest_url=${s3_root_url}/LATEST
log "Downloading latest builder tarball manifest from ${manifest_url}"
${curl} --remote-name ${manifest_url} >&2
latest_archive=$(${awk} 'NR==1' LATEST)

# Now that we know the latest tarball, let's download it, too.
latest_package_url=${s3_root_url}/${latest_archive}
log "Downloading ${latest_archive} from ${latest_package_url}"
${curl} --remote-name ${s3_root_url}/${latest_archive} >&2

# Verify the tarball; the SHA256 checksum is the 2nd line of the
# manifest file.
checksum=$(${awk} 'NR==2' LATEST)
log "Verifying ${latest_archive} with checksum ${checksum}"
${shasum} --algorithm 256 --check <<< "${checksum}  ${latest_archive}" >&2

########################################################################
# Unpack the archive

tmpdir=hab_bootstrap_$(date +%s)
mkdir -p ${tmpdir}

${tar} --extract \
       --verbose \
       --file=${latest_archive} \
       --directory=${tmpdir}

# This is the hab binary from the bootstrap bundle. We'll use this to
# install everything.
hab_bootstrap_bin=${tmpdir}/bin/hab

########################################################################
# Install the desired packages
#
# Note that this only puts the packages into /hab/cache/artifacts; it
# does not run `hab svc load`. We'll want to do that later, to ensure
# that the Supervisor running in the proper environment (e.g., under
# systemd, and not this script).

# Install the key(s) first. These need to be in place before
# installing any packages; otherwise, hab will try to contact a depot
# to retrieve them to verify the packages.
log "Installing public origin keys"
mkdir -p /hab/cache/keys
cp ${tmpdir}/keys/* /hab/cache/keys

# When installing packages (even from a .hart file), we pull
# dependencies from Builder, but we pull them *through the artifact
# cache*. If we put all the hart files in the cache first, we should
# be able to install everything we need. There will be some extra
# artifacts, but that's a minor concern.
log "Populating artifact cache"
mkdir -p /hab/cache/artifacts
cp ${tmpdir}/artifacts/* /hab/cache/artifacts

for pkg in "${sup_packages[@]}" ${services_to_install[@]:-}
do
    pkg_name=${pkg##core/} # strip "core/" if it's there
    # Using a fake depot URL keeps us honest; this will fail loudly if
    # we need to go off the box to get *anything*
    HAB_BLDR_URL=http://not-a-real-depot.habitat.sh \
                 ${hab_bootstrap_bin} pkg install ${tmpdir}/artifacts/core-${pkg_name}-*.hart
done

# Now we ensure that the hab binary being used on the system is the
# one that we just installed.
#
# TODO fn: The updated binlink behavior is to skip targets that already exist
# so we want to use the `--force` flag. Unfortunetly, old versions of `hab`
# don't have this flag. For now, we'll run with the new flag and fall back to
# running the older behavior. This can be removed at a future date when we no
# lnger are worrying about Habitat versions 0.33.2 and older. (2017-09-29)
${hab_bootstrap_bin} pkg binlink core/hab hab --force \
  || ${hab_bootstrap_bin} pkg binlink core/hab hab
