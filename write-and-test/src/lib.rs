

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be great or equal to 1 got {}.",
            value
            )
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100 got {}.",
            value
            )
        }
        Guess {value}
    }
}
#[cfg(test)]
mod tests{
    #[test]
    fn if_works() -> Result<(), String>{
        if 2+3 == 4{
            Ok(())
        } else{
            Err(String::from("two plus two does not equal to four")
            )    
        }
    }
}