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