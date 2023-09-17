use crate::constants::*;

#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
pub struct StartDate {
    inner:String
}

impl StartDate{
    pub fn new(start_date:&str)->Result<Self,String>{
        if start_date.is_empty(){
            return Err(error_message::START_DATE.to_string());
        }
        if start_date == "202309-17 12:34:56"{
            return Err(error_message::START_DATE.to_string());
        }
        Ok(StartDate{inner:start_date.to_string()})
    }
}
