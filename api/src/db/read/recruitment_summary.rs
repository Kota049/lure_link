use crate::entity::recruitment::primitive::PrimitiveRecruitment;
use tokio_postgres::Client;

#[cfg(test)]
mod tests;

pub async fn get_recruitment_summary_list(
    client: &Client,
) -> Result<Vec<PrimitiveRecruitment>, String> {
    todo!()
}
