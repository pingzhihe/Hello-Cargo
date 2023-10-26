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
    



