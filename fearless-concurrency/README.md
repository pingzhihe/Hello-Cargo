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
* thread::spawn 函数的返回值是 JoinHandle
* JoinHandle 持有值的所有权
    * 调用其join方法, 可以等待对应的其它线程的完成
* join方法: 调用handle 的join 方法会阻止当前运行的线程的执行,直到handle 所表示的这些线程的终结

### 使用move闭包
* move 闭包通常和thread::spawn一起使用, 它允许你使用其它线程的数据
* 在创建线程的时候, 把值的所有权从一个线程转移到另一个线程
```
use std::thread;
fn main() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||  {
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();

}

```

## 使用消息传递来在线程间转移数据
### 消息传递
* 一种很流行且能保证安全并发的技术就说: 消息传递
    * 线程(或Actor)通过彼此发送消息(数据)来进行通信

* Go 语言的名言: 不要用共享内存来通信, 用通信来共享内存

* Rust: Channel (标准库提供)

### Channel 
* Channel 包含: 发送端,接收端
* 调用发送端的方法, 发送数据
* 接收端会检查和接收到达的数据
* 如果发送端, 接受端中任意一端被丢弃了, 那么Channel就关闭了
 
### 创建新的Channel
* 使用`mpsc::channel`函数创建一个新的channel
    * mpsc表示multiple producer, single consumer(多个生产者, 一个消费者)
    * 返回一个tuple(元组): 里面分别是发送端和接收端
