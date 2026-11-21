const TURN_ID: &str = "turn-a";
const USER_INPUT: &str = "用户：实现日期解析";
const TOOL_CALL: &str = "工具调用：读取日期解析代码";
const TOOL_RESULT: &str = "工具结果：当前实现没有检查闰年";

#[derive(Debug, PartialEq, Eq)]
struct ModelRequest {
    turn_id: &'static str,
    input: Vec<&'static str>,
}

fn simulate_turn(first_reply_requests_tool: bool) -> Vec<ModelRequest> {
    let mut history = vec![USER_INPUT];
    let mut requests = vec![ModelRequest {
        turn_id: TURN_ID,
        input: history.clone(),
    }];

    if first_reply_requests_tool {
        history.extend([TOOL_CALL, TOOL_RESULT]);
        requests.push(ModelRequest {
            turn_id: TURN_ID,
            input: history,
        });
    }

    requests
}

fn main() {
    for (index, request) in simulate_turn(true).iter().enumerate() {
        println!("{}/request-{}: {:?}", request.turn_id, index + 1, request.input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_request_keeps_the_turn_and_sees_the_tool_result() {
        let requests = simulate_turn(true);
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].turn_id, requests[1].turn_id);
        assert_eq!(requests[0].input, vec![USER_INPUT]);
        assert_eq!(requests[1].input, vec![USER_INPUT, TOOL_CALL, TOOL_RESULT]);
    }

    #[test]
    fn final_reply_without_other_work_needs_only_one_request() {
        assert_eq!(simulate_turn(false).len(), 1);
    }
}
