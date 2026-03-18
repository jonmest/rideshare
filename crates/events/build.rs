use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    config.compile_well_known_types();

    let protos = &[
        "../../infra/redpanda/schemas/event-metadata.proto",
        "../../infra/redpanda/schemas/ride-request.proto",
        "../../infra/redpanda/schemas/driver-availability.proto",
        "../../infra/redpanda/schemas/driver-location-update.proto",
        "../../infra/redpanda/schemas/ride-assignment.proto",
        "../../infra/redpanda/schemas/ride-offer.proto",
        "../../infra/redpanda/schemas/ride-offer-response.proto",
        "../../infra/redpanda/schemas/trip-event.proto",
    ];

    let includes = &[
        "../../infra/redpanda/schemas",
    ];

    for proto in protos {
        println!("cargo:rerun-if-changed={proto}");
    }
    println!("cargo:rerun-if-changed=../../infra/redpanda/schemas/google/protobuf/timestamp.proto");

    config.compile_protos(protos, includes)?;

    Ok(())
}