use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;

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

pub fn is_organizer(c: &CarPool, u: &User) -> bool {
    c.organizer.id == u.id
}

pub fn can_apl_term(now: &JaTimeStamp, carpool: &CarPool) -> bool {
    &carpool.apl_deadline < now
}

pub fn is_applying(car_pool: &CarPool) -> bool {
    car_pool.status == CarPoolStatus::Applying
}
