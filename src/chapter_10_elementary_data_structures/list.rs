//! 双向链表。

use std::ptr::NonNull;

struct List {
    head: Link,
}

impl List {
    /// 创建一个新链表。
    fn new() -> Self {
        Self { head: None }
    }

    /// 搜索指定键的节点。
    fn search(&self, key: i32) -> Link {
        unsafe {
            let mut x = self.head;
            while let Some(n) = x {
                if key != (*n.as_ptr()).key {
                    x = (*n.as_ptr()).next;
                } else {
                    break;
                }
            }
            x
        }
    }

    /// 在顶端插入一个新节点。
    fn insert(&mut self, key: i32) {
        let x = Some(
            Box::leak(Box::new(Node {
                key,
                prev: None,
                next: self.head,
            }))
            .into(),
        );
        unsafe {
            if let Some(h) = self.head {
                (*h.as_ptr()).prev = x;
            }
        }
        self.head = x;
    }

    /// 删除指定节点。
    fn delete(&mut self, mut node: NonNull<Node>) {
        unsafe {
            let node = node.as_mut();
            if let Some(n) = node.prev {
                (*n.as_ptr()).next = node.next;
            } else {
                self.head = node.next;
            }

            if let Some(n) = node.next {
                (*n.as_ptr()).prev = node.prev;
            }
        }
    }
}

type Link = Option<NonNull<Node>>;

struct Node {
    key: i32,
    prev: Link,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();
        list.insert(5);
        list.insert(3);
        list.insert(8);

        let mid = list.search(3);
        assert!(mid.is_some());
        mid.map(|n| unsafe {
            assert_eq!(3, (*n.as_ptr()).key);

            list.delete(n);
            assert!(list.search(3).is_none());
        });
    }
}
