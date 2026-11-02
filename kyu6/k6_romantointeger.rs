pub struct NumeralValues{
    xs: Vec<(char, u16)>
}

impl NumeralValues{
    pub fn new() -> Self{
        NumeralValues { xs: vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]}
    }

    fn get_value(&self, c: char)-> Option<u16> {
        if let Some(x) = self.xs.iter().find(|&e| e.0 == c){
            return Some(x.1);
        }else{
            return Some(0);
        }
    }
}
 

// Converts roman to integers
pub fn roman_to_integer(s: &str, vs: NumeralValues) -> u16{
    // say XIX is 19
    //let s = "XIX";
    let mut prev = s.chars().next().unwrap();
    let mut acc = 0;
    acc += vs.get_value(prev).unwrap();
    for elem in s.chars().rev().skip(1){
        if let Some(x) = vs.get_value(elem){
            if let Some(y) = vs.get_value(prev){
                if x >= y {
                    acc+=x;
                }else{
                    acc-=x;
                }
            }
            
        }
        // Updates prev
        prev = elem;
    }
    acc
}

