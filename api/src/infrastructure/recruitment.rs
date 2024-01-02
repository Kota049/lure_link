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
        let rows = self.db.client().await.query(GET_RECRUITMENT_SUMMARY_QUERY, &[&now]).await.map_err(|e| Error::DbError(e.to_string()))?;
        let mut recruitments = vec![];
        for row in rows{
            recruitments.push(
                Recruitment{
                    id: row.try_get("id").map_err(|_|Error::DbError("parse error".to_string()))?,
                    organizer_nick_name: row.try_get("organizer_nick_name").map_err(|_|Error::DbError("parse error".to_string()))?,
                    start_time: row.try_get("start_datetime").map_err(|_|Error::DbError("parse error".to_string()))?,
                    apl_deadline: row.try_get("apl_deadline").map_err(|_|Error::DbError("parse error".to_string()))?,
                    rendezvous_prefecture: row.try_get("rendezvous_prefecture").map_err(|_|Error::DbError("parse error".to_string()))?,
                    rendezvous_municipality:row.try_get("rendezvous_municipality").map_err(|_|Error::DbError("parse error".to_string()))?,
                    rendezvous_point: row.try_get("rendezvous_point").map_err(|_|Error::DbError("parse error".to_string()))?,
                    destination_prefecture:row.try_get("destination_prefecture").map_err(|_|Error::DbError("parse error".to_string()))?,
                    destination_municipality: row.try_get("destination_municipality").map_err(|_|Error::DbError("parse error".to_string()))?,
                    destination_point: row.try_get("destination_point").map_err(|_|Error::DbError("parse error".to_string()))?,
                    budget: row.try_get("budget").map_err(|_|Error::DbError("parse error".to_string()))?,
                    participant_count: row.try_get("participant_count").map_err(|_|Error::DbError("parse error".to_string()))?,
                }
            )
        }
        Ok(recruitments)
    }
}


const GET_RECRUITMENT_SUMMARY_QUERY: &'static str = "
        SELECT
            r.id,
            u.nick_name AS organizer_nick_name,
            r.start_datetime AS start_date,
            r.application_datetime AS apl_deadline,
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
        WHERE application_datetime > $1
        ORDER BY start_date ASC NULLS LAST;
";