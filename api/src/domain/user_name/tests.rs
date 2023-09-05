pub mod error{
    use super::super::*;
    #[test]
    pub fn empty_value(){
        let empty= "";
        let result = UserName::new(empty);
        assert_eq!(result,Err("空文字です".to_string()));
    }

    #[test]
    pub fn half_width_value(){
        let half_width="abcde";
        let result = UserName::new(half_width);
        assert_eq!(result,Err("半角の文字が含まれています".to_string()));
    }
}