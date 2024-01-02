use axum::async_trait;
use chrono::{DateTime, Utc};
use crate::db::connection::DbManager;
use crate::entity::recruitment::Recruitment;
use crate::error::Error;
use crate::repository::recruitment::{RecruitmentTrait};

#[cfg(test)]
mod tests;

pub struct RecruitmentRepository {
    db: DbManager,
}
impl RecruitmentRepository{
    pub fn new()->Self{
        Self{ db: DbManager::new() }
    }
}

#[async_trait]
impl RecruitmentTrait for RecruitmentRepository {
    async fn find_all(&self, now: &DateTime<Utc>) -> Result<Vec<Recruitment>, Error> {
        let rows = self.db.client().await.query(GET_RECRUITMENT_SUMMARY_QUERY, &[]).await.map_err(|e| Error::DbError(e.to_string()))?;
        // todo: add test & mapping
        todo!()
    }
}


const GET_RECRUITMENT_SUMMARY_QUERY: &'static str = "
        SELECT
            r.id::TEXT,
            u.nick_name AS organizer_nick_name,
            r.start_datetime AS start_date,
            rp.name AS rendezvous_prefecture,
            rm.name AS rendezvous_municipality,
            r.rendezvous_point AS rendezvous_point,
            dp.name AS destination_prefecture,
            dm.name AS destination_municipality,
            r.destination_point AS destination_point,
            r.budget AS budget,
            r.participant_count AS participant_count
        FROM recruitments AS r
        LEFT JOIN users  AS u ON u.id = r.organizer_id
        LEFT JOIN prefectures AS rp ON rp.id = r.rendezvous_prefecture_id
        LEFT JOIN prefectures AS dp ON dp.id = r.destination_prefecture_id
        LEFT JOIN prefectures AS rm ON rm.id = r.rendezvous_municipality_id
        LEFT JOIN prefectures AS dm ON dm.id = r.destination_municipality_id
        ORDER BY start_date ASC NULLS LAST;
";