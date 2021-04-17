# Report

2018011365 张鹤潇

### 编程作业

我在模板代码的基础上，于内核中实现了 `Mails` 结构体和读写邮件的系统调用。`Mails` 结构体实现为最大容量 16 的队列，由进程的 PCB 维护。读邮件时，先检查是否有邮件可读，如果有，就从队列头获取报文；写邮件的流程类似。实现 `mail_read/write` 系统调用时，注意 PCB mutex 的获取状态，不要造成死锁。

为了保证前向兼容，我将前五章实现的系统调用也移植到内核中。特别的，在`fork` 时，我将父进程的 `Mails` 复制给子进程。

### 问答题

#### 1

管道运算符 `|` 的实现用到了 pipe.

```sh
cat log.txt | grep 'word'
```

将 `|` 运算符前一个程序的 `stdout 文件描述符` 修改为到后一个程序的 pipe，后一个程序的 `stdin 文件描述符` 修改为来自前一个程序的 pipe，就可以实现管道运算符了。

#### 2

1. 多核情况下，如果不加互斥锁，两个消费者同时向一个生产者提出订单时可能产生问题，一个例子如下：

   ```
   消费者 A 发起系统调用
   检查生产者 C 邮箱是否已满
   C 邮箱未满				
   									消费者 B 发起系统调用
   									检查生产者 C 邮箱是否已满
   									C 邮箱未满
   向生产者 C 提交订单
   现在 C 的邮箱满了
   									向生产者 C 提交订单
   									但 C 的邮箱已满，B 的请求失败
   ```

   单核情况下，如果内核态下会发生时钟中断并切换进程，上述情况仍可能发生。而如果内核态下不会切换进程，系统调用期间不会被时钟中断打断，那邮箱的读写就不会有数据冲突问题。

2. 用信号量解决读者-写者问题，实现为读者优先，伪代码如下：

   ```python
   def mails_write():
       WriteMutex.P()
       write()
       WriteMutex.V()
       
   def mails_read():
       ReaderCntMutex.P()
       if ReaderCnt == 0:	# 第一个读者开启锁，防止读写冲突
           WriteMutex.P()
       ReaderCnt += 1
       ReaderCntMutex.V()
       
       read()
       
       ReaderCntMutex.P()
     	ReaderCnt -= 1
       if ReaderCnt == 0:	# 最后一个读者释放锁，现在写者可以写入了
           WriteMutex.V()
       ReaderCntMutex.V()
   ```

   其中 `WriteMutex` 是控制读写操作的信号量，`ReaderCntMutex` 是保护读者计数器 `ReaderCnt` 的信号量。

3. 为每封邮件都维护一个 `WriteMutex`，`ReaderCnt` 和 `ReaderCntMutex`。读写时加锁的粒度从整个邮箱队列减小到当前邮件，有效减少等待互斥锁的开销。

### 感想

第二道思考题又好难啊，不会做。