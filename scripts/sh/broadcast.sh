#!/bin/bash
# scripts/broadcast.sh - Fixed JSON with heredoc.
: "${MINIO_USER?Error: MINIO_USER not set}"
: "${MINIO_KEY?Error: MINIO_KEY not set}"
: "${BROADCAST_ENDPOINT?Error: BROADCAST_ENDPOINT not set}"
cat <<EOF | curl -X POST "$BROADCAST_ENDPOINT" -H "Content-Type: application/json" -d @-
{
  "creds": {
    "username": "$MINIO_USER",
    "apiKey": "$MINIO_KEY"
  },
  "action": "deploy"
}
EOF
echo "Broadcast OK"
