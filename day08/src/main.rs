use std::collections::HashMap;

type MessageKey = (&'static str, &'static str, &'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Sending,
    Accepted,
    Recorded,
    Rejected,
}

#[derive(Default)]
struct Transcript(HashMap<MessageKey, Status>);

impl Transcript {
    fn submit(&mut self, key: MessageKey) {
        self.0.entry(key).or_insert(Status::Sending);
    }

    fn acknowledge(&mut self, key: MessageKey, accepted: bool) {
        if let Some(status) = self.0.get_mut(&key) {
            if *status == Status::Sending {
                *status = if accepted { Status::Accepted } else { Status::Rejected };
            }
        }
    }

    fn user_item_started(&mut self, key: MessageKey) {
        self.0.insert(key, Status::Recorded);
    }
}

fn main() {
    let key = ("thread-1", "turn-a", "client-steer-1");
    let mut ui = Transcript::default();
    ui.submit(key);
    println!("send  {key:?}: {:?}", ui.0[&key]);
    ui.acknowledge(key, true);
    println!("ack   {key:?}: {:?}", ui.0[&key]);
    ui.user_item_started(key);
    println!("item  {key:?}: {:?}", ui.0[&key]);

    let late = ("thread-1", "turn-a", "client-steer-2");
    ui.submit(late);
    ui.acknowledge(late, false); // Mock: no active Turn.
    println!("error {late:?}: {:?}; no new Turn", ui.0[&late]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notifications_and_ack_in_either_order_merge_into_one_message() {
        let key = ("thread-1", "turn-a", "client-steer-1");
        for event_first in [true, false] {
            let mut ui = Transcript::default();
            ui.submit(key);
            if event_first {
                ui.user_item_started(key);
                ui.acknowledge(key, true);
            } else {
                ui.acknowledge(key, true);
                assert_eq!(ui.0[&key], Status::Accepted);
                ui.user_item_started(key);
            }
            assert_eq!(ui.0.len(), 1);
            assert_eq!(ui.0[&key], Status::Recorded);
        }
    }

    #[test]
    fn rejected_steer_does_not_create_a_turn() {
        let key = ("thread-1", "turn-a", "client-steer-2");
        let mut ui = Transcript::default();
        ui.submit(key);
        ui.acknowledge(key, false);
        assert_eq!(ui.0.len(), 1);
        assert_eq!(ui.0[&key], Status::Rejected);
    }
}
