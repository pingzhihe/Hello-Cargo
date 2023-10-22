# Vector
## 使用Vector储存多个值
* `Vec<T>`,叫做vector
    * 由标准库提供
    * 可以储存多个值
    * 所有值的类型相同
    * 值在内存中连续存放

## 创建一个vector
```
let  v: Vec<i32> = Vec::new();
```
* 使用初始值创建Vec`<T>`,使用`vec!`宏
```
let v = vec![1, 2, 3, 4, 5];
```
* 更新vector
```
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```
* 与其他类型一样，当vector离开作用域时，vector和其所有元素都被丢弃

* Vector 根据index查找元素
```
let v = vec![1, 2, 3, 4, 5];
let third = &v[2];
println!("The third element is {}", third);

match v.get(2){
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```
第一种在索引不存在时会引发panic，第二种会返回None
```
match v.get(100){
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```
返回：`There is no third element.`

## 所有权和借用规则
* 不能在同一作用域中同时存在可变和不可变引用
这里编译器就会报错：
```
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first);
```

## 遍历vector
```
let v = vec![1, 2, 3, 4, 5];
for i in &v {
    println!("{}", i);
}
```

```
let mut v = vec![1, 2, 3, 4, 5];
for i in &mut v {
    *i += 50;
}
for i in &v {
    println!("{}", i);
}

```
## 使用枚举储存多种类型
* Enum 的变体可以附加不同类型的数据
* Enum 的定义在同一个enum下
```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

}
```
# String
String 的数据结构复杂
* Rust 的核心语言层面，只有一个字符串类型:字符串切片`str`或 `&str`
* 字符串切片: 对储存在其他地方, UTF-8编码的字符串的引用
    * 字符串字面值: 储存在二进制文件中, 也就是字符串切片

* String 类型:
    * 来自标准库, 可增长, 可变, 有所有权, UTF-8编码
## 创建一个新的字符串
* 很多`Vec<T>`的操作都可用于`String`
* String::new() 创建一个空的字符串

*  使用`to_string()`方法,  可用于实现了`Display` trait的类型, 包括字符串字面值
```
let data = "initial contents";
let s = data.to_string(); // convert string literal to String
let s1 = "initial contents".to_string(); // same as above
```
* 使用`String::from()`方法, 从字面值创建String
```
let s = String::from("hello");
```

