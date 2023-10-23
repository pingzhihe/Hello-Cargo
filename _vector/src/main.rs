use std::collections::HashMap;

fn main() {
    let text =  "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
        // 这里我们将这个可变引用储存在 count 变量中
        let count = map.entry(word).or_insert(0); //返回的是一个可变引用
        *count += 1;
    }

    println!("{:?}", map);

}
