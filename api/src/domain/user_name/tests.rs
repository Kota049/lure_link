pub mod error{
    use super::super::*;
    #[test]
    pub fn empty_value(){
        let empty= "";
        let result = UserName::new(empty);
        assert_eq!(result,Err("空文字です".to_string()));
    }
}