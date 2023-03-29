//个人方案

// #[derive(Default)]
// struct MinStack {
//     a: Vec<i32>,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MinStack {
//     /** initialize your data structure here. */
//     fn new() -> Self {
//         Default::default()
//     }

//     fn push(&mut self, x: i32) {
//         self.a.push(x);
//     }

//     fn pop(&mut self) {
//         self.a.pop();
//     }

//     fn top(&self) -> i32 {
//         if self.a.len() > 0 {
//             return (self.a[0]);
//         }
//         -1
//     }

//     fn min(&self) -> i32 {
//         let mut min = -1;
//         if self.a.len() > 0 {
//             for &i in &self.a {
//                 if (i < min) {
//                     min = i;
//                 }
//             }
//         }
//         return min;
//     }
// }

//优化

struct MinStack {
    stack_a: Vec<i32>,
    stack_b: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        return MinStack {
            stack_a: vec![],
            stack_b: vec![],
        };
    }

    fn push(&mut self, x: i32) {
        self.stack_a.push(x);

        if self.stack_b.len() == 0 {
            self.stack_b.push(x);
        } else if self.min() >= x {
            self.stack_b.push(x);
        } else {
            self.stack_b.push(self.min());
        }
    }

    fn pop(&mut self) {
        self.stack_a.pop();
        self.stack_b.pop();
    }

    fn top(&self) -> i32 {
        let top: i32 = self.stack_a.last().unwrap().clone();
        top
    }

    fn min(&self) -> i32 {
        let min: i32 = self.stack_b.last().unwrap().clone();
        min
    }
}

/*
是一个 Rust 语言的栈数据结构实现，支持在常数时间内查找栈中的最小元素。

我们使用了两个栈：stack_a 和 stack_b。stack_a 存储栈中实际的元素，而 stack_b 存储任何时候栈顶的最小元素。

当一个新元素被推入栈时，如果 stack_b 为空，则新元素也被推入 stack_b。否则，如果新元素小于等于当前最小值，则它被推入 stack_b。否则，当前最小值被推入 stack_b。

当从栈中弹出元素时，同时从 stack_a 和 stack_b 弹出元素。

top() 方法返回 stack_a 的顶部元素。min() 方法返回 stack_b 的顶部元素，它始终是 stack_a 中的最小元素。

需要注意的是，这个实现假定在调用 min() 方法时栈不会为空。喵~ */
