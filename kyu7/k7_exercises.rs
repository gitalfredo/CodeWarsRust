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

// 