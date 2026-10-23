// Helper function to convert take a single char to its value (in terms of xo)
fn convert_xo(c: char) -> i32{
    let v = if c=='x' {1} else {-1};
    v
}
 

// true if x==0 else false
pub fn xos(s: &str) -> bool {
    let s1 = s.to_lowercase();
    s1.chars().fold(0, |acc ,x| acc+convert_xo(x)) == 0
}