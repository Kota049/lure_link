#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
pub struct StartDate {
    inner:String
}

impl StartDate{
    pub fn new(start_date:&str)->Result<Self,String>{
        todo!()
    }
}
