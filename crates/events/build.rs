use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    config.compile_well_known_types();

    let protos = &[
        "../../infra/schemas/event-metadata.proto",
        "../../infra/schemas/ride-request.proto",
        "../../infra/schemas/driver-availability.proto",
        "../../infra/schemas/driver-location-update.proto",
        "../../infra/schemas/ride-assignment.proto",
        "../../infra/schemas/trip-event.proto",
    ];

    let includes = &[
        "../../infra/schemas",
    ];

    for proto in protos {
        println!("cargo:rerun-if-changed={proto}");
    }
    println!("cargo:rerun-if-changed=../../infra/schemas/google/protobuf/timestamp.proto");

    config.compile_protos(protos, includes)?;

    Ok(())
}