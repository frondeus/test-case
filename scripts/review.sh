#!/usr/bin/env bash

# This script is used to review acceptance tests made to the project

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR="${SCRIPT_DIR}/.."

for file in $(find "${REPO_DIR}/tests/snapshots" -type f -name "*.actual"); do
  echo "Reviewing ${file}"
  diff --color "${file%.actual}.snap" "$file"
  while IFS= read -n 1 -r -s -p 'Enter "a" to accept the changes, "d" to discard them, or "q" to quit' key; do
    case $key in
      [aA])
        echo "Accepting changes"
        mv "$file" "${file%.actual}.snap"
        break
        ;;
      [dD])
        echo "Discarding changes"
        rm "$file"
        break
        ;;
      [qQ])
        echo "Quitting"
        exit 0
      ;;
    esac
  done
  echo "-----"
done
