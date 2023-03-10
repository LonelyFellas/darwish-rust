## 1. const 和 static 的区别是：

- const 不可变，static 具有静态生命周期的变量。
- const 必须是数据类型，static 可以省略数据类型。
- const 在编译的时候内联到代码中，static 在运行时分配内存
- static 可以是可变，但是必须用 unsafe 块来修改

## 2. let 和 const 的区别是：

- let 声明的是变量，const 声明的是常量
- let 可以省略数据类型，常量必须指定数据类型
- const 在编译的时候内联到代码中，let 在运行时分配内存

## 3. unsafe 块的作用是什么？

- 修改 static 变量 比如 unsafe { COUNTER += 1};
- 调用不安全的函数或方法
- 实现不安全 trait
- 访问和修改不安全的静态应用

## 4. rust 使用变量 shadowing 有几个原因：

- 可以重用变量名，而不需要创建新的变量。
- 可以改变变量的类型，不需要强制转换
- 可以增强代码的可读性和安全性，因为遮蔽后的变量不会被意外的修改

## 5. 变量 shadowing 和 mutability 是的区别：

- 可变性是指变量是否可以变量是否可以被修改，需要 let 语句中的 mut 关键字来声明
- 遮蔽是指使用 let 语句重新声明一个已存在的变量，从而覆盖之前的值和类型
- 可变性只能改变变量的值，不能改变类型。
- 遮蔽可以改变变量的值和类型，也可以改变可变性。
- 可变性需要注意借用规则，避免数据竞争和悬垂引用。
- 遮蔽不会违反借用规则，因为它创建了一个新的绑定 123。
