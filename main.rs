mod kyu7;
mod kyu6;

use kyu7::k7_sum_digits::sum_digits;
use kyu7::k7_is_triangle::is_triangle;
use kyu7::k7_xos::xos;

use kyu6::k6_romantointeger::roman_to_integer;
use crate::kyu6::k6_romantointeger::NumeralValues;

struct TriangleTuple(i32,i32,i32);

// If you use `main()`, declare it as `pub` to see it in the output:
pub fn main() { 
    let s = String::from("The30quick20brown10f0x1203jumps914ov3r1349the102l4zy");
    let adding: u32 = sum_digits(s);    

    assert_eq!(adding, 53, "Failed to match value (53)");

    // Testing checking if is triangle
    let a = 3;
    let b = 4;
    let c = 5;
    let t = TriangleTuple(a,b,c);
    let triangle = is_triangle(t);
    assert_eq!(triangle, true, "Failed to check that it is a triangle");
    
    // xos (strings of xoxoxxx return true if amount of x,o match)
    let s = "xooxxxoxoooooooxxxxx";  // true
    let x0 = xos(s);
    assert_eq!(x0, true, "Failed to check that {s} has the same amount of x and o");

    let s = "xooX";  // true
    let x0 = xos(s);
    assert_eq!(x0, true, "Failed to check that {s} has the same amount of x and o");

    // 6 kyu - romans to integer
    let s = "XIX";  // 19
    let table = NumeralValues::new();
    let conversion = roman_to_integer(s,table);
    assert_eq!(conversion, 19, "Failed to convert {s} to integers ({conversion})");
}
