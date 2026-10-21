use crate::TriangleTuple;

// Make a function that parses a string and adds numbers found in it
pub fn sum_digits(s: String) -> u32{
    let v: Vec<u32> = s.chars().flat_map(|e | String::from(e).parse::<u32>().ok()).collect();
    v.iter().sum()
}

// Make a function to check if geometric figure is triangle
// The sum of any 2 sides is gt other one (Triangle Inequality Theorem)
pub fn is_triangle(t: TriangleTuple) -> bool {
    let TriangleTuple(a, b, c) = t;
    a>0 && b>0 && c>0 && a+b>c && b+c>a && a+c>b
}

// Helper function to convert take a single char to its value (in terms of xo)
fn convert_xo(c: char) -> i32{
    let v = if c=='x' {1} else {-1};
    v
}

// true if x==0 else false
pub fn xos(s: &str) -> bool {
    s.chars().fold(0, |acc ,x| acc+convert_xo(x)) == 0
}