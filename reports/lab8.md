# Report

2018011365 张鹤潇

### 概述

我完成了拓展作业题目 14：改进 shell 程序，实现 “|” 这种经典的管道机制，代码见 [user_shell.rs](user/src/bin/user_shell.rs).

对 fork/exec/spawn 等进行扩展，支持命令行参数传递。

我还用 ch8 的新测例测试了业已完成的 OS syscall 稳定性，并修复了测试中暴露的一些问题，可以通过 ch8_01 ch8_04 ch8_05 这三个先前无法通过的测例。

### 拓展作业

为了使 shell 支持 "|" 运算符，我根据 | 字符，将输入命令分为多段，每段都代表一个用户进程，据此得出进程 IO 间的依赖关系。

```sh
hello_world | getchar | getchar
```

我将用户程序的 IO 种类区分为文件、管道和继承父进程三种，根据输入命令确定当前进程的 IO 种类。

```rust
#[derive(Debug, Clone)]
enum IOType {
    File(String),		// 文件名
    Pipe([usize; 2]),	// [read end, write end]
    Inherit,
}
```


利用 fork + exec 的灵活性，在 exec 之前，fork 之后对进程的 STDIO 进行重定向：利用 dup系统调用，将 “|” 前进程的 STDOUT 重定向为到 “|” 后进程的管道，将 “|” 后进程的 STDIN 重定向为来自前一个进程的管道。

```rust
let pid = fork();
if pid == 0 {
     match input.clone() {
         // ...
 		IOType::Pipe(pipes) => {
            close(pipes[1]); // close write end
            close(STDIN);
            // 重定向 stdin 为 pipes[0]
            assert_eq!(dup(pipes[0]) as usize, STDIN);
            close(pipes[0]);
         }
    }
}
```

这就是我的 "|" 运算符实现原理。

### 基本作业

用 ch8 的七个测例测试 OS 实现的稳定性。测试表明，我的 OS 内核运行编号为 1, 4, 5, 6, 7 的测例时会崩溃，能够杀死编号为 2, 3 的测例。

#### 对测例的分析

1, 5, 6 号测例会通过 fork 生成大量的子进程。

- 在我实现的内核中，没有对 fork 时可用内存不足的情况做错误处理。运行测例时，内核发生错误而崩溃。

4 号测例测试了文件相关系统调用的鲁棒性，包括 close, read, write, mail_write, link, unlink. 

- 我实现的内核中，没有检查文件名是否超过了最大长度，没有对 link 时文件不存在的情况做错误处理。运行测例时，内核发生错误而崩溃。

- 该测例还暴露了 user 模块实现的一个微妙错误。

  为保证输出的有序性，user 模块在 `println` 时会对缓存区加锁，

  ```rust
  pub fn print(args: fmt::Arguments) {
      let mut buf = CONSOLE_BUFFER.lock();
      buf.write_fmt(args).unwrap();
  }
  ```

  如果用户程序在关闭 STDOUT 后调用 `println`，则 `buf.write_fmt` 会触发 panic，而 `panic_handler` 中又会调用 `println`打印错误信息，再次尝试获得缓存区的锁，这将造成用户态程序死锁。

6 号测例会测试 fork 的子进程是否能继承父进程通过 mmap 申请的内存区域。

- 在我实现的内核中，fork 没有复制 mmap 生成的内存区块，因此相应功能无法得到满足。

#### 改进以通过测例

根据上述分析，我做出了如下改进：

- 对 fork 时可用内存不足的情况做错误处理， fork 失败时向用户进程返回 -1.
- 创建文件时，检查文件名是否超过文件系统允许的最大长度 (27)，如果超过，就终止系统调用并返回 -1.
- 创建硬链接时，如果文件不存在，就终止系统调用并返回 -1.

改进后，OS 可以通过第 1， 4， 5 号测例。