use std::collections::BTreeMap;

struct Page {
    session_id: &'static str,
    // 页面内按 turn_id 存放每一轮的输出。
    turns: BTreeMap<&'static str, String>,
}

impl Page {
    fn new(session_id: &'static str, turn_ids: &[&'static str]) -> Self {
        Self {
            session_id,
            turns: turn_ids.iter().map(|&id| (id, String::new())).collect(),
        }
    }
}

struct Delta {
    thread_id: &'static str,
    turn_id: &'static str,
    text: &'static str,
}

fn main() {
    // session_id 用于分组；根 thread 的 ID 与 session_id 相同。
    let session_id = "root-1";
    // 两个不同的 thread，各有一个页面；主页面有两轮。
    let mut pages = BTreeMap::from([
        ("root-1", Page::new(session_id, &["turn-a", "turn-b"])),
        ("test-agent", Page::new(session_id, &["turn-c"])),
    ]);

    // 模拟来自不同聊天页和轮次的增量交错到达。
    let events = [
        Delta {
            thread_id: "root-1",
            turn_id: "turn-a",
            text: "读取代码。",
        },
        Delta {
            thread_id: "test-agent",
            turn_id: "turn-c",
            text: "测试通过。",
        },
        Delta {
            thread_id: "root-1",
            turn_id: "turn-a",
            text: "完成修复。",
        },
        Delta {
            thread_id: "root-1",
            turn_id: "turn-b",
            text: "补了边界用例。",
        },
    ];

    for event in events {
        let page = pages.get_mut(event.thread_id).expect("unknown thread");
        let output = page.turns.get_mut(event.turn_id).expect("unknown turn");
        output.push_str(event.text);
    }

    assert_eq!(pages["root-1"].session_id, pages["test-agent"].session_id);
    assert_eq!(pages["root-1"].turns["turn-a"], "读取代码。完成修复。");
    assert_eq!(pages["root-1"].turns["turn-b"], "补了边界用例。");
    assert_eq!(pages["test-agent"].turns["turn-c"], "测试通过。");

    for (thread_id, page) in &pages {
        for (turn_id, output) in &page.turns {
            println!(
                "session={} | thread={thread_id} | turn={turn_id} | {output}",
                page.session_id
            );
        }
    }
}
