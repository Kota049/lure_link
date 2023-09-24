use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct ParticipantCount {
    inner: i16,
}

impl Domain for ParticipantCount {
    fn new(participant_count: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let participant_count = participant_count
            .parse::<i16>()
            .map_err(|_| "不正なparticipant_countです".to_string())?;
        Ok(ParticipantCount {
            inner: participant_count,
        })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
