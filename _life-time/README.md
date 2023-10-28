# life time:生命周期
* Rust 的每一个引用都有自己的生命周期
* 生命周期: 引用保持有效的作用域
* 大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样
* 当引用的生命周期可能以不同的方式相互关联时: 手动标注生命周期

## 生命周期-避免悬垂引用(dangling reference)
* 悬垂引用: 指向了其数据被释放的内存
```
fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }
}
```
输出报错: `` `x` does not live long enough ``

## 借用检查器
* Rust 编译器有一个 借用检查器(borrow checker) 来比较作用域来确保所有的借用都是有效的
图示:
```
fn main() {
    {
        let r;                //----------+-- 'a
                              //          |
        {                     //          |
        let x = 5;            // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //----------+
    }

}

```
这样就可以通过编译了
```
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
```
## 函数中的泛型生命周期
```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest (x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
报错: `` expected named lifetime parameter ``
纠正:
```
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str  {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
## 生命周期标注
* 生命周期标注并不改变任何引用的生命周期的长短
* 当指定了泛型生命周期, 函数可以接受带有任何生命周期的引用。
* 生命周期标注: 描述了多个引用生命周期相互的关系, 而不影响其生命周期

### 生命周期标注-语法
* 生命周期参数名:
    * 以撇号(')开头
    * 通常小写且非常短, 如: 'a

* 生命周期标注的位置:
    * 在引用的 & 之后
    * 使用空格将标注和引用类型分开

### 生命周期标注-例子
* &i32        // 引用
* &'a i32     // 带有显式生命周期的引用
* &'a mut i32 // 带有显式生命周期的可变引用

* 单个生命周期标注没有意义

### 函数签名中的生命周期标注
* 泛型生命周期参数声明在: 函数名和参数列表之间的尖括号<>中
    * `fn longest<'a> (x: &'a str, y: &'a str) -> &'a str  {`
    * 当我们把具体的引用传入longest 函数时, 被用来替代'a 的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。而且返回值的生命周期也是 `` `a ``。
    
## 深入了解生命周期
* 指定生命周期参数的方式依赖于函数所作的事情
* 从函数返回引用时，返回类型的生命周期参数需要与一个参数的生命周期参数相匹配
* 如果返回的引用 没有 指向任何一个参数, 那么唯一的可能就是它指向一个函数内部创建的值。
    * 这就是悬垂引用的情况: 该值在函数结束时走出了作用域
```
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str  {
    let result = String::from("really long string");
    result.as_str();             
}
```
报错: `` `result` does not live long enough ``
* 修正: 返回 String 而不是 &str
```
fn longest<'a> (x: &'a str, y: &'a str) ->String{
    let result = String::from("really long string");
    result            
}
```
这样就相当于把所有权转移给了调用者。



## Struct 中的生命周期
* Struct 里可以包括:
    * 自持有的类型
    * 引用: 需要在每个引用上添加生命周期标注
```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt {
         part: first_sentence 
        };
    println!("{}", i.part);

}

```

## 生命周期的省略
* 我们知道:
    * 每一个引用都有其生命周期
    * 需要为函数中的每一个引用都标注生命周期
* 在 Rust 引用分析中所编入的模式被称为 生命周期省略规则(lifetime elision rules)
    * 这些规则无需在大部分情况下手动标注生命周期
* 生命周期省略规则并不提供完整的推断:
    * 如果应用规则后, 引用的生命周期仍然模糊不清 -> 编译器报错
    * 解决办法: 显式标注生命周期

### 生命周期省略的三个规则
* 编译器使用三个规则在没有显式标注生命周期的情况下， 来确定引用的生命周期：
    * 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
    * 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
    * 这些规则适用于 fn 定义，以及 impl 块。

* 规则1: 每一个是引用的参数都有它自己的生命周期参数
* 规则2: 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
* 规则3: 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期

## 方法定义中的生命周期标注
