use crate::domain::domain_object::budget::Budget;
use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::fishing_point::PointName;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::municipality::Municipality;
use crate::domain::domain_object::prefecture::Prefecture;
use crate::entity::users::User;
#[cfg(test)]
use chrono::{TimeZone, Utc};
#[cfg(test)]
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CarPool {
    pub id: Id,
    pub organizer: User,
    pub start_time: JaTimeStamp,
    pub end_time: JaTimeStamp,
    pub apl_deadline: JaTimeStamp,
    pub departure: Point,
    pub destination: Point,
    pub budget: Budget,
    pub max_participant: i16,
    pub current_participant: i16,
    pub status: CarPoolStatus,
    pub description: String,
}

#[cfg(test)]
impl Default for CarPool {
    fn default() -> Self {
        Self {
            id: 1i64.try_into().unwrap(),
            organizer: Default::default(),
            start_time: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            end_time: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            apl_deadline: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            departure: Default::default(),
            destination: Default::default(),
            budget: 1000i32.try_into().unwrap(),
            max_participant: 0,
            current_participant: 0,
            status: CarPoolStatus::Applying,
            description: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point {
    pub prefecture: Prefecture,
    pub municipality: Municipality,
    pub point_name: PointName,
}

#[cfg(test)]
impl Default for Point {
    fn default() -> Self {
        Self {
            prefecture: Prefecture::Hokkaido,
            municipality: "municipality".to_string().try_into().unwrap(),
            point_name: "a".to_string().try_into().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateCarPool {
    pub start_time: JaTimeStamp,
    pub end_time: JaTimeStamp,
    pub apl_deadline: JaTimeStamp,
    pub departure_prefecture_id: Id,
    pub departure_municipality_id: Id,
    pub departure_point: PointName,
    pub destination_prefecture_id: Id,
    pub destination_municipality_id: Id,
    pub destination_point: PointName,
    pub budget: Budget,
    pub max_participant: i16,
    pub description: String,
}

#[cfg(test)]
impl Default for CreateCarPool {
    fn default() -> Self {
        Self {
            start_time: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            end_time: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            apl_deadline: JaTimeStamp(
                Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&Tokyo),
            ),
            departure_prefecture_id: 1i64.try_into().unwrap(),
            departure_municipality_id: 1i64.try_into().unwrap(),
            departure_point: "a".to_string().try_into().unwrap(),
            destination_prefecture_id: 1i64.try_into().unwrap(),
            destination_municipality_id: 1i64.try_into().unwrap(),
            destination_point: "a".to_string().try_into().unwrap(),
            budget: 1000i32.try_into().unwrap(),
            max_participant: 0,
            description: "".to_string(),
        }
    }
}
