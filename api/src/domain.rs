pub mod prefecture;
pub mod recruitment_deadline;
pub mod start_time;

pub trait Domain {
    fn new(_: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn to_string(self) -> String;
}
