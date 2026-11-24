# Day 06 · Stop 请求与 Turn 终态

沿用日期解析器的 `turn-a`：前端单独订阅事件。点击 Stop 会发出一个 JSON-RPC 请求，而 Turn 卡片仍靠事件收尾。这里的 Rust demo 只演示 GPUI 可使用的状态更新逻辑，不连接真实 App Server，不运行 GUI，不复现真实线程竞态。

从仓库根目录运行（无外部依赖）：

```sh
cargo build --offline --manifest-path day06/Cargo.toml
cargo run --offline --manifest-path day06/Cargo.toml
cargo test --offline --manifest-path day06/Cargo.toml
```

## 固定源码：Codex `3e5f072`

1. [turn_interrupt_inner](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/request_processors/turn_processor.rs#L1532-L1633)：非空目标通过校验后登记等待者，关联终态 Turn ID 与请求 ID；成功提交中断操作不立即构造 RPC 回复。字面值 `turnId: ""` 是启动阶段的单独路径，提交成功即可回复。请求也可能被拒绝。
2. [终态映射与应答函数](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/bespoke_event_handling.rs#L1582-L1658)：匹配的 Core `TurnComplete` 和 `TurnAborted` 都可用相同的空对象回复 Stop；前者根据错误摘要映射到 Completed 或 Failed，后者映射到 Interrupted。公共通知均为 `turn/completed`。
3. [上游夹具](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/request_processors/turn_interrupt_tests.rs#L179-L259) 分别断言 A 的成功回复后是 Completed、B 的成功回复后是 Interrupted。阅读了夹具断言，未执行上游测试。夹具观察的是处理器出站顺序，不是所有客户端或 GUI 回调的顺序保证。

## GPUI 状态建议

- 请求状态按 JSON-RPC request ID 结算；卡片终态按 `(threadId, turnId)` 匹配通知。界面可在请求未结算时显示「正在停止」，但这不是 Turn 终态。
- `{}` 或 RPC 错误本身都不改卡片终态。终态通知先被 UI 消费时也应立即更新；迟到的 RPC 结果不能覆盖它。断连漏通知则需要另行对账，不在本例范围。
- 样例顺序驱动三种终态，另有单测覆盖迟到回复和请求错误与卡片状态分离。本次归档未运行 Cargo 或上游测试。
