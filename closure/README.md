# 闭包(closure)
*  闭包: 可以捕获所在环境的匿名函数
* 闭包: 
    * 是匿名函数
    * 保存为变量, 作为参数
    * 可在一个地方创建闭包, 然后在另外一个上下文调用闭包来完成运算
    * 可从其定义的作用域捕获

## 生成自定义运动计划的程序
* 算法的逻辑并不是重点, 重点是算法的计算过程需要几秒钟时间
* 目标: 不让用户发生不必要等待
    * 仅在必要时调用该算法
    * 只调用一次

## 闭包的类型推断
* 闭包不要求标注参数和返回值类型
* 闭包通常很短小, 只在狭小的上下文中工作, 编译器通常能推断出类型
* 可以手动添加类型标注

## 函数和闭包的定义语法
`fn add_one_v1(x: u32) -> u32 { x + 1 }`
`let add_one_v2 = |x: u32| -> u32 { x + 1 };`
`let add_one_v3 = |x|   { x + 1 };`
`let add_one_v4 = |x|     x + 1 ;`

```
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
```
* 注意: 闭包的定义最终只会为参数/返回值推断出唯一具体的类型

## 运动程序的另一种解决方案
* 创建一个struct, 它持有闭包及其调用结果
* 闭包只会被调用一次

### 如何让struct 持有闭包
* struct 的定义需要知道所有字段的类型
    * 需要指明闭包的类型

* 每个闭包实例都有自己唯一的匿名类型, 即使两个闭包签名完全一样。
* 所以需要用: 泛型和Trait Bound (第十章)

## Fn Trait
* Fn traits 由标准库提供
* 所有的闭包都至少实现了以下trait之一
    * Fn
    * FnMut
    * FnOnce  

例子, 利用缓存器 Cacher：
```
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25{
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

```


```
#[cfg(test)]
mod tests {

    #[test]
    fn call_with_different_values(){
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }
}
```
`test result: FAILED`

### 使用缓存器Cacher的限制
* Cacher实例假定针对不同参数arg, value 方法总会得到相同的值
* 可以用HashMap 代替单个值
    * key: arg 参数
    * value: 闭包调用的结果

* 只能接受一个`u32`类型的参数和`u32`类型的返回值
    * 可以用泛型和trait bound 来改进


## 闭包可以捕获其环境
