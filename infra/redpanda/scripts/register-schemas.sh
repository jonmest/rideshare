#!/usr/bin/env bash
set -euo pipefail

REGISTRY_HOST="${REGISTRY_HOST:-localhost:18081}"
SCHEMA_DIR="${SCHEMA_DIR:-schemas}"

if ! command -v rpk >/dev/null 2>&1; then
  echo "rpk is not installed or not in PATH"
  exit 1
fi

if [ ! -d "$SCHEMA_DIR" ]; then
  echo "Schema directory not found: $SCHEMA_DIR"
  exit 1
fi

create_schema() {
  local subject="$1"
  local file="$2"
  shift 2

  echo "Registering $file as subject $subject"
  rpk registry schema create "$subject" \
    --schema "$file" \
    "$@" \
    -X registry.hosts="$REGISTRY_HOST"
}

echo "Registering shared dependency schemas..."

create_schema \
  "event-metadata-value" \
  "$SCHEMA_DIR/event-metadata.proto" || true

create_schema \
  "google-protobuf-timestamp" \
  "$SCHEMA_DIR/google/protobuf/timestamp.proto" || true

echo "Registering event schemas..."

create_schema \
  "driver-availability-value" \
  "$SCHEMA_DIR/driver-availability.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "driver-location-update-value" \
  "$SCHEMA_DIR/driver-location-update.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "ride-assignment-value" \
  "$SCHEMA_DIR/ride-assignment.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "ride-request-value" \
  "$SCHEMA_DIR/ride-request.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "ride-offer-value" \
  "$SCHEMA_DIR/ride-offer.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "ride-offer-response-value" \
  "$SCHEMA_DIR/ride-offer-response.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

create_schema \
  "trip-event-value" \
  "$SCHEMA_DIR/trip-event.proto" \
  --references "event-metadata.proto:event-metadata-value:1,google/protobuf/timestamp.proto:google-protobuf-timestamp:1"

echo "Done."