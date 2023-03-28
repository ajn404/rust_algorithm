#[derive(Default)]
struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        Self::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if let Some(head) = self.stack2.pop() {
            return head;
        }
        while let Some(tail) = self.stack1.pop() {
            self.stack2.push(tail);
        }
        self.stack2.pop().unwrap_or(-1)
    }
}
