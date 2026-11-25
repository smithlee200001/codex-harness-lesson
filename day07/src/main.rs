const TURN_A: &str = "turn-a";
const INITIAL: &str = "实现日期解析";
const STEER: &str = "注意闰年边界";

#[derive(Debug)]
struct ModelInput {
    turn_id: &'static str,
    user_messages: Vec<&'static str>,
}

// Deliberately models only the ordinary, nonempty-input Turn.
struct Turn {
    id: &'static str,
    history: Vec<&'static str>,
    pending: Vec<&'static str>,
    can_drain_pending_input: bool,
}

impl Turn {
    fn start(id: &'static str, initial: &'static str) -> Self {
        assert!(!initial.is_empty());
        Self {
            id,
            history: vec![initial],
            pending: Vec::new(),
            can_drain_pending_input: false,
        }
    }

    fn steer(&mut self, expected: &str, text: &'static str) -> Result<&'static str, &'static str> {
        if self.id != expected {
            return Err("wrong Turn");
        }
        self.pending.push(text);
        Ok(self.id)
    }

    fn prepare_model_input(&mut self) -> ModelInput {
        if self.can_drain_pending_input {
            self.history.append(&mut self.pending);
        }
        ModelInput {
            turn_id: self.id,
            user_messages: self.history.clone(),
        }
    }

    // This models a successful step, with the budget open and hooks allowing input.
    fn finish_sample(&mut self, model_needs_follow_up: bool) -> bool {
        self.can_drain_pending_input = true;
        model_needs_follow_up || !self.pending.is_empty()
    }
}

fn main() {
    let mut turn = Turn::start(TURN_A, INITIAL);
    let accepted = turn.steer(TURN_A, STEER).unwrap();
    println!("steer accepted: {accepted}");
    let first = turn.prepare_model_input();
    println!("sample 1 ({}): {:?}", first.turn_id, first.user_messages);
    if turn.finish_sample(false) {
        let second = turn.prepare_model_input();
        println!("sample 2 ({}): {:?}", second.turn_id, second.user_messages);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_before_first_snapshot_is_still_queued() {
        let mut turn = Turn::start(TURN_A, INITIAL);
        assert_eq!(turn.steer(TURN_A, STEER), Ok(TURN_A));
        let first = turn.prepare_model_input();
        assert_eq!(first.turn_id, TURN_A);
        assert_eq!(first.user_messages.as_slice(), &[INITIAL]);
        assert_eq!(turn.pending.as_slice(), &[STEER]);
        assert!(turn.finish_sample(false));

        let second = turn.prepare_model_input();
        assert_eq!(second.turn_id, first.turn_id);
        assert_eq!(second.user_messages.as_slice(), &[INITIAL, STEER]);
        assert_eq!(first.user_messages.as_slice(), &[INITIAL]); // snapshot unchanged
        assert!(turn.pending.is_empty());
    }

    #[test]
    fn no_follow_up_when_nothing_is_pending_and_model_is_done() {
        let mut turn = Turn::start(TURN_A, INITIAL);
        let _first = turn.prepare_model_input();
        assert!(!turn.finish_sample(false));
    }
}
