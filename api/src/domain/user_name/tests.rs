pub mod error{
    use super::super::*;
    #[test]
    pub fn empty_value(){
        let empty= "";
        let result = UserName::new(empty);
        assert_eq!(result,Err("空文字です".to_string()));
    }

    #[test]
    pub fn half_width_num_value(){
        let half_width_num="1234";
        let result = UserName::new(half_width_num);
        assert_eq!(result,Err("半角数字が含まれています".to_string()));
    }

    #[test]
    pub fn full_width_num_value(){
        let full_width_num = "１２３４";
        let result = UserName::new(full_width_num);
        assert_eq!(result,Err("全角数字が含まれています".to_string()));
    }

    #[test]
    pub fn full_width_sym_value(){
        let sym = "★";
        let result = UserName::new(sym);
        assert_eq!(result,Err("全角記号が含まれています".to_string()));
    }

    #[test]
    pub fn length_limit(){
        let length_limit_value = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふ";
        let result = UserName::new(length_limit_value);
        assert_eq!(result,Err("26字以内に収めてください".to_string()));
    }
}