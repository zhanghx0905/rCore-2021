# Report

2018011365 张鹤潇

### 编程作业

我以模板代码为基础，面向测例进一步开发：

- 按测例标准实现 `sys_gettime` 系统调用，将当前时间不是以返回值传递，而是存放在用户程序传入的 `*mut TimeVal` 指针中。
- 为每个用户程序设置时间片上线，若超出上限，就令用户程序 `Exit`.
- 将调度算法从 RR 修改为 stride：用 `buddy_system_allocator` 库实现了全局堆分配器，用 `BinaryHeap` 高效的实现了 stride 算法，并增加 `sys_set_priority` 系统调用以设置用户程序优先级。

### 简答题

#### 1.

切换进程：在用户程序当前时间片用尽，或主动调用 `sys_yield` 时切换进程；

进程调度：本章 tutorial 的进程调度策略是 RR 算法，在编程作业中我实现了 stride 算法。

- RR 算法：OS 维护一个队列, 每次将时间片用尽的进程放入队尾，取出队头的第一个就绪任务运行。
- stride 算法：为每个进程维护一个优先级和一个 stride 值, 每次选择 stride 最小的进程来运行, 并令该进程的 stride 加上 `BIG_STRIDE/优先级` 。这里的 `BIG_STRIDE` 是一个大数。

处理新进程：在本章中没有新进程。

#### 2.

##### 1.

- 本章 Rust 和 C 的调度策略都是 RR 算法，没有不同。
- 如果考虑新进程的产生，则 C 的实现可能导致进程的运行顺序与产生顺序不一致，详细分析见下一问。

##### 2.

| 进程队列        | 事件                 |
| --------------- | -------------------- |
| p1 p2 p3        | p1 p2 p3 产生        |
| **p1** p2 p3    | 执行 p1              |
| p1 **p2** p3    | 执行 p2，p2 结束     |
| p1 p4 **p3** p5 | p4，p5 产生，执行 p3 |
| p1 p4 p3 **p5** | 执行 p5              |

在这种情况下，`p4`、`p5` 的产生顺序与其执行顺序不一致。

在我的 stride 调度算法下，运行顺序是 `P1 P2 P3 P4 P5 P1 P3`，在 `p4`, `p5` stride 相同时，选择编号较小的 `p4` 执行。

#### 3.

##### 1.

不是，仍是 p2 执行，因为 p2.stride 溢出了。

##### 2.

反证如下：

假设在某个时间片 $t_k$ 首次出现 $p_1.stride(t_k)-p_2.stride(t_k)>\text{BIG_STRIDE}/2$. 

则上一个时间片必然运行 $p_1$, 否则在上个时间片这种情况就会出现。

注意到 $p_1.pass\le \text{BIG_STRIDE}/2$，
$$
\begin{align}
p_1.stride(t_{k-1})&=p_1.stride(t_k)-p1.pass\\
&\ge p_1.stride(t_k) - \text{BIG_STRIDE}/2\\
&>p_2.stride(t_k) = p_2.stride(t_{k-1})
\end{align}
$$
这表明上一个时间片 $p_2$ 的优先级更高，推出矛盾，故欲证结论成立。

##### 3.

Rust 的 `BinaryHeap` 是大顶堆，为了将其以小顶堆使用，应该逆向比较元素大小。

```rust
struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if max(self.0, other.0) - min(self.0, other.0) <= BigStride / 2{
            other.0.partial_cmp(&self.0)
        }else{
            self.0.partial_cmp(&other.0) 
        }
    }
}
```

