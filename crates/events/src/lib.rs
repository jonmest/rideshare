pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}

pub mod rideshare {
    pub mod events {
        include!(concat!(env!("OUT_DIR"), "/rideshare.events.rs"));
    }
}

pub use rideshare::events::*;