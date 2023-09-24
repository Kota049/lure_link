pub mod budget;
pub mod municipality;
pub mod nick_name;
pub mod participant_count;
pub mod point_name;
pub mod prefecture;
pub mod recruitment_deadline;
pub mod start_time;

pub trait Domain {
    fn new(_: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn to_string(self) -> String;
}
