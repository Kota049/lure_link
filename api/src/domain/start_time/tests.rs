use super::*;

#[test]
fn valid(){
    let date = "2023-09-17 12:34:56";
    let result = StartDate::new(date);
    assert_eq!(result,Ok(StartDate{inner:"2023-09-17 12:34:56".to_string()}));
}
