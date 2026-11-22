# Day 04 · 流式消息与 Turn 状态

在仓库根目录执行：

```sh
cargo build --manifest-path day04/Cargo.toml
cargo run --manifest-path day04/Cargo.toml
cargo test --manifest-path day04/Cargo.toml
```

## 模拟什么

同一个 `thread-1 / turn-a / msg-1` 先收到两个 delta（「你」「好」），再收到同 ID 的 `item/completed`（完整文本「你好!」），最后收到该 Turn 的 `turn/completed`。

- delta 追加到同一个消息条目。
- `item/completed` 用完整文本校准消息，不再追加；Turn 仍是「正在执行」。
- 匹配 `threadId + turnId` 的 `turn/completed` 才改变该 Turn 的状态。Demo 只演示成功状态；实际 UI 应读取事件里的状态，而不是统一显示「已完成」。

这是无依赖的 Rust 状态更新演示，不连接 Codex，不解析 JSON，不启动 GPUI。假定同一事件流按顺序消费，不模拟重连、重复或乱序通知。GPUI 版可以把 `View::apply` 放进事件订阅后的状态更新层。

## 固定源码

- [Harness：处理模型的 OutputTextDelta](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L3681-L3712)
- [App Server：转发文本 delta](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/bespoke_event_handling.rs#L1057-L1068)及[完成项](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/bespoke_event_handling.rs#L1167-L1185)
- [独立客户端 le-agent 的文本校准测试](https://github.com/smithlee200001/le-agent/blob/4b06e2a441e3d77b563aa6248e0d9562197f4c03/src/state.rs#L306-L327)，不是本课 Demo 的测试结果。
