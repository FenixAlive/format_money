//! `format_money` is a simple function that takes a string reference &str as a number, evaluates it and returns a String with money format $ #'###,###.##
/// 
/// # Examples:
/// 
/// ```
/// use format_money::format_money;
/// 
/// fn main() {
///     println!("{}",format_money("00010"));
/// }
/// //it prints $ 10.00
/// ```
/// # Tests:
/// ```
///     use format_money::format_money;
/// 
///    assert_eq!(format_money("."), "$ 0.00");
///
///    assert_eq!(format_money("000.157"), "$ 0.15");
///
///    assert_eq!(format_money("103"), "$ 103.00");
///
///    assert_eq!(format_money("001000000334533461000813.5792"), "$ 1,000'000,334'533,461'000,813.57");
///
///    assert_eq!(format_money("0000.00"), "$ 0.00");
///
///    assert_eq!(format_money("00000010.00000"), "$ 10.00");
///
///    assert_eq!(format_money("0100.00000"), "$ 100.00");
///
///    assert_eq!(format_money("F1c0.0A000"), "$ 0.00");
/// ```

pub fn format_money(num: &str) -> String {
    //Real number
    if let Ok(val) = num.parse::<f64>(){
        if val == 0.0 {
            return String::from("$ 0.00")
        }
    }else{
        return String::from("$ 0.00");
    }
    let mut num = num;
    //decimal
    let mut decimal = String::from(".00");
    if let Some(p) = num.find(".") {
        if p+3 > num.len(){
            if p+2 == num.len(){
                decimal= format!("{}0",&num[p..num.len()]);
            }
        } else{
            decimal = String::from( &num[p..p+3]);
        }
        num = &num[..p]
    }
    //integers
    let mut it = 0;
    for i in num.chars(){
        if i == '0' {
            it += 1;
        }else{
            break;
        }
    }
    num = &num[it..];
    if num.len() == 0{
        return format!("$ 0{}", decimal)
    }
    let mut final_num = String::new();
    let mut i = 0;
    while num.len() > 3 {
        let num_tale = &num[num.len()-3..];
        num= &num[..num.len()-3];
        if let Ok(val) = num.parse::<f64>(){
            if val == 0.0 {
                final_num = format!("{}{}", num_tale, final_num);
                num = "";
            }else{
                if i%2==0{
                    final_num = format!(",{}{}", num_tale, final_num);
                }else{
                    final_num = format!("'{}{}", num_tale, final_num);
                }
            }
        }
        i += 1;
    }
    format!("$ {}{}{}", num, final_num, decimal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_point() {
        assert_eq!(format_money("."), "$ 0.00");
    }
    #[test]
    fn only_decimals(){
        assert_eq!(format_money("000.157"), "$ 0.15");
    }
    #[test]
    fn only_integers(){
        assert_eq!(format_money("103"), "$ 103.00");
    }
    #[test]
    fn all(){
        assert_eq!(format_money("001000000334533461000813.5792"), "$ 1,000'000,334'533,461'000,813.57");
    }
    #[test]
    fn zeros(){
        assert_eq!(format_money("0000.00"), "$ 0.00");
    }
    #[test]
    fn more_zeros(){
        assert_eq!(format_money("00000010.00000"), "$ 10.00");
    }
    #[test]
    fn comma_zeros(){
        assert_eq!(format_money("0100.00000"), "$ 100.00");
    }
    #[test]
    fn no_number(){
        assert_eq!(format_money("F1c0.0A000"), "$ 0.00");
    }
}
