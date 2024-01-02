use crate::domain::domain_object::budget::Budget;
use crate::domain::domain_object::fishing_point::PointName;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::name::Name;
use crate::domain::domain_object::prefecture::Prefecture;
use crate::domain::domain_object::municipality::Municipality;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;

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

