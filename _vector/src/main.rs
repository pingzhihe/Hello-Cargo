use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    let e= scores.entry(String::from("Yellow"));
    println!("{:?}", e);
    e.or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); //不会覆盖原有的值
    
    println!("{:?}", scores);

}
