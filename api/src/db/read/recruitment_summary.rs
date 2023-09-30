use crate::entity::recruitment::primitive::PrimitiveRecruitment;
use tokio_postgres::Client;

#[cfg(test)]
mod tests;

pub async fn get_recruitment_summary_list(
    client: &Client,
) -> Result<Vec<PrimitiveRecruitment>, String> {
    let query = GET_RECRUITMENT_SUMMARY_QUERY;

    let rows = client.query(query, &[]).await.unwrap();
    Ok(rows
        .iter()
        .map(|row| PrimitiveRecruitment {
            id: row
                .try_get::<&str, String>("id")
                .unwrap_or_else(|_| "".to_string()),

            organizer_nick_name: row
                .try_get::<&str, String>("organizer_nick_name")
                .unwrap_or_else(|_| "".to_string()),
            start_date: row
                .try_get::<&str, String>("start_date")
                .unwrap_or_else(|_| "".to_string()),
            rendezvous_prefecture: row
                .try_get::<&str, String>("rendezvous_prefecture")
                .unwrap_or_else(|_| "".to_string()),
            rendezvous_municipality: row
                .try_get::<&str, String>("rendezvous_municipality")
                .unwrap_or_else(|_| "".to_string()),
            rendezvous_point: row
                .try_get::<&str, String>("rendezvous_point")
                .unwrap_or_else(|_| "".to_string()),
            destination_prefecture: row
                .try_get::<&str, String>("destination_prefecture")
                .unwrap_or_else(|_| "".to_string()),
            destination_municipality: row
                .try_get::<&str, String>("destination_municipality")
                .unwrap_or_else(|_| "".to_string()),
            destination_point: row
                .try_get::<&str, String>("destination_point")
                .unwrap_or_else(|_| "".to_string()),
            budget: row
                .try_get::<&str, String>("budget")
                .unwrap_or_else(|_| "".to_string()),
            participant_count: row
                .try_get::<&str, String>("participant_count")
                .unwrap_or_else(|_| "".to_string()),
        })
        .collect())
}
const GET_RECRUITMENT_SUMMARY_QUERY: &'static str = "
        SELECT
            r.id::TEXT,
            u.nick_name::TEXT AS organizer_nick_name,
            r.start_datetime::TEXT AS start_date,
            rp.name::TEXT AS rendezvous_prefecture,
            rm.name::TEXT AS rendezvous_municipality,
            r.rendezvous_point::TEXT AS rendezvous_point,
            dp.name::TEXT AS destination_prefecture,
            dm.name::TEXT AS destination_municipality,
            r.destination_point::TEXT AS destination_point,
            r.budget::TEXT AS budget,
            r.participant_count::TEXT AS participant_count
        FROM recruitments AS r
        LEFT JOIN users  AS u ON u.id = r.organizer_id
        LEFT JOIN prefectures AS rp ON rp.id = r.rendezvous_prefecture_id
        LEFT JOIN prefectures AS dp ON dp.id = r.destination_prefecture_id
        LEFT JOIN prefectures AS rm ON rm.id = r.rendezvous_municipality_id
        LEFT JOIN prefectures AS dm ON dm.id = r.destination_municipality_id
        ORDER BY start_date ASC NULLS LAST;
";
