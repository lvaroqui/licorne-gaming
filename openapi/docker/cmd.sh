#!/usr/bin/env bash

HASH_FILE="/opt/openapi/out/spec.json.md5"
SPEC_FILE="./spec.json"

touch "$HASH_FILE"
touch "$SPEC_FILE"

# Run client code generation when spec.json changes.
export JAVA_OPTS="-Dlog.level=warn"
inotifywait -m -e modify "$SPEC_FILE" |
    while read; do
        HASH=`cat $HASH_FILE`
        NEW_HASH=$(md5sum "$SPEC_FILE")
        if [ "$HASH" != "$NEW_HASH" ]; then
            if /usr/local/bin/docker-entrypoint.sh generate \
                -i "$SPEC_FILE" \
                -g rust \
                -o out \
                --minimal-update; then
                echo "$NEW_HASH" > "$HASH_FILE"
            fi
        fi
    done
