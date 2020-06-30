# format_money

`format_money` is a simple function that takes a string reference &str as a number, evaluates it and returns a String with money format $ #'###,###.##

# Examples:

 ```
 use format_money::format_money;

 fn main() {
     println!("{}",format_money("00,010"));
 }
 //it prints $ 10.00
 ```