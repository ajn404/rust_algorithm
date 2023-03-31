#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// 1->2->3->4->5->NULL
//5->4->3->2->1->NULL
pub fn _reverse_print(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // if head.is_none() {
    //     return None;
    // }

    let mut prev = None;
    let mut current = head;
    while let Some(mut tmp) = current.take() {
        let next = tmp.next.take();
        tmp.next = prev;
        prev = Some(tmp);
        current = next;
    }
    prev
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;

    while cur.is_some() {
        let next = cur.as_mut().unwrap().next.take();
        cur.as_mut().unwrap().next = pre;
        pre = cur;
        cur = next;
    }

    pre
}
