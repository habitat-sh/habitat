#!/usr/bin/env bash
#
# Returns a list of all fully qualified package identifiers that exist in an
# origin's release channel. Prints the package identifiers to standard out, one
# entry per line.
#
set -eu
if [ -n "${DEBUG:-}" ]; then set -x; fi

main() {
  need_cmd curl
  need_cmd jq

  # Set the default Builder URL
  : ${HAB_BLDR_URL:=https://bldr.habitat.sh}
  # Origin name is required
  origin="${1:-}"
  if [[ -z "$origin" ]]; then
    echo "Usage: $(basename $0) <ORIGIN> <CHANNEL>" >&2
    exit 1
  fi
  shift
  # Channel name is required
  channel="${1:-}"
  if [[ -z "$channel" ]]; then
    echo "Usage: $(basename $0) <ORIGIN> <CHANNEL>" >&2
    exit 1
  fi
  # Calculate the channel URL from which to fetch
  url="$HAB_BLDR_URL/v1/depot/channels/$origin/$channel/pkgs"

  # Determine the number of results per page
  npp="$(curl -s "$url" | jq '.range_end - .range_start + 1')"

  # Determine the number of pages to fetch. It sort of sucks that jq's `ceil`
  # doesn't work...going to do this the hard way...
  num_pages=$(curl -s "$url" \
    | jq "if ((.total_count / $npp) - ((.total_count / $npp) | floor)) > 0 then
          ((.total_count / $npp) | floor) + 1
        else
          ((.total_count / $npp) | floor)
        end")

  # Fetch each page and output a fully qualified package identifier, one per
  # line
  for (( n=0; n<$num_pages; n+=1 )) do
    curl -s "${url}?range=$(( ${n}*${npp} ))" \
      | jq --raw-output '.data[] | "\(.origin)/\(.name)/\(.version)/\(.release)"'
  done
}

need_cmd() {
  if ! command -v "$1" > /dev/null 2>&1; then
    echo "Required command '$1' not found on PATH" >&2
    exit 127
  fi
}

main "$@" || exit 99
