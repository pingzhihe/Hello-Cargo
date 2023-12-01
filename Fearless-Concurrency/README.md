# 无畏并发
## 并发
* Concurrent: 程序的不同部分之间独立的执行
* Parallel: 程序的不同部分之间同时执行

* Rust 无畏并发: 允许你编写没有细微Bug的代码, 并在不引入新bug的情况下易于重构

* 注意: 本章的内容泛指concurrent和parallel

## 使用线程来同时运行代码
### 进程与线程
* 在大部分OS里, 代码运行在进程(process)中, OS 同时管理多个进程
* 在你的程序里, 各部分可以分别同时运行, 运行这些独立部分的就是线程(thread)

* 多线程运行: 
    * 提升性能表现
    * 增加复杂性, 无法保障各线程的执行顺序

### 多线程可导致的问题
* 竞争状态, 线程以不一致的顺序访问数据或资源
* 死锁, 两个线程彼此等待对方使用完其所拥有的资源, 线程无法继续
* 只在某些情况下发生的bug, 很难可靠地复制现象和修复

### 实现线程的方式
* 通过调用OS的API来创建线程1:1模型
    * 需要比较小的运行时
* 语言自己实现的线程(绿色线程)M:N模型
    * 需要更大的运行时

* Rust: 需要权衡运行时的支持
* Rust 标准库仅提供1:1模型的线程

### 通过spawn创建新线程
* 使用thread::spawn函数来创建新线程
    * 参数: 一个闭包(在新线程中运行的代码)
```
use std::thread;
use std::time::Duration;


fn main() {
    thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
### 通过join Handle来等待所有线程完成
