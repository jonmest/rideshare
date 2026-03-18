use h3o::{CellIndex, LatLng};
use kafka::consumer::KafkaConsumer;
use redis::Commands;
use ride_events::{RideRequest, TopicName};

const MIN_CANDIDATES: usize = 5;
const MAX_RING: u32 = 3;

fn find_candidates(
    conn: &mut r2d2::PooledConnection<redis::Client>,
    origin: CellIndex,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut drivers: Vec<String> = Vec::new();

    // Check the origin cell first (ring 0)
    let cell_key = format!("cell:{origin}");
    drivers.extend(conn.zrange::<_, Vec<String>>(&cell_key, 0, -1)?);

    // Expand ring by ring until we have enough candidates or hit the max
    for k in 1..=MAX_RING {
        if drivers.len() >= MIN_CANDIDATES {
            break;
        }
        let ring = origin.grid_ring_fast(k).flatten();
        for neighbor in ring {
            let key = format!("cell:{neighbor}");
            drivers.extend(conn.zrange::<_, Vec<String>>(&key, 0, -1)?);
        }
    }

    Ok(drivers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let pool = r2d2::Pool::builder().build(client).unwrap();
    let mut conn = pool.get().unwrap();

    let consumer = KafkaConsumer::new("localhost:19092", "ride-request-debugger")?;
    consumer.subscribe(&[RideRequest::TOPIC.as_str()])?;

    loop {
        let evt = consumer.recv::<RideRequest>().await?;
        match evt.event.pickup {
            Some(pickup) => {
                let coord = LatLng::new(pickup.lat, pickup.lon).expect("valid coord");
                let cell = coord.to_cell(h3o::Resolution::Nine);
                let candidates = find_candidates(&mut conn, cell)?;

                if candidates.len() >= MIN_CANDIDATES {
                    let top_n = &candidates[..MIN_CANDIDATES];
                    // TODO: rank by distance, emit RideOffer to best candidate
                    println!("ride_id={} found {} candidates, top {MIN_CANDIDATES}: {:?}",
                        evt.event.ride_id, candidates.len(), top_n);
                } else if !candidates.is_empty() {
                    // Fewer than N but still some — use what we have
                    println!("ride_id={} found only {} candidates: {:?}",
                        evt.event.ride_id, candidates.len(), candidates);
                } else {
                    println!("ride_id={} no candidates found within {MAX_RING} rings",
                        evt.event.ride_id);
                }
            },
            None => ()
        }

        println!(
            "topic={} partition={} offset={} key={:?} ride_id={}",
            evt.topic,
            evt.partition,
            evt.offset,
            evt.key,
            evt.event.ride_id
        );

        consumer.commit_message(&evt)?;
    }
}