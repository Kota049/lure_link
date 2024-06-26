use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::payment_status::PaymentStatus;
use crate::entity::car_pool::Point;
use serde::{Deserialize, Serialize};

pub mod proposal_user_status;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PaymentInfo {
    pub customer_id: String,
    pub ephemeral_key: String,
    pub payment_intent_key: String,
}

impl IntoResponse for PaymentInfo {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AplProposal {
    pub car_pool_id: Id,
    pub hope_pick_up_location_1: Point,
    pub hope_pick_up_location_2: Option<Point>,
    pub hope_pick_up_location_3: Option<Point>,
    pub payment_status: PaymentStatus,
    pub description: String,
}

#[cfg(test)]
impl Default for AplProposal {
    fn default() -> Self {
        Self {
            car_pool_id: 1i64.try_into().unwrap(),
            hope_pick_up_location_1: Default::default(),
            hope_pick_up_location_2: None,
            hope_pick_up_location_3: None,
            payment_status: Default::default(),
            description: "".to_string(),
        }
    }
}

/*#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UpdateProposal {
    pub id: Id,
    hope_pick_up_location_1: Point,
    hope_pick_up_location_2: Option<Point>,
    hope_pick_up_location_3: Option<Point>,
    description: String,
}

#[cfg(test)]
impl Default for crate::entity::proposal::UpdateProposal {
    fn default() -> Self {
        Self {
            id: 1i64.try_into().unwrap(),
            hope_pick_up_location_1: Default::default(),
            hope_pick_up_location_2: None,
            hope_pick_up_location_3: None,
            description: "".to_string(),
        }
    }
}*/
