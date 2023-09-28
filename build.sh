#!/bin/sh

set -o nounset # (set -u) No unset variables
set -o errexit # (set -e) Exit if any statement returns non-true value

NAME="file_upload"

docker build . -t "$NAME"

docker image save -o "${NAME}.tar" "$NAME"

docker run -p 8000:8000 -v data:/opt/file_upload/data -it --rm --name "$NAME" "$NAME"
#docker run --entrypoint 'sh' -it --rm --name "$NAME" "$NAME"
