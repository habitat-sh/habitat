set -eu
BLDR_REPO=${BLDR_REPO:-"http://52.11.158.96:32768"}
if [ -n "${DEBUG:-}" ]; then set -x; fi

pkg_file="$1"
pkg_ident="$(gpg --decrypt $pkg_file 2>/dev/null \
  | tar xOf - --wildcards --no-anchored 'IDENT')"

set -x
${WGET:-wget} --post-file=$pkg_file -O- $BLDR_REPO/pkgs/$pkg_ident
