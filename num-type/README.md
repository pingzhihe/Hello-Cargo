## Shadowing
可以用相同的名字声明新的变量，此时新的变量会覆盖之前的变量，这种情况叫做变量的shadowing。
```
let x = 5;
let x = x + 1;
```
使用let声明的同名新变量，它的类型可以与之前的变量不同。
```
let spaces = "   ";
let spaces = spaces.len();
```
## Data Types
rust 是静态类型语言，编译器在编译时就必须知道所有变量的类型。

```
let guess: u32 = "42".parse().expect("Not a number!");
```
## Expression and Argument
表达式返回一个值，语句不返回值。
这里大括号里面的就是一个表达式，它返回一个值。
```
    let y = {
        let x = 3;
        x + 1 //expression
    };
```
这里的就是statement，它返回空tuple。
```
    let y = {
        let x = 3;
        x + 1; //statement
    };
```
## 流控制
if else 是表达式，所以可以用在let语句的右边。
```
    let number = if condition { 5 } else { 6 };
```
if else 一般语法：
```
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
```
loop 是一个无限循环，可以用break跳出循环。
```
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
```


