#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current_node = &mut head;

    while let Some(node) = current_node {
        println!("{:?}", node);

        if let Some(ref mut next_node) = node.next {
            let gcd_value = gcd(node.val, next_node.val);
            let new_node = Box::new(ListNode {
                val: gcd_value,
                next: node.next.take(),
            });

            node.next = Some(new_node);
        }

        if node.next.is_some() {
            current_node = &mut node.next.as_mut().unwrap().next;
        } else {
            break;
        }
    }

    head
}
