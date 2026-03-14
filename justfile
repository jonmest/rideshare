api:
  cargo run -p api-service

post-ride:
  curl -s -X POST http://127.0.0.1:3000/rides \
    -H 'Content-Type: application/json' \
    -d '{"rider_id": "rider-1", "pickup_lat": 40.7128, "pickup_lon": -74.0060, "dropoff_lat": 40.7580, "dropoff_lon": -73.9855}' | jq .

update-driver-location driver_id="driver-1":
  curl -s -X POST http://127.0.0.1:3000/driver/{{driver_id}} \
    -H 'Content-Type: application/json' \
    -d '{"driver_id": "{{driver_id}}", "lat": 40.7128, "lon": -74.0060, "update_time": 1710000000}' | jq .
