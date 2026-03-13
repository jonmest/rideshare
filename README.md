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