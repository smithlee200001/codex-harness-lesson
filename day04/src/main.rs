use std::collections::BTreeMap;

type TurnKey = (&'static str, &'static str);
type ItemKey = (TurnKey, &'static str);

enum Event {
    Delta(ItemKey, &'static str),
    ItemCompleted(ItemKey, &'static str),
    TurnCompleted(TurnKey),
}

#[derive(Default)]
struct View {
    messages: BTreeMap<ItemKey, String>,
    turns: BTreeMap<TurnKey, &'static str>,
}

impl View {
    fn apply(&mut self, event: Event) {
        match event {
            Event::Delta(key, text) => self.messages.entry(key).or_default().push_str(text),
            Event::ItemCompleted(key, full_text) => {
                self.messages.insert(key, full_text.to_owned());
            }
            Event::TurnCompleted(key) => {
                self.turns.insert(key, "已完成");
            }
        }
    }
}

const TURN: TurnKey = ("thread-1", "turn-a");
const MESSAGE: ItemKey = (TURN, "msg-1");

fn streaming_view() -> View {
    let mut view = View::default();
    view.turns.insert(TURN, "正在执行");
    view.apply(Event::Delta(MESSAGE, "你"));
    view.apply(Event::Delta(MESSAGE, "好"));
    view
}

fn main() {
    let mut view = streaming_view();
    println!("delta: {}", view.messages[&MESSAGE]);
    view.apply(Event::ItemCompleted(MESSAGE, "你好!"));
    println!("item/completed: {} | {}", view.messages[&MESSAGE], view.turns[&TURN]);
    view.apply(Event::TurnCompleted(TURN));
    println!("turn/completed: {} | {}", view.messages[&MESSAGE], view.turns[&TURN]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_item_is_reconciled_without_finishing_its_turn() {
        let mut view = streaming_view();
        assert_eq!(view.messages[&MESSAGE], "你好");
        view.apply(Event::ItemCompleted(MESSAGE, "你好!"));
        assert_eq!(view.messages.len(), 1);
        assert_eq!(view.messages[&MESSAGE], "你好!");
        assert_eq!(view.turns[&TURN], "正在执行");

        let other_turn = ("thread-1", "turn-b");
        view.apply(Event::TurnCompleted(other_turn));
        assert_eq!(view.turns[&TURN], "正在执行");

        view.apply(Event::TurnCompleted(TURN));
        assert_eq!(view.turns[&TURN], "已完成");
    }
}
