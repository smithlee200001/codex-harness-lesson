# Day 09：Steer 超时后用历史回读核对原气泡

固定版本：`openai/codex@3e5f072ca1e3e75e3151656d411d094489e3faa2`。

## 源码证据

同一个 [`thread_read.rs` 测试](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server/tests/suite/v2/thread_read.rs#L2197-L2344)：测试先写入没有客户端 ID 的 `steer-1`，再以同一个服务端 Item ID 写入带 `client_id` 的条目；它断言 `thread/read`（`includeTurns: true`）返回的 `turn-1` 包含带客户端 ID 的那一条。它是构造历史再读回的测试，不是实际丢失 RPC 响应的端到端实验。

## GPUI 客户端设计建议

Steer 的 RPC 超时意味着提交待确认。如果回读在目标 Thread 和 Turn 下找到了同一客户端消息 ID，就更新已有气泡，记录服务端 Item ID。回读确认的是消息已被记录，不是模型已经读取。如果当前快照没有匹配，不要推断请求一定失败，不要自动重发或开启新 Turn。Thread/Turn 的完成状态独立处理。

## 运行

在仓库根目录执行：

```sh
cargo build --manifest-path day09/Cargo.toml
cargo run --manifest-path day09/Cargo.toml
cargo test --manifest-path day09/Cargo.toml
```

示例不依赖外部 crate，不连接 Codex、不使用网络，也不模拟真实传输或分页。示例的测试只覆盖 UI 关联规则。
