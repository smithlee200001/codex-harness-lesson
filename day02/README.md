# Day 02：从 App Server 到 Core，启动或追加输入

从仓库根目录运行：

```sh
cargo run --manifest-path day02/Cargo.toml
cargo test --manifest-path day02/Cargo.toml
```

预期依次看到 `Started(1)`、`Steered(1)`、第一轮收集的两条输入，以及 `Started(2)`。示例仅用标准库。它用同步状态机模拟 ID 变化；没有模拟真实任务队列、并发、拒绝条件或事件顺序，不能把 Steer 当成 RPC 幂等。

源码对照（固定 commit `3e5f072`）：

- [App Server 把输入交给 Core](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/request_processors/turn_processor.rs#L722-L773)
- [成功时返回对应的 Turn ID](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/src/request_processors/turn_processor.rs#L789-L804)
- [Core 尝试 Steer，NoActiveTurn 才进入启动路径](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn_input.rs#L461-L535)
