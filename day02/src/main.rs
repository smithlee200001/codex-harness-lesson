#[derive(Debug, PartialEq, Eq)]
enum Submission {
    Started(u64),
    Steered(u64),
}

#[derive(Default)]
struct Thread {
    active_turn: Option<u64>,
    next_id: u64,
    pending_input: Vec<String>,
}

impl Thread {
    fn start_or_steer(&mut self, input: &str) -> Submission {
        self.pending_input.push(input.to_owned());
        if let Some(id) = self.active_turn {
            return Submission::Steered(id);
        }

        self.next_id += 1;
        self.active_turn = Some(self.next_id);
        Submission::Started(self.next_id)
    }

    fn finish(&mut self) -> Vec<String> {
        self.active_turn = None;
        std::mem::take(&mut self.pending_input)
    }
}

fn main() {
    let mut thread = Thread::default();
    println!("{:?}", thread.start_or_steer("修复日期格式"));
    println!("{:?}", thread.start_or_steer("注意闰年边界"));
    println!("第一轮接收的输入：{:?}", thread.finish());
    println!("{:?}", thread.start_or_steer("补充测试说明"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steering_keeps_the_current_turn_id() {
        let mut thread = Thread::default();
        assert_eq!(thread.start_or_steer("修复"), Submission::Started(1));
        assert_eq!(thread.start_or_steer("注意边界"), Submission::Steered(1));
        assert_eq!(thread.finish(), vec![String::from("修复"), String::from("注意边界")]);
        assert_eq!(thread.start_or_steer("补测试"), Submission::Started(2));
    }
}
