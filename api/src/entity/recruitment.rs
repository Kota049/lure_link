use crate::domain::domain_object::budget::Budget;
use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::fishing_point::PointName;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::municipality::Municipality;
use crate::domain::domain_object::name::Name;
use crate::domain::domain_object::prefecture::Prefecture;
use crate::entity::users::User;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Recruitment {
    pub id: Id,
    pub organizer_nick_name: Name,
    pub start_time: JaTimeStamp,
    pub apl_deadline: JaTimeStamp,
    pub rendezvous_prefecture: Prefecture,
    pub rendezvous_municipality: Municipality,
    pub rendezvous_point: PointName,
    pub destination_prefecture: Prefecture,
    pub destination_municipality: Municipality,
    pub destination_point: PointName,
    pub budget: Budget,
    pub participant_count: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CarPool {
    pub id: Id,
    pub organizer: User,
    pub start_time: JaTimeStamp,
    pub end_time: JaTimeStamp,
    pub apl_deadline: JaTimeStamp,
    pub departure: Point,
    pub destination: Point,
    pub budget: Budget,
    pub max_participant: i16,
    pub current_participant: i16,
    pub status: CarPoolStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point {
    pub prefecture: Prefecture,
    pub municipality: Municipality,
    pub point_name: PointName,
}
