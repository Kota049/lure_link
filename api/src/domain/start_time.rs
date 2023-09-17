#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
pub struct StartDate {
    inner:String
}

impl StartDate{
    pub fn new(start_date:&str)->Result<Self,String>{
        if start_date.is_empty(){
            return Err("不正なstart_dateです".to_string());
        }
        Ok(StartDate{inner:start_date.to_string()})
    }
}
