#!/bin/sh
#
# Copyright (c) 2010-2016 Chef Software, Inc. and/or applicable contributors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# Fails on unset variables & whenever a command returns a non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Borrowed from Omnitruck.
# helpers.sh
############
# This section has some helper functions to make life easier.
#
# Outputs:
# $tmp_dir: secure-ish temp directory that can be used during installation.
############

# Check whether a command exists - returns 0 if it does, 1 if it does not
exists() {
  if command -v $1 >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# Output the instructions to report bug about this script
report_bug() {
  echo ""
  echo "Please file a Bug Report at https://github.com/habitat-sh/habitat/issues/new"
  echo ""
  echo "Please include as many details about the problem as possible i.e., how to reproduce"
  echo "the problem (if possible), type of the Operating System and its version, etc.,"
  echo "and any other relevant details that might help us with troubleshooting."
  echo ""
}

checksum_mismatch() {
  echo "Package checksum mismatch!"
  report_bug
  exit 1
}

unable_to_retrieve_package() {
  echo "Unable to retrieve a valid package!"
  echo ""
  echo "We attempt to use several different programs to download the package from the"
  echo "downloads site. Usually, this means we could not find the 'wget' or 'curl'"
  echo "program on your system. We need one of these installed before we can proceed."
  report_bug
  if test "x$download_url" != "x"; then
    echo "Download URL: $download_url"
  fi
  if test "x$stderr_results" != "x"; then
    echo "\nDEBUG OUTPUT FOLLOWS:\n$stderr_results"
  fi
  exit 1
}

http_404_error() {
  echo ""
  echo "Either this means:"
  echo "   - We do not support $platform"
  echo "   - We do not have an artifact for $version"
  echo ""
  # deliberately do not call report_bug to suppress bug report noise.
  if test "x$download_url" != "x"; then
    echo "Download URL: $download_url"
  fi
  if test "x$stderr_results" != "x"; then
    echo "\nDEBUG OUTPUT FOLLOWS:\n$stderr_results"
  fi
  exit 1
}

capture_tmp_stderr() {
  # spool up /tmp/stderr from all the commands we called
  if test -f "$tmp_dir/stderr"; then
    output=`cat $tmp_dir/stderr`
    stderr_results="${stderr_results}\nSTDERR from $1:\n\n$output\n"
    rm $tmp_dir/stderr
  fi
}

# do_wget URL FILENAME
do_wget() {
  echo "trying wget..."
  wget -O "$2" "$1" 2>$tmp_dir/stderr
  rc=$?
  # check for 404
  grep "ERROR 404" $tmp_dir/stderr 2>&1 >/dev/null
  if test $? -eq 0; then
    echo "ERROR 404"
    http_404_error
  fi

  # check for bad return status or empty output
  if test $rc -ne 0 || test ! -s "$2"; then
    capture_tmp_stderr "wget"
    return 1
  fi

  return 0
}

# do_curl URL FILENAME
do_curl() {
  echo "trying curl..."
  curl --retry 5 -sL -D $tmp_dir/stderr "$1" > "$2"
  rc=$?
  # check for 404
  grep "404 Not Found" $tmp_dir/stderr 2>&1 >/dev/null
  if test $? -eq 0; then
    echo "ERROR 404"
    http_404_error
  fi

  # check for bad return status or empty output
  if test $rc -ne 0 || test ! -s "$2"; then
    capture_tmp_stderr "curl"
    return 1
  fi

  return 0
}

# do_fetch URL FILENAME
do_fetch() {
  echo "trying fetch..."
  fetch -o "$2" "$1" 2>$tmp_dir/stderr
  # check for bad return status
  test $? -ne 0 && return 1
  return 0
}

# do_perl URL FILENAME
do_perl() {
  echo "trying perl..."
  perl -e 'use LWP::Simple; getprint($ARGV[0]);' "$1" > "$2" 2>$tmp_dir/stderr
  rc=$?
  # check for 404
  grep "404 Not Found" $tmp_dir/stderr 2>&1 >/dev/null
  if test $? -eq 0; then
    echo "ERROR 404"
    http_404_error
  fi

  # check for bad return status or empty output
  if test $rc -ne 0 || test ! -s "$2"; then
    capture_tmp_stderr "perl"
    return 1
  fi

  return 0
}

# do_python URL FILENAME
do_python() {
  echo "trying python..."
  python -c "import sys,urllib2 ; sys.stdout.write(urllib2.urlopen(sys.argv[1]).read())" "$1" > "$2" 2>$tmp_dir/stderr
  rc=$?
  # check for 404
  grep "HTTP Error 404" $tmp_dir/stderr 2>&1 >/dev/null
  if test $? -eq 0; then
    echo "ERROR 404"
    http_404_error
  fi

  # check for bad return status or empty output
  if test $rc -ne 0 || test ! -s "$2"; then
    capture_tmp_stderr "python"
    return 1
  fi
  return 0
}

# do_download URL FILENAME
do_download() {
  echo "downloading $1"
  echo "  to file $2"

  url=`echo $1`
  if test "x$platform" = "xsolaris2"; then
    if test "x$platform_version" = "x5.9" -o "x$platform_version" = "x5.10"; then
      # solaris 9 lacks openssl, solaris 10 lacks recent enough credentials - your base O/S is completely insecure, please upgrade
      url=`echo $url | sed -e 's/https/http/'`
    fi
  fi

  # we try all of these until we get success.
  # perl, in particular may be present but LWP::Simple may not be installed

  if exists wget; then
    do_wget $url $2 && return 0
  fi

  if exists curl; then
    do_curl $url $2 && return 0
  fi

  if exists fetch; then
    do_fetch $url $2 && return 0
  fi

  if exists perl; then
    do_perl $url $2 && return 0
  fi

  if exists python; then
    do_python $url $2 && return 0
  fi

  unable_to_retrieve_package
}

case `uname` in
  "Darwin")
    platform="darwin"
    file_ext="zip"
    shasum_cmd="shasum -a 256"
    ;;
  "Linux")
    platform="linux"
    file_ext="tar.gz"
    shasum_cmd="sha256sum"
    ;;
  *)
    platform="unknown"
    file_ext="unknown"
    report_bug
    ;;
