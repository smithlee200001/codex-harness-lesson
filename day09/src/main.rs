#[derive(Debug, PartialEq, Eq)]
struct Bubble<'a> {
    thread_id: &'a str,
    turn_id: &'a str,
    client_id: &'a str,
    server_item_id: Option<&'a str>,
}

struct HistoryItem<'a> {
    thread_id: &'a str,
    turn_id: &'a str,
    client_id: Option<&'a str>,
    server_item_id: &'a str,
}

fn reconcile<'a>(bubble: &mut Bubble<'a>, history: &[HistoryItem<'a>]) {
    if let Some(item) = history.iter().find(|item| {
        item.thread_id == bubble.thread_id
            && item.turn_id == bubble.turn_id
            && item.client_id == Some(bubble.client_id)
    }) {
        bubble.server_item_id = Some(item.server_item_id);
    }
}

fn pending_bubble() -> Bubble<'static> {
    Bubble {
        thread_id: "thread-1",
        turn_id: "turn-a",
        client_id: "msg-17",
        server_item_id: None,
    }
}

fn item(turn_id: &'static str) -> HistoryItem<'static> {
    HistoryItem {
        thread_id: "thread-1",
        turn_id,
        client_id: Some("msg-17"),
        server_item_id: "server-item-42",
    }
}

fn main() {
    let mut bubble = pending_bubble();
    println!("RPC 超时：提交状态待确认");
    reconcile(&mut bubble, &[item("turn-b")]);
    println!("目标 Turn 未匹配：{:?}", bubble.server_item_id);
    reconcile(&mut bubble, &[item("turn-a")]);
    println!("回读匹配：原气泡已记录，服务端 Item ID = {:?}", bubble.server_item_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_history_confirms_existing_bubble() {
        let mut bubble = pending_bubble();
        reconcile(&mut bubble, &[item("turn-a")]);
        assert_eq!(bubble.server_item_id, Some("server-item-42"));
    }

    #[test]
    fn missing_or_wrong_turn_stays_pending() {
        let mut bubble = pending_bubble();
        reconcile(&mut bubble, &[]);
        reconcile(&mut bubble, &[item("turn-b")]);
        assert_eq!(bubble.server_item_id, None);
    }

    #[test]
    fn wrong_thread_or_client_id_stays_pending() {
        let mut bubble = pending_bubble();
        reconcile(
            &mut bubble,
            &[
                HistoryItem { thread_id: "thread-2", ..item("turn-a") },
                HistoryItem { client_id: Some("msg-18"), ..item("turn-a") },
            ],
        );
        assert_eq!(bubble.server_item_id, None);
    }

    #[test]
    fn incomplete_read_does_not_erase_confirmation() {
        let mut bubble = pending_bubble();
        reconcile(&mut bubble, &[item("turn-a")]);
        reconcile(&mut bubble, &[]);
        assert_eq!(bubble.server_item_id, Some("server-item-42"));
    }
}
