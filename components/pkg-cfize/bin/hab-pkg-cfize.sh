#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-cfize [PKG] [MAPPING]
# ```
#
# # Synopsis
#
# Create a Docker container from a set of Habitat packages.

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# ## Help

# **Internal** Prints help
print_help() {
  echo -- "$program $version

$author

Habitat Package CFize - Create a Cloud Foundry ready Docker image from a given package.

USAGE:
  $program <PKG> <MAPPING>
"
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    echo -- "ERROR: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m%s\033[0m\n" "$1"
        ;;
      *)
        printf -- "ERROR: %s\n" "$1"
        ;;
    esac
  fi
  exit "$2"
}

dockerize_tags() {
  local docker_output_file="$1"
  grep tagged "$docker_output_file" | awk '{ print $3}'
}

sh_shebang() {
  local docker_output_file="$1"
  echo "#!$(grep ENV "$docker_output_file" | \
    tr ":" "\n" | \
    grep busybox-static | \
    head -n1)/sh"
}

build_cf_image() {
  local hab_package=${1}
  local mapping=${2}

  tmp_dir="$(mktemp -t -d "${program}-XXXX")"

  dockerize_out="${tmp_dir}/dockerize-out"
  hab-pkg-export-docker "${hab_package}" | tee "${dockerize_out}"

  docker_tag_array=$(dockerize_tags "${dockerize_out}")
  cf_docker_tag_array=("${docker_tag_array[@]/:/:cf-}")

  DOCKER_CONTEXT=${tmp_dir}/docker
  mkdir -p "${DOCKER_CONTEXT}"
  render_helpers > "${DOCKER_CONTEXT}"/helpers.sh

  cat <<EOT > "$DOCKER_CONTEXT"/cf-init.sh
$(sh_shebang "${dockerize_out}")
source /helpers.sh
( echo "cat <<EOF >~/user.toml";
  cat /config.toml;
  echo "EOF";
) >~/render.sh
. ~/render.sh
mv ~/user.toml /hab/svc/$(basename "${hab_package}")/user.toml
exec /init.sh "\$@"
EOT
  chmod +x "$DOCKER_CONTEXT"/cf-init.sh

  cat "${mapping}" > "${DOCKER_CONTEXT}"/config.toml
  cat <<EOT > "$DOCKER_CONTEXT"/Dockerfile
FROM ${docker_tag_array[0]}
RUN hab pkg install core/jq-static
ADD cf-init.sh /
ADD helpers.sh /
ADD config.toml /
ENTRYPOINT ["/cf-init.sh"]
CMD ["start", "$1"]
EOT

  docker build --force-rm --no-cache "${cf_docker_tag_array[@]/#/-t }" "${DOCKER_CONTEXT}"
  rm -rf "${tmp_dir}"
}

render_helpers() {
  cat <<EOT
#!/bin/bash

_jq=\$(find /hab/pkgs/core/jq-static -name jq)

port() {
  echo \${PORT}
}

service() {
  local service_name=\${1:?Helper method 'service' requires a service name as argument}
  local path=\$2

  echo \${VCAP_SERVICES} | \${_jq} --arg name "\${service_name}" -r 'to_entries[].value[] | select(.name == \$name)' | \${_jq} -r "\$path"
}

services() {
  if [[ \$1 != "" ]]; then
    echo \${VCAP_SERVICES} | \${_jq} -r "\$1"
  else
    echo \${VCAP_SERVICES}
  fi
}

application() {
  if [[ \$1 != "" ]]; then
    echo \${VCAP_APPLICATION} | \${_jq} -r "\$1"
  else
    echo \${VCAP_APPLICATION}
  fi
}
EOT
}

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=hab-pkg-export-cf

if [ "$#" -eq 0 ]; then
  print_help
  exit_with "You must specify 1 Habitat package to CFize." 1
elif [ "$1" == "--help" ]; then
  print_help
elif [ "$#" -ne 2 ]; then
  print_help
  exit_with "You must provide a mapping file." 1
else
  build_cf_image "$@"
fi
