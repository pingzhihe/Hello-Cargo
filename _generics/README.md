# 泛型， Trait
## 提取函数消除重复
```
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
fn main() {
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest(&num_list);

    println!("The largest number is {}", result);
}
```
当你写 `for number in list` 时，`number` 实际上是每个元素的引用，因为 `list`是一个切片引用。在这种情况下，`number` 的类型是 `&i32`。  
当你写 `for &number in list` 时，你实际上是在使用模式匹配来解引用每个元素，使得 `number` 直接获得了元素的值，而不是它们的引用。在这种情况下，`number` 的类型是  `i32`。  
`&i32` = &`i32`

* 泛型: 提高代码的复用能力
    * 处理重复代码的问题
* 泛型是具体类型或其他属性的抽象代替:
    * 你编写的代码不是最终代码, 而是一个模板, 里面有一些"占位符"
    * 编译器在编译的时将占位符替代为具体的类型。
* 例如: `fn largest<T>(list: &[T]) -> T {}`
    * T 是一个占位符, 代表任何类型
* 该函数可以接受任何类型的 slice, 并返回同样类型的值
* 类型参数: 
    * 很短, 通常一个字母
    * CamalCase
    * T: Type的缩写

## 函数定义中使用泛型
* 泛型函数
    * 参数类型
* 返回值类型  
会在后面详细解释
## Struct中定义中使用泛型
```
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1, y: 4.0 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
}
```
T 和 U 代表了Point里x和y可以是不同的类型

## 枚举定义中使用泛型
* 可以让枚举的变体持有泛型数据类型
    * 例如 `Option<T>`,`Result<T, E>`
```
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
## 方法定义中使用泛型
* 为struct和enum定义方法的时候, 可以在定义中使用泛型
```
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```
* 注意:
    * 把 T 放在impl关键字后, 表示在类型T上实现的方法
        * 例如: `impl<T> Point<T> {}`
    * 只针对具体类型实现方法(其余类型没实现方法):
        * 例如: `impl Point<f32> {}` (这里impl后没有`<T>`)

## 泛型代码的性能
* 使用泛型的代码和使用具体类型的代码运行速度是一样的
* Rust通过在编译时进行泛型代码的单态化(monomorphization)来保证效率
    * 单态化: 将泛型代码转换为特定代码的过程

# Trait
* Trait 告诉Rust 编译器:
    * 某些类型具有哪些并且可以
