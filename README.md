# format_money

`format_money` is a simple function that takes a string reference &str as a number, evaluates it and returns a String with money format $ #'###,###.##

# Examples:

 ```
 use format_money::format_money;

 fn main() {
     println!("{}",format_money("0100010"));
     //it prints $ 100,010.00

    assert_eq!(format_money("."), "$ 0.00");

    assert_eq!(format_money("000.157"), "$ 0.15");

    assert_eq!(format_money("103"), "$ 103.00");

    assert_eq!(format_money("001000000334533461000813.5792"), "$ 1,000'000,334'533,461'000,813.57");

    assert_eq!(format_money("0000.00"), "$ 0.00");

    assert_eq!(format_money("00000010.00000"), "$ 10.00");

    assert_eq!(format_money("0100.00000"), "$ 100.00");

    assert_eq!(format_money("F1c0.0A000"), "$ 0.00");

    assert_eq!(format_money("00,0010.0A000"), "$ 0.00");
    // last prints $ 0.00 because it has a comma or any other digit but a number o dot which can´t be parse to f64
 }
 
 ```