# Day 08 · 补充指令如何绑定原 Turn 和原气泡？

从仓库根目录运行，只用 Rust 标准库：

```sh
cargo build --offline --manifest-path day08/Cargo.toml
cargo run --offline --manifest-path day08/Cargo.toml
cargo test --offline --manifest-path day08/Cargo.toml
```

固定上游版本：OpenAI Codex `3e5f072ca1e3e75e3151656d411d094489e3faa2`。

- [Core 转发预期 Turn ID](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/core/src/session/turn_input.rs#L722-L775)：显式 steer 把预期 Turn ID 传给 `steer_input`，本分支不调用启动新 Turn 的路径。
- [App Server 消息关联测试](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/tests/suite/v2/turn_steer.rs#L330-L388)：请求指定 `expectedTurnId` 并带 `clientUserMessageId`。测试断言 RPC 返回原 Turn ID，并在用户消息的 `item/started` 中找到同样的客户端 ID（Rust 字段 `client_id`）。
- [没有活动 Turn 的测试](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/tests/suite/v2/turn_steer.rs#L75-L101)：返回错误，而不是代替用户新开 Turn。

示例只模拟 GPUI 可用的消息归并逻辑。发送前分配客户端消息 ID，按 (Thread ID, Turn ID, 客户端消息 ID) 对齐本地和服务端气泡。RPC 成功是已受理；用户消息项出现是服务端已生成该消息项；两者都不等于模型已读。事件和响应两种到达顺序是客户端防御性处理，不是对上游时序的断言。消息 ID 也不代表协议提供重试幂等性。明确拒绝时保留文本，不自动改成 `turn/start`；网络超时不等于明确拒绝。

本例不连接 App Server 或 GPUI，不发送模型请求。归档时做了源文件回读；环境未提供 Cargo，未编译或运行。
