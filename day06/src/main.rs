use std::collections::BTreeMap;

type TurnKey = (&'static str, &'static str); // (threadId, turnId)
type RequestId = u64;

const A: TurnKey = ("date-parser", "turn-a");
const B: TurnKey = ("date-parser", "turn-b");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Running,
    Completed,
    Interrupted,
    Failed,
}

#[derive(Default)]
struct Ui {
    turns: BTreeMap<TurnKey, Status>,
    pending_stop: BTreeMap<RequestId, TurnKey>,
}

impl Ui {
    fn register_stop(&mut self, id: RequestId, turn: TurnKey) {
        assert_eq!(self.turns.get(&turn), Some(&Status::Running));
        assert!(!self.pending_stop.contains_key(&id));
        self.pending_stop.insert(id, turn);
    }

    // Result ({}) or RPC error settles only the matching request.
    // A real client can surface the error separately from the turn card.
    fn settle_stop_request(&mut self, id: RequestId) -> Option<TurnKey> {
        self.pending_stop.remove(&id)
    }

    // Notification: its routing key is not the JSON-RPC request ID.
    fn turn_completed(&mut self, turn: TurnKey, status: Status) {
        assert_ne!(status, Status::Running);
        if let Some(card) = self.turns.get_mut(&turn) {
            *card = status;
        }
    }

    fn stopping(&self, turn: TurnKey) -> bool {
        self.turns.get(&turn) == Some(&Status::Running)
            && self.pending_stop.values().any(|pending| *pending == turn)
    }
}

fn date_parser() -> Ui {
    let mut ui = Ui::default();
    ui.turns.insert(A, Status::Running);
    ui
}

fn main() {
    for status in [Status::Completed, Status::Interrupted, Status::Failed] {
        let mut ui = date_parser();
        ui.register_stop(2, A);
        let _ = ui.settle_stop_request(2);
        println!("Stop #2 => {{}}; card = {:?}", ui.turns[&A]);
        ui.turn_completed(A, status);
        println!("turn/completed => {:?}; card = {:?}", status, ui.turns[&A]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_empty_ack_does_not_choose_the_terminal_status() {
        for status in [Status::Completed, Status::Interrupted, Status::Failed] {
            let mut ui = date_parser();
            ui.register_stop(2, A);
            assert!(ui.stopping(A));
            assert_eq!(ui.settle_stop_request(2), Some(A)); // success {}
            assert_eq!(ui.turns[&A], Status::Running);
            ui.turn_completed(A, status);
            assert_eq!(ui.turns[&A], status);
        }
    }

    #[test]
    fn a_late_response_cannot_override_a_terminal_card() {
        let mut ui = date_parser();
        ui.register_stop(2, A);
        ui.turn_completed(A, Status::Completed);
        assert!(!ui.stopping(A));
        assert_eq!(ui.settle_stop_request(2), Some(A));
        assert_eq!(ui.turns[&A], Status::Completed);
    }

    #[test]
    fn request_error_does_not_end_a_turn_and_event_for_a_does_not_change_b() {
        let mut ui = date_parser();
        ui.register_stop(2, A);
        assert_eq!(ui.settle_stop_request(2), Some(A)); // RPC error
        assert_eq!(ui.turns[&A], Status::Running);
        ui.turn_completed(A, Status::Completed);
        ui.turns.insert(B, Status::Running);
        ui.register_stop(4, B);
        ui.turn_completed(A, Status::Completed);
        assert!(ui.stopping(B));
        assert_eq!(ui.turns[&B], Status::Running);
    }
}
