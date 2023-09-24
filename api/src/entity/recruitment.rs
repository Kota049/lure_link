use crate::domain::budget::Budget;
use crate::domain::id::ID;
use crate::domain::municipality::Municipality;
use crate::domain::nick_name::NickName;
use crate::domain::participant_count::ParticipantCount;
use crate::domain::point_name::PointName;
use crate::domain::prefecture::Prefecture;
use crate::domain::start_time::StartDate;
use serde_json::Value;

#[cfg(test)]
mod tests;

pub struct Recruitment {
    id: ID,
    organizer_nick_name: NickName,
    start_date: StartDate,
    rendezvous_prefecture: Prefecture,
    rendezvous_municipality: Municipality,
    rendezvous_point: PointName,
    destination_prefecture: Prefecture,
    destination_municipality: Municipality,
    destination_point: PointName,
    budget: Budget,
    participant_count: ParticipantCount,
}

impl Recruitment {
    pub fn to_value(self) -> Value {
        todo!()
    }
}
