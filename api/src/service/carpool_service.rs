use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::entity::car_pool::CarPool;
use crate::entity::user::User;
use crate::error::Error;
use crate::error::Error::Other;

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
    now < &carpool.apl_deadline
}

pub fn is_applying(car_pool: &CarPool) -> bool {
    car_pool.status == CarPoolStatus::Applying
}

pub fn add_one_accept(car_pool: CarPool) -> Result<CarPool, Error> {
    let update_participant = car_pool.current_participant + 1;
    if update_participant > car_pool.max_participant {
        return Err(Other("over max participant".to_string()));
    }
    if update_participant == car_pool.max_participant {
        return Ok(CarPool {
            current_participant: update_participant,
            status: CarPoolStatus::AplComplete,
            ..car_pool
        });
    }
    Ok(CarPool {
        current_participant: update_participant,
        ..car_pool
    })
}

pub fn delete_one_accept(car_pool: CarPool) -> Result<CarPool, Error> {
    if car_pool.current_participant < 1 {
        return Err(Other("there is no participant".to_string()));
    }
    let updated_status = match car_pool.status {
        CarPoolStatus::Applying => CarPoolStatus::Applying,
        CarPoolStatus::AplComplete => CarPoolStatus::Applying,
        CarPoolStatus::Cancel => return Err(Other("cancel car pool".to_string())),
        CarPoolStatus::Finished => return Err(Other("finished car pool".to_string())),
    };

    Ok(CarPool {
        current_participant: car_pool.current_participant - 1,
        status: updated_status,
        ..car_pool
    })
}
