#!/usr/bin/env bash

main() {
  # Fails on unset variables and whenever a command returns a non-zero exit
  # code, inside or outside of a pipeline
  set -euo pipefail
  # If the variable `$DEBUG` is set, then print the shell commands as we
  # execute.
  if [ -n "${DEBUG:-}" ]; then set -x; fi

  need_cmd basename
  program="$(basename "$0")"
  author="The Habitat Maintainers <humans@habitat.sh>"

  parse_cli_args "$@"

  setup
  lint_files
  exit 0
}

print_help() {
  need_cmd cat

  cat <<USAGE
$program

Authors: $author

Lints Rust source code files based on Git commits.

USAGE:
    $program [FLAGS] [OPTIONS]

FLAGS:
    -a, --all         Lints all source files, ignoring status of Git repository
    -c, --cached      Lints all currently staged files in Git repository
    -u, --unstaged    Lints all currently unstaged files in Git repository
    -h, --help        Prints help information

OPTIONS:
    -f, --files <FILE ..>   Lints all specified source files, relative to the
                            current directory
    -g, --git <REF>         Lints all source files in the Git reference

ENVIRONMENT VARIABLES:
    TRAVIS_COMMIT_RANGE     If set and with no other options or flags, Git
                            mode will be used with the git reference contained
                            in this environment variable
    TRAVIS_COMMIT           If set and with no other options or flags, Git
                            mode will be used with the git reference contained
                            in this environment variable
EXAMPLES:

    # Lint all files
    $program --all

    # Lint all staged Git files, ready to commit
    $program --cached

    # Lint all unstaged Git files, not yet staged for commit
    $program --unstaged

    # Lint specific files
    $program --files ./src/lib.rs ./src/main.rs

    # Lint files changed in the Git commit bbef2
    $program --git bbef2

    # Lint files changed in Git in the range abc123 to HEAD
    $program --git abc123..

    # Lint files in a Git changeset on the TravisCI platform (uses one of
    # two Travis-specific environment variables)
    $program

USAGE
}

parse_cli_args() {
  if [[ -z "${1:-}" ]]; then
    info "No explicit mode, attempting to auto detect..."

    need_cmd git
    need_cmd wc

    if [[ $(git diff --name-only | wc -l) -gt 0 ]]; then
      lint=unstaged
      info "Unstaged changes detected running in '$lint' lint mode"
    elif [[ $(git diff --name-only --cached | wc -l) -gt 0 ]]; then
      lint=cached
      info "Staged changes detected, running in '$lint' lint mode"
    else
      # Fix commit range in Travis, if set.
      # See: https://github.com/travis-ci/travis-ci/issues/4596
      if [[ -n "${TRAVIS_COMMIT_RANGE:-}" ]]; then
        TRAVIS_COMMIT_RANGE="${TRAVIS_COMMIT_RANGE/.../..}"
      fi
      lint=git
      git="${TRAVIS_COMMIT_RANGE:-${TRAVIS_COMMIT:-HEAD}}"
      info "Selecting files from Git using ref: '$git'"
    fi
  else
    case "$1" in
      -a|--all)
        lint=all
        shift
        if [[ -n "${1:-}" ]]; then
          warn "Cannot combine --all with other flags or files"
          print_help
          exit_with "Invalid usage" 1
        fi
        ;;
      -c|--cached)
        lint=cached
        shift
        if [[ -n "${1:-}" ]]; then
          warn "Cannot combine --cached with other flags or files"
          print_help
          exit_with "Invalid usage" 1
        fi
        ;;
      -f|--files)
        lint=files
        shift
        files="$*"
        if [[ -z "$files" ]]; then
          warn "--files option requires one or more file values"
          print_help
          exit_with "Invalid usage" 1
        fi
        ;;
      -g|--git)
        lint=git
        shift
        if [[ -z "${1:-}" ]]; then
          warn "--git option requires a Git ref value"
          print_help
          exit_with "Invalid usage" 1
        fi
        git="$1"
        shift
        if [[ -n "${1:-}" ]]; then
          warn "Cannot provide multiple --git values"
          print_help
          exit_with "Invalid usage" 1
        fi
        ;;
      -h|--help)
        print_help
        exit 0
        ;;
      -u|--unstaged)
        lint=unstaged
        shift
        if [[ -n "${1:-}" ]]; then
          warn "Cannot combine --staged with other flags or files"
          print_help
          exit_with "Invalid usage" 1
        fi
        ;;
    esac
  fi
}

setup() {
  local _tmp
  # Create a temporary work directory into which we can render files for
  # diff'ing. Note that the following conditions and invocation of `mktemp`
  # allows this to work on all appropriate Linux and macOS systems.
  if [[ -n "${TMPDIR:-}" ]]; then
    _tmp="${TMPDIR}"
  elif [[ -d /var/tmp ]]; then
    _tmp=/var/tmp
  else
    _tmp=/tmp
  fi
  workdir="$(mktemp -d -p "$_tmp" 2> /dev/null || mktemp -d "${_tmp}/lint.XXXX")"
  # shellcheck disable=2154
  trap 'code=$?; rm -rf $workdir; exit $code' INT TERM EXIT

  # Prepare a file to track files which failed linting
  failed="$workdir/failed.log"

  need_cmd rustfmt
  info "Running rustfmt version '$(rustfmt --version)'"
}

