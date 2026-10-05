mod kyu7;

use kyu7::sum_digits::sum_digits;
// If you use `main()`, declare it as `pub` to see it in the output:
pub fn main() { 
    let s = String::from("The30quick20brown10f0x1203jumps914ov3r1349the102l4zy");
    let adding: u32 = sum_digits(s);    

    assert_eq!(adding, 53, "Failed to match value (53)");
    
}
