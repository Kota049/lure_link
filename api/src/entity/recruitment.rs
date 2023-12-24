use crate::domain::budget::Budget;
use crate::domain::id::ID;
use crate::domain::municipality::Municipality;
use crate::domain::nick_name::NickName;
use crate::domain::participant_count::ParticipantCount;
use crate::domain::point_name::PointName;
use crate::domain::prefecture::Prefecture;
use crate::domain::start_time::StartDate;
use crate::domain::Domain;
use serde_json::{json, Value};

pub mod primitive;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Recruitment {
    pub id: ID,
    pub organizer_nick_name: NickName,
    pub start_date: StartDate,
    pub rendezvous_prefecture: Prefecture,
    pub rendezvous_municipality: Municipality,
    pub rendezvous_point: PointName,
    pub destination_prefecture: Prefecture,
    pub destination_municipality: Municipality,
    pub destination_point: PointName,
    pub budget: Budget,
    pub participant_count: ParticipantCount,
}

impl Recruitment {
    pub fn to_value(self) -> Value {
        json!({
            "id":self.id.to_string(),
            "organizer_nick_name":self.organizer_nick_name.to_string(),
            "start_date":self.start_date.to_string(),
            "rendezvous_prefecture":self.rendezvous_prefecture.to_string(),
            "rendezvous_municipality":self.rendezvous_municipality.to_string(),
            "rendezvous_point":self.rendezvous_point.to_string(),
            "destination_prefecture":self.destination_prefecture.to_string(),
            "destination_municipality":self.destination_municipality.to_string(),
            "destination_point":self.destination_point.to_string(),
            "budget":self.budget.to_string(),
            "participant_count":self.participant_count.to_string()
        })
    }
}
