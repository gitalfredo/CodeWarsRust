// Make a function that parses a string and adds numbers found in it
pub fn sum_digits(s: String) -> u32{
    let v: Vec<u32> = s.chars().flat_map(|e | String::from(e).parse::<u32>().ok()).collect();
    v.iter().sum()
}
