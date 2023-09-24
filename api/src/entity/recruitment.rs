use crate::domain::budget::Budget;
use crate::domain::municipality::Municipality;
use crate::domain::nick_name::NickName;
use crate::domain::participant_count::ParticipantCount;
use crate::domain::prefecture::Prefecture;
use crate::domain::start_time::StartDate;

pub struct Recruitment {
    id: String,
    organizer_nick_name: NickName,
    start_date: StartDate,
    rendezvous_prefecture: Prefecture,
    rendezvous_municipality: Municipality,
    rendezvous_point: String,
    destination_prefecture: Prefecture,
    destination_municipality: Municipality,
    destination_point: String,
    budget: Budget,
    participant_count: ParticipantCount,
}
