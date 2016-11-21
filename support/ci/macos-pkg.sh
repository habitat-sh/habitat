#!/bin/bash

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

src_root=$(dirname $0)/../../

if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
  sudo -E $src_root/components/hab/mac/mac-build.sh $src_root/components/hab/mac
  mkdir -p $src_root/out/hab-x86_64-darwin
  source $src_root/results/last_build.env
  cp /hab/pkgs/$pkg_ident/bin/hab $src_root/out/hab-x86_64-darwin/hab
  zip -9 -r $src_root/out/hab-x86_64-darwin.zip $src_root/out/hab-x86_64-darwin

cat <<- EOF > $src_root/out/hab-bintray.json
{
  "package": {
    "name": "hab-x86_64-darwin",
    "repo": "unstable",
    "subject": "habitat",
    "desc": "",
    "website_url": "https://www.habitat.sh",
    "issue_tracker_url": "https://github.com/habitat-sh/habitat/issues",
    "vcs_url": "https://github.com/habitat-sh/habitat",
    "github_use_tag_release_notes": true,
    "github_release_notes_file": "RELEASE.md",
    "licenses": ["Apache-2.0"],
    "labels": [],
    "public_download_numbers": false,
    "public_stats": false
  },
  "version": {
    "name": "${pkg_version}-${pkg_release}",
    "gpgSign": true
  },
  "files": [
    {
      "includePattern": "out/hab-x86_64-darwin.zip",
      "uploadPattern": "darwin/x86_64/hab-${pkg_version}-${pkg_release}-x86_64-darwin.zip"
    }
  ],
  "publish": true
}
EOF
fi
