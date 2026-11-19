use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

const SESSION_ID: &str = "root-1";

struct Turn {
    id: &'static str,
    prompt: &'static str,
}

struct Thread {
    id: &'static str,
    title: &'static str,
    turns: &'static [Turn],
}

const THREADS: &[Thread] = &[
    Thread {
        id: SESSION_ID,
        title: "主 Agent",
        turns: &[
            Turn { id: "turn-a", prompt: "修复日期解析失败的问题。" },
            Turn { id: "turn-b", prompt: "再补一个闰年的边界用例。" },
        ],
    },
    Thread {
        id: "test-agent",
        title: "测试子 Agent",
        turns: &[Turn { id: "turn-c", prompt: "主 Agent 委派：复现日期解析失败。" }],
    },
];

struct Delta {
    thread_id: &'static str,
    turn_id: &'static str,
    text: &'static str,
}

// turn-c 的事件夹在 turn-a 的两段回复之间。
const EVENTS: &[Delta] = &[
    Delta { thread_id: "root-1", turn_id: "turn-a", text: "我定位到了日期格式校验的问题。" },
    Delta { thread_id: "test-agent", turn_id: "turn-c", text: "已复现：合法的 2 月 29 日被拒绝。" },
    Delta { thread_id: "root-1", turn_id: "turn-a", text: "已修复校验逻辑，原测试通过。" },
    Delta { thread_id: "root-1", turn_id: "turn-b", text: "已补上闰年的测试，验证通过。" },
];

fn reply(events: &[Delta], thread_id: &str, turn_id: &str) -> String {
    let mut output = String::new();
    for event in events {
        if event.thread_id == thread_id && event.turn_id == turn_id {
            output.push_str(event.text);
        }
    }
    output
}

struct ChatDemo {
    thread_id: &'static str,
    revealed: usize,
}

impl Render for ChatDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let thread = THREADS.iter().find(|t| t.id == self.thread_id).unwrap();
        let visible = &EVENTS[..self.revealed];
        let last_event = visible.last().map_or_else(
            || "点击「下一条事件」，开始回放。".to_owned(),
            |e| format!("最近事件：{} / {}", e.thread_id, e.turn_id),
        );

        let sidebar = div()
            .flex().flex_col().flex_none().gap_2().w(px(210.)).p_4()
            .bg(rgb(0xe9eef8))
            .child(div().text_sm().child(format!("session_id: {SESSION_ID}")))
            .children(THREADS.iter().map(|t| {
                let id = t.id;
                let background = if id == self.thread_id { 0xd1deff } else { 0xffffff };
                div().id(id).cursor_pointer().rounded_md().p_2()
                    .bg(rgb(background))
                    .child(div().child(t.title))
                    .child(div().text_sm().child(format!("thread_id: {id}")))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.thread_id = id;
                        cx.notify();
                    }))
            }));

        let controls = div().flex().gap_2().items_center()
            .child(
                div().id("next").cursor_pointer().rounded_md().p_2()
                    .bg(rgb(0x2455ae)).text_color(rgb(0xffffff))
                    .child(if self.revealed == EVENTS.len() { "已播完" } else { "下一条事件" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.revealed = (this.revealed + 1).min(EVENTS.len());
                        cx.notify();
                    })),
            )
            .child(
                div().id("reset").cursor_pointer().rounded_md().p_2()
                    .bg(rgb(0xe9eef8)).child("重置回放")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.revealed = 0;
                        cx.notify();
                    })),
            )
            .child(format!("{} / {}", self.revealed, EVENTS.len()));

        div().flex().size_full().bg(rgb(0xf6f8fc)).text_color(rgb(0x18243a))
            .child(sidebar)
            .child(
                div().id("thread-panel").flex_1().min_w(px(0.)).overflow_y_scroll()
                    .flex().flex_col().gap_3().p_5()
                    .child(div().text_xl().child(thread.title))
                    .child(div().text_sm().child(format!("thread_id: {}", thread.id)))
                    .child(controls)
                    .child(div().text_sm().child(last_event))
                    .children(thread.turns.iter().map(|turn| {
                        let text = reply(visible, thread.id, turn.id);
                        div().flex().flex_col().flex_none().gap_2().p_3().rounded_md()
                            .bg(rgb(0xffffff)).border_1().border_color(rgb(0xe0e6ef))
                            .child(div().text_sm().text_color(rgb(0x2455ae))
                                .child(format!("turn_id: {}", turn.id)))
                            .child(format!("输入：{}", turn.prompt))
                            .child(if text.is_empty() {
                                "回复：等待事件…".to_owned()
                            } else {
                                format!("回复：{text}")
                            })
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(860.), px(560.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ChatDemo { thread_id: SESSION_ID, revealed: 0 }),
        )
        .expect("open demo window");
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaved_deltas_stay_in_their_thread_and_turn() {
        assert_eq!(
            reply(&EVENTS[..3], "root-1", "turn-a"),
            "我定位到了日期格式校验的问题。已修复校验逻辑，原测试通过。"
        );
        assert_eq!(
            reply(&EVENTS[..3], "test-agent", "turn-c"),
            "已复现：合法的 2 月 29 日被拒绝。"
        );
        assert!(reply(&EVENTS[..3], "root-1", "turn-b").is_empty());
        assert!(reply(EVENTS, "root-1", "turn-c").is_empty());
    }
}
