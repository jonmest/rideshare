This will be a personal project for developing a rideshare like backend system in Rust.

## Actors
- Drivers
- Passengers
- The matcher (between drivers and passengers)
- The trip service

1. A driver requests a ride
2. Nearby available drivers exist
3. Matcher assigns driver
4. Driver accepts
5. Trip starts
6. Trip ends



---

## Services
api-service
publishes commands/events into Kafka

driver-service
tracks driver status and location
emits driver availability/location events

matching-service
consumes ride requests + driver availability
decides assignment

trip-service
owns trip lifecycle
consumes assignment/accept/start/complete events
materializes current trip state

## Kafka topics
ride-requests
driver-location-updates
driver-availability
ride-assignments
trip-events


| Topic                     | Key         | Producer                      | Consumer(s)                      | Why this key                          |
| ------------------------- | ----------- | ----------------------------- | -------------------------------- | ------------------------------------- |
| `ride-requests`           | `ride_id`   | api-service                   | matching-service, trip-service   | preserve per-ride ordering            |
| `driver-availability`     | `driver_id` | api-service / driver-service  | driver-service, matching-service | preserve per-driver status ordering   |
| `driver-location-updates` | `driver_id` | api-service / simulator       | driver-service, matching-service | preserve per-driver location ordering |
| `ride-assignments`        | `ride_id`   | matching-service              | trip-service                     | assignment belongs to ride lifecycle  |
| `trip-events`             | `ride_id`   | trip-service / driver adapter | trip-service, read model         | preserve per-ride lifecycle ordering  |
