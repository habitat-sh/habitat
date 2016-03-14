set -eu
BLDR_REPO=${BLDR_REPO:-"http://52.37.151.35:9632"}
if [ -n "${DEBUG:-}" ]; then set -x; fi

pkg_file="$1"
pkg_ident="$(gpg --decrypt $pkg_file 2>/dev/null \
  | tar xOf - --wildcards --no-anchored 'IDENT')"
pkg_sha="$(openssl dgst -sha256 $pkg_file | awk '{ print $2 }')"

set -x
${WGET:-wget} --post-file=$pkg_file -O- $BLDR_REPO/pkgs/$pkg_ident?checksum=$pkg_sha
