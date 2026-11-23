# Day 05 · Turn 收尾与迟到的 steer

在仓库根目录执行：

```sh
cargo build --manifest-path day05/Cargo.toml
cargo run --manifest-path day05/Cargo.toml
cargo test --manifest-path day05/Cargo.toml
```

回到日期解析的 `turn-a`：最后一段回复已显示，用户补一句「注意闰年边界」。以显式 `turn/steer` 携带 `expectedTurnId`，只允许补给同一轮。

- **steer 先取得准入锁**，且收尾时仍满足继续条件：输入进入队列；运行器可用同一 Turn 上下文继续。
- **收尾先移除当前任务**：若还没有另一轮在执行，显式 steer 返回无活动 Turn，即使 GPUI 端尚未处理 `turn/completed`。如果另一个 Turn 已活动，通常会得到 ID 不匹配；不应自动改发 `turn/start`。
- GPUI 可保留未提交的草稿，旧轮状态继续按匹配的终态事件对齐，而不是把准入失败等同于旧轮的最终状态。

这是无依赖、无并发、无网络的 Rust 演示。两种次序由代码显式安排，不复现实际异步调度。继续分支只演示成功的 regular task、允许向当前轮投递输入、没有取消或其他终止条件的情形；实际源码还检查上下文身份、采样中断、终态错误和工作预算。

## 固定源码

- [Core：显式 steer 获取 active_turn 锁并检查目标 ID](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn_input.rs#L1000-L1046)
- [Core：收尾时原子检查待处理输入与移除任务](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/tasks/mod.rs#L781-L837)
- [运行器：用同一个 TurnContext 再进入 RegularTask](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/tasks/mod.rs#L497-L515)
- [Core：正常路径的终态事件发布](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/tasks/mod.rs#L1047-L1097)

两种次序的协议结果是从上述源码推导的，不是实际网络到达顺序的保证。
