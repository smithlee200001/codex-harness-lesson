# Day 01：CLI、GUI 与进程内 App Server

这个独立的 Rust demo 模拟两个前端通过同一个进程内客户端向 App Server 发起 `TurnStart`。它不是 Codex 的源码，也不依赖 GPUI、网络、Tokio 或外部 crate。

```text
CLI / GUI 回调 → InProcessClient → AppServer::turn_start → TurnStarted
```

在 macOS、Linux 或 Windows PowerShell 中，从仓库根目录运行：

```sh
cd day01
cargo build
cargo run -- "解释项目结构"
```

`Cargo.toml` 将 `demo.rs` 声明为二进制入口，不需要第三方依赖。

预期 CLI 打印 `turn #1`，模拟的 GUI 回调打印 `turn #2`，因为两者共用一个 Server 实例。返回 ID 只模拟启动请求已受理，不表示任务完成。这个 demo 没有实际调度 Agent 或推送事件。

源码对照（固定 commit `3e5f072`）：[CLI 分发](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/cli/src/main.rs#L1206-L1227) · [进程内客户端](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/exec/src/lib.rs#L1106-L1115)。

## 补充示例：Session、Thread、Turn 三种 ID

从仓库根目录运行：

```sh
cargo run --manifest-path day01/Cargo.toml --example ids
```

```text
session=root-1 | thread=root-1 | turn=turn-a | 读取代码。完成修复。
session=root-1 | thread=root-1 | turn=turn-b | 补了边界用例。
session=root-1 | thread=test-agent | turn=turn-c | 测试通过。
```

`session_id` 把主 Agent 和子 Agent 分到同一组；`thread_id` 定位聊天页；`turn_id` 定位该页面中的一轮执行。示例用固定 ID 和模拟事件展示路由，不运行 Codex 或 GPUI。Codex 对照：[协议中的 Session ID](https://github.com/openai/codex/blob/3e5f072ca1e3e75e3151656d411d094489e3faa2/codex-rs/app-server-protocol/schema/json/v2/ThreadStartResponse.json#L1752-L1755)。
