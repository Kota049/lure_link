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
        todo!()
    }
}
