#!/usr/bin/env bash
# Active les crochets git versionnés dans .githooks/ (une fois par clone).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."
git config core.hooksPath .githooks
echo "Crochets git activés (.githooks/)."
