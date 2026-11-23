const TURN: &str = "turn-a";
const FOLLOW_UP: &str = "注意闰年边界";

#[derive(Debug, PartialEq, Eq)]
enum SteerReply {
    Accepted,
    NoActiveTurn,
    ExpectedTurnMismatch,
}

#[derive(Debug, PartialEq, Eq)]
enum Finalize {
    ContinueSameTurn(&'static str),
    CloseAdmission,
}

struct Harness {
    active_turn: Option<&'static str>,
    pending: Vec<&'static str>,
}

impl Harness {
    fn new() -> Self {
        Self { active_turn: Some(TURN), pending: Vec::new() }
    }

    fn steer(&mut self, expected: &str, text: &'static str) -> SteerReply {
        match self.active_turn {
            None => SteerReply::NoActiveTurn,
            Some(actual) if actual != expected => SteerReply::ExpectedTurnMismatch,
            Some(_) => {
                self.pending.push(text);
                SteerReply::Accepted
            }
        }
    }

    // Demo assumes a successful regular task, an open mailbox and no interrupt.
    // An exclusive &mut models each operation as one deterministic critical section.
    fn finalize(&mut self) -> Finalize {
        if let Some(id) = self.active_turn {
            if !self.pending.is_empty() {
                return Finalize::ContinueSameTurn(id);
            }
        }
        self.active_turn = None;
        Finalize::CloseAdmission
    }
}

fn main() {
    let mut a = Harness::new();
    let reply = a.steer(TURN, FOLLOW_UP);
    println!("A. steer 先到: {reply:?}; {:?}", a.finalize());

    let mut b = Harness::new();
    let decision = b.finalize();
    println!(
        "B. 收尾先到: {decision:?}; {:?}（终态事件还没送到前端）",
        b.steer(TURN, FOLLOW_UP)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitted_input_keeps_the_same_turn() {
        let mut harness = Harness::new();
        assert_eq!(harness.steer(TURN, FOLLOW_UP), SteerReply::Accepted);
        assert_eq!(harness.finalize(), Finalize::ContinueSameTurn(TURN));
        assert_eq!(harness.pending, vec![FOLLOW_UP]);
    }

    #[test]
    fn removal_closes_admission_before_delivery() {
        let mut harness = Harness::new();
        assert_eq!(harness.finalize(), Finalize::CloseAdmission);
        assert_eq!(harness.steer(TURN, FOLLOW_UP), SteerReply::NoActiveTurn);
        assert!(harness.pending.is_empty());
    }

    #[test]
    fn explicit_steer_cannot_admit_input_for_another_turn() {
        let mut harness = Harness::new();
        assert_eq!(harness.steer("turn-b", FOLLOW_UP), SteerReply::ExpectedTurnMismatch);
        assert!(harness.pending.is_empty());
    }
}
