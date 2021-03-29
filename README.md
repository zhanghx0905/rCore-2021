# my-rCore

2021 Spring, OS Assignment.

### dev-ch8

- [x] 在 testcases 的基础上修改用户模块，以移植 tutorial 仓库 ch7 分支上的的用户程序。

- [x] 修改 `spawn` 接口，支持参数传递。

  详见 [process.rs](./os/src/syscall/process.rs)

- [x] 重写 `user_shell`，在 `<` `>` 的基础上，支持 `|` 运算符。Refer to [bubble shell](https://github.com/JoshMcguigan/bubble-shell).

  详见 [user_shell.rs](user/src/bin/user_shell.rs)

  简单的测试：

  ```bash
  hello_world | getchar
  hello_world | getchar | getchar
  ```

  或者

  ```bash
  hello_world > file1
  cat file1 | getchar
  ```

### 存在的问题

已知存在如下功能实现不完整或不合理的问题：

- lab4 `removed_framed_area`，没有操作 `Vec<MapArea>`，将在引入 `fork` 后带来问题。 
- lab5 `spawn` 简单实现为 `fork+exec`，不高效。
- 用 `transform_ref_mut` 为结构体赋值没有考虑跨页的情况。

如果测例不改变就不修复了，在此记录。

## References

[rCore-Tutorial-Book 第三版](https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html)

[rCore-Tutorial-v3](https://github.com/rcore-os/rCore-Tutorial-v3)

[rCore_tutorial_tests](https://github.com/DeathWish5/rCore_tutorial_tests)
