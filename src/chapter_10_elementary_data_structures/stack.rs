//! 栈。

struct Stack {
    s: Vec<i32>,
    top: usize,
}

impl Stack {
    /// 创建一个新的空栈。
    fn new() -> Self {
        Self { s: vec![], top: 0 }
    }

    /// 栈是否为空。
    fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// 在栈顶压入一个元素。
    fn push(&mut self, x: i32) {
        self.top += 1;
        self.s.push(x);
    }

    /// 从栈顶弹出一个元素。
    fn pop(&mut self) -> Option<i32> {
        let item = self.s.pop();
        if item.is_some() {
            self.top -= 1;
        }
        item
    }
}
