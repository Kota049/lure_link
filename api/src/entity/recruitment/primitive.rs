use crate::domain::budget::Budget;
use crate::domain::id::ID;
use crate::domain::municipality::Municipality;
use crate::domain::nick_name::NickName;
use crate::domain::participant_count::ParticipantCount;
use crate::domain::point_name::PointName;
use crate::domain::prefecture::Prefecture;
use crate::domain::start_time::StartDate;
use crate::domain::Domain;
use crate::entity::recruitment::Recruitment;
use serde::Deserialize;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Deserialize)]
pub struct PrimitiveRecruitment {
    id: String,
    organizer_nick_name: String,
    start_date: String,
    rendezvous_prefecture: String,
    rendezvous_municipality: String,
    rendezvous_point: String,
    destination_prefecture: String,
    destination_municipality: String,
    destination_point: String,
    budget: String,
    participant_count: String,
}

impl PrimitiveRecruitment {
    pub fn sophisticate(self) -> Result<Recruitment, String> {
        Ok(Recruitment {
            id: ID::new(&self.id)?,
            organizer_nick_name: NickName::new(&self.organizer_nick_name)?,
            start_date: StartDate::new(&self.start_date)?,
            rendezvous_prefecture: Prefecture::new(&self.rendezvous_prefecture)?,
            rendezvous_municipality: Municipality::new(&self.rendezvous_municipality)?,
            rendezvous_point: PointName::new(&self.rendezvous_point)?,
            destination_prefecture: Prefecture::new(&self.destination_prefecture)?,
            destination_municipality: Municipality::new(&self.destination_municipality)?,
            destination_point: PointName::new(&self.destination_point)?,
            budget: Budget::new(&self.budget)?,
            participant_count: ParticipantCount::new(&self.participant_count)?,
        })
    }
}
