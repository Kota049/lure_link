use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::entity::recruitment::CarPool;
#[cfg(test)]
mod tests;

pub fn is_canceled(c: &CarPool) -> bool {
    c.status == CarPoolStatus::Cancel
}

pub fn modify_to_cancel(c: CarPool) -> CarPool {
    CarPool {
        status: CarPoolStatus::Cancel,
        ..c
    }
}
