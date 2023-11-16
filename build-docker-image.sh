#!/bin/sh

set -o nounset # (set -u) No unset variables
set -o errexit # (set -e) Exit if any statement returns non-true value

NAME="file_upload"

# Choose one of the following:
DOCKER_OR_PODMAN="docker"
#DOCKER_OR_PODMAN="podman"

$DOCKER_OR_PODMAN build . -t "$NAME"

$DOCKER_OR_PODMAN image save -o "${NAME}.tar" "$NAME"

#$DOCKER_OR_PODMAN run -p 8000:8000 -v data:/opt/file_upload/data -it --rm --name "$NAME" "$NAME"
#$DOCKER_OR_PODMAN run --entrypoint 'sh' -it --rm --name "$NAME" "$NAME"
