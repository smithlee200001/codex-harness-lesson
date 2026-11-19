use std::{cell::Cell, env};

struct TurnStart {
    prompt: String,
}

struct TurnStarted {
    turn_id: u64,
}

// Server：统一处理启动请求，分配 turn_id。
#[derive(Default)]
struct AppServer {
    next_turn_id: Cell<u64>,
}

impl AppServer {
    fn turn_start(&self, request: TurnStart) -> TurnStarted {
        let turn_id = self.next_turn_id.get() + 1;
        self.next_turn_id.set(turn_id);

        // 实际项目会在这里把任务交给 Harness 调度。
        println!("[server] 接收 turn #{turn_id}: {}", request.prompt);
        TurnStarted { turn_id }
    }
}

// 本地客户端：直接调用同进程的 Server，不走网络。
struct InProcessClient<'a>(&'a AppServer);

impl InProcessClient<'_> {
    fn start_turn(&self, prompt: String) -> TurnStarted {
        self.0.turn_start(TurnStart { prompt })
    }
}

fn cli_exec(client: &InProcessClient<'_>, prompt: String) {
    let started = client.start_turn(prompt);
    println!("[CLI] 已启动 turn #{}", started.turn_id);
}

// 模拟 GPUI 的点击回调，不需要安装 GPUI。
fn gui_on_submit(client: &InProcessClient<'_>, text: String) {
    let started = client.start_turn(text);
    println!("[GUI] 已启动 turn #{}", started.turn_id);
}

fn main() {
    let server = AppServer::default();
    let client = InProcessClient(&server);

    let prompt = env::args().nth(1).unwrap_or_else(|| "解释 main.rs".into());
    cli_exec(&client, prompt);
    gui_on_submit(&client, "给这个函数补测试".into());
}
