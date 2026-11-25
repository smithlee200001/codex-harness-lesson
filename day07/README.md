# Day 07 · steer 已入队，第一次模型请求读到了吗？

模拟一个普通、非空输入的 Turn：在第一次取模型输入快照之前，`turn-a` 已接受补充消息。这个 Rust 样例只表示两条用户输入在逻辑快照中的位置；不复刻完整 Prompt、JSON-RPC、线程竞态或网络请求。

从仓库根目录运行（仅标准库）：

```sh
cargo build --offline --manifest-path day07/Cargo.toml
cargo run --offline --manifest-path day07/Cargo.toml
cargo test --offline --manifest-path day07/Cargo.toml
```

## 固定源码：Codex `3e5f072`

1. [Turn 初始化](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L454-L464)：初始输入非空时，`can_drain_pending_input` 为 false，先经过 hooks 并记录初始输入。
2. [待处理队列检查](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L535-L572) · [历史快照](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L647-L665)：首轮跳过待处理队列，之后才克隆历史交给采样。两条新消息里，第一次只包含初始输入；实际采样还有适用的历史和其他上下文。
3. [采样后的继续条件](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L695-L718)：成功采样后可以在下一轮检查并取出待处理输入。即使模型本身不要求继续，队列里有待处理内容也能触发后续步骤。普通路径下，仍是同一个 Turn；下一次取出还受预算、hooks、压缩分支等约束。
4. [上游 mock 夹具](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/tests/suite/v2/turn_steer.rs#L403-L445) 验证后续模型请求包含 steer，不断言「早到的 steer 一定不在首次请求」这个时序；首个快照的解释来自上述 Rust 路径。

## 对 GPUI 的启发

可以立即显示一条已提交的补充消息，但不要把 steer RPC 成功标成「模型已读」。示例不覆盖空输入、重试、中断或压缩；归档时仅检查源码与文件一致性，未编译或执行测试。
