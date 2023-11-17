# 编写自动化测试
##  编写和运行测试
### 测试(函数)
* 测试: 
    * 函数
    * 验证非测试代码的功能是否和预期一致

* 测试函数体(通常)执行的3个操作
    * 准备数据/状态
    * 运行被测试的代码
    * 断言(Assert)结果

### 解剖测试函数
* 测试函数需要使用test属性(attribute)进行标注
    * Attribute 就是一段Rust代码的元数据
    * 在函数上加上`#[test]`, 可以把一个函数变成测试函数

### 运行测试
* 使用 `cargo test` 命令运行所有的测试函数
    * Rust 会构建一个 Test Runner 可执行文件
    * Test Runner 会运行所有测试函数并报告结果是否成功

* 当使用cargo 创建library项目的时候, 会生成一个test module, 里面有一个test 函数
    * 你可以添加任意数量的test module 或函数  

`lib.rs`里:   
```
#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        assert_eq!(2 + 2, 4);
    }
}
```

### 测试失败
* 测试函数panic 就表示失败
* 每个函数都运行在一个新线程
* 当主线程看到某个测试现场挂了, 那个测试标记为失败了
```
#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another(){
        panic!("Make this test fail");
    }
}
```
## 断言(Assert)
### 使用assert! 宏检查测试结果
* assert! 宏, 来在标准库, 用来确定某个状态是否为true
    * true, 测试通过
    * false, 测试失败, 并调用panic

###  使用assert_eq! 和 assert_ne! 测试相等性
* 都来自标准库
* 判断两个参数是否相等或不等
* 实际上,  它们就是 == 和 != 操作符
* 当断言失败时, 会打印出参数的值
    * 使用debug格式打印参数
    * 要求参数实现了PartialEq 和 Debug trait (所有基本类型和标准库里打大部分类型都实现了)

```
pub fn add_two(a: i32) -> i32{
    a + 2
}
#[cfg(test)]
mod tests{
    #[test]
    fn it_adds_two(){
        assert_eq!(4, super::add_two(2));
    }

}
```

### 添加自定义的信息
* 可以向assert!, assert_eq!, assert_ne! 添加可选的自定义信息
    * 这些自定义信息和失败消息都会被打印出来
    * assert!: 第1参数必填, 自定义参数为第2个参数
    * assert_eq!, assert_ne!: 第1,2参数必填, 自定义参数为第3个参数
    * 自定义参数会被传递给format! 宏, 可以使用{} 占位符
```
pub fn greeting(name: &str) -> String {
    format!("Hello !")
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result);
    }

}
```
## 使用should_panic 检查恐慌
### 验证错误处理的情况
* 测试除了代码的返回值是否正确, 还需验证代码是否如期的处理了发生错误的情况
* 可验证代码在特定情况下是否发生了panic
* should_panic 属性(attribute):
    * 函数panic: 测试通过
    * 函数没有panic: 测试失败


```
pub fn greeting(name: &str) -> String {
    format!("Hello !")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {value}
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
}
```
因为Guess::new(200) 会panic, 所以测试通过  

### 让should_panic 的测试更精确
* 为should_panic 添加可选的expected 参数:
    * 将检查失败信息中是否包含所指定的文字

```


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
    use super::*;
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(200);
    }
}
```
## 使用Result`<T, E>` 的测试




