#!/bin/bash

# Create a tarball of all the Habitat artifacts needed to run the
# Habitat Supervisor on a system and upload it to S3. This includes
# *all* dependencies. The goal is to have everything needed to run the
# Supervisor *without* needing to talk to a running Builder.
#
# Because you have to bootstrap yourself from *somewhere* :)
#
# You must run this as root, because `hab` is going to be installing
# packages. Since it also uploads to S3, you'll probably want to run
# it with `sudo -E` if you've got your AWS creds in your environment,
# too.
#
# This generates a tar file (not tar.gz!) that has the following
# internal structure:
#
# |-- ARCHIVE_ROOT
# |   |-- artifacts
# |   |   `-- all the hart files
# |   |-- bin
# |   |   `-- hab
# |   `-- keys
# |       `-- all the origin keys
#
# Note that this script is *not* intended to be run by Terraform! It
# is closely related to the other scripts in this directory that *do*
# get run by Terraform, though, so it makes sense to keep them
# together.

set -euo pipefail

self=${0}
log() {
  >&2 echo "${self}: $1"
}

########################################################################
# Command Line Arguments

# Pass the version of the Supervisor you want to be using.
# TODO: Alternatively, just dispense with versions altogether and just
# get the latest stable?
hab_version=${1}
# TODO: Validate version?
log "Version ${hab_version}"

########################################################################
# Preliminaries, Helpers, Constants

find_if_exists() {
  command -v ${1} || { log "Required utility '${1}' cannot be found!  Aborting."; exit 1; }
}

# These are the key utilities this script uses. If any are not present
# on the machine, the script will exit.
hab=$(find_if_exists hab)
hab pkg install core/aws-cli core/coreutils core/gawk core/tar
hab pkg binlink core/aws-cli aws
hab pkg binlink core/gawk awk
# hab pkg binlink core/tar tar
hab pkg binlink core/coreutils sha256sum
hab pkg binlink core/coreutils sort

# The packages needed to run a Habitat Supervisor. These will be
# installed on all machines.
#
# hab-launcher is versioned differently than the other packages. It is
# also changed and released relatively infrequently. We can just ask
# the depot for the latest stable version of it.
sup_packages=(core/hab-launcher
              core/hab/${hab_version}
              core/hab-sup/${hab_version}
              core/hab-butterfly/${hab_version})

# If the HAB_BLDR_URL environment variable is set, we'll use that
# when downloading packages. Otherwise, we'll just default to the
# production depot.
if [[ ! -z "${HAB_BLDR_URL:-}" ]]
then
    log "Using HAB_BLDR_URL from environment: ${HAB_BLDR_URL}"
    depot_flag="HAB_BLDR_URL=${HAB_BLDR_URL}"
else
    log "No HAB_BLDR_URL detected; using the default"
    depot_flag=
fi

# All packages that compose the Builder service. Not all need
# to be installed on the same machine, but all need to be present in
# our bundle.
builder_packages=(core/builder-api
                  core/builder-api-proxy
                  core/builder-admin
                  core/builder-admin-proxy
                  core/builder-datastore
                  core/builder-jobsrv
                  core/builder-originsrv
                  core/builder-router
                  core/builder-scheduler
                  core/builder-sessionsrv
                  core/builder-worker)

# Helper packages. Not all need to to be installed on the same machine,
# but all need to be present in our bundle.
helper_packages=(core/sumologic)

# This is where we ultimately put all the things in S3.
s3_bucket="habitat-builder-bootstrap"

# This is the name by which we can refer to the bundle we're making
# right now. Note that other bundles can be made that contain the
# exact same packages.
this_bootstrap_bundle=hab_builder_bootstrap_$(date +%Y%m%d%H%M%S)

########################################################################
# Download all files locally

# Because Habitat may have already run on this system, we'll want to
# make sure we start in a pristine environment. That way, we can just
# blindly copy everything in ${sandbox_dir}/hab/cache/artifacts, confident
# that those artifacts are everything we need, and no more.
sandbox_dir=${this_bootstrap_bundle}
mkdir ${sandbox_dir}
log "Using ${sandbox_dir} as the Habitat root directory"

for package in "${sup_packages[@]}" "${builder_packages[@]}" "${helper_packages[@]}"
do
  env FS_ROOT=${sandbox_dir} ${depot_flag} ${hab} pkg install --channel=stable ${package} >&2
done

########################################################################
# Package everything up

artifact_dir=${sandbox_dir}/hab/cache/artifacts
log "Creating TAR for all artifacts"

sup_artifact=$(echo ${artifact_dir}/core-hab-sup-*)
archive_name=${this_bootstrap_bundle}.tar
log "Generating archive: ${archive_name}"

tar --create \
       --verbose \
       --file=${archive_name} \
       --directory=${sandbox_dir}/hab/cache \
       artifacts >&2

# We'll need a hab binary to bootstrap ourselves; let's take the one
# we just downloaded, shall we?
hab_pkg_dir=$(echo ${sandbox_dir}/hab/pkgs/core/hab/${hab_version}/*)
tar --append \
       --verbose \
       --file=${archive_name} \
       --directory=${hab_pkg_dir} \
       bin >&2

# We're also going to need the public origin key(s)!
tar --append \
       --verbose \
       --file=${archive_name} \
       --directory=${sandbox_dir}/hab/cache \
       keys >&2

########################################################################
# Upload to S3

checksum=$(sha256sum ${archive_name} | awk '{print $1}')

# Encapsulate the fact that we want our uploaded files to be publicly
# accessible.
s3_cp() {
  aws s3 cp --acl=public-read ${1} ${2} >&2
}

s3_cp ${archive_name} s3://${s3_bucket}

manifest_file=${this_bootstrap_bundle}_manifest.txt
echo ${archive_name} > ${manifest_file}
echo ${checksum} >> ${manifest_file}
echo >> ${manifest_file}
tar --list --file ${archive_name} | sort >> ${manifest_file}

s3_cp ${manifest_file} s3://${s3_bucket}
s3_cp s3://${s3_bucket}/${manifest_file} s3://${s3_bucket}/LATEST

rm -rdf $sandbox_dir
rm -Rdf $archive_name
