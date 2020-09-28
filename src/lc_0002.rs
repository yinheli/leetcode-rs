use std::fmt::{Display, Formatter, Result};

// https://leetcode.com/problems/add-two-numbers/

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        } else if l2.is_none() {
            return l1;
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut p = l1;
        let mut q = l2;
        let mut current = dummy.as_mut();
        let mut carry = 0;

        while p.is_some() || q.is_some() {
            let mut sum = carry;

            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }

            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }

            carry = sum / 10;

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = node.next.as_mut();
            }
        }

        if carry > 0 {
            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(carry)));
            }
        }

        dummy.unwrap().next
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = write!(f, "({}", self.val);
        let mut next = &self.next;
        while let Some(v) = next {
            let _ = write!(f, " -> {}", v.val);
            next = &v.next;
        }
        write!(f, ")")
    }
}

#[test]
fn test_add_two_numbers_1() {
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    };

    let l2 = ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    println!("input: {} + {}", l1, l2);
    let result = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))).unwrap();
    println!("output:{}", result);
    assert_eq!("(7 -> 0 -> 8)", format!("{}", result));

    // -----------

    let l1 = ListNode::new(5);
    let l2 = ListNode::new(5);

    println!("input: {} + {}", l1, l2);
    let result = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))).unwrap();
    println!("output:{}", result);
    assert_eq!("(0 -> 1)", format!("{}", result));
}
