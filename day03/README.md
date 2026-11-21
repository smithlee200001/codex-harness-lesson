# Day 03：一个 Turn 如何发出第二次模型请求

从仓库根目录运行（仅使用 Rust 标准库）：

```sh
cargo build --manifest-path day03/Cargo.toml
cargo run --manifest-path day03/Cargo.toml
cargo test --manifest-path day03/Cargo.toml
```

示例模拟「实现日期解析」：模型先请求读取现有代码；工具成功返回后，Harness 把调用与结果放入历史，再用同一个 Turn ID 构造下一次请求。测试还覆盖反例：第一次就返回最终回答、没有待处理工作时，只发一次请求。

这是同步教学模拟，不联网、不调用真实工具。打印的是用于说明的逻辑历史条目，不是实际 API 请求体。示例省略了流式输出、异常、用户追加入队、Hooks 和上下文压缩。

固定源码版本 `3e5f072`：

1. [识别工具调用并标记需要继续](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/stream_events_utils.rs#L351-L421)
2. [当前采样步骤排空在途工具结果](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L3849-L3865)，[把结果记入历史](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L2904-L2924)
3. [判断是否继续以及下一次请求如何读取历史](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn.rs#L647-L724)
