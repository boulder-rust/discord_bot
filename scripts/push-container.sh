#!/usr/bin/env bash

set -euox pipefail

CREDENTIALS="$1"; shift
REGISTRY_URL="$1"; shift
IMAGE_NAME="$1";

nix develop -c skopeo copy \
    --insecure-policy \
    --dest-creds "$CREDENTIALS" \
    "docker-archive://$(nix build .#container --no-link --print-out-paths --no-warn-dirty)" \
    "docker://$REGISTRY_URL/$IMAGE_NAME"