esac

do_extract() {
  case $file_ext in
    "tar.gz")
      zcat "$1" | tar x -C "$tmp_dir"
      archive_dir="$(echo $archive | sed 's/.tar.gz$//')"
    ;;
    "zip")
      unzip "$1" -d "$tmp_dir"
      archive_dir="$(echo $archive | sed 's/.zip$//')"
    ;;
  esac
}

do_install() {
  case $platform in
    "darwin")
      if test -d /usr/local/bin; then
        install -v $archive_dir/hab /usr/local/bin/hab
      else
        echo "This system does not have '/usr/local/bin'"
        echo "Create this directory and rerun this script, or"
        echo "copy '$archive_dir/hab' to a location in the PATH,"
        echo "PATH=$PATH"
        return 1
      fi
      ;;
    "linux")
      ident="core/hab"
      if [ ! -z "${version-}" ]; then ident="$ident/$version"; fi
      # Install hab release using the extracted version and add/update symlink
      "$archive_dir/hab" install "$ident"
      "$archive_dir/hab" pkg binlink "$ident" hab
      ;;
  esac
}

# Download location for the temporary files
tmp_dir="${TMPDIR:-/tmp}/hab"

# use stable channel by default
channel="stable"

# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts "c:v:" opt; do
  case $opt in
    c)
      channel=$OPTARG
      ;;
    v)
      version=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

if [ "$channel" = "unstable" ]; then
  export HAB_DEPOT_URL="${HAB_DEPOT_URL:-https://app.acceptance.habitat.sh/v1/depot}"
fi

# Add a trap to clean up any interrupted file downloads
trap 'rm -rf $tmp_dir; exit $?' INT TERM EXIT
rm -rf "$tmp_dir"
(umask 077 && mkdir -p $tmp_dir) || exit 1

download_url="https://api.bintray.com/content/habitat/$channel/$platform/x86_64/hab-%24latest-x86_64-$platform.$file_ext"
bt_query="?bt_package=hab-x86_64-$platform"

do_download "${download_url}${bt_query}" "${tmp_dir}/hab-latest.${file_ext}"
do_download "${download_url}.sha256sum${bt_query}" "${tmp_dir}/hab-latest.${file_ext}.sha256sum"

cd "$tmp_dir"
archive="${tmp_dir}/$(cat hab-latest.${file_ext}.sha256sum | cut -d ' ' -f 3)"
mv -v "${tmp_dir}/hab-latest.${file_ext}" "${archive}"
sha_file="${archive}.sha256sum"
mv -v "${tmp_dir}/hab-latest.${file_ext}.sha256sum" "${archive}.sha256sum"

# If gnupg is available, verify that the shasum digest is properly signed
if command -v gpg >/dev/null; then
  sha_sig_url="${download_url}.sha256sum.asc$bt_query"
  sha_sig_file="${archive}.sha256sum.asc"
  key_url="https://bintray.com/user/downloadSubjectPublicKey?username=habitat"
  key_file="$tmp_dir/habitat.asc"
  do_download "$sha_sig_url" "$sha_sig_file"
  do_download "$key_url" "$key_file"
  gpg --no-permission-warning --dearmor "$key_file"
  gpg --no-permission-warning --keyring "${key_file}.gpg" --verify "$sha_sig_file"
fi

# Verify the provided shasum digest matches the downloaded archive
$shasum_cmd -c "$sha_file"

do_extract $archive
do_install
