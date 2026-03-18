pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}

pub mod rideshare {
    pub mod topic {
        pub enum Topic {
            RideRequests,
            DriverLocationUpdates,
            DriverAvailability,
            RideOffers,
            RideOfferResponses,
            RideAssignments,
            TripEvents,
        }

        impl Topic {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    Topic::RideRequests => "ride-requests",
                    Topic::DriverLocationUpdates => "driver-location-updates",
                    Topic::DriverAvailability => "driver-availability",
                    Topic::RideOffers => "ride-offers",
                    Topic::RideOfferResponses => "ride-offer-responses",
                    Topic::RideAssignments => "ride-assignments",
                    Topic::TripEvents => "trip-events",
                }
            }
        }
    }
    pub mod events {
        include!(concat!(env!("OUT_DIR"), "/rideshare.events.rs"));
    }
}

pub use rideshare::events::*;


pub trait TopicName {
    const TOPIC: rideshare::topic::Topic;
}

pub trait KafkaKey {
    fn kafka_key(&self) -> &str;
}

impl TopicName for RideRequest {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::RideRequests;
}

impl KafkaKey for RideRequest {
    fn kafka_key(&self) -> &str {
        &self.ride_id
    }
}

impl TopicName for DriverLocationUpdate {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::DriverLocationUpdates;
}

impl KafkaKey for DriverLocationUpdate {
    fn kafka_key(&self) -> &str {
        &self.driver_id
    }
}

impl DriverStatus {
    pub fn to_int(&self) -> i32 {
        match &self {
            Self::Unspecified => 0,
            Self::Available => 1,
            Self::Unavailable => 2
        }
    }
}

impl TopicName for DriverAvailability {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::DriverAvailability;
}

impl KafkaKey for DriverAvailability {
    fn kafka_key(&self) -> &str {
        &self.driver_id
    }
}

impl TopicName for RideOffer {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::RideOffers;
}

impl KafkaKey for RideOffer {
    fn kafka_key(&self) -> &str {
        &self.driver_id
    }
}

impl TopicName for RideOfferResponse {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::RideOfferResponses;
}

impl KafkaKey for RideOfferResponse {
    fn kafka_key(&self) -> &str {
        &self.ride_id
    }
}

impl TopicName for RideAssignment {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::RideAssignments;
}

impl KafkaKey for RideAssignment {
    fn kafka_key(&self) -> &str {
        &self.ride_id
    }
}

impl TopicName for TripEvent {
    const TOPIC : rideshare::topic::Topic = rideshare::topic::Topic::TripEvents;
}

impl KafkaKey for TripEvent {
    fn kafka_key(&self) -> &str {
        &self.ride_id
    }
}

