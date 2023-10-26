use core::num;

use _generics::Summary;
use _generics::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false,
    };
    // println!("1 new tweet: {}", tweet.summarize());
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    
}   

        
fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy
{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest { // std::cmp::PartialOrd
            largest = item;
        }
    }
    largest
}
