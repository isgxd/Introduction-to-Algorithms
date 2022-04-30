//! 双向链表。

use std::ptr;

struct List {
    head: Link,
}

impl List {
    /// 创建一个新链表。
    fn new() -> Self {
        Self {
            head: ptr::null_mut(),
        }
    }

    /// 搜索指定键的节点。
    fn search(&self, key: i32) -> Option<&Node> {
        let mut x = self.head;
        unsafe {
            while !x.is_null() && (*x).key != key {
                x = (*x).next;
            }
            x.as_ref()
        }
    }

    /// 在顶端插入一个新节点。
    fn insert(&mut self, key: i32) {
        let mut x = Box::into_raw(Box::new(Node {
            key,
            prev: ptr::null_mut(),
            next: self.head,
        }));
        unsafe {
            if !self.head.is_null() {
                (*self.head).prev = x;
            }
            self.head = x;
            (*x).prev = ptr::null_mut();
        }
    }

    /// 删除指定节点。
    fn delete(&mut self, key: i32) -> Option<Node> {
        unsafe {
            let mut x = self.head;
            while !x.is_null() && (*x).key != key {
                x = (*x).next;
            }

            if x.is_null() {
                return None;
            }

            if (*x).prev.is_null() {
                self.head = (*x).next;
            } else {
                (*(*x).prev).next = (*x).next;
            }

            if !(*x).next.is_null() {
                (*(*x).next).prev = (*x).prev;
            }

            Some(*Box::from_raw(x))
        }
    }
}

type Link = *mut Node;

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
        assert_eq!(3, mid.unwrap().key);

        let n = list.delete(3);
        assert!(n.is_some());
        assert_eq!(3, n.unwrap().key);
        assert!(list.search(3).is_none());
    }
}
