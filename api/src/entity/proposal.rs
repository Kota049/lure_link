use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::payment_status::PaymentStatus;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::recruitment::{CarPool, Point};
use crate::entity::users::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Proposal {
    pub id: Id,
    pub user: User,
    pub carpool: CarPool,
    pub status: ProposalStatus,
    pub hope_pick_up_location_1: Point,
    pub hope_pick_up_location_2: Option<Point>,
    pub hope_pick_up_location_3: Option<Point>,
    pub pick_up_location: Option<Point>,
    pub payment_status: PaymentStatus,
    pub apl_time: JaTimeStamp,
    pub description: String,
}

#[cfg(test)]
impl Default for Proposal {
    fn default() -> Self {
        Self {
            id: 1i64.try_into().unwrap(),
            user: Default::default(),
            carpool: Default::default(),
            status: Default::default(),
            hope_pick_up_location_1: Default::default(),
            hope_pick_up_location_2: None,
            hope_pick_up_location_3: None,
            pick_up_location: None,
            payment_status: Default::default(),
            apl_time: Default::default(),
            description: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateProposal {
    pub carpool: CarPool,
    pub hope_pick_up_location_1: Point,
    pub hope_pick_up_location_2: Option<Point>,
    pub hope_pick_up_location_3: Option<Point>,
    pub payment_status: PaymentStatus,
    pub apl_time: JaTimeStamp,
    pub description: String,
}

#[cfg(test)]
impl Default for CreateProposal {
    fn default() -> Self {
        Self {
            carpool: Default::default(),
            hope_pick_up_location_1: Default::default(),
            hope_pick_up_location_2: None,
            hope_pick_up_location_3: None,
            payment_status: Default::default(),
            apl_time: Default::default(),
            description: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UpdateProposal {
    pub id: Id,
    hope_pick_up_location_1: Point,
    hope_pick_up_location_2: Option<Point>,
    hope_pick_up_location_3: Option<Point>,
    description: String,
}

#[cfg(test)]
impl Default for UpdateProposal {
    fn default() -> Self {
        Self {
            id: 1i64.try_into().unwrap(),
            hope_pick_up_location_1: Default::default(),
            hope_pick_up_location_2: None,
            hope_pick_up_location_3: None,
            description: "".to_string(),
        }
    }
}
