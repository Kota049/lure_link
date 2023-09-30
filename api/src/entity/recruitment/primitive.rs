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

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct PrimitiveRecruitment {
    pub id: String,
    pub organizer_nick_name: String,
    pub(crate) start_date: String,
    pub(crate) rendezvous_prefecture: String,
    pub(crate) rendezvous_municipality: String,
    pub(crate) rendezvous_point: String,
    pub(crate) destination_prefecture: String,
    pub(crate) destination_municipality: String,
    pub(crate) destination_point: String,
    pub(crate) budget: String,
    pub(crate) participant_count: String,
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
