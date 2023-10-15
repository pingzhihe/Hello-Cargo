# 所有权 ownership
所有权是rust最独特的特性，可以让rust无需gc(garbage collecter)就能保证内存安全。
### Stack vs Heap
Stack 和 Heap 都是可用的内存，但是结构差别很大

## 所有权规则
* 每个值都有一个变量，这个变量是这个值改的所有者
* 每个值同时只能有一个所有者
* 当所有者离开作用域(scope), 这个值就会被删除。

## 变量作用域

