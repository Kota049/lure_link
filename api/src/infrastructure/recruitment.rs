use axum::async_trait;
use tokio_postgres::Row;
use crate::db::connection::DbManager;
use crate::entity::recruitment::Recruitment;
use crate::error::Error;
use crate::repository::recruitment::{RecruitmentTrait};

#[cfg(test)]
mod tests;

pub struct Repo {
    db: DbManager,
}

#[async_trait]
impl RecruitmentTrait for Repo {
    async fn get_all(&self) -> Result<crate::entity::recruitment::Recruitment, Error> {
        let rows = self.db.client().await.query(GET_RECRUITMENT_SUMMARY_QUERY, &[]).await.map_err(|e| Error::DbError(e.to_string()))?;
        // todo: add test & mapping
        todo!()
    }
}

impl Repo {
    fn map(r: &Row) -> Result<Recruitment, Error> {
        todo!()
        // Ok(Recruitment{
        //     id: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     organizer_nick_name: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     start_date: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     rendezvous_prefecture: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     rendezvous_municipality: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     rendezvous_point: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     destination_prefecture: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     destination_municipality: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     destination_point: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     budget: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        //     participant_count: r.try_get("id").map_err(|e|Error::DbError(e.to_string()))?,
        // })
    }
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