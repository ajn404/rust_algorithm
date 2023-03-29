// Definition for singly-linked list.
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
pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    let mut p = &head;
    while let Some(node) = p {
        res.push(node.val);
        p = &node.next;
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {

    use crate::util;

    use super::*;
    #[test]
    fn test() {
        let l = ListNode::new(12);
        let head = Some(Box::new(l));

        reverse_print(head);
    }
}
