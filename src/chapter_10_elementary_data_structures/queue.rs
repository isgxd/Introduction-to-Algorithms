//! 队列。

struct Queue {
    s: Vec<i32>,
    head: usize,
    tail: usize,
}

impl Queue {
    /// 创建一个环形队列。
    fn new(len: usize) -> Self {
        Self {
            s: vec![0; len],
            head: 0,
            tail: 0,
        }
    }

    /// 将指定元素加入队列末尾。
    fn enqueue(&mut self, item: i32) {
        if self.tail == self.s.capacity() && self.head == 0 {
            panic!("队列溢出");
        }

        // 如果队列已达末尾，则考虑从开头添加元素。
        if self.tail == self.s.capacity() {
            self.s[0] = item;
            // 如果队列元素全部弹出，则初始化队列的头指针。
            if self.head == self.tail {
                self.head = 0;
            }
            self.tail = 0;
        } else {
            let i = self.tail;
            self.s[i] = item;
        }
        self.tail += 1;
    }

    /// 从队列头部取出一个元素。
    fn dequeue(&mut self) -> Option<i32> {
        if self.head == self.tail {
            return None;
        }

        let value = self.s[self.head];
        self.head += 1;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut q = Queue::new(2);
        q.enqueue(2);
        assert_eq!(0, q.head);
        assert_eq!(1, q.tail);
        assert_eq!(&[2, 0], &q.s[..]);

        q.enqueue(5);
        assert_eq!(0, q.head);
        assert_eq!(2, q.tail);
        assert_eq!(&[2, 5], &q.s[..]);

        let v = q.dequeue();
        assert_eq!(Some(2), v);
        assert_eq!(1, q.head);
        assert_eq!(2, q.tail);
        assert_eq!(&[2, 5], &q.s[..]);

        let v = q.dequeue();
        assert_eq!(Some(5), v);
        assert_eq!(2, q.head);
        assert_eq!(2, q.tail);
        assert_eq!(&[2, 5], &q.s[..]);

        q.enqueue(8);
        assert_eq!(0, q.head);
        assert_eq!(1, q.tail);
        assert_eq!(&[8, 5], &q.s[..]);
    }
}
