use std::time::SystemTime;

use kafka::consumer::KafkaConsumer;
use redis::Commands;
use ride_events::{DriverLocationUpdate, TopicName};
use h3o::{CellIndex, LatLng, Resolution};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let pool = r2d2::Pool::builder().build(client).unwrap();
    let mut conn = pool.get().unwrap();

    let consumer = KafkaConsumer::new("localhost:19092", "driver-location-debugger")?;
    consumer.subscribe(&[DriverLocationUpdate::TOPIC.as_str()])?;

    loop {
        let evt = consumer.recv::<DriverLocationUpdate>().await?;
        let driver_key = format!("driver:{}", evt.event.driver_id);
        let old_cell: Option<CellIndex> = conn
            .get::<_, Option<u64>>(&driver_key)?
            .map(CellIndex::try_from)
            .transpose()?;

        let coord = LatLng::new(evt.event.lat, evt.event.lon).expect("valid coord");
        let cell = coord.to_cell(Resolution::Nine);

        let new_cell_key = format!("cell:{}", cell.to_string());

        println!(
            "topic={} partition={} offset={} key={:?} driver_id={} cell={cell}",
            evt.topic,
            evt.partition,
            evt.offset,
            evt.key,
            evt.event.driver_id,
        );

        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let stale_cutoff = now - 30;

        let mut pipe = redis::pipe();
        pipe.atomic();

        if let Some(old) = old_cell {
            let old_cell_key = format!("cell:{}", old);
            pipe.zrem(&old_cell_key, &evt.event.driver_id);
        }

        pipe.zadd(&new_cell_key, &evt.event.driver_id, now)
            .set_ex(&driver_key, cell.to_string(), 30)
            .zrembyscore(&new_cell_key, 0, stale_cutoff);

        pipe.query::<()>(&mut *conn)?;

        consumer.commit_message(&evt)?;
    }



}
