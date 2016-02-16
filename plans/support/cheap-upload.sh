set -eu
if [ -n "${DEBUG:-}" ]; then set -x; fi

pkg_file="$1"
pkg_ident="$(gpg --decrypt $pkg_file 2>/dev/null \
  | tar xOf - --wildcards --no-anchored 'IDENT')"

set -x
${WGET:-wget} --post-file=$pkg_file -O- $BLDR_REPO/pkgs/$pkg_ident
