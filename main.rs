mod kyu7;

use kyu7::k7_exercises::sum_digits;
use kyu7::k7_exercises::is_triangle;

struct TriageTuple(i32,i32,i32);

// If you use `main()`, declare it as `pub` to see it in the output:
pub fn main() { 
    let s = String::from("The30quick20brown10f0x1203jumps914ov3r1349the102l4zy");
    let adding: u32 = sum_digits(s);    

    assert_eq!(adding, 53, "Failed to match value (53)");

    // Testing checking if is triangle
    let a = 3;
    let b = 4;
    let c = 5;
    let t = TriageTuple(a,b,c);
    let triangle = is_triangle(t);
    assert_eq!(triangle, true, "Failed to check that it is a triangle");
    
    
}
