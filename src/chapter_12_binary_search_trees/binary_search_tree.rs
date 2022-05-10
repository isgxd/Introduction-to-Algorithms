use std::{fmt::Display, ptr};

type Node = *mut TreeNode;
struct BinarySearchTree {
    head: Node,
}

impl Display for BinarySearchTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let head = self.inorder_walk();
        write!(f, "({head})")
    }
}

impl BinarySearchTree {
    fn new() -> Self {
        Self {
            head: ptr::null_mut(),
        }
    }

    /// 按中序遍历打印整棵树。
    fn inorder_walk(&self) -> String {
        let mut s = String::new();
        self.inorder_walk_impl(&mut s, self.head);
        s.trim_end().into()
    }

    fn inorder_walk_impl(&self, s: &mut String, node: Node) {
        unsafe {
            if !node.is_null() {
                self.inorder_walk_impl(s, (*node).left);
                s.push_str(&format!("{} ", (*node).key));
                self.inorder_walk_impl(s, (*node).right);
            }
        }
    }

    /// 搜索二叉树。
    fn iterative_search(&self, key: i32) -> Node {
        unsafe {
            let mut node = self.head;
            while !node.is_null() {
                if (*node).key == key {
                    return node;
                }
                node = if (*node).key > key {
                    (*node).left
                } else {
                    (*node).right
                }
            }
        }
        ptr::null_mut()
    }

    /// 获取指定节点的最小值（必定是左叶）。
    fn minimum(&self, node: Node) -> Node {
        let mut node = node;
        while !node.is_null() {
            node = unsafe { (*node).left };
        }
        node
    }

    /// 获取指定节点的后继（必定在右子树中）。
    fn successor(&self, mut node: Node) -> Node {
        unsafe {
            // 如果右子树非空，后继是右子树的最左节点。
            if !(*node).right.is_null() {
                return self.minimum((*node).right);
            }

            let mut y = (*node).parent;
            // 如果 node 是其父的左节点，退出循环。
            while !y.is_null() && *node == *((*y).right) {
                // 如果 node 是其父的右节点，向上寻找。
                node = y;
                y = (*y).parent;
            }
            y
        }
    }

    /// 插入节点并排序。
    fn insert(&mut self, key: i32) {
        unsafe {
            let mut trail = ptr::null_mut();
            let mut head = self.head;

            while !head.is_null() {
                trail = head;
                head = if key < (*head).key {
                    (*head).left
                } else {
                    (*head).right
                };
            }

            let mut node = TreeNode::new(key);
            node.parent = trail;

            let node = Box::leak(node);
            if trail.is_null() {
                self.head = node;
            } else if key < (*trail).key {
                (*trail).left = node;
            } else {
                (*trail).right = node;
            }
        }
    }

    /// 用 v 代替 u。
    fn transplant(&mut self, u: Node, v: Node) {
        unsafe {
            // u 是根结点。
            if (*u).parent.is_null() {
                self.head = v;
            } else if u == (*(*u).parent).left {
                // u 是其父的左节点。
                (*(*u).parent).left = v;
            } else {
                // u 是其父的右节点。
                (*(*u).parent).right = v;
            }
            if !v.is_null() {
                (*v).parent = (*u).parent;
            }
        }
    }

    /// 删除指定节点。
    fn delete(&mut self, z: Node) {
        unsafe {
            if (*z).left.is_null() {
                self.transplant(z, (*z).right);
            } else if (*z).right.is_null() {
                self.transplant(z, (*z).left);
            } else {
                let y = self.minimum((*z).right);
                if (*y).parent != z {
                    self.transplant(y, (*y).right);
                    (*y).right = (*z).right;
                    (*(*y).right).parent = y;
                }
                self.transplant(z, y);
                (*y).left = (*z).left;
                (*(*y).left).parent = y;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct TreeNode {
    key: i32,
    left: Node,
    right: Node,
    parent: Node,
}

impl TreeNode {
    fn new(key: i32) -> Box<Self> {
        Box::new(Self {
            key,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tree = BinarySearchTree::new();
        tree.insert(3);
        tree.insert(5);
        tree.insert(2);
        println!("{tree}");

        let n = tree.iterative_search(2);
        assert!(!n.is_null());

        tree.delete(n);
        println!("{tree}");
    }
}
