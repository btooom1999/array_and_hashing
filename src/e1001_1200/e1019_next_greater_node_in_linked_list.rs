#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn dfs(
    head: Option<Box<ListNode>>,
    stack: &mut Vec<i32>,
    result: &mut Vec<i32>,
) {
    if let Some(node) = head {
        dfs(node.next, stack, result);
        while stack.last().is_some_and(|&last| last <= node.val) {
            stack.pop();
        }
        result.push(*stack.last().unwrap_or(&0));
        stack.push(node.val);
    }
}

fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    dfs(head, &mut vec![], &mut res);
    res.reverse();

    res
}

pub fn main() {
    let head = [2,1,5].to_vec();
    println!("{:?}", next_larger_nodes(vec_to_list(head)));
}