lint_files() {
  local _input_files_cmd _file

  case "$lint" in
    all)
      need_cmd find
      _input_files_cmd="find . -type f -name '*.rs'"
      info "Linting all files, selecting files via: '$_input_files_cmd'"
      ;;
    cached)
      need_cmd git
      _input_files_cmd="git diff --name-only --cached"
      info "Linting staged changes, selecting files via: '$_input_files_cmd'"
      ;;
    files)
      _input_files_cmd="echo '$files'"
      info "Linting specific files: $files"
      ;;
    git)
      need_cmd git
      _input_files_cmd="git diff-tree --no-commit-id --name-only -r $git"
      info "Linting files from Git via: '$_input_files_cmd'"
      ;;
    unstaged)
      need_cmd git
      _input_files_cmd="git diff --name-only"
      info "Linting Unstaged changes, selecting files via: '$_input_files_cmd'"
      ;;
    *)
      exit_with "Invalid lint type: $lint" 5
      ;;
  esac

  echo

  eval "$_input_files_cmd" | while read -r _file; do
    case "${_file##*.}" in
      rs)
        lint_file "$_file"
        ;;
    esac
  done

  if [[ -s "$failed" ]]; then
    echo
    warn "Summary: One or more files failed linting:"
    while read -r _file; do
      warn "  * $_file"
    done < "$failed"
    echo
    exit_with "File(s) failed linting" 10
  else
    echo
    info "Summary: All checked files passed their lints."
    echo
  fi
}

lint_file() {
  local _file="$1"
  local _rf_out _rf_exit _diff_out _diff_exit

  if [[ ! -e "$_file" ]]; then
    # Skip files which were deleted
    return 0
  fi
  if echo "$_file" | grep -q '/target/' > /dev/null; then
    # Skip files in a `target/` directory
    return 0
  fi
  if [[ "$(basename "$(dirname "$_file")")" == "generated" ]]; then
    # Skip files directly under a `generated/` directory
    return 0
  fi

  need_cmd dirname
  need_cmd rustfmt

  info "Running rustfmt on $_file"
  mkdir -p "$(dirname "$workdir/$_file")"

  set +e
  _rf_out="$(rustfmt < "$_file" > "$workdir/$_file")"
  _rf_exit="$?"
  set -e

  case $_rf_exit in
    0|3)
      # 0 is a clean exit and 3 signals that a line was too long to properly
      # parse the file. Either scenario is considered success.
      ;;
    *)
      # All other exit codes are errors
      warn "File $_file exited from rustfmt with $_rf_exit"
      warn "Error output:"
      echo "$_rf_out"
      echo "$_file" >> "$failed"
      return 0
      ;;
  esac

  set +e
  _diff_out="$(diff --color=always --unified "$_file" "$workdir/$_file" 2>&1)"
  _diff_exit="$?"
  set -e

  case $_diff_exit in
    0)
      # Diff between commited source and formatted source is empty, meaning
      # it's well formatted
      ;;
    1)
      # Exit of 1 means that there is a non-empty diff generated, so we will
      # report and track the file
      warn "File $_file generates a diff after running rustfmt"
      warn "Perhaps you forgot to run \`rustfmt' or \`cargo fmt'?"
      warn "Diff for $_file:"
      echo "$_diff_out"
      echo "$_file" >> "$failed"
      ;;
    *)
      # All other exit codes are errors, so we will report and track the file
      warn "Running diff on file $_file unexpectedly exited with $_diff_exit"
      warn "Error output:"
      echo "$_diff_out"
      echo "$_file" >> "$failed"
      ;;
  esac
}

need_cmd() {
  if ! command -v "$1" > /dev/null 2>&1; then
    warn "Required command '$1' not found on PATH"
    exit 127
  fi
}

info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- '   \033[1;32m%s: \033[1;37m%s\033[0m\n' "${program:-}" "$1"
      ;;
    *)
      printf -- '   %s: %s\n' "${program:-}" "$1"
      ;;
  esac
}

warn() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- '   \033[1;32m%s: \033[1;33mWARN \033[1;37m%s\033[0m\n' \
        "${program:-}" "$1" >&2
      ;;
    *)
      printf -- '   %s: WARN %s\n' "${program:-}" "$1" >&2
      ;;
  esac
}

exit_with() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- '\033[1;31mERROR: \033[1;37m%s\033[0m\n\n' "$1" >&2
      ;;
    *)
      printf -- 'ERROR: %s\n\n' "$1" >&2
      ;;
  esac
  exit "${2:-89}"
}

main "$@" || exit 99
