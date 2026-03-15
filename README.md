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


---

The api-service now has endpoints for posting driver location and availability updates, which then posts corresponding events consumed by the driver service. 

Next order of business:
- Consume events in driver service
- Maintain a Redis + Postgres setup
  - Compute H3 bucket for event location, update e.g. cell:<h3_cell> by adding the driver and setting driver:<driver_id>  with their new H3 cell.

The matcher service will then take a ride request, compute their H3 cell, and start finding N driver candidates by checking in the same cell, then expanding in a ring outwards. Standard radius will be e.g. 5.


If no driver is found within radius 5, expand up to radius 10 and pick first one.

Then compute Haversine distance from each driver to the rider, rank them by shortest distance. Select top K.

